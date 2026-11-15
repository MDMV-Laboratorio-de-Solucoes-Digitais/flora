# Requirements Checklist: 008 — Select-Organization Tests & Reproducible Dev Smoke

- [x] CI-safe automated tests added for `select-organization` critical validation/auth cases (no live DB required).
- [x] New/affected tests pass locally (`cargo test -p flora-tests --lib contract::auth`).
- [x] Reproducible dev smoke path documented (docker compose + seed + session/JWT bootstrap without OIDC) with deterministic artifacts.
- [x] Known runtime/contract nuances and NV limitations explicitly documented (DB-backed cases in CI, integrated runtime execution, assignee-scoped task list).
- [x] No production API behavior changes introduced; scope limited to tests/docs/scripts/artifacts.
- [~] DB-backed automated cases (success, non-member, invalid/inactive/missing session) — not added as CI-runnable in this increment (NV in CI without Postgres); preconditions documented.
- [~] Integrated smoke executed end-to-end against running services in this environment — NV.
