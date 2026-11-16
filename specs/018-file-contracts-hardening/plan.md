# Plan: 018 — File Slice Hardening + DB-gated Contracts

## Inputs
- Assessment: `.specify/assessments/018-next-vertical-slice/decision.md`
- Research: `docs/research/2026-11-16-018-file-contracts-hardening-vps.md`, `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`, `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`, `docs/research/2026-11-16-018-macro-search-notifications-parity.md`
- Audits: `ses_e1d8d03e6ffehEXmO6kEraL1aX`, `ses_e1d8b443cffe9g6ewGSBRnZNNn`

## Design (minimal, safe)
1. **Workspace/tenant validation:** resolve workspace by id and ensure `workspace.organization_id == org_id` (and required membership/access) in file upload and metadata-create paths; use consistent 404/403/not-found semantics for cross-tenant/missing resources. Apply consistent org/workspace-aware checks to get/download/delete.
2. **Multipart hardening:** parse multipart fields robustly (accept `workspace_id` in query or form field in any order), enforce single `file` part and required fields, deterministic validation errors.
3. **Quota/cleanup semantics:** keep per-file cap/checksum; add minimal aggregate usage query/check for org/workspace if low-risk (best-effort transactional/advisory guard for Seed), otherwise codify explicit behavior; ensure best-effort cleanup on failure and reasonable delete/purge behavior.
4. **Storage parity:** align `LocalFileSystem` trait-path operations with containment/sanitization expectations used by streaming paths.
5. **DB-gated tests:** add env-gated helpers/fixtures in `flora-tests` (PG URL, migrations, seeded user/org/role/membership/session/workspace, JWT, temp storage config, multipart builder, router `oneshot`) and implement core contract cases (C1–C6/C8 subset + key integration cases) as feasible and deterministic.

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib` and targeted tests
- `cargo test -p flora-api`
- `cargo test -p flora-tests --lib` (non-DB) and, when `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` available, DB-gated file contract tests (`-- --ignored` or env-gated as implemented)
- `apps/frontend` checks only if FE touched (not expected)

## Risks/NVs
- Authz tightening and legacy metadata endpoint semantics must remain explicit/tested.
- Aggregate quota concurrency correctness may be partial; document NV if not fully transactional.
- DB-gated tests depend on disposable PG availability in CI/local; keep env-gated and deterministic.
