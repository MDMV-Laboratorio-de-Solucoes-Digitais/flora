# Requirements Checklist: 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

- [x] DB-backed automated tests for `select-organization` added and env-gated/`#[ignore]` (success/rotation, non-member, missing/inactive/mismatched session).
- [x] DB-backed tests executed successfully against Postgres (local, 5 passed) with migrations/unique fixtures.
- [x] CI updated with separate `db-tests` job (Postgres service) while preserving existing `build` job.
- [x] Integrated smoke `select org → workspace → task` executed with evidence (rc=0) using reproducible seed/bootstrap flow (non-conflicting service ports/env overrides); NV/compatibility notes recorded.
- [x] Minimal frontend slice implemented (login state persistence, callback state validation + select-org redirect, token rotation on org select, app guard, workspace/task API flow).
- [~] Frontend deterministic verification (`check/lint/build/tests`) — NV/partial depending on local toolchain availability; to be confirmed in CI/prepared env.
- [x] NV/gaps and next 80/20 documented in cycle artifacts.
