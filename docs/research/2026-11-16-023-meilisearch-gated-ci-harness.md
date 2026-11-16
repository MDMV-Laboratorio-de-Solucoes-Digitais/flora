# 023 — Feasibility of a Deterministic, Cheap Meilisearch-Gated CI Job for Flora

**Date:** 2026-11-16
**Status:** Research-only (no implementation, no workflow changes)
**Scope:** Feasibility of adding an optional, deterministic Meilisearch-gated integration/CI job alongside Flora's existing DB-gated Postgres contracts. Focus: GitHub Actions service containers (image pinning, healthchecks, keys, ports, lifecycle), test isolation via per-test index names, bounded task/indexing waits (no fixed sleeps), teardown, flakiness/resource cost on hosted runners, and keeping PG-gated contracts authoritative.
**Method:** Multilingual sweep (EN primary docs + pt-BR/es/de/fr/zh-CN/ja indexed sources). Normative behavior is grounded in upstream English primary sources (GitHub Docs, Meilisearch Docs, docs.rs, Meilisearch/SDK repos). Non-EN sources corroborate operational practice. All Flora facts re-verified against the checkout on 2026-11-16.
**Relation to 022:** This document **extends and cross-checks** `docs/research/2026-11-16-022-db-meilisearch-gated-test-harness.md`. It does not restate the general DB harness design (022 §3) or the full parity/precedence matrix (022 §4–5); it answers the narrower CI-feasibility question and corrects two stale/under-specified points in 022 (see §2.3).

---

## 1. Executive summary — verdict

**Feasible, with one prerequisite. Recommended: a separate, advisory `search-tests` job with two service containers (pinned `postgres:18` + pinned `getmeili/meilisearch:vX.Y.Z`), running doubly-gated `db_meili_*` contract tests serially, with explicit Meilisearch task waits. Keep `db-tests` unchanged and authoritative.**

Key findings:

