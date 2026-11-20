# Tasks: 016 — Seed/VPS Operability Hardening

- [x] T016.1 Wire config-driven DB pool and single init path (main/AppState/database/config) with conservative Seed defaults.
- [x] T016.2 Make search and messaging truly optional/degraded-aligned (config, AppState init, route/health handling, env contract).
- [x] T016.3 Update compose/env/docs for lean Seed profiles, limits/healthchecks, canonical `FLORA_*`, and minimal backup/restore guidance.
- [x] T016.4 Update deterministic tests and test AppState constructors for new optionality/init behavior; fix regressions (compile/check).
- [ ] T016.5 Run full verification (fmt/clippy/tests, DB-gated as applicable, compose config), record SDD evidence, open PR to `dev`, ensure CI green, merge/sync, update OBJETIVO/ROADMAP.
