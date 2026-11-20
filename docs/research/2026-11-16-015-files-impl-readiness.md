# 015 — Files binary upload/download implementation readiness (local disk, minimal safe MVP)

**Date:** 2026-11-16  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (read-only analysis; this document only)  
**Inputs:** `docs/research/2026-11-16-015-binary-file-upload-download.md`, `docs/research/2026-11-16-015-files-codebase-audit.md`, `flora-api/src/routes/files.rs`, `flora-api/src/state/app_state.rs`, `flora-core/src/{models/file.rs,services/file_service.rs,traits/storage_provider.rs,storage/local.rs,config/storage.rs,repositories/file_repository.rs}`, `flora-files/src/lib.rs`, `apps/frontend/src/lib/components/files/{FileUpload.svelte,FilePreview.svelte}`, `apps/frontend/src/lib/{api/client.ts,types/models.ts,utils/files.ts}`, `apps/frontend/tests/e2e/files.spec.ts`, migration `20240818000001_init_schema.sql`.

## 1. Goal and constraints

Deliver a **low-risk, minimal, self-hosted** binary file flow on **local disk** (cheap VPS), aligned with existing frontend expectations, without changing application source in this task.

- **Frontend contract today:** `POST /api/v1/files/upload` (multipart `FormData` with `file`) expecting `FileAttachment { id,fileName,mimeType,byteSize,rustFsUrl,uploadedAt }`; `GET /api/v1/files/download/{id}` for download (`FileUpload.svelte`, `FilePreview.svelte`, `files.spec.ts`).
- **Backend reality today:** metadata-only JSON `POST /api/v1/files` + `GET/DELETE /{id}`; no bytes stored/served; `FileService` records DB row only; storage impls not wired (`app_state.rs` injects `Arc<StorageConfig>`).
- **Repo constraints:** Rust 2024, strict lints (no `unwrap/expect/panic/todo/unsafe` in prod), `Result`/`tracing`, crate-local changes preferred, config via `FLORA_*` (`__` nesting).

## 2. Multilingual research synthesis (EN, pt-BR, es, de, fr)

Grounded in Axum `Multipart`/`DefaultBodyLimit` docs, `tower_http::limit::RequestBodyLimitLayer`, and OWASP File Upload/Input Validation guidance (fetched 2026-11-16). `websearch` unavailable in-session; `npx -y skills find "file upload streaming multipart axum rust storage security"` returned no matches.

| Lang | Key guidance for this MVP |
|---|---|
| **EN** | Treat uploads as untrusted: stream multipart fields to disk (never buffer whole file in RAM), enforce layered size limits (`DefaultBodyLimit`/`RequestBodyLimitLayer` + byte-counted write cap), server-generated storage paths/names, allowlist + magic-byte sniffing (not client `Content-Type`), canonicalized root containment to prevent path traversal, authz on every download, safe headers (`Content-Disposition`, `X-Content-Type-Options: nosniff`), cleanup temp/partial blobs on failure. |
| **pt-BR** | **Upload/download binário seguro:** gravar em fluxo para disco local, limitar tamanho no HTTP e por arquivo, lista de permissões de tipo/extensão, nome/caminho gerado no servidor, validar assinatura/magic bytes, metadados no Postgres, autorização no download e prevenção de *path traversal*; não confiar em nome/tipo do cliente. |
| **es** | **Subida/descarga segura de archivos binarios:** límites estrictos, lista blanca, nombres de servidor, almacenamiento fuera de contextos ejecutables, validación por firma/magic bytes, control de acceso en descarga y defensa contra *path traversal*; limpieza de temporales ante errores. |
| **de** | **Sicherer Datei-Upload/Download:** strikte Größenlimits, Allowlist der Dateitypen, serverseitig erzeugte Pfade, Streaming auf lokale Festplatte (wenig RAM), Prüfung von Magic Bytes, sichere Auslieferung mit korrekten Headern und Autorisierung pro Zugriff; Schutz vor *Path Traversal*. |
| **fr** | **Téléversement/téléchargement sécurisé de fichiers binaires:** limites de taille strictes, liste blanche, noms/chemins générés côté serveur, écriture en streaming sur disque local, vérification par signature/magic bytes, contrôle d’accès systématique et protection contre le *path traversal*. |

