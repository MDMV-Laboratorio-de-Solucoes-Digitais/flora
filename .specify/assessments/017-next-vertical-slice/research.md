# Research: 017 — Secure binary file upload/download (slice B)

**Date:** 2026-11-16  
**Method:** synthesis of 016/015 research + codebase audit + in-progress multilingual deep research (EN, pt-BR, es, de, fr, zh-CN, ja; ko/ru as available).

## Primary inputs
- `docs/research/2026-11-16-016-binary-file-upload-hardening.md` — Axum multipart/streaming, layered limits, allowlist+magic-byte, server-controlled paths/containment, temp+atomic rename, SHA-256/checksum, aggregate quota guidance, safe download headers/authz, FE alignment, NVs.
- `docs/research/2026-11-16-016-macro-files-ops-parity.md` — Macro file/doc value is coupled to ingestion/parsing/collab/search/agents; Flora Seed 80/20 = durable tenant-scoped blob+metadata+basic permissions/quotas+download on local disk/Postgres.
- `docs/research/2026-11-16-015-binary-file-upload-download.md`, `docs/research/2026-11-16-015-files-impl-readiness.md` (prior baseline).
- 017 deep-dive outputs (pending/linked): `docs/research/2026-11-16-017-secure-binary-upload-download-vps.md`, `docs/research/2026-11-16-017-macro-files-mvp-parity.md`.

## Codebase cross-check (current)
- **API (`flora-api/src/routes/files.rs`):** metadata-only `POST /api/v1/files` (JSON `workspace_id,name,file_type,size_bytes`), `GET /{id}` metadata, `DELETE /{id}` soft-delete with auth/membership/owner checks. No multipart upload, no binary download route.
- **Service/model (`flora-core/src/services/file_service.rs`, `flora-core/src/models/file.rs`):** validates filename traversal chars and blocks limited executable MIME set; enforces per-request size against quota config values; generates `storage_path`; persists DB record only (no bytes write/read, no server-observed checksum/size/MIME, aggregate quota/accounting NV, workspace linkage/listing semantics need confirmation).
- **Storage/config/wiring:** `flora-core/src/storage/local.rs` and `flora-files` abstractions exist but are not wired into a real bytes flow; `StorageConfig` has local/S3/multipart/quota fields; `AppState` injects config into `FileService` without bytes storage backend.
- **Frontend (`apps/frontend/src/lib/components/files/{FileUpload.svelte,FilePreview.svelte}`):** expects multipart upload and download endpoint/contract (per 016 audit) — contract mismatch with current API.
- **DB schema:** `files` table exists in init migration with `storage_path`, `checksum`, `size_bytes`, `file_type`, soft-delete metadata (details to confirm for workspace/accounting needs).

## Consensus (80/20, Seed/VPS)
- Keep: authenticated tenant-scoped metadata + local-disk blob storage, server-generated paths, streaming I/O, conservative limits, basic checksums/quotas, safe download headers.
- Replace/simplify: Macro ingestion/parsing/CRDT/canvas/cloud-object complexity; avoid S3/CDN/search indexing/prescans in 017.
- Defer: S3/presigned/resumable, AV/transcoding/previews, content indexing, multi-node object semantics, advanced DMS/versioning/collab docs.
