# 023 — Deterministically verifying Meilisearch indexing from Rust (task model, SDK waits, no fixed sleeps)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` — **research-only** (this document only; no source changes outside this doc, no commits).  
**Version/context:** Flora pins `getmeili/meilisearch:v1.12` in `docker-compose.yml` (profile `search`, localhost `7700`, `MEILI_MASTER_KEY=dev-meilisearch-master-key`, `MEILI_MAX_INDEXING_MEMORY=256Mb`, `MEILI_MAX_INDEXING_THREADS=1`, healthcheck `GET /health`). Rust SDK is `meilisearch-sdk = "0.33"` in root `Cargo.toml`, resolved to **0.33.0** in `Cargo.lock` (published 2026-09-23 per docs.rs). Server docs cited are current v1 docs; exact flag/status names can differ between v1 minors — see NV-1.  
**Audience/target:** Seed profile (~2 vCPU / 4 GB), Postgres-first, optional Meilisearch; cycle 023 planning.

---

## 1. Scope and method

**Scope.** (a) Meilisearch's task / asynchronous operation model and what "indexed" actually means; (b) the exact `meilisearch_sdk` 0.33 APIs for task handles, `task_uid`, `wait_for_task`, `wait_for_completion`, `get_task`, task status, settings vs document-addition tasks; (c) batching / eventual-consistency semantics; (d) patterns for asserting "the document is searchable now" without fixed sleeps; (e) a Flora cross-check and an 80/20 recommendation for a small verifiable slice in cycle 023.

