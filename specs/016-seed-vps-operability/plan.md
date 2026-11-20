# Plan: 016 — Seed/VPS Operability Hardening

## Inputs
- Decision: `.specify/assessments/016-next-vertical-slice/decision.md` (go, slice C)
- Research: `docs/research/2026-11-16-016-seed-vps-operability-hardening.md`, `docs/research/2026-11-16-016-operability-codebase-audit.md`, `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docs/research/2026-11-16-016-postgres-vps-backup-tuning.md`

## Design choices (minimal/safe)
- **Single init:** `main` creates pool, runs migrations, passes pool into `AppState::new_with_pool` (or updated signature); `database::create_pool` honors `DatabaseConfig` (conservative defaults for 4GB).
- **Search optional:** `search_service: Option<Arc<flora_search::SearchService>>`; if disabled or init fails in degraded-safe path, log and continue with None; search route returns clear unavailable response when None.
- **Messaging optional:** add `enabled` to `MessagingConfig` (default true for compat) and skip Valkey connection when disabled; keep degraded behavior on connect failure when enabled.
- **Health:** `/health/ready` performs lightweight DB check; `/health` and `/health/live` remain simple; include capability hints if low-risk.
- **Compose/env/docs:** profiles and resource guidance in compose, canonical `FLORA_*` vars in `.env.example`, dev-smoke/backup notes updated for lean core profile.

## Files (expected)
- `flora-api/src/{main.rs,state/app_state.rs,routes/{health.rs,search.rs,mod.rs}}`
- `flora-core/src/{database.rs,config/{mod.rs,database.rs,search.rs,messaging.rs}}`
- `docker-compose.yml`, `.env.example`, `docs/dev-smoke.md` (+ minimal backup/restore notes)
- `flora-tests` / `flora-api` / `flora-core` tests and AppState test constructors
- `.specify/{validations,releases}/016-seed-vps-operability/*`, `OBJETIVO.md`, `ROADMAP.md`

## Verification
- `cargo fmt --all -- --check`, `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib`, `cargo test -p flora-api`, `cargo test -p flora-tests --lib`
- DB-gated contracts on disposable PG18 if impacted
- Frontend checks if touched (not expected for slice C core)
- `docker compose config` sanity where applicable
