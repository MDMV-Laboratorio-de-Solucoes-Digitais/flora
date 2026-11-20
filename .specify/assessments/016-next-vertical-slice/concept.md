# Concept: 016 — Seed/VPS operability hardening (slice C)

**Date:** 2026-11-16  
**Verdict target:** **go** (scoped) — **primary slice: C (Seed/VPS operability hardening)**

## 1) Solution concept (minimal, low-risk, 80/20)

Harden the Seed runtime so **Postgres remains the only mandatory dependency** and optional capabilities (**search/Meilisearch, realtime messaging/Valkey**) are **profile- and config-gated with explicit degraded behavior**. Align pool/init, compose, env contract, and minimal backup/restore guidance for a **2 vCPU / 4 GB RAM** single-node VPS — without adding mandatory services or broad feature work.

### Design principles

- **Postgres-first correctness:** DB + migrations + core API must boot deterministically; optional deps never block core boot unless explicitly configured as required.
- **Explicit optionality:** `enabled`/profiles drive initialization and health/capability reporting (or clear logs) — no silent assumptions.
- **Cheap-VPS defaults:** conservative pool/limits, no always-on heavy components in lean profile.
- **Additive/debloat:** prefer config/wiring/compose/docs over new abstractions; avoid schema/product-surface expansion.

## 2) In-scope behavior (target)

1. **Single pool/init path:** create pool and run migrations once; inject/reuse pool in `AppState` (remove duplicate `create_pool`).
2. **DB pool config honored:** `flora-core/src/database.rs` uses `DatabaseConfig` (`max/min_connections`, acquire/idle timeouts) with safe small-VPS defaults.
3. **Search truly optional:** gate `SearchService` init on `config.search.enabled`; on disabled skip init; on enabled-but-failing choose explicit policy (fail-fast with clear message) and expose degraded/unavailable handling as needed — adjust state type/call sites minimally.
4. **Messaging/Valkey config aligned:** make `messaging` optionality truthful in config (e.g. `enabled` flag or allow empty URL when disabled) consistent with existing runtime fallback; no behavior regression for core REST/WS auth paths.
5. **Compose Seed profiles & limits:** `core` (postgres [+ api when introduced later, NV]), `realtime` (valkey), `search` (meilisearch) or equivalent; per-service `mem_limit`/`cpus`/reservation guidance compatible with target runtime, healthchecks retained, localhost binds, persistent volumes; conservative Meilisearch env (`MEILI_MAX_INDEXING_MEMORY`/threads) and Valkey `maxmemory`+policy when enabled.
6. **`FLORA_*` env contract:** document canonical keys (`FLORA_DATABASE__*`, `FLORA_MESSAGING__*`, `FLORA_SEARCH__ENABLED/URL/API_KEY` as supported), align `.env.example` (including OIDC/Zitadel naming notes), mark SQLite as unsupported runtime path (docs/guardrail) unless trivial fix.
7. **Backup/restore basics:** minimal runbook for `pg_dump` + local storage volume backup, off-host retention note, and periodic restore drill on disposable PG18 (tie to `docs/dev-smoke.md`); document Valkey/Meili as rebuildable/optional.

## 3) Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Gating search breaks routes/call sites expecting `Arc<SearchService>` | Compile/runtime failures | Minimal API: `Option<Arc<_>>` or null/degraded adapter; audit usages in `flora-api`/`flora-search` consumers; keep disabled-path deterministic. |
| Making `valkey_url` optional changes validation for existing envs | Boot/config regressions | Additive compatibility: accept legacy required URL, prefer explicit `enabled`/empty-disabled semantics with clear docs; validate combinations. |
| Pool downsizing affects throughput | Slower under concurrency | Conservative defaults (e.g. 5–10) appropriate for 4 GB; keep configurable via `FLORA_DATABASE__*`; document tuning. |
| Compose `deploy.resources` not enforced in non-Swarm mode | False sense of safety | Document verification (`docker stats`, runtime behavior), provide `mem_limit/cpus` compatible forms where useful, treat as guardrails not guarantees. |
| Meilisearch memory flags are hints not hard caps | OOM risk if enabled on 1–4 GB | Recommend **search profile off** for lean Seed; when on, cap indexing memory/threads and rely on container limits + monitoring. |
| Env renames confuse operators | Misconfig | Keep legacy fallbacks only if already supported/additive; clearly mark canonical keys and migration notes in `.env.example`/docs. |

## 4) NVs (not claimed/verified in 016 assessment)

- NV-1: No runtime/load benchmarks on 2 vCPU/4 GB; numeric defaults require empirical tuning.
- NV-2: `deploy.resources` enforcement and cgroup behavior depend on Docker/Compose mode/host; must be verified.
- NV-3: Exhaustive route/feature coupling to search or messaging beyond inspected boot paths not fully proven here.
- NV-4: Full OIDC/Zitadel env unification and SQLite runtime support may exceed low-risk scope.
- NV-5: No CI E2E with optional-service matrices; compose/profile validation partly manual.
- NV-6: ko/ru research evidence is triangulated, not locale-primary authoritative retrieval.

## 5) Debloat notes (aligned with north star)

- **Default lean profile:** `postgres + api` only; no mandatory Meilisearch/Valkey/JVM/broker.
- **Disable > constrain:** prefer turning search off for Seed over running constrained Meilisearch on 4 GB.
- **Fewer connections/processes:** config-driven pool, single init path, modest `max_connections` guidance.
- **No new infra/components:** reuse existing crates/services; optional adapters only where required by gating.
- **Truthful config:** remove/soften misleading “optional”/fallback surfaces that do not match runtime.

