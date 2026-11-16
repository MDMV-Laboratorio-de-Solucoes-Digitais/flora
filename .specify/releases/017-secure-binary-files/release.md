# Release: 017 — Secure Binary File Upload/Download

## Summary
- Implements 017 slice B (80/20) per `.specify/assessments/017-next-vertical-slice/decision.md`.
- Adds additive `files.workspace_id` migration and persists workspace-scoped file metadata.
- Implements contained local-disk streaming storage (temp + atomic finalize), SHA-256 checksum and actual-size capture, and conservative upload limits.
- Adds multipart upload (`POST /api/v1/files/upload`) and streaming download (`GET /api/v1/files/{id}/download`, FE-compatible `/api/v1/files/download/{id}`) with authz/tenant checks and safe headers; keeps metadata/delete endpoints aligned.
- Aligns frontend file upload component to send `workspace_id` and use 017 contract.

## Verification
- `cargo fmt --all -- --check` ✅
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` ✅
- `cargo test -p flora-core --lib` ✅
- `cargo test -p flora-api` ✅
- `apps/frontend` `bun run check` ✅
- CI to confirm full gates before merge.

## NV / Follow-ups
- Add DB-gated file contract tests (upload/download/authz/containment/quota) on disposable PG18.
- Validate behavior on real 2vCPU/4GB VPS and with reverse-proxy body limits.
- Strengthen MIME sniffing/policy, aggregate quota accounting, and attachment/linkage semantics as needed.
- Consider consolidation of `flora-files` duplicate abstractions in a later cycle.