**Axum/tower specifics:** `Multipart` consumes body and must be last extractor; default body limit is 2 MB unless changed (`DefaultBodyLimit`). For cheap VPS, prefer `field.chunk()` loop + `tokio::fs::File`/`BufWriter` with hard cap (covers chunked bodies); reject early and remove temp files on any error.

## 3. Proposed API contract (minimal, FE-aligned)

### 3.1 Upload (new, canonical for frontend)
- `POST /api/v1/files/upload`
- **Auth:** `AuthExtractor`; requires org context; verifies membership (existing pattern).
- **Request:** `multipart/form-data`
  - required file part: `file` (binary stream). Ignore extra unknown fields for MVP.
  - required metadata field (text): `workspace_id` (UUID) — needed for tenant scoping/quota/listing; FE currently omits it, so FE must add `formData.append('workspace_id', ...)` (see §5.1). *Optional compatibility:* accept `organization_id`-scoped default only if product allows, else `400 validation`.
  - optional: `name` override unused; server derives display name from sanitized filename/header metadata but stores server-generated storage path.
- **Behavior:** stream to temp file under storage root, count bytes + compute SHA-256 + sniff type, validate limits/allowlist/quota, atomically persist, insert DB row, return metadata.
- **Response `201`:** JSON `FileAttachment`-compatible shape for current FE: `{ "id": "<uuid>", "fileName": "<sanitized display>", "mimeType": "<detected>", "byteSize": <u64>, "rustFsUrl": "/api/v1/files/download/<id>", "uploadedAt": "<rfc3339>" }`.
- **Errors:** `400` validation (missing field/bad name/type), `401/403` authz, `413` too large (route/body or stream cap), `415` unsupported media/type, `409/429-like` not needed (use `413`/`400` for quota in MVP unless dedicated error exists), `500` storage/db.

### 3.2 Download (new)
- `GET /api/v1/files/download/{id}` (and optionally keep `/api/v1/files/{id}/content` as alias later, not required).
- **Authz:** same org as file; deny cross-org; `404` for missing/soft-deleted (avoid existence leak across tenants) or `403` per existing style — prefer consistent `403` for cross-org, `404` for absent/deleted in-org.
- **Response `200`:** streamed body from disk (`ReaderStream`/`Body`), headers: `Content-Type: <stored validated mime>`, `Content-Disposition: attachment; filename*=UTF-8''<encoded display name>` (inline only if explicitly allowlisted safe preview types in future), `X-Content-Type-Options: nosniff`, `Content-Length` when known, conservative `Cache-Control` (e.g. `private, no-store` for MVP).
- **Errors:** `401/403/404/500`.

### 3.3 Existing metadata endpoints (keep, minimally adjust)
- Keep `GET /api/v1/files/{id}` and `DELETE /api/v1/files/{id}` for compatibility; may align response fields/add `uploadedAt`/`rustFsUrl` adapter if FE needs, but current FE path uses upload/download routes only.
- **Backward compat option:** keep JSON `POST /api/v1/files` as metadata-only behind feature flag or deprecate after FE switch; prefer routing FE to `/upload` and leaving legacy path unchanged in MVP to reduce risk.

## 4. Storage layout and safety model

- **Root:** canonicalized `storage.local_path` (must exist/be creatable and be a directory at startup validation; reject empty/unsafe configs).
- **Blob path (server-generated, never from client):** `orgs/{org_id}/workspaces/{workspace_id}/files/{file_id}` (no extension) or `.../{file_id}.bin` — fixed safe charset, no user-controlled segments. Persist relative path in `storage_path`.
- **Temp path:** `{root}/.tmp/{uuid}` for streaming writes; `fsync` file then atomic rename into final dir; delete temp on any failure.
- **Containment:** before any FS op, join relative path to canonical root and verify normalized/canonical target remains under root (`starts_with(root_canon)`), reject symlink escape attempts; do not follow untrusted symlinks.
- **Display name:** store sanitized original name in DB `name` (length <=255, strip/control chars/NUL, reject/normalize separators already handled) for UI/download filename; never use for path resolution.
- **Lifecycle:** soft-delete marks DB row; MVP should also delete blob on delete/purge path (or schedule purge) to reclaim VPS disk — choose one deterministic policy and test it.

## 5. File-by-file change plan (low-risk, crate-local)

