# 015 — Files/storage codebase audit (binary upload/download readiness)

**Date:** 2026-11-16  
**Scope:** implementation-readiness for a minimal, safe binary file upload/download flow on local disk (cheap VPS).  
**Source:** background audit subagent `ses_e1de14d32ffeM0x4WKp8dk8dJH` (read-only; file persisted by orchestrator).

## Summary

- Current implementation is metadata-only: API creates/lists/deletes file records as JSON, with no multipart upload, no binary download/streaming, and no real blob persistence path.
- Storage abstractions exist in `flora-files` and traits/config exist in `flora-core`, but service/API/state wiring does not use a real storage provider for bytes.
- Frontend (canonical `apps/frontend`) and e2e expectations assume `/api/v1/files/upload` and `/api/v1/files/download/{id}` endpoints that are not implemented by the current API.
- Minimal MVP requires real storage wiring, streamed upload/download, tenant/auth scoping, path safety, size/content checks, quota enforcement, and metadata/cleanup consistency.

## Current behavior (evidence)

### API (metadata-only)
- `flora-api/src/routes/files.rs:51-159` — registers `POST /`, `GET /{id}`, `DELETE /{id}` (JSON create/list-meta/delete style); no upload/download/binary endpoints, no multipart/streaming.

### Domain/service
- `flora-core/src/models/file.rs:13-82` — stores `storage_path/checksum/metadata` but input is metadata-oriented.
- `flora-core/src/services/file_service.rs:59-109` — validates name/MIME and checks quota against request `size_bytes`; inserts DB record only (no bytes, no `StorageProvider` usage, no actual stored-bytes accounting).

### Storage/wiring
- `flora-core/src/traits/storage_provider.rs:12-31` — defines async `put/get/delete/exists`.
- `flora-files/src/lib.rs:15-149` — separate `StorageProvider` + `LocalStorage` implementation, but not wired into the active file service path.
- `flora-api/src/state/app_state.rs:129-132` — wires `flora_core::services::FileService(file_repo, Arc<StorageConfig>)` (no storage backend for bytes).

### Schema/config
- `flora-core/src/migrations/sql/20240818000001_init_schema.sql:98-112,177-180` — `files` table has `storage_path/checksum/is_deleted`, RLS enabled, no workspace FK.
- `flora-core/src/config/storage.rs:7-75` — exposes local/S3 + quota/multipart knobs, but env handling may bypass intended `FLORA_` merge and local path safety is not enforced end-to-end.

### Frontend mismatch (canonical `apps/frontend`)
- `apps/frontend/src/lib/components/files/FileUpload.svelte:20-33` — posts multipart to `/api/v1/files/upload`.
- `apps/frontend/src/lib/components/files/FilePreview.svelte:7-24` — downloads from `/api/v1/files/download/{id}`.
- `apps/frontend/tests/e2e/files.spec.ts:4-44` — asserts those routes.
- `apps/frontend/src/lib/utils/files.ts:1-4` — enforces 50MB client-side only.

## Key gaps for minimal safe local-disk MVP

- Add binary routes: multipart upload and streamed download (and align/list/delete semantics as needed).
- Inject and use a real storage provider with canonicalized root and tenant/org-scoped paths (prevent traversal/escape).
- Enforce auth + org/workspace scoping and ownership/permission checks on read/write/delete.
- Validate actual content length, enforce server-side size limits, and compute/persist checksum.
- Persist metadata after successful write (or roll back/clean up blob on failure); handle soft-delete blob lifecycle.
- Enforce aggregate quotas based on stored usage, not just request-declared size.
- Align config limits with frontend behavior and add deterministic tests (unit/contract, DB-gated as applicable).

## Minimal file-by-file checklist (candidate)

- `flora-api/src/routes/files.rs` — upload/download handlers, auth/tenant checks, streaming responses, limits.
- `flora-api/src/state/app_state.rs` — wire storage provider/service dependencies.
- `flora-core/src/services/file_service.rs` — real store/retrieve/delete flow, quota/accounting, validation.
- `flora-core/src/traits/storage_provider.rs` — ensure trait fits streaming/metadata needs (or extend minimally).
- `flora-core/src/config/storage.rs` — enforce safe defaults/path validation and `FLORA_*` config behavior.
- `flora-core/src/models/file.rs`, `flora-core/src/repositories/file_repository.rs` — metadata/checksum/size/workspace linkage as needed.
- `flora-files/src/lib.rs` (or consolidate with core storage impl) — safe local-disk implementation.
- DB migration(s) only if required (workspace linkage/constraints) — prefer additive/safe changes.
- `apps/frontend/src/lib/{api,components/files,types,utils/files.ts}` and tests — align with implemented API contract.
