# Validation: 016 — Seed/VPS Operability Hardening

## Scope
- Slice C only (Seed/VPS operability): Postgres-only mandatory, search/messaging optional/profile-gated, single DB init/pool path, compose/env/docs alignment, minimal backup/restore notes.

## Commands & Results
- `cargo check -p flora-core -p flora-api -p flora-tests` — **pass** (dev profile).
- `docker compose config -q` — **pass**.
- `cargo fmt --all -- --check` — pending/long-running in this environment (rerun in CI/clean env if needed).
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` — pending.
- `cargo test -p flora-core --lib` — pending.
- `cargo test -p flora-api` — pending.
- `cargo test -p flora-tests --lib` — pending.
- DB-gated contracts on disposable Postgres 18 — NV/pending as applicable.

## Notes / NV
- Compose resource limits are guardrails; enforcement varies by runtime (`docker stats`).
- No 2vCPU/4GB load/OOM or off-host restore drill executed.
- Real OIDC/JWKS, browser/multi-client e2e, and full optional-service CI matrix remain NV.
- `apps/web` not canonical/touched.
