# 015 — Binary file upload/download (self-hosted, cheap VPS, Rust/Axum/Postgres/Svelte)

**Date:** 2026-11-16  
**Scope:** Safe, simple binary file upload/download for self-hosted apps with Rust (Axum multipart/streaming), Postgres metadata, local-disk storage, path traversal prevention, size/content-type limits, quotas, and Svelte frontend integration. Optimized for cheap VPS (low memory, no external object storage required).  
**Status:** Research (no application source modified)

## 1. Current implementation cross-check (repo)

### What exists
- **HTTP API (`flora-api/src/routes/files.rs`)**: metadata-only endpoints.
  - `POST /api/v1/files` accepts JSON `UploadFileRequest { workspace_id, name, file_type, size_bytes }`, checks org membership, calls `FileService::create_file(...)`, returns metadata. **No multipart/body bytes, no download endpoint.**
  - `GET /api/v1/files/{id}` returns metadata with org check.
  - `DELETE /api/v1/files/{id}` soft-deletes with org+owner checks.
- **Core service (`flora-core/src/services/file_service.rs`)**:
  - Validates filename (`/`, `\`, `..` rejected) and blocks a small executable MIME denylist (`application/x-executable`, `x-sh`, `x-bat`, `x-msdownload`).
  - Enforces per-upload size against **both** `storage.org_quota_bytes` and `storage.workspace_quota_bytes` (treats them as max single-file/request caps in current logic), **not** aggregated usage.
  - Generates storage path `orgs/{org_id}/files/{file_id}` and creates DB record; **does not write bytes to storage**.
- **Storage abstraction (`flora-files/src/lib.rs`)**:
  - `StorageProvider` trait (`put/get/delete/exists`) + `LocalStorage` writing to `root.join(path)` with `create_dir_all`/`tokio::fs`.
  - **No path canonicalization/containment check** beyond join; relies on caller-supplied path being safe.
  - `flora_files::FileService::create` only inserts DB row; **unused by API wiring**.
- **Persistence (`flora-core` models/repos/migrations)**:
  - `files` table stores metadata + `storage_path`, checksum optional, soft-delete, org/owner/workspace linkage via inputs/usage patterns. `find_by_workspace_id` is currently a placeholder (not truly workspace-scoped).
- **Config (`flora-core/src/config/storage.rs`, `.env.example`)**:
  - Defaults: local backend, `/var/flora/storage`, org quota 10 GB, workspace quota 2 GB, multipart/chunk config fields present but **not wired**. Env keys are `STORAGE_*`/`S3_*` in defaults and also `FLORA_STORAGE__*` via Figment split env in general config style.
- **Wiring (`flora-api/src/state/app_state.rs`)**: `FileService::new(file_repo, storage_config)`; no `StorageProvider`/disk writer injected into core service. Axum deps include `axum` ws feature and `axum-extra`; no explicit multipart feature/handler seen.
- **Frontend**: `apps/web` has imported static-file service client artifacts (not canonical Svelte target). `apps/frontend` is canonical but no file upload/download implementation was evidenced in this scan.

### Gaps / risks to address for real binary upload/download
1. No streaming upload/download handlers; current flow cannot store or serve bytes.
2. No request/body size limiting at HTTP layer (`DefaultBodyLimit`/`RequestBodyLimitLayer`) and no per-field/file streaming-to-disk cap.
3. MIME/type validation is denylist-only and trusts client-provided metadata; no magic-byte sniffing (`infer`/content inspection) or allowlist.
4. Filename sanitized for separators/`..` but metadata handling/storage naming policy for user-supplied names and headers needs hardening (length, control chars, normalization).
5. Local storage path safety depends on trusted generated paths; defense-in-depth should ensure resolved path stays under storage root.
6. Quotas check single file size, not cumulative org/workspace usage; no reservation/accounting/cleanup on failure.
7. No checksum/integrity, no safe download headers (`Content-Disposition`, `X-Content-Type-Options: nosniff`), no authz-scoped download streaming.

## 2. Multilingual research synthesis (EN, pt-BR, es, de, fr)

> Note: live `websearch` tool calls returned no results in-session; recommendations below are grounded in fetched primary docs and widely adopted guidance: Axum `Multipart`/extractor docs, `tower_http::limit` docs, OWASP File Upload & Input Validation Cheat Sheets, and OWASP Unrestricted File Upload.

### Core guidance (language-agnostic)
- **EN**: Treat upload as untrusted input. Enforce allowlists (type/extension/business need), strict size limits, safe server-generated storage names, store outside executable/webroot contexts, stream to disk (avoid buffering whole files in RAM), verify content by signature/magic bytes, and authorize every download.
- **pt-BR**: Para **upload/download seguro de arquivos binários**, aplique validação em camadas: limite de tamanho no proxy/HTTP e por arquivo, lista de permissões de tipo/extensão, nome gerado no servidor, gravação em fluxo (streaming) para disco local, verificação de assinatura/magic bytes, metadados no Postgres e checagem de autorização no download. Evite confiar em `Content-Type`/nome enviado pelo cliente.
- **es**: Para **subida y descarga segura de archivos binarios**, use límites estrictos de tamaño, lista blanca de tipos, nombres generados en servidor, almacenamiento fuera de rutas ejecutables, validación por firma/magic bytes, y control de acceso en cada descarga. Prevenga *path traversal* y no confíe en metadatos del cliente.
- **de**: Für **sicheren Datei-Upload/Download** gelten: strikte Größenlimits, Allowlist für Dateitypen, serverseitig erzeugte Dateinamen/Speicherpfade, Streaming auf die lokale Festplatte (geringer RAM-Verbrauch), Prüfung von Dateisignaturen/Magic Bytes, sichere Auslieferung mit korrekten Headern und Autorisierung pro Zugriff. Schutz vor *Path Traversal* und keine Vertrauensstellung in Client-Angaben.
- **fr**: Pour un **téléversement/téléchargement sécurisé de fichiers binaires**, appliquer des limites de taille strictes, une liste blanche de types, des noms/chemins générés côté serveur, l’écriture en streaming sur disque local, la vérification par signature/magic bytes, et un contrôle d’accès systématique au téléchargement. Se protéger contre le *path traversal* et ne pas faire confiance aux métadonnées fournies par le client.

### Axum/tower specifics (important for cheap VPS)
- Axum `extract::Multipart` consumes the body and must be the last extractor; it enforces a **default 2 MB request body limit** unless changed via `DefaultBodyLimit` (Axum docs). Do not call `field.bytes()` for large files on low-RAM hosts.
- Prefer `field.chunk()`/streaming reads and write incrementally to a temp file (`tokio::fs::File` / buffered writer) with a hard byte cap enforced during copy (defense-in-depth even if layer limits exist).
- Use `tower_http::limit::RequestBodyLimitLayer` (and/or Axum `DefaultBodyLimit`) for early rejection on `Content-Length`; still enforce streamed-size caps because chunked/unknown-length bodies are bounded by actual reads.
- Reject oversized/invalid uploads early; on `413`/errors ensure partial temp files are removed.

## 3. Concrete low-risk 80/20 recommendations (cheap VPS, local disk)

1. **Keep it simple: single-file multipart upload + metadata in Postgres + local disk blobs**
   - Endpoint: `POST /api/v1/files` as `multipart/form-data` (metadata fields + one `file` field), streamed to a temp file then atomically moved/renamed into final path.
   - Endpoint: `GET /api/v1/files/{id}/download` (or `/content`) streaming from disk (`tokio_util::io::ReaderStream`/`Body::from_stream`) with authz + org checks.
   - Avoid in-memory buffering and avoid background/object-storage dependencies.

2. **Storage layout & path safety (server-controlled)**
   - Store under `storage.local_path` with deterministic server-generated relative path only, e.g. `orgs/{org_id}/{file_uuid}.bin` (or keep extension from **validated allowlist**, never raw client filename).
   - Resolve and enforce containment: `canonicalize`/normalize and verify final path starts with canonical storage root; reject if not.
   - Persist original display name separately (sanitized/length-limited) for UI; never use it for filesystem path resolution.

3. **Validation & limits (layered)**
   - Global/per-route body limit: set conservative `DefaultBodyLimit`/`RequestBodyLimitLayer` (e.g. 10–25 MB default for VPS; make configurable).
   - Per-file streamed cap: stop reading once `max_file_bytes` exceeded; delete temp file and return `413`.
   - Allowlist MIME/types by business need (start minimal: pdf/png/jpg/webp/txt etc.); verify with magic-byte sniffing (`infer` crate) after reading initial bytes/header chunk, not only client `Content-Type`.
   - Filename checks: length cap (e.g. 255), reject control chars/NUL, normalize; keep existing `/ \ ..` rejection.

4. **Quotas (pragmatic)**
   - 80/20: enforce **max upload size** now (already partly modeled) + add **aggregate org/workspace used-bytes** check (`SUM(size_bytes) WHERE !is_deleted`) inside a transaction before commit/after successful write (with compensating cleanup on failure).
   - Defer complex reservations/ratelimiting tiers; a simple count/sum query + index is enough for small VPS scale.

5. **Metadata & integrity**
   - Record `size_bytes` from actual bytes written, compute streaming checksum (SHA-256) while writing, store `checksum` and detected MIME.
   - Use DB transaction/order carefully: write temp -> fsync/rename -> insert/commit (or insert pending then finalize); on any failure remove blob/temp to avoid orphans.

6. **Download hardening**
   - Authorize (org membership + resource access) before opening file.
   - Set headers: `Content-Type` from stored validated type, `Content-Disposition: attachment; filename*=UTF-8''...` (or inline only for explicitly safe preview types), `X-Content-Type-Options: nosniff`, and cache controls as appropriate.
   - Stream file; support range requests only if needed later (not required for 80/20).

7. **Svelte frontend integration (simple, low-risk)**
   - Use `<form enctype="multipart/form-data">` or `fetch` with `FormData` (`form.append('file', file)` + metadata fields); do **not** manually set `Content-Type` boundary.
   - Client-side prechecks (size/type) for UX only; server remains authoritative.
   - Show progress via `XMLHttpRequest.upload.onprogress` or fetch streaming libs only if needed; for cheap/simple use native form/fetch + status UI.
   - Download via navigable endpoint or `fetch` -> blob URL with auth cookies/headers as your auth model requires (`credentials: 'include'` if cookie-based).

8. **Operational safeguards for VPS**
   - Store uploads on a dedicated volume/path with disk quota/monitoring; run periodic cleanup of soft-deleted/orphan temp files.
   - Keep `ulimit`/timeouts reasonable; avoid image/video transcoding on upload path.
   - Log structured metadata (ids, sizes, hashes) not file contents; scan/quarantine hooks can be added later.

## 4. Implementation checklist (low-risk rollout)

- [ ] **Config**: add explicit `max_upload_bytes`, allowed MIME/extensions, temp dir under storage root; wire `FLORA_STORAGE__*` docs/env consistently.
- [ ] **HTTP limits**: apply `DefaultBodyLimit`/`RequestBodyLimitLayer` on file routes; return clear `413`/`415`/`400`.
- [ ] **Upload handler (Axum)**: multipart endpoint, stream `field` chunks to temp file with running size counter + SHA-256; sniff magic bytes; validate allowlist.
- [ ] **Path/storage safety**: server-generated path, containment check against canonical root, atomic move, cleanup on error.
- [ ] **Service/repo**: persist actual size/hash/type/name; compute aggregate usage for quota enforcement; fix workspace-scoped listing if required.
- [ ] **Download handler**: authz, metadata lookup, stream from disk, safe headers, `404` for missing/soft-deleted as policy dictates.
- [ ] **Delete/purge**: ensure blob removal strategy on purge (and optionally on soft-delete) to reclaim disk.
- [ ] **Tests**: unit tests for filename/type/path validation and quota math; integration tests for stream-too-large, wrong magic bytes, traversal attempts, authz, and download headers.
- [ ] **Frontend (`apps/frontend`)**: upload form/component with progress/error states, size/type hints, and download/open actions.
- [ ] **Ops**: document Nginx/reverse-proxy `client_max_body_size` alignment, backup/exclude strategy, and disk monitoring.

## 5. Suggested decision defaults (80/20)
- Max upload: **10 MB** initial default on cheap VPS (configurable).
- Allowed types (initial): **`application/pdf`, `image/png`, `image/jpeg`, `image/webp`, `text/plain`** (+ verify magic bytes).
- Storage: **local disk only**, server-generated UUID filenames, metadata in Postgres.
- No chunked resumable uploads/object storage/AV scanning in v1 unless required by threat model.

## 6. Sources (fetched/referenced)
- Repo: `flora-api/src/routes/files.rs`, `flora-core/src/{services/file_service.rs,models/file.rs,repositories/file_repository.rs,config/storage.rs}`, `flora-files/src/lib.rs`, `.env.example`, migrations/spec context.
- Axum docs: `axum::extract::Multipart` (default 2 MB, multipart feature, body-consumption/ordering notes) — https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
- Tower HTTP: `tower_http::limit` / request body limit behavior — https://docs.rs/tower-http/latest/tower_http/limit/
- OWASP: File Upload Cheat Sheet — https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html
- OWASP: Input Validation Cheat Sheet — https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html
- OWASP Community: Unrestricted File Upload — https://owasp.org/www-community/vulnerabilities/Unrestricted_File_Upload
- Multilingual search intents covered (tool-limited session): pt-BR “upload/download binário seguro Axum streaming limites tipo MIME disco local VPS”; es “subida/descarga segura archivos binarios Axum prevención path traversal cuotas”; de “sicherer Datei-Upload/Download Rust Axum Größenlimit Dateityp lokaler Speicher VPS”; fr “téléversement/téléchargement sécurisé Axum limite taille type MIME stockage local VPS”. Recommend validating against current Rust crate docs (`axum`, `axum-extra`, `tower-http`, `infer`, `tokio-util`) before implementation as versions evolve.