### 5.1 `apps/frontend` (canonical, align to real multipart contract)
- `apps/frontend/src/lib/components/files/FileUpload.svelte`: append required metadata (`workspace_id` from current workspace context/prop/store) to `FormData`; keep client precheck but **align max size to server config** (read from env/config endpoint or shared constant) to avoid 50 MB client / smaller server mismatch; map response to `FileAttachment` fields.
- `apps/frontend/src/lib/components/files/FilePreview.svelte`: keep `/api/v1/files/download/{id}`; ensure auth headers/cookies sent as needed by `fetch` navigation context (anchor download relies on browser session/cookies — verify auth model; if bearer-only, use fetch->blob URL path).
- `apps/frontend/src/lib/types/models.ts`: confirm `FileAttachment` matches API (`uploadedAt`, `fileName`, `mimeType`, `byteSize`, `rustFsUrl`); extend if backend returns additive fields.
- `apps/frontend/src/lib/utils/files.ts`: make `MAX_SIZE` configurable/consistent with backend default.
- `apps/frontend/src/lib/api/client.ts`: already supports `FormData` without forcing JSON `Content-Type` — keep.
- `apps/frontend/tests/e2e/files.spec.ts`: update/add tests for `workspace_id` in multipart and success path once backend exists (deterministic mocks for network/error cases).

### 5.2 `flora-api`
- `flora-api/Cargo.toml`: enable Axum multipart support (`multipart` feature) and any needed extract/body util deps already in workspace (e.g. `tokio-util` stream, `futures-util` for `StreamExt`/`TryStreamExt` if required) — verify minimal additions.
- `flora-api/src/routes/files.rs`: add `POST /upload` multipart handler and `GET /download/{id}` streaming handler; keep existing routes; enforce org/workspace membership/authz; apply per-route body limit via `DefaultBodyLimit`/`RequestBodyLimitLayer` on upload route; map domain/storage errors to `413/415/400/403/404`.
- `flora-api/src/state/app_state.rs`: inject a real bytes-capable storage provider/service (compose `LocalFileSystem`/safe local impl from config root) into `file_service`; fail fast on invalid local storage config at startup.
- `flora-api/src/lib.rs`/router wiring: mount limits/layers as needed (no global behavior change except upload route).

### 5.3 `flora-core`
- `flora-core/src/config/storage.rs`: add explicit `max_upload_bytes` (default **10 MiB** for cheap VPS), allowed MIME allowlist, temp subdir name; prefer `FLORA_STORAGE__*` as source of truth in runtime config loading (audit `Config::load` merge behavior) while keeping safe defaults; validate local root non-empty, quotas/limits sane.
- `flora-core/src/traits/storage_provider.rs`: extend minimally for streaming-safe MVP **without breaking existing callers** — e.g. add `put_stream`/`open_read_stream`/`put_file` (temp-path/rename aware) or keep `put/get` and add a separate blob-store trait used by new service path; include path containment helper contract in impl docs.
- `flora-core/src/storage/local.rs` and/or consolidate with `flora-files/src/lib.rs`: implement canonical-root containment, safe parent creation, temp+atomic rename, streaming read handle for download, and no full-file buffering in upload path.
- `flora-core/src/services/file_service.rs`: refactor `create_file` into real upload flow (or add `upload_file`): accept streamed temp artifact metadata (size/hash/detected mime/storage path, workspace_id), enforce per-file max, validate allowlist+sniffed type, compute **aggregate usage** quota (`SUM(size_bytes)` active files by org and/or workspace) in a transaction/check, persist record with actual values, cleanup blob on DB failure; fix workspace semantics (currently ignored). Keep `validate_filename`/MIME checks but move to allowlist-first.
- `flora-core/src/models/file.rs`: extend input/response model as needed (workspace_id linkage, checksum set from server, detected mime). Consider additive fields only.
- `flora-core/src/repositories/file_repository.rs` & `traits`: add `sum_active_bytes_by_org`/`by_workspace`, `find_by_workspace_id` real implementation, and workspace-aware create if schema supports it (see §5.4); ensure soft-deleted excluded from quota/listing.
- `flora-core/src/error.rs` (if needed): map quota/unsupported type/too large distinctly for API.

