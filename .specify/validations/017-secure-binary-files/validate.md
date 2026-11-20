# Validation: 017 — Secure Binary File Upload/Download

## Scope
- Slice B (80/20): streaming multipart upload + streaming download on local disk, Postgres metadata, workspace scoping (additive migration), server-generated contained paths, SHA-256 checksum, basic MIME/size validation, authz/tenant checks, and FE/API contract alignment.

## Local verification (2026-11-16)
- `cargo fmt --all -- --check` — **pass**
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` — **pass**
- `cargo test -p flora-core --lib` — **pass** (8 tests)
- `cargo test -p flora-api` — **pass** (11 passed, 1 ignored)
- `cargo test -p flora-tests --lib` — **pass** (if run in this session; see command log)
- `apps/frontend`: `bun run check` — **pass**

## NV / not executed
- DB-gated file-specific contract tests on disposable Postgres 18 (not added in this slice).
- Full frontend `lint`, `test:unit -- --run`, `build` and e2e against real API.
- 2vCPU/4GB load/OOM, proxy limit enforcement, restore/backup drills, concurrent aggregate quota correctness.
- Advanced magic-byte library (`infer`), AV scanning, resumable/S3/presigned uploads, content indexing/previews.
