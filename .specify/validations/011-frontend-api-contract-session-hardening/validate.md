# Validation: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

**Date:** 2026-11-15  
**Base:** `dev@556b78cc8` (post cycle 010)  
**Verdict:** **Approved (local deterministic checks green; smoke/remote CI NV as noted)**

## Deterministic checks (local)

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo clippy -p flora-api -p flora-core -p flora-tests --all-targets -- -D warnings` | pass (fixed pre-existing/unfulfilled lint expectations in `flora-api` extractor/auth route surfaced by pedantic/warnings policy) |
| `cargo check -p flora-api -p flora-core -p flora-tests` | pass |
| `cargo test -p flora-core --lib` | pass (8 passed) |
| `cargo test -p flora-tests --lib` | pass (56 passed, 5 ignored) |
| `cargo test -p flora-api` | pass (7 passed, 1 ignored) |
| `cd apps/frontend && bun run check` | pass (0 errors, 0 warnings) |
| `cd apps/frontend && bun run lint` | pass (after formatting `ApiClient` and test lint fix) |
| `cd apps/frontend && bun run test:unit -- --run --project server` | pass (7 files, 16 tests) |
| `cd apps/frontend && bun run build` | pass |

## Smoke / runtime

- **Integrated dev smoke (`select org → workspace → task`):** **NV (not re-executed in this cycle/environment).** Host default port `5432` remains occupied by an external non-Flora process in this environment. The documented PG18 flow was unchanged functionally and was evidenced in cycle 010 (`smoke_rc=0`, `.specify/validations/010-.../validate.md`); this cycle added a concrete alt-port override example to `docs/dev-smoke.md` and contract/UX fixes compatible with that flow.
- **DB-backed ignored tests (5):** not run in default lib test; covered by CI `db-tests` job on PR (to be confirmed).

## Changes validated

- `flora-api/src/routes/tasks.rs`: `create_task` now persists `description` and `assignee_id` from request.
- `apps/frontend/src/lib/api/client.ts`: typed `ApiError` preserving HTTP status/body/code/field/message; 401 still triggers `authState.lockSession()`; robust body parsing (JSON/text/204).
- `apps/frontend/src/lib/state/AuthState.svelte.ts`: `setOrganization` clears `isInvalidated` (and resets workspace/lock state as before).
- `apps/frontend/src/routes/(app)/tasks/+page.svelte`: auth/org guard before load/refresh.
- `apps/frontend/tests/unit/AuthState.test.ts`, `apps/frontend/tests/integration/api-interceptor.test.ts`: updated for current behavior and error contract.
- `docs/dev-smoke.md`: concrete port-override example for host conflicts (NV/environmental).

## NV / limitations

- Remote CI required checks and merge state: NV until PR execution.
- Default-port smoke on hosts with `5432` conflict: NV/environmental (alt-port path documented).
- Global DB-backed session revalidation in `AuthExtractor` and broader tenancy hardening: intentionally deferred (follow-up) per assessment decision.
- OIDC/JWKS real e2e: NV.
