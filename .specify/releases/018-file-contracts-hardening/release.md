# Release: 018 — File Slice Hardening + DB-gated Contracts

## Summary
- Implements 018 slice A1 per `.specify/assessments/018-next-vertical-slice/decision.md`.
- Hardens file API: validates `workspace_id` exists and belongs to the authenticated organization for multipart upload and legacy metadata create; improves multipart/input determinism (single file, required workspace via query/form with clear validation) and preserves tenant-safe access behavior for read/download/delete.
- Aligns frontend upload to send `workspace_id` as query parameter (and form field) for robust contract compatibility.
- Replaces placeholder file contract tests with DB-gated tests on disposable PostgreSQL 18 + temp storage covering core happy/error/isolation/authz/delete/legacy cases.

## Verification
- `cargo fmt --all -- --check` ✅
- `cargo clippy -p flora-api -p flora-tests --all-targets -- -D warnings` ✅
- `cargo test -p flora-core --lib` ✅ (8)
- `cargo test -p flora-api` ✅ (11 passed, 1 ignored)
- `cargo test -p flora-tests --lib` ✅ (48 passed, 30 ignored)
- DB-gated file contracts: added and compile; execution in CI db-tests/disposable PG (NV locally without DB URL)
- CI to confirm full gates before merge.

## NV / Follow-ups
- Aggregate/concurrent quota reservation/accounting and stronger purge/storage-DB consistency guarantees.
- Broader DB-gated coverage (quota boundaries, multipart edge cases, cleanup on failure, workspace membership model refinements).
- 2vCPU/4GB/proxy/restore validation, browser e2e.
- 019: wiring essencial busca/notificações (PG FTS fallback + 1 evento in-app) or further file/operability slices per updated planning.
