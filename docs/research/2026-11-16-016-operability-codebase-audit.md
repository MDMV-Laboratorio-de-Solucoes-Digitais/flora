# 016 — Operability/Seed VPS codebase audit (readiness)

**Date:** 2026-11-16  
**Scope:** implementation-readiness for Seed/VPS operability hardening (candidate C).  
**Source:** background audit subagent `ses_e1dc44e8affeuxZM64S6XI4jEH` (read-only; file persisted by orchestrator).  
**Cross-check:** `docs/research/2026-11-16-015-seed-vps-operability-quickwins.md` (aligns on boot contradictions, pool/config/env/compose gaps).

## Summary

- Startup is effectively Postgres/migrations-hard and duplicates pool initialization; search is configured as optional in docs but treated as required in boot/routes; messaging is runtime-degraded in behavior but not cleanly optional in config contract.
- Health/readiness endpoints are placeholders (always 200) and do not reflect DB/search/Valkey capability.
- Compose provides local infra with basic healthchecks and localhost binds but lacks profiles, resource caps, and production-safe secret/backup posture; env naming/contract has mismatches.

## Current behavior and gaps (evidence)

### Startup/boot hard dependencies
- `flora-api/src/main.rs:27-35` — loads+validates config, creates pool + runs migrations, then `AppState::new`; Postgres/migrations are fail-fast.
- `flora-api/src/state/app_state.rs:67` — creates a second pool (duplicate init).
- No capability/readiness summary logging at startup.

### DB pool/config
- `flora-core/src/database.rs:14-20` — hardcodes `max_connections(20)` and ignores `DatabaseConfig` fields.
- `flora-core/src/config/database.rs:12-33` — defines defaults/max/min/timeout/idle fields not fully wired into runtime pool creation.
- SQLite fields exist in config surface but runtime is Postgres-only (`database.rs:48-66`).

### Search (not truly optional)
- `flora-core/src/config/search.rs:18-47` — defaults `enabled=true`, requires URL when enabled.
- `flora-api/src/state/app_state.rs:49,125-132` — always builds `SearchService` and hard-fails on init.
- `flora-api/src/routes/search.rs:89-92` — unconditionally calls search service (`routes/mod.rs:37`).
- `.env.example:29-32` — states optional but documents commented `FLORA_SEARCH__MEILISEARCH_URL/KEY` (mismatch vs config `url/api_key` and possible legacy `MEILISEARCH_URL` naming).

### Messaging/Valkey
- Runtime-degraded behavior: `app_state.rs:134-152,168-170`; `routes/websocket.rs:309-320` (best-effort publish).
- Config contract requires non-empty URL: `flora-core/src/config/messaging.rs:43-47` (default `redis://localhost:6379`), so "optional" is not explicit/clean.
- `flora-messaging/src/lib.rs:39-99` assumes a connected manager in its API shape/usage patterns.

### Health/readiness
- `flora-api/src/routes/health.rs:9-30` (`/health`, `/health/ready`, `/health/live`) always return 200 with TODO comments; no DB/search/Valkey checks despite optional/degradable dependencies.

### Compose/env
- `docker-compose.yml:6-61` — runs postgres/valkey/meilisearch with healthchecks, localhost binds, volumes; but no `profiles`, no resource limits/reservations, dev credentials/`MEILI_MASTER_KEY`, and no API service.
- `.env.example` — mixes canonical `FLORA_*__*` with non-prefixed `OIDC_*`/`ZITADEL_*` and incomplete pool/ops knobs.

## Low-risk checklist (file-by-file, candidate)

- **Search optionality/degraded mode:** gate search on `enabled` and use `Option`/adapter with graceful fallback in `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`, and related config/docs.
- **DB pool/init:** inject/reuse pool (avoid duplicate init) and wire `DatabaseConfig` into pool builder in `flora-api/src/main.rs`, `flora-api/src/state/app_state.rs`, `flora-core/src/database.rs`.
- **Messaging config contract:** add explicit `enabled` or clearly optional URL/contract in `flora-core/src/config/messaging.rs` and align docs/env.
- **Readiness/health:** implement real readiness/capability probes (DB required; search/Valkey reported as optional capabilities) in `flora-api/src/routes/health.rs`.
- **Compose/ops:** add profiles, resource caps/reservations, safer defaults/secrets guidance, and backup/restore notes in `docker-compose.yml` and `.env.example`.
- **Env alignment:** unify/clarify canonical `FLORA_*__*` naming and document persistence/restore expectations.

## NV
- Audit is static/read-only; runtime validation on target 2vCPU/4GB VPS and behavior under real service outages not exercised here.