### 5.4 Persistence / schema (additive, only if required)
- `files` table currently has no `workspace_id`; service/API/frontend expect workspace-scoped uploads. **Low-risk MVP choice:** add nullable/additive `workspace_id UUID` + FK/index in a new migration (safe rollout), backfill if needed, enforce NOT NULL for new uploads once wired; alternatively store workspace in `metadata` temporarily (riskier for quota/query correctness) — prefer explicit column.
- Keep `checksum`, `size_bytes`, `storage_path`, `file_type` authoritative from server-observed stream.

### 5.5 `flora-files` crate
- Currently parallel/unused implementation; either (a) **preferred low-risk:** make `flora-core::storage::local` the single canonical safe implementation used by API/core service and leave `flora-files` untouched/deprecated, or (b) harden and reuse it — avoid duplicate divergent path-safety logic.

## 6. Deterministic test strategy

### 6.1 Unit tests (fast, no DB/network, deterministic)
- Filename sanitization/length/control chars and rejection of separators/`..`/NUL.
- Allowlist MIME + magic-byte decision table (fixed byte fixtures for png/jpeg/pdf/webp/txt and a mismatched/spoofed case).
- Streamed size-cap enforcement: feed fixed-size chunk iterator/reader exceeding cap -> error and no final blob.
- Path containment: generated relative paths resolve under canonical root; crafted traversal/symlink-like inputs rejected.
- Quota math: aggregate used + new <= limit (boundary at exact limit), soft-deleted excluded.
- Checksum: known content -> stable SHA-256 hex.

### 6.2 API/contract tests (`flora-api`, integration-style as repo supports)
- `POST /api/v1/files/upload` happy path (multipart with `workspace_id`+`file`) returns `201` and `FileAttachment` shape; bytes persisted; DB row has actual size/hash/type.
- Reject: too large (`413`), disallowed/spoofed type (`415/400`), missing auth/org/workspace membership (`401/403/400`), cross-org download forbidden (`403/404` per policy).
- Download streams correct bytes/headers (`Content-Type`, `nosniff`, attachment filename).
- Cleanup: induced DB/storage failure leaves no orphan final blob/temp file (use temp test dir).

### 6.3 DB-gated tests (`flora-tests` or crate integration) — only where DB available
- Aggregate quota enforcement with real `SUM` queries.
- Workspace-scoped listing once `workspace_id` is implemented.
- Soft-delete visibility and purge/blob lifecycle behavior.

### 6.4 Frontend tests (deterministic)
- `apps/frontend` unit/component: upload appends `workspace_id`, handles `413/415` error messages, displays server-aligned max size.
- E2E (`files.spec.ts`): keep mocked network-failure/oversize cases; add mocked success response shape for `/api/v1/files/upload` and download link assertion.

## 7. NVs (non-goals / deferred risks)

- **NV1:** No resumable/chunked multipart protocol, no S3/object storage, no CDN, no image/video transcoding/AV scanning in MVP (add hooks later).
- **NV2:** No presigned direct-to-disk URLs; local backend remains server-proxied.
- **NV3:** Full-text/search indexing of file contents out of scope.
- **NV4:** Complex quota reservations/rate limiting/abuse detection beyond max size + aggregate sum + authz.
- **NV5:** Existing RLS policies in DB are defense-in-depth placeholders; application-level `organization_id` checks remain mandatory.
- **NV6:** Frontend auth model for `<a download>` vs bearer tokens must be confirmed during implementation (cookie session vs header) — may require fetch-to-blob flow.
- **NV7:** Reverse-proxy alignment (`client_max_body_size` in Nginx/Caddy) and disk monitoring/backups are operational requirements, not code-only fixes.

## 8. Recommended rollout defaults (80/20)

- `max_upload_bytes`: **10 MiB** (configurable, cheap VPS).
- Allowlist (initial): `application/pdf`, `image/png`, `image/jpeg`, `image/webp`, `text/plain` — verify with `infer`/magic bytes.
- Streaming write + SHA-256 + atomic rename; server-generated UUID path under canonical org/workspace directory.
- Preserve legacy metadata `POST /api/v1/files` until FE migration verified; prioritize `/upload` and `/download/{id}` for FE.
- **Implementation order:** config+storage safety primitives -> core service/repo/quota -> API handlers+limits -> FE wiring -> deterministic tests -> ops docs. 