- **Service containers are almost free on public repos** (standard Linux runners are free/unlimited); on private repos a separate job adds roughly **$0.01–0.04 per run** at `$0.006/min` (rounded up), most of it duplicated checkout/Rust setup, not the Meilisearch container itself. ([GitHub pricing](https://docs.github.com/en/billing/reference/actions-runner-pricing), [billing concepts](https://docs.github.com/en/billing/concepts/product-billing/github-actions))
- **Container lifecycle solves teardown for free:** GitHub creates a fresh container per job and destroys it when the job ends; there is no managed-cloud `if: always()` cleanup and no leaked billable resource. ([GH: about service containers](https://docs.github.com/en/actions/tutorials/use-containerized-services/use-docker-service-containers))
- **Determinism is achievable, but only with a task-wait enabler.** `flora-search::SearchService` discards every `TaskInfo` (`let _task = ...`), so tests cannot currently observe indexing completion. Either return `TaskInfo` from the mutating methods or poll tasks from a test-side SDK client. Without this, any "search after write" assertion is a fixed-sleep flake. (022 O4; re-verified in `flora-search/src/lib.rs:67,95,119,142,157`.)
- **Two-phase readiness is mandatory and often missed:** a container healthcheck (`GET /health` → `available`) proves the server answers requests; it does **not** prove your documents are searchable. Index visibility must be awaited separately via task status. ([Meilisearch health API](https://www.meilisearch.com/docs/reference/api/health/get-health), [async ops](https://meilisearch.com/docs/learn/async/asynchronous_operations), [CI healthcheck caveat](https://starsling.dev/best-practices/github-actions/wait-for-container-healthchecks))
- **Do not add Meilisearch to the existing `db-tests` job.** GitHub blocks *all* steps until *every* service with a healthcheck is healthy; a Meilisearch startup failure would then fail the authoritative Postgres contracts. Separate job preserves PG authority and isolates failure signal. ([GH service/health semantics](https://www.warpbuild.com/guides/service-containers-github-actions), [GH docs](https://docs.github.com/actions/tutorials/use-containerized-services/use-docker-service-containers))
- **Enablers already present in the checkout:** `flora-tests/Cargo.toml:37` already depends on `flora-search`; `flora-search` pins `meilisearch-sdk = "0.33"` whose README guarantees compatibility with Meilisearch **v1.x**; `SearchConfig::default()` already reads `MEILISEARCH_URL`/`MEILISEARCH_API_KEY`. No new dependency or config surface is strictly required. (`Cargo.toml:174`, [`meilisearch-rust` README](https://github.com/meilisearch/meilisearch-rust))
- **Current stable server is `v1.57.0`** (published 2026-11-16). Pin an exact tag (or digest), never `latest` — Meilisearch itself advises against `latest`. ([GitHub releases API](https://api.github.com/repos/meilisearch/meilisearch/releases/latest), [Meilisearch Docker guide](https://www.meilisearch.com/docs/guides/docker))

**Defer only if:** the task-wait enabler in `flora-search` is deemed out of scope, *and* the team accepts that Meilisearch behavior stays covered only by local opt-in (`docker run`) testing while PG fallback contracts remain the CI gate. Reasons and triggers in §10.

---

## 2. Flora baseline cross-check (verified on checkout)

### 2.1 CI baseline — `.github/workflows/rust.yml`

- Two jobs: `build` (`cargo build` + `cargo test`, no services) and `db-tests`.
- `db-tests`: `postgres:18` service, `POSTGRES_USER/PASSWORD/DB`, ports `5432:5432`, `--health-cmd "pg_isready -U postgres -d flora"`, `10s/5s/10` interval/timeout/retries; job env `FLORA_DATABASE__POSTGRES_URL` and `DATABASE_URL`.
- Six ignored suites run serially with `--test-threads=1`: `contract::auth::db_`, `contract::messaging::db_`, `contract::websocket::db_`, **`contract::search::db_`**, `contract::notifications::db_`, `contract::files::db_` (lines 60–71).
- No Meilisearch service, no `MEILISEARCH_*` env, no Meilisearch-gated test command.

### 2.2 Code facts relevant to Meilisearch

| Fact | Evidence | Consequence for a Meili CI job |
|---|---|---|
| `SearchService::new(url, api_key, index_template)` constructs a client; index name = template with `{org_id}` replaced | `flora-search/src/lib.rs:34–58` | Tests can build a service with a **unique index template per test**; `flora-tests` already depends on `flora-search` (`Cargo.toml:37`). |
| All index mutations discard `TaskInfo` (`set_filterable_attributes`, `add_documents` ×3, `delete_document`) | `flora-search/src/lib.rs:67,95,119,142,157` | **No deterministic visibility today**; needs the task-wait enabler (§5). |
| `search()` executes immediately and maps hits to `{id,type,snippet}` | `flora-search/src/lib.rs:169–226` | Parity comparisons should normalize to `{id,type}` sets, not rank/score (022 §4.4). |
| `SearchConfig::default()` reads `MEILISEARCH_URL` (default `http://localhost:7700`) and `MEILISEARCH_API_KEY`; `enabled: true`, `index_template: "flora_org_{org_id}"` | `flora-core/src/config/search.rs:22–34` | Job env `MEILISEARCH_URL`/`MEILISEARCH_API_KEY` is enough; tests can override only the template. |
| `AppState::new` builds `Some(SearchService)` when `search.enabled`, degrading to `None` **only on constructor error** | `flora-api/src/state/app_state.rs:128–146` | **`Client::new` sends no request** ([SDK README](https://github.com/meilisearch/meilisearch-rust)), so an unreachable Meilisearch still yields `Some(...)`; failure surfaces at query time. CI must gate on `/health` and tests must assert task success explicitly. |
| Router health reports `search: Some(search_service.is_some())` | `flora-api/src/routes/health.rs` (022 §2.2) | Health reflects *configuration*, not connectivity; don't use it as the readiness oracle for Meili. |
| Contract `search.rs` hand-builds `AppState` with `search_service: None`, uses `tower::ServiceExt::oneshot`, unique UUIDv7 seeds, `db_state()` env gate, `#[ignore]` | `flora-tests/src/contract/search.rs:40–119,233–260` | The same pattern extends cleanly to `make_test_state_with_search(...)` + `#[ignore = "requires Postgres + Meilisearch"]`. |
| `flora-search` uses `meilisearch-sdk = "0.33"` | root `Cargo.toml:174` | SDK guarantees **v1.x** server compatibility; pin a v1.x server tag (§11 NV-2). |

### 2.3 Corrections / additions to 022

1. **022 §2.2 and §6 are stale on one point:** they state search DB tests "are not listed in current CI steps." The current `rust.yml` **does** run `cargo test -p flora-tests contract::search::db_ -- --ignored --test-threads=1` (line 67). Therefore the PG-fallback search contracts are already authoritative in CI; the Meilisearch work is purely additive.
2. **022's "degradation on init failure" is imprecise:** `SearchService::new` fails only on URL parse/key construction, never on an unreachable server. Tests that want a `None` state must construct it explicitly; a "Meilisearch unavailable" test cannot rely on `AppState::new` to degrade on connection refusal.
3. **New positive finding:** `flora-tests` already declares `flora-search` as a dependency, so the 022 O1/O4 enabler needs no `Cargo.toml` change in the test crate.
4. **New CI-specific constraint (this doc):** because GitHub waits for *all* healthy services before *any* step, co-locating services in `db-tests` would let a Meili failure fail PG contracts — a strong argument for a separate job that 022 did not make explicit.

---

## 3. GitHub Actions service containers — primary-source feasibility facts

Grounded in [GH: Communicating with Docker service containers](https://docs.github.com/en/actions/tutorials/use-containerized-services/use-docker-service-containers), [GH: workflow syntax `jobs.<job_id>.services`](https://docs.github.com/actions/using-workflows/workflow-syntax-for-github-actions), [GH: creating PostgreSQL service containers](https://docs.github.com/actions/tutorials/use-containerized-services/create-postgresql-service-containers).

| Topic | Primary-doc fact | Flora implication |
|---|---|---|
| **Lifecycle** | GitHub creates a fresh Docker container for each configured service and destroys it when the job completes. Services cannot be used inside composite actions. | No manual teardown; each `search-tests` run starts with an empty Meilisearch instance. |
| **Runner requirement** | Service containers require a Linux runner; GitHub-hosted must be Ubuntu. | `runs-on: ubuntu-latest` already used; satisfied. |
| **Networking (runner-hosted job, no `container:`)** | Service ports are **not** exposed to the host by default; declare `ports:` and connect via `localhost:<mapped port>`. `job.services.<name>.ports['7700']` exposes dynamically assigned ports. | Flora's jobs are runner-hosted → use `ports: [7700:7700]` and `MEILISEARCH_URL=http://localhost:7700`. Inside `--health-cmd`, however, `localhost` means the *service container itself* ([explanation](https://github.com/Pandiyarajk/github-actions-course/blob/main/solutions/module-22-solutions.md)). |
| **Networking (job `container:`)** | All ports open between containers on the shared bridge; address service by label hostname, no `ports:` needed. | Not needed; adopting a job container would add Rust toolchain complexity. Keep runner-hosted. |
| **Healthchecks** | `options:` is passed verbatim to `docker create`. Available: `--health-cmd`, `--health-interval`, `--health-timeout`, `--health-retries`, `--health-start-period`. Runner polls Docker health and **blocks the first step until every service with a health command is healthy**. Without a health command the container is considered ready immediately (the classic "local passes, CI connection-refused" flake). | Add a real `/health` probe (§4). Do not use `sleep`. |
| **Probe quality** | Probe the query path, not just the port; use the service's own client/command. `--health-start-period` is preferred over inflated `--health-retries` for slow boots. | `/health` is a real HTTP query; add a small start period. ([starsling](https://starsling.dev/best-practices/github-actions/wait-for-container-healthchecks), [zh-CN writeup](https://plumephp.com/github-actions-container-jobs), [ja writeup](https://www.issoh.co.jp/tech/details/16961)) |
| **Image pinning** | `image:` accepts tag or `tag@sha256:...`. Third-party hardening guidance recommends digest pinning so upgrades are deliberate commits ([fr example](https://www.stephane-robert.info/en/docs/github-actions/workflows/service-containers)). | Pin `getmeili/meilisearch:v1.57.0` (or digest). Avoid `latest` — Meilisearch explicitly warns it can yield different engines on different machines. |
| **Teardown** | Runner stops/removes containers and the network in a post-job group. | No `if: always()` script needed (contrast with managed-cloud recipes that do require it, e.g. [Stackhero es](https://www.stackhero.io/es-ES/services/Meilisearch/documentations/GitHub-Actions-y-GitLab-CI), [fr](https://www.stackhero.io/fr-FR/services/Meilisearch/documentations/GitHub-Actions-GitLab-CI), [ja](https://www.stackhero.io/ja-JP/services/Meilisearch/documentations/GitHub-Actions-GitLab-CI)). |
| **Resource cost of services** | Image pull + create appear under "Initialize containers"; services themselves add no per-minute billing, but lengthen wall-clock job time. | Meilisearch image pull adds seconds on warm runners, up to ~a minute cold; minor. |

---

## 4. Meilisearch container — primary-source configuration facts

Sources: [Meilisearch Docker guide](https://www.meilisearch.com/docs/guides/docker), [Configuration reference](https://meilisearch.com/docs/resources/self_hosting/configuration/reference), [Health API](https://www.meilisearch.com/docs/reference/api/health), [Master/API keys](https://www.meilisearch.com/docs/resources/self_hosting/security/master_api_keys), [GitHub releases API](https://api.github.com/repos/meilisearch/meilisearch/releases/latest), [Docker Hub tags](https://hub.docker.com/hardened-images/catalog/dhi/meilisearch/guides).

- **Image:** `getmeili/meilisearch`. Tags observed: `latest`, `v1.57.0`, `v1.57`, `v1`. Current stable **v1.57.0** (published 2026-11-16). Meilisearch advises against `latest` because machines can drift to different images.
- **Port:** `7700` (configurable via `MEILI_HTTP_ADDR`). `config.toml` shows default `localhost:7700`; Docker image docs show `0.0.0.0:7700`. **Set `MEILI_HTTP_ADDR=0.0.0.0:7700` explicitly** so port mapping is reliable (NV-8).
- **Master key:** ≥16 bytes; protects every route except `GET /health`. In `development` mode it is optional; in `production` it is mandatory. For a throwaway CI instance use `development` **with** a fixed ≥16-byte key so Flora's `api_key` path is exercised. `/health` stays unauthenticated either way — good for the healthcheck.
- **Analytics:** `MEILI_NO_ANALYTICS` is a **flag** (no value). Setting a value is an error; in GitHub `env:` use the string `"true"` per common Compose practice — verify against the pinned image (NV-1). Telemetry opt-out is desirable in CI.
- **Health semantics:** `GET /health` → `200 {"status":"available"}` or `500 {"status":"mustRestart"}`. It says nothing about index/task state. The Rust SDK's `client.health()` returns `Err` when unhealthy.
- **Index creation:** Meilisearch creates an index on first document insert (and settings updates can also create it). With a fresh container there is no stale index; still wait for the settings task before issuing filtered queries (022 §4.3).
- **Async tasks:** statuses `enqueued`, `processing`, `succeeded`, `failed`, `canceled`; the last three are *finished*. A successful write response means *accepted/enqueued*, not searchable. ([async ops](https://meilisearch.com/docs/learn/async/asynchronous_operations))
- **Rust SDK wait API:** `client.wait_for_task(task_id, interval, timeout)`, `TaskInfo::wait_for_completion(client, interval, timeout)`, `Index::wait_for_task`. Defaults: **50 ms interval, 5000 ms timeout**; timeout yields `Error::Timeout`. Assert `Task::Succeeded { .. }`; on failure use `task.unwrap_failure()` for the error code. ([docs.rs client](https://docs.rs/meilisearch-sdk/latest/meilisearch_sdk/client/struct.Client.html), [task_info.rs source](https://docs.rs/meilisearch-sdk/latest/src/meilisearch_sdk/task_info.rs.html), [Rust quickstart](https://www.meilisearch.com/docs/getting_started/sdks/rust))
- **Supply chain:** prefer the first-party image as a `services:` entry over third-party wrappers such as `moy2010/meilisearch-github-action` (5 stars, examples still on `v0.25.2`, defaults to `latest`). Managed-cloud recipes (Stackhero) work but add secrets, network calls, and `if: always()` teardown — unnecessary here.

---

## 5. Deterministic waiting: two-phase readiness

| Phase | Owner | Oracle | Failure mode |
|---|---|---|---|
| **A. Server ready** | GitHub runner | `--health-cmd` hitting `GET /health` until `available` | Job fails at initialization with service named — before any test runs. No sleep. |
| **B. Data ready** | Test code | Meilisearch task status for the exact mutation (or bounded query poll as fallback) | Test fails with task uid/status/error code; never with "0 results, maybe wait longer". |

Why both: a passing `/healthcheck` does not make just-written documents searchable — a widely repeated CI warning ([starsling](https://starsling.dev/best-practices/github-actions/wait-for-container-healthchecks), [WarpBuild](https://www.warpbuild.com/guides/service-containers-github-actions)).

**Phase B implementation options for Flora (in order of preference):**

1. **Return `TaskInfo` from `flora-search` mutators** (022 O4). Changing `Result<()>` → `Result<TaskInfo>` is largely source-compatible for statement-position callers (`service.index_message(...).await?;` still compiles), but **verify all call sites** in `flora-api` (NV-4). Tests then call `task.wait_for_completion(&client, Some(50ms), Some(10s))` and assert `Succeeded`. Most faithful to production behavior.
2. **Expose a test helper** on `SearchService` (e.g. `pub async fn wait_for_index_tasks(&self, org_id, timeout)` using `client.get_tasks` filtered by index UID) behind a documented test-only API. No production semantics change, but adds a maintenance seam.
3. **Test-side SDK client** (`meilisearch-sdk` already transitive; add explicitly to `flora-tests` if needed): poll tasks filtered by the unique index UID until none are `enqueued`/`processing`, with a hard deadline. No `flora-search` change; slightly less precise about which task belongs to which write.
4. **Bounded query poll (stopgap):** retry the search assertion until the expected `{id,type}` appears or a deadline passes, then fail. Deterministic if bounded and asserted, but it hides task failures — acceptable only until 1–3 land. **Plain `sleep` is never acceptable.**

Settings note: `setup_index()` sets `filterableAttributes`; wait for that task before any filtered search, since changing filterable attributes rebuilds the index ([SDK README](https://github.com/meilisearch/meilisearch-rust)).

---

## 6. Recommended minimal design (concrete)

### 6.1 CI job — new `search-tests` job in `.github/workflows/rust.yml`

```yaml
  search-tests:
    runs-on: ubuntu-latest
    # Advisory until the flake rate is observed; remove once stable.
    continue-on-error: true
    services:
      postgres:
        image: postgres:18
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: flora
        ports:
          - 5432:5432
        options: >-
          --health-cmd "pg_isready -U postgres -d flora"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 10
      meilisearch:
        image: getmeili/meilisearch:v1.57.0   # pin exact tag; ideally tag@sha256:...
        env:
          MEILI_MASTER_KEY: flora_ci_master_key_0123456789   # >= 16 bytes
          MEILI_ENV: development
          MEILI_HTTP_ADDR: 0.0.0.0:7700
          MEILI_NO_ANALYTICS: "true"
        ports:
          - 7700:7700
        options: >-
          --health-cmd "curl -fsS http://localhost:7700/health | grep -q '\"status\":\"available\"'"
          --health-interval 2s
          --health-timeout 3s
          --health-retries 15
          --health-start-period 5s
    env:
      FLORA_DATABASE__POSTGRES_URL: postgresql://postgres:postgres@localhost:5432/flora
      DATABASE_URL: postgresql://postgres:postgres@localhost:5432/flora
      MEILISEARCH_URL: http://localhost:7700
      MEILISEARCH_API_KEY: flora_ci_master_key_0123456789
    steps:
      - uses: actions/checkout@v4
      - name: Set up Rust 1.98.0
        shell: bash
        run: |
          set -euo pipefail
          rustup toolchain install 1.98.0 --profile minimal --no-self-update
          rustup default 1.98.0
      # Optional: actions/cache or Swatinem/rust-cache to cut duplicated compile time.
      - name: Run Meilisearch-gated search contracts
        run: cargo test -p flora-tests contract::search::db_meili_ -- --ignored --test-threads=1
```

Notes on the YAML:

- `db-tests` is left **untouched**; it never receives `MEILISEARCH_URL`, so PG contracts remain independent of Meili health.
- `postgres:18` is repeated here because router-level search tests still need auth/membership seeding (022 §5.2 matrix). A cheaper *engine-level* variant (test `SearchService` directly, no PG, no router) could drop the Postgres service; use it if you want Meili coverage without DB coupling, accepting that precedence/degradation at the router remain untested.
- `--health-cmd` runs **inside** the Meilisearch container, so `localhost` there is correct. If `curl` is absent from the pinned image (NV-1), drop the `--health-cmd` and add a runner-side **bounded** poll step using `curl http://localhost:7700/health` with a deadline and failure — still no fixed sleep.
- `MEILI_MASTER_KEY` is a throwaway value for an ephemeral container; if repo policy forbids literals, use `${{ secrets.* }}` or `${{ github.run_id }}`-derived value. At least 16 bytes either way.
- Use `v1.57.0` as of this research date; bump deliberately. Do not use `latest`.

### 6.2 Test-side shape (in `flora-tests/src/contract/search.rs` or a new `search_meili.rs`)

```
#[tokio::test]
#[ignore = "requires Postgres + Meilisearch; set DB URL and MEILISEARCH_URL"]
async fn db_meili_search_returns_indexed_message() -> Result<()> {
    let Some((mut state, config)) = meili_state().await? else { return Ok(()); };  // gate
    let seed = seed_member(&state, &config).await?;                                 // PG auth seed
    let uniq = Uuid::now_v7().simple();
    let template = format!("flora_ci_{uniq}_org_{{org_id}}");                       // per-test index
    state.search_service = Some(Arc::new(SearchService::new(
        &meili_url, meili_key.as_deref(), &template)?));

    let task = state.search_service.as_ref().unwrap()
        .index_message_for_test(&seed.org.id, &msg).await?;                          // enabler
    wait_succeeded(&task, Duration::from_millis(50), Duration::from_secs(10)).await?;

    let (status, body) = authed_get_json(search_app(state), "/search?q=xyz...", ...).await?;
    ensure!(status == OK && body["results"].iter().any(|r| r["id"] == msg_id), ...);
    Ok(())
}
```

Gate helper mirrors `db_state()`: return `Ok(None)` with an `eprintln!` skip reason when `MEILISEARCH_URL` (and DB URL) are unset. Assert on normalized `{id,type}` sets; do **not** assert rank, score, or global counts.

### 6.3 Isolation & teardown

- **Physical isolation:** fresh Meilisearch container per job → no cross-run state.
- **Logical isolation:** per-test `index_template` containing a UUID suffix plus the existing unique `org_id` → no cross-test collisions even if the suite is parallelized later.
- **Teardown:** none required at job level (runner destroys container). If a test reuses an index name, call `client.index(name).delete().await?.wait_for_completion(...)` in cleanup; otherwise skip.
- **Serial execution:** keep `--test-threads=1` initially to match the proven DB pattern; per-test indices make parallelism safe later, but serial first reduces variables.

---

## 7. Keeping PG-gated contracts authoritative

1. **Unchanged `db-tests` job** is the required status check; `search-tests` starts advisory (`continue-on-error: true`) and is promoted only after a stable run window.
2. **Separate job prevents cross-contamination of signal** (§3): Meili health cannot block PG steps, and a Meili-only regression does not mask a PG failure.
3. **No Meili env in `build`/`db-tests`**: `cargo test` without the gate returns early with a skip reason, so default local/CI runs never require Meilisearch (022 §7 checklist).
4. **PG fallback remains the authoritative behavior contract**: Meili tests assert *additional* behavior (Meilisearch path hit, precedence, optional parity subset) rather than redefining search semantics. Parity compares normalized IDs/types only (022 §4.4).
5. **Degradation tests stay deterministic** by constructing `search_service: None` directly; do not simulate network faults (022 §5.3).

---

## 8. Cost analysis (hosted runners)

Grounded in [GitHub runner pricing](https://docs.github.com/en/billing/reference/actions-runner-pricing), [billing concepts](https://docs.github.com/en/billing/concepts/product-billing/github-actions), [GitHub-hosted runners reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners), with independent corroboration from [Convika 2026-11](https://convika.com/blog/github-actions-pricing-2026) and [craci.com](https://craci.com/alternatives/github-hosted-runners).

| Cost driver | Public repo | Private repo |
|---|---|---|
| Standard `ubuntu-latest` runner | **Free and unlimited** (4 vCPU / 16 GB per public-repo table) | `$0.006/min` (2 vCPU / 8 GB), billed per job rounded **up** to whole minute |
| Included monthly compute | n/a | Free $12 (≈2,000 min), Pro/Team $18 (≈3,000 min), Enterprise $300 (≈50,000 min) |
| Meilisearch service container | No separate per-minute charge; image pull adds wall-clock (seconds warm, up to ~1 min cold) | Same; contributes to job minutes only via elapsed time |
| Extra `search-tests` job | $0 cost; consumes one of 20 concurrent Free-plan jobs | ~3–6 min/run without cache ⇒ **~$0.018–0.036/run**; with `rust-cache` ~2–3 min ⇒ **~$0.012–0.018/run** |
| Monthly illustration (private, 2 runs/day, 4 min) | $0 | ≈240 min ≈ **$1.44/mo**, covered by the $12/$18 allowance unless the repo is already near quota |

**Cheapest variants:**
- **Cheapest correct:** separate job + Rust dependency cache (Swatinem/rust-cache or `actions/cache` on `~/.cargo` + `target/`).
- **Cheapest absolute:** add the Meilisearch service to `db-tests` (no duplicate setup) — **not recommended**, because it makes Meili health a prerequisite for authoritative PG contracts (§3).
- **Tooling red herrings:** third-party actions and managed cloud instances add setup calls, secrets, and teardown scripts; they do not reduce minutes and increase supply-chain surface.

Memory: tiny corpora (a handful of docs per test) keep Meilisearch well within runner RAM even on 2 vCPU/8 GB private runners. Only if corpora grow would `MEILI_MAX_INDEXING_MEMORY` be worth setting ([production guide](http://codenx.com/meilisearch/meilisearch-production-setup)).

---

## 9. Flakiness analysis & controls

| Risk of flake | Control | Status in Flora today |
|---|---|---|
| Tests start before service accepts requests | Container `--health-cmd` on `/health`; runner blocks first step | Present for PG; to add for Meili |
| Documents not yet searchable after write | Task-status wait with explicit timeout; assert `Succeeded` (§5) | **Missing** — needs enabler |
| Settings (`filterableAttributes`) race with filtered query | Wait the settings task before query | **Missing** |
| Engine drift across runs | Pin exact image tag/digest; avoid `latest` | N/A (new) |
| SDK/server mismatch | SDK 0.33 ↔ v1.x guaranteed; pin a v1.x server | SDK pinned; server to pin |
| Cross-test contamination | Fresh container per job + unique index template/org per test | Orgs already unique (UUIDv7) |
| Parallel shared-state contention | `--test-threads=1` initially | Already used for DB suites |
| Silent skip hides a broken gate | Gate helper prints skip reason; CI step asserts the test actually ran (e.g. `--exact` filter or a non-empty test count) | Partially (skip reason present) |
| Meili regression blocks authoritative signal | Separate advisory job; promote after stable window | To add |
| Unbounded waits hanging a job | Explicit `timeout`, `Error::Timeout` handled with task uid/status diagnostics | To add |
| Global counts/rank assertions | Normalize to `{id,type}` sets | Already the contract style |

Also note the general CI principle that healthchecks prove reachability, not eventual consistency — do not stretch `/health` to cover indexing ([starsling](https://starsling.dev/best-practices/github-actions/wait-for-container-healthchecks), [ja analysis](https://www.issoh.co.jp/tech/details/16961)).

---

## 10. Options considered & recommendation

| Option | Effort | Value | Determinism | CI cost | Verdict |
|---|---|---|---|---|---|
| **O-A: Separate advisory `search-tests` job + doubly-gated `db_meili_*` tests + task-wait enabler** (§6) | Med | High (Meili path, precedence, parity subset) | High | 2 services, one extra job | **Recommend** |
| **O-B: Same as O-A but engine-level only** (no Postgres, test `SearchService` directly) | Low | Medium (indexing/search behavior only; no router precedence) | High | 1 service, one job | Good cheaper first slice |
| **O-C: Add Meilisearch service to existing `db-tests`** | Low | High | High | No extra job | **Reject** — Meili health gates PG contracts |
| **O-D: Managed Meilisearch cloud for CI (e.g. Stackhero)** | Med | Medium | Medium | Network + secrets + teardown | Reject — unnecessary, less isolated |
| **O-E: Third-party `moy2010/meilisearch-github-action`** | Low | Medium | Depends on `latest` default | Same | Reject — supply chain, stale examples |
| **O-F: Defer; local `docker run` + docs only** | Very low | Low (no CI signal) | Medium | Zero | Acceptable **only** if enabler is out of scope |

**Recommendation:** proceed with **O-A**, gated on the small `flora-search` task-observability change (022 O4). Start with 2–3 tests: (1) Meili-path indexed message with task wait, (2) precedence smoke via fixture that differs between Meili and PG, (3) normalized parity subset. Keep the job advisory for a stabilization window, then make it required. If the team wants the absolute smallest step, land O-B first and add router-level tests after the enabler proves stable.

**Defer triggers:** (a) task observability cannot be added without an API change the team rejects; (b) private-repo minute budget is already tight and PG fallback coverage is judged sufficient; (c) no owner for ongoing image/tag bumps. In those cases, document a local opt-in command:
`docker run --rm -p 7700:7700 -e MEILI_MASTER_KEY=flora_ci_master_key_0123456789 -e MEILI_NO_ANALYTICS getmeili/meilisearch:v1.57.0`
plus the env vars and `cargo test -p flora-tests contract::search::db_meili_ -- --ignored --test-threads=1`.

---

## 11. Risks / NVs / open questions

| ID | Item | Why it matters | Resolution |
|---|---|---|---|
| **NV-1** | Does `getmeili/meilisearch:v1.57.0` ship `curl` (or `wget`) for `--health-cmd`? | Official image is minimal; a probe calling a missing binary fails with `executable file not found`. ([CodeNx warning](http://codenx.com/meilisearch/meilisearch-production-setup)) Official `meilisearch/saas-demo` Compose uses `curl -fsS localhost:7700/health`, suggesting it is present in recent images, but verify on the pinned tag. | Run `docker run --rm --entrypoint sh getmeili/meilisearch:v1.57.0 -c 'command -v curl wget || echo none'`; if none, use runner-side bounded poll. |
| **NV-2** | SDK `0.33` ↔ server `v1.57.0` feature coverage | SDK guarantees v1.x compatibility but "some features may not be present" ([README](https://github.com/meilisearch/meilisearch-rust)). Basic `add_documents`/`search`/`wait_for_task` are low risk. | Smoke-test the pinned pair; if a task field is missing, pin an older v1.x server that SDK CI covers. |
| **NV-3** | `MEILI_NO_ANALYTICS` flag semantics in `env:` | Config reference says the flag takes **no value**; assigning one errors. Compose examples use `"true"`. Verify the exact env spelling accepted by the pinned image. | Check config reference / image; adjust to bare flag via `options:` if needed. |
| **NV-4** | Call sites of `index_message`/`index_task`/`index_file`/`setup_index`/`delete_document` | Changing return type to `TaskInfo` is mostly source-compatible but must be verified across `flora-api`. | `grep` call sites before editing; add `let _ =` where needed. |
| **NV-5** | `SearchService::new` does not probe connectivity | A wrong `MEILISEARCH_URL` yields `Some(service)` and query-time errors, not degraded `None`. | CI health gate + explicit test assertion; consider a `client.health()` check in the test gate. |
| **NV-6** | Index creation ordering | Settings vs documents can both create an index; filtered queries before settings task completion can fail with "attribute not filterable". | Wait both tasks explicitly. |
| **NV-7** | `FLORA_SEARCH__*` env nesting | This checkout has no `FLORA_SEARCH__ENABLED` gate observed in tests; `SearchConfig` defaults read bare `MEILISEARCH_*`. | Tests build `AppState` directly; no config-level feature flag required. |
| **NV-8** | `MEILI_HTTP_ADDR` default discrepancy | `config.toml` says `localhost:7700`; Docker docs say `0.0.0.0:7700`. | Set `MEILI_HTTP_ADDR=0.0.0.0:7700` explicitly. |
| **NV-9** | Master key in YAML | Disposable value is acceptable for an ephemeral container; policy may require secrets. | Confirm with repo secret policy; use `secrets.*` if required. |
| **NV-10** | "Test actually ran" verification | An ignored test whose gate returns `None` still exits 0; a typo in the filter could green-wash. | Assert test count in CI log or use exact test names; keep gate skip messages loud. |
| **NV-11** | Minutes accounting on private repo | Jobs round up; many small jobs waste minutes. | One job, cached toolchain, tiny corpus. |
| **NV-12** | Concurrency limits | Free plan caps 20 concurrent jobs; one extra job is negligible. | No action. |

---

## 12. Sources & citations

### Flora (local cross-check, verified 2026-11-16)
- `.github/workflows/rust.yml` — `build` + `db-tests`, `postgres:18` service, six serial ignored suites incl. `contract::search::db_`.
- `flora-search/src/lib.rs` — `SearchService::new`, per-org index template, discarded `TaskInfo`, `search()`.
- `flora-core/src/config/search.rs` — `SearchConfig` defaults from `MEILISEARCH_URL`/`MEILISEARCH_API_KEY`.
- `flora-api/src/state/app_state.rs` — conditional `Some(SearchService)` with degraded `None` only on constructor error.
- `flora-tests/src/contract/search.rs` — hand-built `AppState`, `search_service: None`, `db_state()` gate, UUIDv7 seeding, `oneshot`.
- `flora-tests/Cargo.toml:37`, root `Cargo.toml:174` — `flora-search` test dependency; `meilisearch-sdk = "0.33"`.
- `docs/research/2026-11-16-022-db-meilisearch-gated-test-harness.md` — prior design; cross-checked and extended here.

### GitHub Actions (primary)
- [Communicating with Docker service containers](https://docs.github.com/en/actions/tutorials/use-containerized-services/use-docker-service-containers)
- [Workflow syntax — `jobs.<job_id>.services`](https://docs.github.com/actions/using-workflows/workflow-syntax-for-github-actions)
- [Creating PostgreSQL service containers](https://docs.github.com/actions/tutorials/use-containerized-services/create-postgresql-service-containers)
- [GitHub-hosted runners reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
- [Actions runner pricing](https://docs.github.com/en/billing/reference/actions-runner-pricing)
- [GitHub Actions billing concepts](https://docs.github.com/en/billing/concepts/product-billing/github-actions)
- [Choosing the runner for a job](https://docs.github.com/en/actions/how-tos/write-workflows/choose-where-workflows-run/choose-the-runner-for-a-job)
- Service-container health/lifecycle practice: [WarpBuild guide](https://www.warpbuild.com/guides/service-containers-github-actions), [starsling "healthchecks, not sleep"](https://starsling.dev/best-practices/github-actions/wait-for-container-healthchecks), [GH Actions course solutions](https://github.com/Pandiyarajk/github-actions-course/blob/main/solutions/module-22-solutions.md)

### Meilisearch (primary)
- [Using Meilisearch with Docker](https://www.meilisearch.com/docs/guides/docker)
- [Configuration reference](https://meilisearch.com/docs/resources/self_hosting/configuration/reference)
- [Get health API](https://www.meilisearch.com/docs/reference/api/health/get-health)
- [Master key and API keys](https://www.meilisearch.com/docs/resources/self_hosting/security/master_api_keys)
- [Tasks and asynchronous operations](https://meilisearch.com/docs/learn/async/asynchronous_operations)
- [Rust quick start](https://www.meilisearch.com/docs/getting_started/sdks/rust)
- [meilisearch-rust repository/README (v1.x compatibility)](https://github.com/meilisearch/meilisearch-rust)
- [docs.rs `Client::wait_for_task`](https://docs.rs/meilisearch-sdk/latest/meilisearch_sdk/client/struct.Client.html), [`TaskInfo::wait_for_completion` source](https://docs.rs/meilisearch-sdk/latest/src/meilisearch_sdk/task_info.rs.html)
- [GitHub releases API — v1.57.0](https://api.github.com/repos/meilisearch/meilisearch/releases/latest)
- [Official `saas-demo` Compose healthcheck example](https://github.com/meilisearch/saas-demo/blob/main/docker-compose.yml)

### Rust / sqlx / Axum testing (authoritative)
- [sqlx `#[sqlx::test]`](https://docs.rs/sqlx/latest/sqlx/attr.test.html) — per-test DB isolation reference (022 §3; not re-adopted here).
- [Axum testing example (`oneshot`)](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs) — already used by Flora contracts.
- [testcontainers-rs](https://rust.testcontainers.org/) — local-only alternative, rejected for CI cost.

### Multilingual evidence (coverage note)
- **EN:** GitHub Docs, Meilisearch Docs, docs.rs, Meilisearch repos — normative sources for every technical claim above.
- **pt-BR:** targeted queries issued; no authoritative pt-BR primary doc for service containers surfaced (022 already cites Rust Brasil SQLx/CI material).
- **es:** [Stackhero Meilisearch GitHub Actions (es-ES)](https://www.stackhero.io/es-ES/services/Meilisearch/documentations/GitHub-Actions-y-GitLab-CI) — managed-cloud lifecycle with `if: always()` teardown (contrast with service containers).
- **de:** targeted queries returned mostly generic GitHub Actions tutorials; no authoritative German primary documentation beyond translations of GitHub topics; no normative claim taken from de sources.
- **fr:** [Stackhero (fr-FR)](https://www.stackhero.io/fr-FR/services/Meilisearch/documentations/GitHub-Actions-GitLab-CI); [stephane-robert.info service containers](https://www.stephane-robert.info/en/docs/github-actions/workflows/service-containers) — digest pinning + healthcheck rationale.
- **zh-CN:** [PlumePHP — container jobs & service containers](https://plumephp.com/github-actions-container-jobs) — `--health-start-period` vs retries, "no healthcheck ⇒ no wait".
- **ja:** [issoh.co.jp — services & startup-wait design](https://www.issoh.co.jp/tech/details/16961) — official images lack `HEALTHCHECK`, runner treats them as immediately ready; [Stackhero (ja-JP)](https://www.stackhero.io/ja-JP/services/Meilisearch/documentations/GitHub-Actions-GitLab-CI).

---

## 13. Bottom line

**Adopt O-A as a small, separate, advisory CI job; keep `db-tests` as the authoritative gate.** The mechanics are well supported by primary docs and cost little (free on public repos, ~$0.01–0.04/run on private). The single hard prerequisite is making Meilisearch task completion observable from tests — without it, no amount of CI plumbing yields a deterministic test. Pin the image (v1.57.0 at time of writing), gate on `/health` for server readiness and on task status for data readiness, isolate with per-test index names, let the runner handle teardown, and promote the job to required only after it proves stable. Defer to local opt-in testing only if the enabler or the ownership for image bumps is unavailable.
