# Decision: 017 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**B — Secure binary file upload/download (local disk, Postgres metadata, Seed/VPS 2 vCPU/4 GB).**  
Primary 80/20 for cycle 017: implement a minimal real bytes path (streaming multipart upload + streaming download) with tenant/workspace authorization, server-controlled contained storage paths, conservative limits/validation, basic checksum/integrity metadata, and minimal frontend/API contract alignment — without S3/presigned/resumable uploads, content indexing, previews/transcoding, or multi-node storage.

## Justification (evidence)

- **North-star/P0 fit:** 016 decision explicitly deferred slice B after completing slice C; `OBJETIVO.md`/`ROADMAP.md` identify real binary upload/download, coherent quotas/metadata, and local storage as core Seed gaps.
- **Current gap:** `flora-api/src/routes/files.rs` is metadata-only (no multipart, no download); `FileService`/storage abstractions are not wired for bytes; frontend file components expect upload/download behavior (016/015 research and codebase audit).
- **80/20/debloat:** delivers concrete SMB value (actual file exchange) while avoiding Macro-coupled complexity (auto-import/parsing/CRDT/canvas/cloud pipelines). Local disk + Postgres aligns with cheap-VPS, Postgres-first strategy.
- **Research basis:** `docs/research/2026-11-16-016-binary-file-upload-hardening.md`, `docs/research/2026-11-16-016-macro-files-ops-parity.md`, `015` file research, and in-progress 017 deep-dive research/audit artifacts.

## Scope summary (in)

- Streaming multipart upload and streaming download endpoints with authz/tenant isolation.
- Local-disk storage wiring with canonical root containment, temp+atomic finalize, cleanup on failure.
- Basic validation/limits: size caps, filename sanitization, server-generated paths, MIME allowlist and/or best-effort sniffing, SHA-256/checksum and actual size persisted.
- Minimal quota/metadata coherence consistent with existing config/schema (explicit semantics; additive changes only if needed).
- Minimal `apps/frontend` file component/API alignment for happy path.
- Deterministic/unit/route/contract tests as feasible; DB-gated tests on disposable Postgres 18 when applicable; CI green.

## Key risks

1. **Security regressions (traversal, spoofing, authz bypass):** mitigate with containment, server-generated keys, allowlist/sniffing, and strict tenant/ownership checks.
2. **Resource exhaustion on 4 GB hosts:** mitigate with streaming, hard byte caps, no in-memory buffering, conservative defaults.
3. **API/FE/schema mismatch:** mitigate with minimal additive contract, targeted tests, and explicit NVs.

## NVs / non-goals

- NV: 2vCPU/4GB load/OOM/restore drills, AV scanning, resumable/chunked CDN-style uploads, S3/presigned, content indexing/previews, multi-node storage semantics, full browser e2e matrix.
- Non-goals: Macro document/collab parity, new mandatory services, broad rewires unrelated to files slice.


## Consolidated inputs (completed)
- Research: `docs/research/2026-11-16-017-secure-binary-upload-download-vps.md`, `docs/research/2026-11-16-017-macro-files-mvp-parity.md`, prior 015/016 file research.
- Codebase audit (017): routes metadata-only/no download; FE expects multipart `/upload` and `/download/{id}` with different DTO; storage trait/impl buffer-based and unwired; `files` lacks `workspace_id` and repo/listing gaps; quota/checksum/streaming/containment not implemented; `flora-files` duplicate unwired.