## 6) File-by-file implementation checklist (cycle 016 execution)

> Planned targets (confirm exact symbols/tests at implementation time; keep diff minimal and crate-local).

### API / runtime
- `flora-api/src/main.rs` — remove duplicate pool/migration ownership: initialize once and pass/inject reusable pool+config handles (or centralize startup) while preserving fail-fast Postgres/migrate behavior and startup capability logs (DB/search/valkey status).
- `flora-api/src/state/app_state.rs` — accept/reuse injected pool (no internal `create_pool`); gate Meilisearch init on `config.search.enabled`; represent search as optional/degraded (`Option<Arc<...>>` or equivalent) and update construction/logging; keep Valkey best-effort fallback and align with messaging config semantics; preserve realtime hub/auth wiring.
- `flora-api/src/routes/*` (only as needed) — handle unavailable search/messaging gracefully where endpoints depend on them (minimal degraded responses/feature-off behavior), avoid panics/unwraps.

### Core config / DB
- `flora-core/src/database.rs` — apply `DatabaseConfig` (`max_connections`, `min_connections`, `acquire_timeout_secs`, `idle_lifetime_secs`) to `PgPoolOptions`; keep `Result`/error mapping.
- `flora-core/src/config/database.rs` — adjust defaults/docs for small VPS if appropriate (conservative pool), clarify SQLite fields as unsupported runtime in comments/docs-level behavior (or minimal validation message) without breaking Postgres path.
- `flora-core/src/config/search.rs` — ensure `enabled` is authoritative for lean profiles; validation remains compatible when disabled.
- `flora-core/src/config/messaging.rs` — align validation with runtime optionality (add `enabled` or permit empty URL when intentionally disabled) while preserving topic/fanout settings; document canonical env.
- `flora-core/src/config/mod.rs` — keep load/validation coherent with new optionality; optional additive legacy-key compatibility only if low-risk and evidenced.
- `flora-core/src/config/app.rs`, `storage.rs`, `oidc.rs` — docs/comments alignment only if needed for truthful env contract (no behavioral churn).

### Infra / env / docs
- `docker-compose.yml` — introduce profiles/overlays for `core` vs `realtime` (`valkey`) vs `search` (`meilisearch`) (or documented equivalent), add resource limits/reservations/healthcheck guidance for 4 GB, keep `127.0.0.1` binds and persistent volumes, add conservative Meilisearch/Valkey runtime flags when enabled; avoid committing real secrets.
- `.env.example` — canonical `FLORA_*__*` keys and examples (including `FLORA_SEARCH__ENABLED=false` lean profile, pool knobs once wired), clarify OIDC/Zitadel variable names, storage/backup notes, remove/flag misleading comments.
- `docs/dev-smoke.md` — update for profile-based bring-up (`core` default), optional service notes, and restore-drill pointer; keep PG18 `uuidv7` guidance.
- `docs/research/` or `docs/ops/` (if repo convention permits) — add concise Seed backup/restore runbook (Postgres `pg_dump`/restore, storage path, optional Valkey/Meili rebuildability) and 4 GB tuning starting points; **else** include in `docs/dev-smoke.md`/README section as minimal doc change (verify allowed scope at implementation).
- `.github/workflows/rust.yml` (only if needed) — no required change for this scope; ensure existing `db-tests` remain compatible.

### Tests (deterministic)
- `flora-core` unit tests — config validation/load behavior for `search.enabled=false`, messaging optionality combinations, pool config application helpers (where testable without live DB).
- `flora-api` tests — startup/state construction behavior with search disabled (mock/unit-level seams if present) and no duplicate-pool assumptions; route behavior when search unavailable (if touched).
- `flora-tests` / integration (DB-gated, PG18 disposable) — only if pool/init refactor affects DB bootstrap contracts; otherwise rely on existing contracts plus targeted new cases.
- Compose sanity (manual/NV-documented): `docker compose config` for profiles and `docker stats` observation guidance.

## 7) Deterministic verification plan

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings` (document any preexisting/unavoidable scoped exception; do not blanket-allow)
3. `cargo test -p flora-core --lib`
4. `cargo test -p flora-api`
5. `cargo test -p flora-tests --lib`
6. DB-gated (Postgres 18 disposable/alt-port ok), if impacted or to confirm no regressions:  
   `FLORA_DATABASE__POSTGRES_URL=... cargo test -p flora-tests <relevant_contract_filter> -- --ignored --test-threads=1`
7. `cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build` (only if touched; otherwise record as NV/not required)
8. Compose/profile checks (no services required to stay running): `docker compose config` and profile-specific config rendering as applicable.
9. **NV:** real 2 vCPU/4 GB load/memory/OOM validation, `docker stats` under traffic, off-host backup restore on production-like storage, multi-service CI matrix, live browser e2e, OIDC/JWKS flows.

## 8) Explicit non-goals (016)

- **Not slice B:** no multipart/streaming binary upload/download implementation, no blob storage writes, no checksum/quota overhaul, no files schema/FE contract changes except incidental doc notes.
- No new mandatory services, no Valkey/Meilisearch required for core boot, no Kafka/OpenSearch/FusionAuth/LocalStack adoption.
- No multi-node realtime/fanout, no durable outbox/streams, no search indexing feature work beyond availability gating.
- No full OIDC/Zitadel redesign, no SQLite runtime implementation, no S3/object-storage rollout.
- No performance tuning beyond conservative defaults/config wiring and documented starting points.
- No deploy/production rollout, merges/releases, secrets/services provisioning, `apps/web` migration, or unverified “green” claims.