# Problem: 016 — Why Seed/VPS operability is the 80/20 blocker now

**Date:** 2026-11-16

## Problem statement

Flora’s north star requires a **reproducible, self-hosted Seed operable on a cheap VPS (2 vCPU / 4 GB RAM)** with a **Postgres-first, debloated** profile. Current code/config state contradicts this in ways that create avoidable boot failures, resource pressure, and operator ambiguity — reducing confidence that the core (post-015) can run reliably on the target host and slowing safe delivery of subsequent P0 slices (notably real binary files).

## Evidence of the problem

1. **Optional services not truly optional at boot**
   - `flora-api/src/state/app_state.rs` unconditionally initializes Meilisearch (`SearchService::new(...)?`) and returns `Search service init failed` on error, despite `SearchConfig.enabled`/“optional for MVP” intent (`flora-core/src/config/search.rs`, `.env.example`).
   - `MessagingConfig::validate()` requires non-empty `valkey_url` (`flora-core/src/config/messaging.rs`), while runtime connection is best-effort with degraded `None` paths — config contract and runtime behavior are misaligned for a lean profile.

2. **Resource/connection behavior unsafe for 4 GB by default**
   - `flora-core/src/database.rs` hardcodes `max_connections(20)` and ignores `DatabaseConfig` pool/timeouts.
   - `flora-api/src/main.rs` creates pool + runs migrations, then `AppState::new` creates another pool — duplicate initialization/connection pressure risk.

3. **Compose not shaped for cheap VPS Seed**
   - `docker-compose.yml` starts `postgres`, `valkey`, `meilisearch` unconditionally with no `profiles`, no `deploy.resources` limits/reservations, dev secrets/keys inline, and no API service contract; suitable for dev convenience but not a least-resource Seed profile.

4. **Configuration/operability contract drift**
   - `.env.example` mixes canonical `FLORA_*__*` keys with non-prefixed `OIDC_*`/`ZITADEL_*` and comments that do not fully reflect runtime naming/behavior; SQLite config surface exists but runtime is Postgres-only — false “lean fallback”/operator confusion risk.
   - No minimal, repo-integrated backup/restore runbook for Postgres + local storage as a Seed operational baseline.

5. **Sequencing pressure**
   - **B (binary files)** is a real P0 gap but introduces higher-risk multipart/FS/security/schema/FE-contract work. Executing B on an unstable/ambiguous runtime profile increases verification cost and regression risk on 2 vCPU/4 GB.

## Impact if unaddressed

- Lean Seed cannot be started reliably without Meilisearch (or with intentional degraded optionality).
- Higher likelihood of memory/connection contention on 4 GB hosts (unbounded containers, fixed pool, duplicate pools).
- Operator errors from env/profile ambiguity; weak recovery posture without minimal backup/restore guidance.
- Slower, less deterministic validation of future P0 vertical slices.

## Desired outcome (016)

A **truthful, minimal, VPS-friendly operability baseline**: core `api+postgres` can run with optional `realtime/search` profiles; boot respects `enabled`/degraded modes for optional deps; DB pool/init is config-driven and non-duplicative; compose provides safe defaults/limits/healthchecks guidance for 4 GB; env contract and backup/restore basics are documented/aligned — all with deterministic verification and explicit NVs.