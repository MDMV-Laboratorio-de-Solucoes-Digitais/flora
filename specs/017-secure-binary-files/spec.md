# Feature Specification: 017 — Secure Binary File Upload/Download (80/20)

**Feature Branch**: `feat/017-secure-binary-files`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/017-next-vertical-slice/{intake,research,problem,concept,decision}.md` (verdict: go, slice B)

## Goal
Deliver a minimal, secure, Seed/VPS-safe binary file slice: streaming multipart upload and streaming download on local disk with Postgres metadata, tenant/workspace authorization, server-generated contained paths, conservative limits/validation, basic checksum/integrity, and aligned frontend/API contract for the happy path.

## Scope
- API: `POST /api/v1/files/upload` (multipart `file` + required `workspace_id`), `GET /api/v1/files/{id}` metadata, `GET /api/v1/files/{id}/download` (or FE-compatible download path), `DELETE /api/v1/files/{id}` (soft-delete + best-effort local cleanup as feasible). Keep/align existing behavior compatibly where low-risk.
- Streaming I/O: no whole-file buffering in memory; stream upload to temp file under canonical storage root, enforce byte cap during copy, atomic finalize/move; stream download from disk.
- Security/validation: authenticated org context + membership/workspace access checks, tenant isolation on read/download/delete, server-generated storage key/path with canonical containment (reject traversal/symlink escape), filename sanitization/display-only, basic MIME allowlist and best-effort sniffing, SHA-256 checksum and actual size persisted.
- Storage/config/wiring: use `FLORA_STORAGE__LOCAL_PATH` canonical root, wire storage backend into file service/state, conservative `max_upload_bytes` and quota semantics (per-file limit; minimal aggregate/accounting improvement where feasible, explicit NV otherwise).
- Schema: additive migration for `workspace_id` on `files` (and index/FK as appropriate) if required for correct scoping/listing; persist and use it.
- Frontend (`apps/frontend`): align file upload/download components and types/API calls to 017 contract (happy path), including `workspace_id` in upload and correct download URL/response shape.
- Tests: deterministic unit/route/service coverage for validation/path safety/checksum/contract where feasible; DB-gated file contract tests on disposable Postgres 18 when applicable.

## Non-goals
- S3/presigned/resumable/chunked uploads, CDN, previews/transcoding, AV scanning, content indexing/search wiring, attachment graph to messages/tasks beyond current needs.
- Macro CRDT docs/canvas/collab parity, multi-node object semantics, new mandatory services.
- 2vCPU/4GB load/OOM/restore benchmarking, full browser e2e matrix, broad notification/search rewires.

## NV
- Aggregate quota accounting under concurrency may be minimal/approximate unless transactional checks implemented; document behavior.
- Compose/runtime limit enforcement and real VPS perf not validated here.
- Existing `flora-files` duplicate crate remains unwired unless explicitly consolidated (prefer minimal safe changes in `flora-core`/`flora-api`).
