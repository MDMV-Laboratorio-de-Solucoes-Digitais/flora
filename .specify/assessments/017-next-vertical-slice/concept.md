# Concept: 017 — Secure binary file upload/download (slice B, 80/20)

**Verdict target:** **go** (scoped)

## 1) Minimal solution concept
Deliver a **real but minimal** file-bytes path for Seed/VPS: multipart (streaming) upload to local disk + streaming download, with Postgres metadata as source of truth, tenant/workspace authorization, server-generated contained paths, conservative size/type controls, and basic integrity/quota accounting — without introducing S3, presigned URLs, indexing pipelines, previews/transcoding, or multi-node object semantics.

## 2) Target behavior (in scope)
1. **Upload (binary):** authenticated endpoint accepting multipart file + required context (e.g. `workspace_id`), streamed to temp file under canonical storage root, hard byte cap enforced during copy, filename sanitized/display-only, server-generated storage path/key, basic MIME allowlist + best-effort magic-byte/content checks where low-risk, compute actual size and SHA-256 checksum, persist metadata/commit, atomic move/finalize, cleanup on failure.
2. **Download (binary):** authenticated/authorized endpoint streaming file bytes from local disk by file id with safe headers (`Content-Type`/detected or stored, `Content-Disposition` as appropriate, `X-Content-Type-Options: nosniff`, length when known) and tenant isolation (403/404 semantics consistent).
3. **Metadata/quota/authz:** keep/align metadata endpoints as needed; enforce org/workspace membership and ownership/tenant checks; apply conservative per-file limit and minimal aggregate/recorded quota behavior consistent with existing config/schema (document semantics; avoid overengineering reservations).
4. **Wiring/config:** use a single canonical local storage path rooted in `FLORA_STORAGE__LOCAL_PATH` with containment checks; wire storage/backend into `AppState`/`FileService` (or minimal adapter) and honor cheap-VPS defaults.
5. **Frontend alignment (minimal):** update `apps/frontend` file upload/download components/API calls/types to match the implemented 017 contract for the happy path (no broad UI redesign).
6. **Verification:** deterministic tests for path safety/validation/quota/checksum behavior where feasible, route/service tests for happy/error paths, DB-gated contract tests on disposable Postgres 18 if applicable, and CI green.

## 3) Likely files/components (to refine via audit)
- `flora-api/src/routes/files.rs`, `flora-api/src/routes/mod.rs`, `flora-api/src/state/app_state.rs`
- `flora-core/src/services/file_service.rs`, `flora-core/src/models/file.rs`, file repository/traits, `flora-core/src/storage/local.rs` (+ `flora-core/src/traits/storage_provider.rs`), `flora-core/src/config/storage.rs`
- `flora-core/src/migrations/sql/*` (only if additive schema changes required)
- `apps/frontend/src/lib/components/files/*` and related API/client/types
- `specs/017-*`, `.specify/validations/releases/017-*`, `docs/dev-smoke.md` (if helpful)

## 4) Risks / mitigations
1. **Security (path traversal, spoofed types, unauthorized access):** server-generated paths, canonical containment, allowlist+magic-byte checks, strict tenant/authz on read/write/delete.
2. **Memory/DoS on 4 GB VPS:** stream multipart to disk, enforce body/stream caps, avoid buffering; document reverse-proxy limits.
3. **Contract/schema drift:** keep additive/minimal API changes, align FE to actual contract, add targeted tests.
4. **Quota semantics ambiguity:** implement minimal, explicit, tested behavior (per-file + simple aggregate check if feasible) and record NVs.

## 5) NV / non-goals
- NV: real 2vCPU/4GB load/OOM, restore drills, AV scanning, resumable/chunked uploads, CDN/S3/presigned, content indexing/previews, multi-node storage, browser e2e matrix.
- Non-goals: Macro parity for docs/CRDT/canvas/email/agents, new mandatory services, broad search/notification rewiring beyond file metadata needs.
