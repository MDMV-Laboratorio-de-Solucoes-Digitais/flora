# Validation: 018 — File Slice Hardening + DB-gated Contracts

## Scope
- Slice A1 (80/20): enforce workspace∈org validation on upload and legacy metadata create, harden multipart/input handling and tenant-safe behavior, preserve streaming/checksum/containment guarantees, and add DB-gated file contract tests (disposable PG18 + temp storage) replacing stubs.

## Local verification (2026-11-16)
- `cargo fmt --all -- --check` — **pass**
- `cargo clippy -p flora-api -p flora-tests --all-targets -- -D warnings` — **pass**
- `cargo test -p flora-core --lib` — **pass** (8 tests)
- `cargo test -p flora-api` — **pass** (11 passed, 1 ignored)
- `cargo test -p flora-tests --lib` — **pass** (48 passed, 30 ignored)

## DB-gated file contracts
- Added env-gated tests in `flora-tests/src/contract/files.rs` (`db_files`): upload/download/get happy path, workspace-not-in-org rejection, cross-org isolation (get/download/delete), missing workspace/empty file validation, delete owner/non-owner/cross-org semantics, legacy metadata workspace validation.
- **Local execution:** NV (no `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` configured in this session). Tests are `#[ignore]`/env-gated and expected to run in CI db-tests job (disposable PG) or any environment with PG + migrations.

## NV / not exercised
- Full aggregate/concurrent quota reservation and accounting under contention.
- Multipart tolerance for `workspace_id` form field appearing after `file` field (FE now sends workspace_id as query param; server requires ws in query or before file).
- Real 2vCPU/4GB load/OOM, reverse-proxy/body-limit enforcement, restore/backup drills, browser e2e.
- Advanced MIME sniffing/AV, resumable/S3/presigned uploads, search/notifications wiring.
