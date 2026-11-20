# 022 — Deterministic DB/Meilisearch-Gated Test Harness (Rust, cargo test/tokio)

**Date:** 2026-11-16  
**Status:** Research-only (no implementation)  
**Scope:** Deterministic integration/contract testing strategies in Rust for (A) DB-gated contracts on disposable PostgreSQL (sqlx migrations, env-gated tests, isolation) and (B) optionally Meilisearch-gated tests in CI/dev: disposable Meilisearch, client configuration, deterministic wait for indexing/tasks, parity/precedence/degradation assertions, and cost/stability on constrained CI. Includes Axum router-level patterns with optional external search service.  
**Research coverage:** Multilingual, high-reliability sweep (EN primary docs + pt-BR/es/de/ja indexed sources where available). Authoritative behavior is grounded in upstream English docs/source (sqlx, Meilisearch, GitHub Actions, Axum/tower).

---

## 1. Executive summary

- **Flora already has a workable DB-gated contract pattern:** `#[ignore]` + env-gated `db_state()` (`FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`) + `connect` + `flora_core::migrations::run_migrations` + per-test unique seeded data (UUIDv7) + in-test `AppState` + Axum router exercised via `tower::ServiceExt::oneshot`. CI runs selected DB suites serially (`--test-threads=1`) against a `postgres:18` service container. This is the cheapest baseline to extend.
- **Isolation today is logical, not physical-per-test:** tests share one migrated database and rely on unique identifiers and scoped assertions (tenant/soft-delete checks). This is deterministic enough if tests never assume global emptiness/counts and avoid cross-test key collisions.
- **Meilisearch is async-by-design:** document/index operations enqueue tasks; results are searchable only after task completion. Deterministic tests must wait on task status (`wait_for_task` / `wait_for_completion`) with bounded timeout/interval and fail on task failure/timeout, never on fixed sleeps alone. ([Meilisearch async ops](https://www.meilisearch.com/docs/learn/async/asynchronous_operations), [Rust SDK task wait](https://docs.rs/meilisearch-sdk/latest/meilisearch_sdk/task_info.rs.html))
- **Flora search path has two modes:** `state.search_service: Some(...)` uses Meilisearch; `None` falls back to PostgreSQL FTS in `routes/search.rs`. Health exposes `search: Some(bool)` from `search_service.is_some()`. This makes precedence/degradation/parity tests naturally expressible at router level by constructing different `AppState` variants.
- **80/20 for 022:** keep the existing env-gated/ignored contract style; add a separate opt-in Meilisearch gate (env var like `FLORA_SEARCH__MEILISEARCH_URL` or `MEILISEARCH_URL` + `FLORA_SEARCH__ENABLED`) and a tiny helper to build `AppState` with a real `SearchService`; run Meilisearch-gated tests in a dedicated CI job (service container) or local opt-in, serially, with unique per-test index names and explicit task waits. Do **not** make Meilisearch required for default `cargo test`.

---

## 2. Flora baseline cross-check (code)

### 2.1 Contract test harness shape
- **Location:** `flora-tests/src/contract/{search,auth,messaging,websocket,...}.rs`.
- **DB gating:** `db_state() -> anyhow::Result<Option<(AppState, Config)>>` returns `None` and prints skip reason when `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` is unset; tests are `#[tokio::test] #[ignore = "requires Postgres; ..."]` and return early on `None`.
  - `flora-tests/src/contract/search.rs`: `db_state()` connects with `PgPool::connect(&url)`, runs `flora_core::migrations::run_migrations(&pool)`, builds state via `make_test_state`.
  - `flora-tests/src/contract/auth.rs`: same env pattern; includes both non-DB tests using `PgPool::connect_lazy(...)` and DB-gated `db_*` tests.
- **State construction:** tests manually assemble `AppState` with repositories/services from pool and set external dependencies to `None`:
  - `search_service: None, messaging_service: None, valkey: None, oidc_transactions: OidcTransactionStore::new(None), realtime_hub: create_hub()`.
- **Router-level execution:** `Router::new().nest("/search", create_search_router()).with_state(state)` and requests driven by `tower::ServiceExt::oneshot` (also used in auth/websocket-style contract tests).
- **Determinism aids already present:** unique seeded entities via `Uuid::now_v7()`, explicit inserts, tenant/soft-delete isolation assertions, bounded limits/type normalization assertions, no dependence on global row counts.

### 2.2 CI and runtime wiring
- **CI (`.github/workflows/rust.yml`):**
  - `build`: `cargo build` + `cargo test` (no external services).
  - `db-tests`: `postgres:18` service with health check, env `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`, runs specific ignored suites serially: `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` (and messaging/websocket). Search DB-gated fallback tests exist but are not listed in current CI steps (important 022 planning input).
- **Search service/config:**
  - `flora-core/src/config/search.rs`: `SearchConfig { url, api_key, default_limit, max_limit, index_template, enabled }`, defaults from `MEILISEARCH_URL`/`MEILISEARCH_API_KEY`, `enabled: true`.
  - `flora-api/src/state/app_state.rs`: builds `Option<SearchService>` when enabled and reachable/configured; logs and degrades to `None` on init failure.
  - `flora-search/src/lib.rs`: per-org index `index_template.replace("{org_id}", ...)`; `index_message/task/file` call `add_documents(...).await` and discard returned task (`let _task = ...`); `delete_document` similar; `setup_index` sets filterable attributes and discards task; `search` executes query immediately.
  - `flora-api/src/routes/search.rs`: membership/auth checks, empty-query short-circuit, type/limit normalization; if `search_service` present delegates to Meilisearch, else `search_pg` FTS fallback with deterministic ordering (`rank DESC, created_at DESC, id DESC`) and merge/truncate.
  - `flora-api/src/routes/health.rs`: reports `search: Some(state.search_service.is_some())`.
- **Migrations:** `flora_core::migrations::run_migrations` uses `sqlx::migrate!("./src/migrations/sql")`.

### 2.3 Gaps relevant to 022
- No Meilisearch-gated contract/integration harness, no CI service/job for Meilisearch.
- `SearchService` currently does not expose/wait task completion for indexing operations; deterministic Meilisearch tests cannot rely on immediate visibility after writes without an explicit wait strategy (SDK/task API).
- No parity tests comparing Meilisearch path vs PG fallback, no precedence/degradation assertions beyond implicit code paths.

---

## 3. (A) DB-gated contracts on disposable PostgreSQL

### 3.1 Strategy options

| Strategy | How it works | Determinism/isolation | Cost/CI fit | Flora fit |
|---|---|---|---|---|
| **Shared disposable DB + env-gated ignored tests + unique data** (current) | One Postgres (service/local), run migrations once per test/process helper, seed unique rows per test, skip when env unset | Good if assertions are scoped; parallelism can introduce contention/flakes unless serialized | **Very low** on constrained CI; simple setup | **Highest now** |
| **`#[sqlx::test]` per-test database** | sqlx connects to superuser `DATABASE_URL`, creates temp DB per test, applies migrations/fixtures, provides pool/conn | Strong isolation by default; failed tests may retain DBs for debug; cleanup on next run | Higher DB churn/startup; requires compatible migration path/layout; macro/runtime constraints | Medium (would reshape harness; Flora uses custom `run_migrations` macro path in core) |
| **Transaction rollback per test** | Begin tx per test, rollback at end | Strong for single-connection/same-tx code; tricky when app code opens its own pool/connections or needs committed visibility across connections | Low runtime if feasible | Low-medium for router-level tests using `PgPool`/repos |
| **Schema-per-test or template-clone DB** | Create schema/DB clone from migrated template per test | Strong isolation, reusable template cost amortization | Moderate complexity; template/fingerprint mgmt | Medium for future scale |
| **Testcontainers Postgres** | Programmatic container lifecycle per test/suite | Strong reproducibility; container-per-test can be slow | Requires Docker; slower/costlier on constrained CI than service container | Low for CI, useful locally/optional |

**Sources:** sqlx `#[sqlx::test]` docs (auto test DB mgmt, migrations/fixtures, cleanup semantics, supported DBs) [docs.rs/sqlx attr.test](https://docs.rs/sqlx/latest/sqlx/attr.test.html); "Database Tests for the Lazy" and Rust testing guides on per-test DB isolation [mattrighetti](https://mattrighetti.com/2025/02/17/rust-testing-sqlx-lazy-people), [rust-exercises isolation](https://rust-exercises.com/advanced-testing/06_database_isolation/00_intro.html); transactional rollback pattern [Rust FAQ DB testing](https://www.rustfaq.org/en/how-to-test-database-code-in-rust); testcontainers-rs patterns [testcontainers-rs](https://rust.testcontainers.org/), [Leapcell guide](https://www.leapcell.io/blog/streamlining-rust-integration-tests-with-ephemeral-database-instances), [helpmetest/sqlx](https://helpmetest.com/blog/rust-sqlx-database-testing); template/fingerprint harness concept [postgres-test-harness](https://github.com/bpcakes/postgres-test-harness).

### 3.2 Migrations and determinism
- **Run migrations against a known-empty/disposable DB** before tests; fail fast on migration errors. Flora does this in `db_state()` — correct for gated suites.
- Prefer **idempotent, ordered migrations** and avoid depending on mutable external state. For `#[sqlx::test]`, sqlx can auto-apply migrations from crate-relative paths or embedded migrator; custom migration runners must be aligned to test DB lifecycle. ([sqlx migrations section](https://docs.rs/sqlx/latest/sqlx/attr.test.html#automatic-migrations-requires-migrate-feature))
- **Fixtures:** sqlx supports composable fixtures; Flora currently seeds inline in helpers — explicit and reviewable, good for contract intent but requires discipline to stay minimal.
- **Cleanup:** per-test DB drop (sqlx/testcontainers/template clone) is cleanest; shared DB requires data uniqueness and avoiding destructive global operations.

### 3.3 Env-gating and CI patterns
- Use **explicit env gates** and **skip/ignore** so default runs stay cheap: Flora's `Option` return + `#[ignore]` is a good pattern (tests discoverable but opt-in). Alternatives: custom cfg/feature flags, but env-gated ignored tests integrate well with existing CI commands.
- GitHub Actions: run disposable Postgres as **service container** with `--health-cmd pg_isready`, expose port, pass `DATABASE_URL`/`FLORA_*`; for jobs in containers use service label as host, for runner jobs map ports and use `localhost`. ([GH Postgres service containers](https://docs.github.com/en/actions/use-cases-and-examples/using-containerized-services/creating-postgresql-service-containers), [service containers networking](https://docs.github.com/en/actions/use-cases-and-examples/using-containerized-services/about-service-containers))
- **Constrained CI stability:** serialize DB-gated suites touching shared DB (`--test-threads=1`, as Flora does), keep datasets tiny, run only targeted `db_*` contract slices, avoid broad `--workspace` DB dependence in non-DB jobs.

### 3.4 Practical isolation guidance for Flora
- Keep current model short-term: **shared service DB + per-test unique seeds + scoped predicates**. Do not assert total counts across tenants/global tables unless test owns the full dataset.
- If flakes emerge, first tighten: deterministic ordering in assertions, no time-based sleeps, avoid parallel `--test-threads>1` for shared-DB tests, and consider per-test schema/DB only when justified.
- `connect_lazy` is fine for tests that never actually hit DB (as some auth unit-contract tests do), but DB-gated paths must use real `connect`/`await` and migration readiness.

---

## 4. (B) Meilisearch-gated tests (optional, deterministic)

### 4.1 Running disposable Meilisearch (dev/CI)

| Approach | Use case | Notes |
|---|---|---|
| **GitHub Actions service container** (`image: getmeili/meilisearch:vX.Y` or pinned tag) | CI job needing real Meilisearch | Map `7700:7700`, set `MEILI_MASTER_KEY`/`MEILI_NO_ANALYTICS=true` as needed, add health/wait step if necessary. Pin version for reproducibility. |
| **Local Docker** (`docker run -p 7700:7700 ...`) | Dev/manual gated runs | Same env vars; cheap and disposable (`--rm`, fresh volume/tmpfs). |
| **Testcontainers (module/custom GenericImage)** | Programmatic lifecycle | Strong isolation but more overhead; good if adopted later, may be heavy for constrained CI. |
| **External managed instance** | Not recommended for contracts | Hard to isolate/keep deterministic; only for explicit environment tests. |

**Sources:** Meilisearch Docker/quickstart patterns and GitHub Action wrappers exist (`moy2010/meilisearch-github-action` etc.) [action example](https://github.com/moy2010/meilisearch-github-action), [marketplace](https://github.com/marketplace/actions/meilisearch-in-github-actions); general GH service-container semantics [GH docs](https://docs.github.com/en/actions/use-cases-and-examples/using-containerized-services/about-service-containers). **Recommendation:** prefer first-party Docker image as service container over third-party actions for supply-chain simplicity.

### 4.2 Client configuration in tests
- Mirror production config surface: `SearchConfig { url, api_key, index_template, enabled }` / `SearchService::new(url, api_key, index_template)` (`flora-search`).
- Use **env-gated opt-in** separate from DB gate, e.g. require `MEILISEARCH_URL` or `FLORA_SEARCH__MEILISEARCH_URL` (and optionally master key). Default CI/test runs must not require it.
- Build `AppState` with `search_service: Some(Arc::new(SearchService::new(...)?))` for Meilisearch-path tests; keep `None` for fallback/parity-baseline tests.
- **Unique index namespace per test/run:** include UUID suffix in `index_template` or derive per-test org IDs and ensure index names do not collide across parallel runs (`flora_org_{org_id}` already includes org UUID; per-test unique orgs are sufficient if index cleanup is not global). Consider explicit delete/teardown only if reusing same org/index names.

### 4.3 Deterministic waiting for indexing/tasks (critical)
- Meilisearch **enqueues async tasks** for document/index/settings operations; a successful write API response means accepted/enqueued, not searchable yet. ([Tasks and async ops](https://www.meilisearch.com/docs/learn/async/asynchronous_operations), [Working with tasks](https://www.meilisearch.com/docs/learn/async/working_with_tasks))
- **Rust SDK:** `TaskInfo::wait_for_completion(client, interval, timeout)` / `client.wait_for_task(...)` polls until `Task::Succeeded|Failed|Canceled` or timeout (`Error::Timeout` default interval/timeout documented; SDK defaults commonly 50ms/5000ms in task_info impl). Always assert success and surface `unwrap_failure()` on failure. ([meilisearch-sdk task_info.rs](https://docs.rs/meilisearch-sdk/latest/src/meilisearch_sdk/task_info.rs.html), [Task enum](https://docs.rs/meilisearch-sdk/0.33.0/meilisearch_sdk/tasks/enum.Task.html))
- **Implementation implication for Flora:** `flora-search::SearchService` currently discards task handles (`let _task = ...`) for `setup_index`, `index_*`, `delete_document`. For deterministic tests (and potentially for stronger runtime guarantees), add/test helpers that either:
  - return task info/uids from indexing methods (API change), or
  - provide an internal/test-only "index and wait" path that awaits completion, or
  - wait on tasks via client/index APIs after invoking operations in tests if handles are accessible.
  **Without this, tests cannot be made reliably deterministic except by fragile sleeps — avoid.**
- **Settings/index setup ordering:** `set_filterable_attributes` and document additions are async; if tests depend on filters/facets/index readiness, wait for each relevant task (or ensure ordering guarantees) before querying.
- **Timeouts:** choose bounded, environment-aware timeout (e.g. a few seconds locally, higher but capped in CI) and small poll interval; on timeout dump diagnostic context (task uid/status if available). Avoid unbounded waits.

### 4.4 Asserting parity, precedence, and degradation

**Parity (Meilisearch vs PG fallback):** compare normalized contract-level outcomes, not brittle engine internals. Suggested normalizations:
- Compare sets/maps of returned `{type,id}` (and optionally key snippet invariants), sorted deterministically.
- Ignore score/rank differences and Meilisearch-specific highlighting unless part of contract.
- Assert same visibility rules: tenant isolation, soft-delete exclusion, type filters, empty-query behavior, limit caps.

**Precedence:** when `search_service` is `Some`, route must use Meilisearch path (not PG fallback). Observable signals: controlled indexed data present only in Meilisearch index / not matching PG FTS expectations, or instrumentation hooks if added later. With current black-box route, prefer designing fixtures where Meilisearch and PG would differ in controlled ways (e.g. content indexed in Meilisearch but not present in PG-searchable fields/rows, or vice versa) — but keep this maintainable.

**Degradation/fallback:** 
- `search_service: None` => PG fallback results (already covered by 019-021 DB tests).
- Init failure/unconfigured search => `AppState::from_config` logs and sets `None`; health reports `search: false/Some(false)` (`is_some()` semantics: current code returns `Some(state.search_service.is_some())`).
- Optional: simulate Meilisearch unavailable by constructing state with `None` (deterministic) rather than flaky network faults. True runtime-failure fallback during query is a separate behavior (route currently propagates Meilisearch error via `?` in Meilisearch branch?) — verify and test explicitly if 022 requires it.

### 4.5 Cost/stability controls (constrained CI)
- Run Meilisearch-gated tests **only in an opt-in job** (label/path filter/manual or dedicated workflow stage), not in default `build` job.
- Pin Meilisearch image tag (avoid `latest`) for reproducibility.
- Reuse one Meilisearch service per job; isolate with **unique index names/org IDs per test** instead of restarting container.
- Keep corpora tiny (few docs/tasks/files), `limit` small, serial execution for gated suite if needed.
- Set strict timeouts and fail with actionable errors (task uid/status, URL/config source redacted).
- Avoid indexing+search assertions in tests that can be satisfied by existing PG fallback contracts unless specifically validating Meilisearch.

---

## 5. Axum router-level tests with optional external search service

### 5.1 Patterns (aligned with Flora)
- **In-process router tests:** build `Router` + `with_state(AppState)` and call with `tower::ServiceExt::oneshot` (and `ServiceExt::ready/call` for multi-request sequences on a cloned/mutable service). No TCP bind needed; fast and deterministic. ([Axum testing example](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs), [codeloopback Axum testing](http://codeloopback.dev/articles/axum-service-testing), [axum-skills testing](https://github.com/melonask/axum-skills/blob/main/references/testing.md))
- **State injection seams:** construct `AppState` directly (as Flora tests do) to force `search_service: None` vs `Some(...)`, custom config, and mocked/real repos. This is preferable to `AppState::from_config` for fine-grained contract tests.
- **Auth/tenant setup:** reuse existing helpers to seed user/org/membership/session/token and send `Authorization` + `X-Organization-ID` headers.

### 5.2 Optional search service matrix

| Test intent | `search_service` | DB | Meilisearch | Gate |
|---|---:|---:|---:|---|
| PG fallback behavior/correctness | `None` | required | no | DB env |
| Precedence: Meilisearch used when available | `Some(real)` | required (for auth/membership/repos today) | required | DB + Meili env |
| Parity: normalized results match fallback | run both states / compare | required | required | DB + Meili env |
| Degradation: disabled/unavailable => fallback or explicit error | `None` or init-fail equivalent | required | optional | DB (+ Meili absent) |
| Health/config signaling | `None`/`Some` | maybe | maybe | minimal/unit-router |

### 5.3 Recommendations
- Keep **fallback tests DB-gated only** (current trajectory) so they run in existing `db-tests` infra.
- Add **Meilisearch-path tests as doubly-gated** (`db` + `meili`) and `#[ignore]` with clear reason.
- For query-path Meilisearch errors, prefer deterministic construction (bad URL in a dedicated test that expects controlled failure) over flaky network interruptions, unless resilience semantics are explicitly specified.
- If future refactor introduces a search abstraction trait, fakes can cover precedence/parity cheaply; until then, real disposable Meilisearch is the highest-fidelity 022 option.

---

## 6. 80/20 feasible options for 022

| Option | Effort | Value | Determinism | CI cost | Recommendation |
|---|---|---|---|---|---|
| **O1. Add Meilisearch-gated router contract tests (minimal harness extension)**: new ignored `db_meili_*` tests/helpers in `flora-tests::contract::search`; env gate for Meilisearch URL/key; `make_test_state_with_search(...)`; unique org/index; explicit `wait_for_task` after index mutations (requires exposing/awaiting tasks in `SearchService` or test helper using SDK client); assertions for Meilisearch-path hit + basic parity subset vs PG fallback | Low-med | High | High if tasks awaited | Low-med (separate job/service) | **Best 80/20** |
| **O2. Introduce a small search-test harness module** in `flora-tests` (shared setup/wait/assert helpers, config builder, task polling wrapper, normalized result comparator) reused by search parity/precedence tests | Med | High (maintainability) | High | Same as O1 | Good if O1 grows beyond 2-3 tests |
| **O3. CI-only: add `meili-tests` job + run existing/future gated tests** (service `getmeili/meilisearch:<pinned>`, env vars, `--ignored --test-threads=1`) while keeping local opt-in docs | Low | High operational confidence | Depends on tests | Med in that job only | Pair with O1 |
| **O4. Make `SearchService` indexing APIs awaitable/test-observable** (return task info or add `*_and_wait` methods) | Low-med | Critical for deterministic Meili tests; useful in prod observability | High | None runtime beyond waits | **Enabler for O1** |
| **O5. `#[sqlx::test]` / template-clone / per-test DB isolation migration** | Med-high | Stronger DB isolation | Very high | Higher churn/complexity | Defer unless current shared-DB model flakes |
| **O6. Testcontainers for Postgres/Meilisearch in Rust tests** | Med | Local reproducibility | High | Higher in CI, Docker-dependent | Optional/local only; not first choice for constrained CI |
| **O7. Full parity matrix across all entity/type/filter/ranking cases** | High | Comprehensive | High but brittle if over-specified | Higher | Defer; start with representative contract-critical cases |

### Suggested 022 execution slice (80/20)
1. **Enable deterministic Meilisearch writes:** adjust `flora-search` API minimally so tests can await task completion (return `TaskInfo/Task` or provide wait helper); keep behavior compatible.
2. **Test helpers:** `meili_url_from_env() -> Option<...>`, build `SearchService` with unique `index_template`/org context, `wait_succeeded(task, timeout)` wrapper with clear errors.
3. **Add 2-4 gated router tests:**
   - Meilisearch-path returns indexed message/task/file (happy path, task-waited).
   - Precedence smoke: with `search_service: Some`, searchable via Meili-indexed content (contract-chosen fixture).
   - Parity smoke: normalized IDs/types for same seeded corpus in fallback vs Meili mode.
   - Degradation/health: `search_service: None` behaves as fallback; health reflects availability.
4. **CI:** new job `meili-tests` (or extend) with pinned Meilisearch service + Postgres, run only `db_meili_*` ignored tests serially; keep default jobs unchanged.
5. **Docs:** document required env vars, local Docker command, and timeout expectations.

---

## 7. Risks and determinism checklist

- [ ] **No fixed sleeps for indexing visibility** — wait on Meilisearch tasks with timeout.
- [ ] **Fail on task failure/timeout** with diagnostic output (task uid/status/error).
- [ ] **Unique indices/org data per test** to avoid cross-test contamination and parallel collisions.
- [ ] **Pin external service versions** in CI (Postgres/Meilisearch tags).
- [ ] **Serialize shared-resource gated suites** when needed (`--test-threads=1`).
- [ ] **Normalize comparisons for parity** (IDs/types/visibility), avoid rank/score brittleness.
- [ ] **Skip cleanly when gates unset**; never make default unit/integration runs depend on Meilisearch.
- [ ] **Keep tests cheap:** tiny datasets, targeted filters, bounded limits/timeouts.
- [ ] **Distinguish path selection vs query failure:** explicitly define/assert fallback-on-error behavior if required (current route propagates Meilisearch search errors in the `Some` branch).

---

## 8. Sources and citations

### Flora (local cross-check)
- `flora-tests/src/contract/search.rs`, `flora-tests/src/contract/auth.rs`, `flora-tests/src/contract/{messaging,websocket,files,notifications}.rs`
- `flora-api/src/routes/search.rs`, `flora-api/src/routes/health.rs`, `flora-api/src/state/app_state.rs`
- `flora-search/src/lib.rs`
- `flora-core/src/config/search.rs`, `flora-core/src/database.rs`, `flora-core/src/migrations/mod.rs`
- `.github/workflows/rust.yml`

### PostgreSQL / sqlx / Rust testing
- sqlx `#[sqlx::test]` macro (test DB mgmt, migrations, fixtures, cleanup): https://docs.rs/sqlx/latest/sqlx/attr.test.html
- sqlx GitHub: https://github.com/launchbadge/sqlx
- Advanced Rust testing — DB isolation (`#[sqlx::test]`, strategies): https://rust-exercises.com/advanced-testing/06_database_isolation/00_intro.html , https://rust-exercises.com/advanced-testing/06_database_isolation/01_sqlx_test
- Rust FAQ — test DB code with rollback: https://www.rustfaq.org/en/how-to-test-database-code-in-rust
- "A tale of TimescaleDB, SQLx and testing in Rust" (isolation/perf/`sqlx::test` internals): https://blog.exein.io/sqlx_testing-blog-post-by-bogdan
- "Database Tests for the Lazy" (per-test DB via `#[sqlx::test]`): https://mattrighetti.com/2025/02/17/rust-testing-sqlx-lazy-people
- testcontainers-rs docs: https://rust.testcontainers.org/ ; modules: https://rust.testcontainers.org/modules/
- "Rust SQLx Async Database Testing with Test Transactions and Testcontainers": https://helpmetest.com/blog/rust-sqlx-database-testing
- "Streamlining Rust Integration Tests with Ephemeral Database Instances": https://www.leapcell.io/blog/streamlining-rust-integration-tests-with-ephemeral-database-instances
- `postgres-test-harness` (template/fingerprint/clone isolation concept): https://github.com/bpcakes/postgres-test-harness
- pt-BR: "SQLx Migrations em Rust: PostgreSQL, CI e Deploy" (CI/isolated DB/migration testing concepts): https://rustlang.com.br/blog/sqlx-migrations-rust-postgresql-ci-deploy-2026

### Meilisearch (async tasks, waiting, SDK)
- Tasks and asynchronous operations: https://www.meilisearch.com/docs/learn/async/asynchronous_operations
- Working with tasks: https://www.meilisearch.com/docs/learn/async/working_with_tasks
- Rust quick start (includes `wait_for_completion`): https://www.meilisearch.com/docs/getting_started/sdks/rust
- meilisearch-sdk-rust task wait implementation/source: https://docs.rs/meilisearch-sdk/latest/src/meilisearch_sdk/task_info.rs.html , https://docs.rs/meilisearch-sdk/0.33.0/meilisearch_sdk/tasks/enum.Task.html
- meilisearch-rust repo: https://github.com/meilisearch/meilisearch-rust
- ja: Meilisearch async/task/index update discussion in practical guides (task polling context): https://opensourcelab.jp/articles/open-source-site-search

### Axum/router testing and CI services
- Axum testing example (`oneshot`): https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
- "Testing Axum Services..." (`oneshot`, `sqlx::test`, integration patterns): http://codeloopback.dev/articles/axum-service-testing
- axum-skills testing notes: https://github.com/melonask/axum-skills/blob/main/references/testing.md
- GitHub Actions — PostgreSQL service containers: https://docs.github.com/en/actions/use-cases-and-examples/using-containerized-services/creating-postgresql-service-containers
- GitHub Actions — service containers communication/networking: https://docs.github.com/en/actions/use-cases-and-examples/using-containerized-services/about-service-containers
- Meilisearch in GitHub Actions (example action; prefer pinned official image/service): https://github.com/moy2010/meilisearch-github-action

### Multilingual search evidence (coverage note)
- EN: sqlx docs, Meilisearch docs, GitHub docs, Axum/testing guides, testcontainers-rs.
- pt-BR: Rust Brasil SQLx migrations/CI article (https://rustlang.com.br/blog/sqlx-migrations-rust-postgresql-ci-deploy-2026).
- es/de/ja/zh-CN/fr: targeted queries issued during research; most authoritative primary specifications remain English-first (Meilisearch/sqlx/GitHub). Non-EN indexed technical sources reviewed where surfaced (e.g. ja: https://opensourcelab.jp/articles/open-source-site-search, https://dev.classmethod.jp/articles/using-meilisearch-official-mcp-server/ for async task/operational context); recommendations above rely on primary English docs for normative behavior.

---

## 9. Bottom line for 022

**Adopt O1+O3+O4 as the 80/20 path:** keep Flora's proven env-gated DB contract style, make Meilisearch indexing awaitable for tests, add a tiny doubly-gated Meilisearch router-contract slice with explicit task waits and normalized parity/precedence/degradation assertions, and run it in a separate pinned-service CI job. This maximizes determinism and signal while preserving cheap/stable defaults on constrained CI.