**Method (high-reliability, multilingual).**
- **Primary sources first:** official Meilisearch documentation (async operations, working with tasks, task API, Rust quick start), the **exact pinned SDK source** on docs.rs (`task_info.rs`, `client.rs`, `tasks.rs`, `Task` enum page), and the official Meilisearch SDK-maintainer discussion (integration-guides issue #243).
- **Engineering references:** practitioner write-ups on flaky search tests caused by async indexing (supun.io EN + FR translation; huggy.moe zh-CN), plus code-graph summaries (DeepWiki) used only as orientation, never as normative evidence.
- **Flora cross-check:** repository code/config/workflows and prior research docs (see §9, `[F]` citations).
- **Multilingual sweep:** targeted queries for EN, pt-BR, es, de, fr, zh-CN, ja via DuckDuckGo region endpoints (`websearch` returned no results in this run; DDG HTML was used as the discovery layer, then primary pages were fetched directly). Findings are reported in §5 with an explicit confidence label; **normative behavior is anchored to English primary docs and Rust SDK source** because the authoritative specifications are English-first. Environment-/version-/corpus-dependent claims are marked **NV**.

---

## 2. Executive summary

1. **A successful write is only an enqueue.** `add_documents`, `set_filterable_attributes`, and `delete_document` are asynchronous; the HTTP 202 response carries a summarized task object (`taskUid`, status always `enqueued`). The document becomes searchable only when the task reaches `succeeded`. ([M1], [M2], [M3])
2. **The SDK already gives Flora everything needed to wait deterministically.** `TaskInfo::wait_for_completion(&client, interval, timeout)` and `Client::wait_for_task(task_uid, interval, timeout)` poll `GET /tasks/{uid}` with defaults **50 ms / 5000 ms**, returning `Task`. No `sleep()` in test code is required. ([S1], [S2], [S3])
3. **Two sharp edges in SDK 0.33 that change the helper design:**
   - `wait_for_task` returns `Ok(Task::Failed { .. })` on failure — it does **not** return `Err`. Callers **must** inspect `task.is_success()` / `task.is_failure()`; `unwrap_failure()` exists but **panics** on a non-failed task. ([S2], [S3])
   - The `Task` enum has only `Enqueued | Processing | Failed | Succeeded`. The server also documents a terminal `canceled` status, which this SDK cannot represent — a canceled task may surface as a deserialization error rather than a handled variant. Low risk for small tests that never cancel; record as NV. ([S3] vs [M1])
4. **Settings and documents are different tasks and cannot share a batch.** `set_filterable_attributes` triggers a `settingsUpdate` task that rebuilds the index; a filter referencing `type` can fail until that task succeeds. A deterministic test must wait the setup task **and** the document task (or avoid filters in the first slice). ([M1], [M4])
5. **Flora currently discards every task handle** (`let _task = ...`) in `flora-search/src/lib.rs`, so no component can await completion. This is the single root cause of nondeterministic Meilisearch assertions. ([F1])
6. **80/20 for 023:** add a **minimal awaitable wrapper** in `flora-search` (return `TaskInfo`/`task_uid` or add `*_and_wait` helpers that map SDK `Task` → `flora_core::Result`), then add **one doubly-gated (DB + Meili) router contract test** that indexes a document, waits for success, and asserts search visibility. An **injectable fake adapter** is a larger refactor with lower fidelity; **PG-only** is cheapest but leaves the Meilisearch path unverified. Recommendation: **awaitable wrapper + 1–2 gated tests**; keep default `cargo test` free of Meilisearch.

---

## 3. Flora baseline cross-check (code)

| Area | Evidence | Baseline / gap |
|---|---|---|
| SDK version | `Cargo.toml:174` `meilisearch-sdk = "0.33"`; `Cargo.lock:2087-2088` `0.33.0` | Pin is recent (2026-09-23). API details below are read from the exact 0.33.0 source, not from `latest` docs. |
| Adapter | `flora-search/src/lib.rs:65-72` (`setup_index`), `:79-100` (`index_message`), `:107-124` (`index_task`), `:131-147` (`index_file`), `:154-162` (`delete_document`) | All five methods `await` the request but bind the result to `_task` and drop it. `search()` (`:169-226`) executes immediately with no visibility barrier. |
| Index settings | `setup_index` sets `filterable_attributes(["type","created_at_timestamp"])` — one `settingsUpdate` task; not called from any mutation path (prior 022 audit [F5]) | If a test uses `?type=...`, it depends on that settings task having completed; Flora never waits it today. |
| Config | `flora-core/src/config/search.rs`: `SearchConfig { url, api_key, default_limit, max_limit, index_template, enabled }`; defaults read `MEILISEARCH_URL` (fallback `http://localhost:7700`) / `MEILISEARCH_API_KEY`; `index_template = "flora_org_{org_id}"`; `enabled = true` | Separate Meili gate needed (env + `#[ignore]`), independent of DB gate. |
| State | `flora-api/src/state/app_state.rs:128-146`: `SearchService::new(...)` only when `config.search.enabled`; failure → `None` + warning | SDK `Client::new` performs **no network call** (SDK source [S2]), so a dead Meilisearch still yields `Some(SearchService)` at boot. Query-time error fallback (022) is what actually protects reads. |
| Route | `flora-api/src/routes/search.rs:90-115`: Meilisearch first; on `Err` logs and falls through to `search_pg` (022) | Black-box tests cannot tell which backend answered unless fixtures differ between PG and Meili. A doc indexed **only** into Meili (no matching PG row) is a clean precedence signal. |
| Test harness | `flora-tests/src/contract/search.rs`: `db_state()` (env `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`), `make_test_state()` sets `search_service: None`, `search_app()`, `authed_get_json()`, `#[tokio::test] #[ignore]`, unique UUIDv7 data | Ready to extend with `make_test_state_with_search(...)` + `db_meili_*` tests; keep unique per-test org → unique index `flora_org_{org_id}`. |
| Prior work | 022 harness/indexing/coherence research [F5], [F6], [F7] | Converges on: PG baseline; task-awaitable harness is the enabler; avoid fixed sleeps; no Meili-gated tests exist yet. |

---

## 4. Primary-source findings

### 4.1 The asynchronous task model (what "indexed" means)

From the official "Tasks and asynchronous operations" page [M1] and "Working with tasks" [M2]:

- Asynchronous operations include **creating/updating/deleting indexes, updating settings, adding/updating/deleting documents, and canceling/deleting tasks**. Each API call creates a task placed in a **persistent queue** processed in enqueue order.
- Every POST/PUT that starts an async operation returns a **summarized task object**, not the full one:

  ```json
  { "taskUid": 0, "indexUid": "movies", "status": "enqueued",
    "type": "documentAdditionOrUpdate", "enqueuedAt": "2021-08-11T09:25:53.000000Z" }
  ```

  It is **confirmation of acceptance, not of indexing**. Use `taskUid` with `GET /tasks/{task_uid}` [M3].
- Statuses: `enqueued`, `processing` (unfinished) and `succeeded`, `failed`, `canceled` (finished). `failed` makes **no changes** to the database. Task objects carry `details` (e.g. `receivedDocuments`, `indexedDocuments` — `null` until finished) and an `error` object on failure.
- Terminating the instance mid-task is safe: `processing` tasks reset to `enqueued` on restart. (Useful context, not a test strategy.)
- **Auto-batching** [M1]: compatible consecutive tasks are grouped only when they target the **same index**, are the **same task type**, and use the **same content type**. Ordering is preserved. **A settings update and a document addition cannot share a batch.** Therefore waiting on *each* relevant `taskUid` is the precise barrier; "wait for one task" does not imply "all earlier settings tasks are applied" unless you ordered the waits accordingly.
- Counts like `indexedDocuments` remain `null` while unfinished — don't assert them before `succeeded`.

### 4.2 Rust SDK 0.33: the exact awaitable surface

Read from the pinned source [S1] `task_info.rs`, [S2] `client.rs`, [S3] `tasks.rs` / `Task` enum page:

| Item | Signature / shape (0.33.0) | Notes for Flora |
|---|---|---|
| `TaskInfo` | `struct TaskInfo { enqueued_at, index_uid: Option<String>, status: String, #[serde(flatten)] update_type: TaskType, task_uid: u32 }`; `get_task_uid() -> u32`; `impl AsRef<u32>` | This is what `add_documents`, `set_filterable_attributes`, `delete_document`, `create_index`, etc. return. Capture `task_uid` **before** consuming the handle if you want diagnostics. |
| `Client::wait_for_task` | `async fn wait_for_task(&self, task_id: impl AsRef<u32>, interval: Option<Duration>, timeout: Option<Duration>) -> Result<Task, Error>` | Defaults **50 ms / 5000 ms**. Loop: `get_task(&task_id)`; on `Failed` or `Succeeded` → `return self.get_task(task_id).await` (fresh fetch); on `Enqueued`/`Processing` → add `interval` to elapsed and `sleep_backend().sleep(interval).await`; on transport error → `Err`; after timeout → `Err(Error::Timeout)`. It **does not error on a failed task**. |
| `TaskInfo::wait_for_completion` | `async fn wait_for_completion(self, &Client, interval, timeout) -> Result<Task, Error>` → delegates to `client.wait_for_task(self, ...)` | The canonical one-liner used in official examples: `add_documents(...).await?.wait_for_completion(&client, None, None).await?; assert!(matches!(task, Task::Succeeded { .. }));` ([S1], [S4]) |
| `Task::wait_for_completion` | Same shape, `Result<Self, Error>` | For when you already have a full `Task` (e.g. from `get_task`). |
| `Index::wait_for_task` | Referenced in SDK docs alongside the above | Equivalent convenience on an `Index`; `TaskInfo::wait_for_completion` is the simplest path from a write call. |
| `Client::get_task` | `async fn get_task(&self, task_id: impl AsRef<u32>) -> Result<Task, Error>` (`GET /tasks/{uid}`) | Accepts `TaskInfo`, `Task` or a `u32` ref thanks to `AsRef<u32>`. Useful for custom polling / diagnostics. |
| `Client::get_tasks_with` | `TasksSearchQuery` with `with_index_uids(...)`, `with_statuses(...)`, `with_uids(...)`, `with_types(...)` | Enables **queue-drain** checks scoped to one index without owning the task handles (see §6-A0). |
| `Task` enum | `Enqueued \| Processing \| Failed \| Succeeded` | No `Canceled` variant in 0.33 (server has `canceled` — see risk R3). |
| Helpers | `is_success()`, `is_failure()`, `is_pending()`, `get_uid()`, `unwrap_failure() -> MeilisearchError` | `unwrap_failure()` **panics** if the task is not `Failed`; prefer `is_failure()` + match `Task::Failed { content } => content.error` for error reporting (also friendlier to Flora's strict lints). |
| `SucceededTask` | `duration`, `enqueued_at`, `started_at`, `finished_at`, `index_uid`, `error`, `update_type`, `uid` | `update_type` lets a test assert the task was really `DocumentAdditionOrUpdate` / `SettingsUpdate`. |
| `Error::Timeout` | Returned only when elapsed exceeds `timeout` | Bump timeout above 5 s in gated CI under load (maintainers explicitly warn the 5 s default is a dev-scale assumption [E2]). |

**Consequence for Flora's helper:** a correct wrapper is not just `.wait_for_completion(...).await?` — it must convert a **failed task into a Flora error**, e.g.:

```rust
// sketch — not applied (research only)
let task = info.wait_for_completion(&client, Some(interval), Some(timeout)).await
    .map_err(|e| Error::Search(e.to_string()))?;
if !task.is_success() {
    return Err(Error::Search(format!("Meilisearch task {} did not succeed: {task:?}", uid)));
}
Ok(())
```

Because the SDK uses an **async sleep**, this does not block the Axum runtime thread; it only defers the awaiting future ([E2], comment by SDK maintainer `sanders41`).

### 4.3 Settings vs document tasks, batching, eventual consistency

- `set_filterable_attributes` → `settingsUpdate` task; Meilisearch **rebuilds the index** on that setting change; the Rust SDK README notes it may take time and recommends tracking via tasks ([S5]). Flora's `setup_index` is fire-and-forget today.
- `add_documents(&[doc], Some("id"))` → `documentAdditionOrUpdate`; `id` is the primary key (upsert semantics). A single-document add is typically fast on Seed-sized corpora, but still asynchronous.
- `delete_document` → `documentDeletion`; same waiting requirement if a test asserts disappearance.
- Because settings and document tasks cannot batch [M1], a filter-dependent assertion should wait in this order: (1) settings task succeeded, (2) document task succeeded, (3) search. The first slice can simply **omit the `type` filter**, removing step (1).
- Polling `GET /tasks/{uid}` is request-per-check; Meilisearch also documents experimental streaming routes (`/tasks/stream`, `/batches/stream`, behind `tasksStreamingRoute`) and **task webhooks** for completion notifications [M2], [E5]. Both are useful in production sync designs, **overkill for a small deterministic test**, and version/flag-sensitive for v1.12 (NV-1).

### 4.4 Patterns for "document is searchable now" without fixed sleeps

| Pattern | How | Pros | Cons / caveats |
|---|---|---|---|
| **P1 — Per-task wait (preferred)** | Keep the `TaskInfo`; `wait_for_completion(interval, timeout)`; assert `is_success()`; then search | Precise, no sleeping in test code, surfaces failed tasks | Requires access to the handle → Flora must stop dropping it. Handle settings tasks too if filtering. |
| **P2 — Bounded poll of search itself** | Loop `search()` until expected id appears or deadline; small backoff; fail on deadline | Backend-agnostic; also works if you cannot get the handle | Can mask indexing failures; must cap retries; still a (bounded, event-driven) loop, not a `sleep(N)` guess. Good as an **assertion amplifier** after P1. |
| **P3 — Queue drain scoped by index** | `get_tasks_with(TasksSearchQuery::with_index_uids([idx]).with_statuses(["enqueued","processing"]))` until empty | Works when handles were discarded (today's Flora) — no adapter change needed | Waits for unrelated tasks on the same index; races with parallel tests; needs a second SDK `Client` in the test with matching URL/key. Serial + unique index mitigates. ([S2], [E1]) |
| **P4 — Webhook / streaming completion** | Register task webhook or stream status changes | Production-grade decoupling | Callback plumbing, experimental flags, version risk; not for a first deterministic test. ([M2], [E5]) |
| **P5 — Fixed `sleep()` / `usleep()`** | `sleep(150ms)` before querying | One line | **Rejected.** Explicitly called out as producing flaky tests by practitioners in EN and FR [E1]/[E1-FR] and by huggy.moe with a bounded retry instead [E3]. |

**Recommended shape:** **P1 as the barrier + P2 as the final assertion** (tiny retry cap, e.g. up to ~10 × 50 ms) — deterministic by task status, resilient to any residual visibility lag, and still bounded.

---

## 5. Multilingual findings (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale | Finding (with source) | Confidence / NV |
|---|---|---|
| **EN** | Primary specs are English: task queue, statuses, summaries, batching [M1]; tutorial shows `taskUid` → `GET /tasks/{uid}` and "when `status` is `succeeded`, Meilisearch has finished processing" [M2]; SDK source gives exact wait semantics [S1]–[S3]; maintainers warn 5 s default timeout is dev-scale and advise against `wait_for_task` in production request paths [E2]. Practitioner report: async document handling made search endpoint tests intermittent; `sleep()` → flaky; polling `/tasks` until no `enqueued`/`processing` works [E1]. | **High** (normative). |
| **pt-BR** | Targeted DDG-BR queries (`tarefas assíncronas`, `fila de tarefas`, `índice`, `wait_for_completion`) surfaced the same EN primary pages (Meilisearch docs, docs.rs, meilisearch-rust); no authoritative pt-BR primary page on task waiting was found in this run. Operational reading: tratar "indexar" como operação assíncrona; o teste deve esperar a tarefa atingir `succeeded`, nunca assumir visibilidade imediata após `add_documents`. | **Medium** (translation of EN primary), no pt-BR-specific normative source — **NV-6**. |
| **es** | DDG-ES queries (`tareas asíncronas`, `cola de tareas`, `esperar a que termine`) likewise returned EN primary docs plus a generic async-wait StackOverflow question, not a Meilisearch-es authority. Reading: una respuesta 202 del API significa "encolado"; la aserción debe esperar a que el task pase a `succeeded` (o fallar con timeout acotado). | **Medium** (translation of EN primary) — **NV-6**. |
| **de** | DDG-DE queries (`asynchrone Aufgaben`, `Task-Warteschlange`, `abwarten`) returned EN primary docs plus DeepWiki code-graph pages orientation only. Reading: `wait_for_task` ist ein Poll auf `GET /tasks/{uid}` mit 50 ms/5 s Defaults; `Failed` kommt als `Ok(Task::Failed)` zurück und muss explizit geprüft werden. | **Medium** (translation of EN primary) — **NV-6**. |
| **fr** | Genuine local-language engineering source: supun.io FR translation "Une façon inefficace d'attendre que Meilisearch termine toutes les tâches asynchrones" [E1-FR] — states `sleep()`/`usleep()` gives un **test bancal** (flaky), and shows polling `/tasks` while any task is `enqueued`/`processing` (150 ms between calls). | **High** for the practitioner claim; not a normative spec. |
| **zh-CN** | Official-docs mirror (unofficial site, clearly labeled) "任务和异步操作" [M1-ZH] reproduces the statuses `enqueued/processing/succeeded/failed/canceled`, `taskUid`, and "按顺序逐一处理". Practitioner post huggy.moe [E3] observed 0.25.0 task API covering indexCreation/settingsUpdate/documentAddition/…, noted real-time visibility is lost, and implemented a **bounded** wait (20 retries, 3 s) checking `status: succeeded` before continuing. | **Medium-High** (mirror + practitioner); mirror is unofficial — **NV-1/NV-6**. |
| **ja** | Qiita self-hosted Meilisearch v1.13 notes [E6] describe per-index settings binding ("各インデックスに検索の設定が紐づいている") and that replace-style index updates require reapplying settings — consistent with treating settings as a separate async task to wait. DDG-JP also surfaced EN docs.rs `Task` pages; no ja primary spec on waits found. | **Medium** (operational, not task-wait normative) — **NV-6**. |
| **Cross-locale** | The determinism lesson is **language-independent** because it is a protocol property (async task queue + terminal status), not a tokenization property. Locale-specific relevance/tokenization (pt-BR accents, es/fr diacritics, de compounds, CJK segmentation) affects *what* results rank, not *when* a document becomes searchable — keep those as separate NVs (cf. [F8]). | **High** for determinism; **NV** for ranking parity. |

**Coverage note:** EN primary sources carry the normative weight; fr/zh-CN provide independent practitioner corroboration; pt-BR/es/de/ja queries were executed and surfaced primarily English primary pages or operational (non-normative) articles. This is an honest coverage boundary, not a claim that local-language sources do not exist.

---

## 6. 80/20 options for cycle 023

### A — Minimal awaitable wrapper in `flora-search` (recommended enabler)

Add the smallest API change that preserves a task handle and maps SDK status to `flora_core::Result`:

- Change `setup_index`, `index_message`, `index_task`, `index_file`, `delete_document` to either **return `TaskInfo`** (or just `u32` task uid) **or** add parallel `*_and_wait(org_id, ..., interval, timeout) -> Result<()>` methods while keeping the existing fire-and-forget methods for production latency.
- Internally use `TaskInfo::wait_for_completion` + `is_success()` check (§4.2); on failure include `task_uid`, `index_uid`, and the `MeilisearchError` fields in `Error::Search`.
- Test side: `flora-tests` builds a second SDK `Client` from env for setup/teardown only if needed; the gated test uses the new awaitable method for its own writes.

**Effort:** low–medium. **Fidelity:** high (exact task). **Risk:** small public API change in one crate; no behavior change for existing callers if additive. **Verdict: best 80/20.**

**A0 — zero-change variant (if 023 must not touch `flora-search`):** in the gated test, construct a separate `meilisearch_sdk::Client` from `MEILISEARCH_URL`/key and use `get_tasks_with(...with_index_uids([unique_index]).with_statuses(["enqueued","processing"]))` to drain the queue (P3), then assert via search. Needs unique per-test index + serial execution to avoid cross-test waits. Lower precision, but **no adapter change** — useful as a bridge or as a fallback if wrapper work slips.

### B — Fake / injectable search adapter

Introduce a `Search` trait (`search`, `index_*`, `delete`, `setup`) and store `Arc<dyn Search>` in `AppState`; route/tests inject an in-memory fake.

**Effort:** medium–high (touches `AppState`, `flora-search`, route generics, every state construction site). **Fidelity:** high for *path selection / precedence / fallback* unit tests, **zero** for verifying actual Meilisearch indexing (the fake is not the engine). **Verdict:** valuable later for cheap precedence/degradation tests, but it does **not** answer the 023 question ("is the document searchable now?") and costs more than A. Defer.

### C — PG-only (no Meilisearch-gated tests)

Declare PG FTS authoritative and keep Meilisearch best-effort; add no Meili tests in 023, rely on existing DB-gated fallback contracts [F4].

**Effort:** lowest. **Fidelity:** none for the Meili path. **Verdict:** correct as the *default profile* and as a risk control, but it leaves the optional profile's core promise untested; choose only if Meilisearch is effectively deprecated for Seed. If chosen, make the degradation contract explicit (Meili enabled ≠ Meili authoritative) per prior 022 work [F7].

### Comparison

| Option | Effort | Answers "searchable now?" | Default `cargo test` impact | CI cost | 023 verdict |
|---|---:|---|---|---|---|
| A (awaitable wrapper) | Low–Med | **Yes**, per task | None (gated tests only) | Low (one opt-in service) | **Recommend** |
| A0 (queue drain, no source change) | Low | Indirectly (queue-level) | None | Low | Bridge / fallback |
| B (fake adapter) | Med–High | No (fake ≠ engine) | None | Very low | Defer |
| C (PG-only) | Lowest | No | None | None | Keep as default profile, not as the whole answer |

---

## 7. Risks and NVs

| ID | Risk / unknown | Mitigation / verification |
|---|---|---|
| R1 | `wait_for_task` returns `Ok(Task::Failed)` — a naive `.await?` test would **pass** after failed indexing. | Wrapper/diagnostic must assert `is_success()` (or `matches!(Task::Succeeded{..})`) — non-negotiable. [S2], [S3] |
| R2 | Default 5 s timeout can be flaky on loaded CI (v1.12 + Postgres + API on 2 vCPU). | Pass explicit `timeout` (e.g. 15–30 s) and small `interval` (10–50 ms); fail with uid/status context. [E2] |
| R3 | SDK `Task` lacks `Canceled` while server documents `canceled` → possible deserialization error. | Don't cancel tasks in tests; treat such an error as NV; check SDK upgrade if it appears. [S3] vs [M1] |
| R4 | Filterable-attributes task not finished → `?type=...` filter fails or yields wrong results. | Wait the settings task too, or use a filter-free first test. [M1], [S5] |
| R5 | Queue-drain (A0) waits on unrelated tasks / races with parallel tests. | Unique org→index per test, `--test-threads=1` for gated suite, scope query by `index_uids`. |
| R6 | Query-error fallback (022) can make a broken Meili path look green via PG. | Fixture with a document indexed **only** in Meili (no PG row) so a hit proves the Meili path. |
| R7 | SDK 0.33 (Sep 2026) vs server v1.12: field/status drift (e.g. `remotes`, streaming flags). | Pin both; smoke-test against the exact compose image; avoid experimental endpoints. |
| R8 | `TaskInfo`/`Task` are moved into `wait_for_completion`; diagnostics lost. | Capture `task_uid = info.get_task_uid()` before consuming. |

**NV-1:** Exact v1.12 behavior of current docs' flags/streaming/`remotes`; validate against the pinned image.  
**NV-2:** Safe timeout/interval values for 2 vCPU/4 GB CI with co-located Postgres+API+Meili.  
**NV-3:** Whether production indexing should ever await tasks (latency vs freshness) — maintainers advise against blocking request paths; tests are a different matter. [E2]  
**NV-4:** Whether a filter-free first test is sufficient acceptance, or whether `?type=` parity must be in the first slice.  
**NV-5:** Meilisearch API key/URL plumbing for `flora-tests` (env names, key from `MEILI_MASTER_KEY`) and CI service wiring.  
**NV-6:** Absence of authoritative local-language (pt-BR/es/de/ja) primary sources in this run; findings there are translations of EN specs.  
**NV-7:** Whether canceled tasks are ever observable in Flora's normal (non-canceling) flow.

---

## 8. Concrete recommendation — small verifiable slice for 023

**Adopt A (minimal awaitable wrapper) + two doubly-gated tests; keep C as the default profile. Do not build B in 023.**

1. **Enabler (`flora-search`, additive):**
   - Add `pub async fn index_message_and_wait(...)` (or a generic private `wait_task(info, interval, timeout) -> Result<()>` used by new `*_and_wait` methods) covering **at least** `setup_index` and `index_message`. Optional: also `index_task`/`index_file`/`delete_document` for symmetry.
   - Convert `Task::Failed`/timeout into `Error::Search` with `task_uid`, `index_uid`, and error message; assert `is_success()`. No panics; no `.unwrap()`.
   - Keep existing fire-and-forget methods unchanged so production latency semantics are untouched.
2. **Harness (`flora-tests/src/contract/search.rs`):**
   - `meili_state_from_env() -> Option<(Config, AppState)>` gated on e.g. `FLORA_SEARCH__URL`/`MEILISEARCH_URL` (+ key); tests are `#[tokio::test] #[ignore = "requires Postgres + Meilisearch; ..."]` and return early when unset.
   - Extend `make_test_state` with a variant that sets `search_service: Some(Arc::new(SearchService::new(url, key, &unique_template)?))`, where `unique_template` embeds a UUID so each test gets its own `flora_org_{org_id}` index.
   - Reuse `search_app` / `authed_get_json` / `seed_member` from the existing DB-gated harness.
3. **Tests (2, not a matrix):**
   - `db_meili_indexed_message_is_searchable`: `setup_index_and_wait` → `index_message_and_wait` → `GET /search?q=<unique token>` → assert `{id,type:"message"}` present. Because no PG row with that token exists, a hit proves the Meilisearch path (also covers precedence).
   - `db_meili_failed_task_surfaces_as_error` (unit-ish, optional second): against an index without the required primary key / malformed doc, assert the wrapper returns `Error::Search` rather than Ok — locks in R1. Alternatively use the first test's setup failure path.
   - Defer filter parity, delete/soft-delete, and full cross-engine parity to a later cycle; do not assert ranking.
4. **CI:** opt-in `meili-tests` job (or local-only runbook first) with pinned `getmeili/meilisearch:v1.12` + `postgres:18` services; run `cargo test -p flora-tests contract::search::db_meili_ -- --ignored --test-threads=1`. Default `build`/`db-tests` jobs unchanged.
5. **Acceptance checklist:** no fixed sleeps; every write barrier ends in `Task::Succeeded`; failures/timeouts produce actionable errors; unique index per test; gated tests skip cleanly; default suite does not require Meilisearch.

**If A cannot land in 023:** use **A0** (test-owned `Client` + index-scoped queue drain) for a single visibility test and track the wrapper as 024 — still deterministic, slightly less precise. Do **not** substitute fixed sleeps.

---

## 9. Sources and citations

### Primary — Meilisearch documentation
- **[M1]** Tasks and asynchronous operations (async op list, summarized task, statuses, queue, auto-batching rules, settings vs documents): https://www.meilisearch.com/docs/learn/async/asynchronous_operations (fetched 2026-11-16; current path also https://www.meilisearch.com/docs/capabilities/indexing/tasks_and_batches/async_operations)
- **[M2]** Working with tasks (`taskUid` → `GET /tasks/{uid}`, "succeeded = finished", streaming routes, webhooks, `error` object): https://www.meilisearch.com/docs/learn/async/working_with_tasks (fetched 2026-11-16)
- **[M3]** Task API reference (get one task / list tasks): https://www.meilisearch.com/docs/reference/api/tasks/get-task , https://www.meilisearch.com/docs/reference/api/async-task-management/list-tasks
- **[M4]** Index settings / filterable attributes: https://www.meilisearch.com/docs/learn/configuration/configuring_index_settings
- **[M1-ZH]** zh-CN docs mirror, "任务和异步操作" (unofficial mirror, clearly labeled): https://meilisearch.com.cn/docs/learn/async/asynchronous_operations (fetched 2026-11-16)
- **[S4]** Rust quick start (official example: `add_documents(...).wait_for_completion(&client, None, None)` before search; note page pins SDK 0.27 → NV): https://www.meilisearch.com/docs/getting_started/sdks/rust

### Primary — Rust SDK 0.33.0 source/docs (exact pinned version)
- **[S1]** `task_info.rs` — `TaskInfo`, `get_task_uid()`, `wait_for_completion` (50 ms/5 s, `Error::Timeout`), example assertion `matches!(status, Task::Succeeded { .. })`: https://docs.rs/meilisearch-sdk/0.33.0/src/meilisearch_sdk/task_info.rs.html
- **[S2]** `client.rs` — `Client::wait_for_task` implementation, `get_task`, `get_tasks_with`, `Client::new` makes no request: https://docs.rs/meilisearch-sdk/0.33.0/src/meilisearch_sdk/client.rs.html
- **[S3]** `tasks.rs` + `Task` enum page — 4 variants, `is_success/is_failure/unwrap_failure`, `SucceededTask` fields: https://docs.rs/meilisearch-sdk/0.33.0/src/meilisearch_sdk/tasks.rs.html , https://docs.rs/meilisearch-sdk/0.33.0/meilisearch_sdk/tasks/enum.Task.html
- **[S5]** meilisearch-rust README — sync-vs-async context, filterable attributes rebuild the index "may take time", v1.x compatibility statement: https://github.com/meilisearch/meilisearch-rust/blob/main/README.md

### Authoritative engineering references
- **[E1]** Supun Fernando, "An inefficient way to wait for Meilisearch to complete all async tasks" (EN) — async indexing breaks search tests; `sleep()` → flaky; poll `/tasks`: https://supun.io/meilisearch-wait-for-tasks
- **[E1-FR]** FR translation — "test bancal", waiting for async tasks: https://supun.io/fr/meilisearch-wait-for-tasks
- **[E2]** Meilisearch integration-guides issue #243 "Future of wait_for_task" (maintainer discussion: 5 s default, dev-scale assumption, advise against production use, Rust SDK uses async sleep): https://github.com/meilisearch/integration-guides/issues/243
- **[E3]** huggy's blog (zh-CN), "教程[16] MeiliSearch 的队列处理" — bounded wait for `status: succeeded`: https://blog.huggy.moe/posts/2022/7-meilisearch-3/
- **[E5]** Hookdeck, Meilisearch webhooks guide (completion notifications; production context): https://hookdeck.com/webhooks/platforms/guide-to-meilisearch-webhooks-features-and-best-practices
- **[E6]** Qiita (ja), self-hosted Meilisearch v1.13 notes — per-index settings binding: https://qiita.com/tamaloon/items/8c4a0d1042ae6d67c958
- **[E7]** DeepWiki — Index Scheduler / Task Queue and Batching (code-graph orientation only, not normative): https://deepwiki.com/meilisearch/meilisearch/2.2-index-scheduler , https://deepwiki.com/meilisearch/meilisearch/5.1-task-queue-and-batching

### Flora repository evidence (cross-check)
- **[F1]** `flora-search/src/lib.rs` (discarded `_task` handles; `setup_index`, `index_message/task/file`, `delete_document`, `search`)
- **[F2]** `Cargo.toml` / `Cargo.lock` (`meilisearch-sdk = "0.33"`, resolved `0.33.0`)
- **[F3]** `flora-core/src/config/search.rs`; `flora-api/src/state/app_state.rs`; `flora-api/src/routes/search.rs`; `flora-api/src/routes/health.rs`; `docker-compose.yml` (`getmeili/meilisearch:v1.12`)
- **[F4]** `flora-tests/src/contract/search.rs` (DB-gated harness, `make_test_state` with `search_service: None`, router/oneshot helpers)
- **[F5]** `docs/research/2026-11-16-022-db-meilisearch-gated-test-harness.md`
- **[F6]** `docs/research/2026-11-16-022-meilisearch-indexing-hooks-reindex-parity-vps.md`
- **[F7]** `docs/research/2026-11-16-022-meilisearch-pg-coherence-degradation-contract-vps.md`
- **[F8]** `docs/research/2026-11-16-021-optional-meilisearch-pg-coherence-vps.md`

---

*End of research deliverable (cycle 023). No source code was changed and no commits were made.*
