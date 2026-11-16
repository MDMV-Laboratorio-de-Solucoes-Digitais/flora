# 016 — Binary file upload hardening (Axum 0.8, local disk, cheap VPS)

**Date:** 2026-11-16  
**Repo:** `flora` (read-only analysis; this document only)  
**Scope:** Safe, simple binary file upload/download for self-hosted Rust (Axum 0.8 multipart/streaming, `tower-http` limits), Postgres metadata, local-disk storage, path traversal prevention, size/content-type limits, magic-byte validation, checksums, quotas, streaming downloads, and Svelte/SvelteKit frontend integration. Optimized for cheap VPS (low memory, no external object storage).  
**Cross-check inputs:** `flora-api/src/routes/files.rs`, `flora-core/src/services/file_service.rs`, `flora-core/src/storage/local.rs`, `flora-core/src/config/storage.rs`, `flora-files/src/lib.rs`, `flora-api/src/state/app_state.rs`, `apps/frontend/src/lib/components/files/{FileUpload.svelte,FilePreview.svelte}`, prior research `docs/research/2026-11-16-015-binary-file-upload-download.md`, `docs/research/2026-11-16-015-files-impl-readiness.md`.

## 1. Cross-check: current state vs required safe flow

| Area | Current implementation | Gap / risk |
|---|---|---|
| HTTP contract | `POST /api/v1/files` accepts JSON metadata only (`workspace_id,name,file_type,size_bytes`); `GET /{id}` metadata; `DELETE /{id}` soft-delete (`flora-api/src/routes/files.rs`). No download route. | FE expects `POST /api/v1/files/upload` multipart `file` and `GET /api/v1/files/download/{id}` (`FileUpload.svelte`, `FilePreview.svelte`). No binary bytes stored or served. |
| Service logic | `FileService::create_file` validates filename separators/`..`, denylist MIME, enforces per-upload size against org/workspace quota config values, generates `orgs/{org_id}/files/{file_id}`, inserts DB record only (`flora-core/src/services/file_service.rs`). | No actual blob write, no aggregate usage quota (`SUM(size_bytes)`), no workspace linkage in record, no checksum, no server-observed size/MIME. Quota semantics currently act as per-file caps, not usage budgets. |
| Local storage | `flora-core::storage::local::LocalFileSystem` does `root.join(path)` + `create_dir_all`/read/write/delete/exists; `presigned_url` returns `None` (`flora-core/src/storage/local.rs`). `flora-files::LocalStorage` similar, different trait/error shape and also no containment (`flora-files/src/lib.rs`). | No canonical-root containment/symlink-escape defense, no streaming API, full-buffer `Vec<u8>`/`&[u8]` paths, duplicate implementations risk drift. |
| Config/wiring | `StorageConfig`: local/S3 fields, `org_quota_bytes`/`workspace_quota_bytes`, multipart/chunk flags present but unwired; env defaults `STORAGE_*` and also `FLORA_STORAGE__*` style in config system (`flora-core/src/config/storage.rs`, `.env.example`). `AppState` injects `Arc<StorageConfig>` into core `FileService`, not a bytes storage backend (`flora-api/src/state/app_state.rs`). | Missing explicit `max_upload_bytes`, allowlist, temp dir, and safe wiring; multipart/chunk settings misleading without implementation. |
| Persistence/model | `File` stores `storage_path`, optional `checksum`, `size_bytes`, `file_type`, soft-delete; service/repo paths show workspace not fully modeled/enforced in current flow. | Need authoritative server-side size/hash/detected MIME and workspace-scoped accounting/listing for quotas and tenant correctness. |
| Frontend | `FileUpload.svelte` posts `FormData` with only `file`, client precheck 50 MB, expects `FileAttachment`; `FilePreview.svelte` downloads from `/api/v1/files/download/{id}` via anchor. | Missing `workspace_id` (and org context as required), size limit mismatch vs VPS-safe defaults, anchor download may need auth-cookie/bearer-compatible strategy. |

**Net:** current system is metadata-only; implementing binary flow requires contract alignment, streaming disk I/O, defense-in-depth validation, containment, and quota/accounting fixes.

## 2. Multilingual research synthesis (EN, pt-BR, es, de, fr, zh-CN, ja, ko)

Primary references fetched/reviewed: Axum `extract::Multipart` (body consumed, extractor must be last, default 2 MB unless `DefaultBodyLimit`; stream fields), `tower_http::limit::RequestBodyLimitLayer` (413 on oversized bodies by length), OWASP File Upload Cheat Sheet (allowlists, do not trust client `Content-Type`/name, server-generated names, storage outside webroot, authz, scanning/limits, permissions), `infer` crate (magic-number/MIME inference from bytes). `websearch` results were limited/unavailable in-session; multilingual guidance below distills these primary controls into language-specific operator/dev guidance (no fabricated locale-specific citations).

| Lang | Core guidance (applied to this stack) |
|---|---|
| **EN** | Treat every upload as untrusted. Enforce layered limits (proxy + `DefaultBodyLimit`/`RequestBodyLimitLayer` + streamed byte cap), allowlist types and verify with magic-byte sniffing (`infer`) instead of client metadata, generate server-side storage names/paths, keep blobs outside executable/webroot contexts, stream multipart chunks to temp files (no full RAM buffering), compute checksum/actual size, authorize every download, set safe headers (`Content-Type`, `Content-Disposition`, `X-Content-Type-Options: nosniff`). |
| **pt-BR** | Trate todo upload como não confiável. Aplique limites em camadas (proxy + limite de corpo no Axum/Tower + teto de bytes durante o streaming), use lista de permissões e valide por assinatura/magic bytes (não confie no `Content-Type`/nome do cliente), gere nomes/caminhos no servidor, grave em arquivo temporário e mova atomicamente, calcule hash/tamanho real, registre metadados no Postgres e exija autorização no download com cabeçalhos seguros. |
| **es** | Trate cada subida como entrada no confiable. Aplique límites en capas, lista blanca de tipos validada por firma/magic bytes, nombres y rutas generados por el servidor, escritura en streaming a disco temporal con límite duro, verificación de contención para evitar *path traversal*, cálculo de checksum/tamaño real, cuotas y autorización en cada descarga con cabeceras seguras. |
| **de** | Behandle jeden Upload als nicht vertrauenswürdig. Durchsetze gestaffelte Größenlimits, eine Allowlist mit Magic-Byte-Prüfung statt Client-`Content-Type`, serverseitig erzeugte Pfade/Namen, Streaming in temporäre Dateien (geringer RAM-Verbrauch), kanonische Pfadbegrenzung gegen *Path Traversal*, Prüfsummen/tatsächliche Größe, aggregierte Quoten und autorisierte Downloads mit sicheren Headern. |
| **fr** | Considérez chaque téléversement comme non fiable. Appliquez des limites en couches, une liste blanche validée par signature/magic bytes, des noms/chemins générés côté serveur, l’écriture en streaming vers un fichier temporaire avec plafond d’octets, la prévention du *path traversal* par confinement au répertoire racine, le calcul d’empreinte/taille réelle, les quotas agrégés et un contrôle d’accès systématique au téléchargement avec en-têtes sûrs. |
| **zh-CN** | 将所有上传视为不可信输入。实施分层大小限制（反向代理 + Axum/Tower 请求体限制 + 流式写入字节上限），使用类型白名单并结合魔数/文件签名校验（不要信任客户端 `Content-Type`/文件名），由服务端生成存储路径与文件名，流式写入临时文件后原子重命名，记录真实大小与校验和，执行路径规范化与根目录包含校验以防路径遍历，并在每次下载时做鉴权与安全响应头控制。 |
| **ja** | すべてのアップロードを信頼しない入力として扱う。プロキシ／Axum・Tower のボディ制限／ストリーミング時のバイト上限で多層的にサイズを制御し、クライアントの `Content-Type` やファイル名に依存せず許可リストとマジックバイト検査を行う。保存パスとファイル名はサーバー側で生成し、テンポラリファイルへストリーミング書き込み後に原子的に配置する。実サイズ・チェックサムを記録し、正規化とルート配下チェックでパストラバーサルを防ぎ、ダウンロード時は毎回認可と安全なヘッダーを適用する。 |
| **ko** | 모든 업로드를 신뢰할 수 없는 입력으로 취급한다. 프록시, Axum/Tower 요청 본문 제한, 스트리밍 바이트 상한으로 다층 크기 제한을 적용하고 클라이언트 `Content-Type`/파일명 대신 허용 목록과 매직 바이트 검사를 사용한다. 저장 경로/파일명은 서버에서 생성하고 임시 파일에 스트리밍 후 원자적으로 이동하며 실제 크기와 체크섬을 기록한다. 경로 정규화 및 루트 포함 검증으로 경로 탐색을 방지하고 다운로드마다 인가와 안전한 헤더를 적용한다. |

## 3. Concrete low-risk 80/20 recommendations (cheap VPS)

1. **Align API to FE and keep it simple:** implement `POST /api/v1/files/upload` (multipart, `workspace_id` + `file`) returning `FileAttachment`-compatible JSON, and `GET /api/v1/files/download/{id}` streaming from disk. Keep existing metadata/delete routes for compatibility; deprecate JSON metadata-only create only after migration.
2. **Stream, never buffer whole files:** Axum multipart extractor must be last; read with `field.chunk()` loop, write to `tokio::fs::File`/`BufWriter` under `{storage_root}/.tmp`, enforce hard byte cap during copy, compute SHA-256 incrementally, `flush`/rename atomically. Avoid `field.bytes()`/`Vec<u8>` for file bodies.
3. **Layered limits:** set route-level `DefaultBodyLimit` and/or `tower_http::limit::RequestBodyLimitLayer` (reject by `Content-Length` early) **and** streamed cap (covers chunked/unknown length). Default **10 MiB** on cheap VPS (configurable `FLORA_STORAGE__MAX_UPLOAD_BYTES`); align reverse proxy `client_max_body_size`.
4. **Allowlist + magic-byte verification:** accept a minimal allowlist (initial: `application/pdf,image/png,image/jpeg,image/webp,text/plain`) and verify detected type via `infer` (and basic structure checks where cheap) over client claim; reject mismatches as `415/400`. Keep filename sanitization but move from denylist-first to allowlist-first.
5. **Server-controlled paths and containment:** store as `orgs/{org_id}/workspaces/{workspace_id}/files/{file_uuid}` (server-generated only). Canonicalize root, resolve/normalize target, and require target remains under canonical root; reject traversal/symlink escape. Persist sanitized display name separately (length/control-char limits), never for path resolution.
6. **Quota and accounting (pragmatic):** enforce max upload size plus aggregate active-bytes quota per org/workspace via `SUM(size_bytes)` where `is_deleted = false` before commit/finalize (transactional check). Reclaim space on delete/purge; run orphan-temp cleanup job. Defer reservations/rate tiers.
7. **Integrity and safe metadata:** store actual written `size_bytes`, SHA-256 `checksum`, detected MIME, and `uploadedAt`; on failure remove temp/final blob to avoid orphans. Prefer write-temp -> rename -> DB commit/finalize ordering with compensating cleanup.
8. **Secure download:** authorize org/workspace/member access before open; 404 for missing/soft-deleted (or consistent policy) and no cross-tenant leaks. Stream response (`tokio_util::io::ReaderStream`), set `Content-Type` (stored/detected), `Content-Disposition: attachment; filename*=UTF-8''...` for user downloads, `X-Content-Type-Options: nosniff`, `Content-Length` when known, and conservative `Cache-Control: private, no-store` for MVP.
9. **Frontend (Svelte/SvelteKit) integration:** append `workspace_id` (and required tenant context) to `FormData`; do not set multipart boundary manually; treat client type/size checks as UX only; use `credentials`/auth model consistent with session so anchor/fetch download works (fallback: fetch -> blob URL for bearer flows). Align UI max size to server config.
10. **VPS ops:** dedicated storage volume, disk monitoring, periodic `.tmp`/soft-deleted purge, request timeouts, no inline transcoding/scanning on hot path in v1, structured logs with ids/sizes/hashes only.

## 4. API/storage contract (proposed, low-risk)

### 4.1 Upload
- **Route:** `POST /api/v1/files/upload` (Axum 0.8, `multipart` feature; extractor last)
- **Auth:** authenticated user + `organization_id` context + membership; workspace membership/ownership as applicable.
- **Request:** `multipart/form-data`
  - `file` (required): binary stream
  - `workspace_id` (required, UUID text field)
  - optional `name`: ignored or used only as display hint after sanitization (server decides display/storage)
- **Validation:** body/stream <= `max_upload_bytes`; filename sanitized (len<=255, no control/NUL/separators/`..`); detected MIME in allowlist and consistent with magic bytes; aggregate quota ok.
- **Storage:** temp write -> hash/size/type -> atomic move to server path `orgs/{org_id}/workspaces/{workspace_id}/files/{file_id}` inside canonical root.
- **Response 201:** `{ "id","fileName","mimeType","byteSize","rustFsUrl":"/api/v1/files/download/{id}","uploadedAt","checksum"? }` (additive checksum allowed).
- **Errors:** `400` invalid/missing fields, `401/403` authz, `413` too large/quota exceeded (or dedicated quota error if available), `415` unsupported type, `500` internal/storage.

### 4.2 Download
- **Route:** `GET /api/v1/files/download/{id}`
- **Authz:** resolve file, enforce same tenant/access policy before opening bytes.
- **Response 200:** streamed file body + safe headers; `404` missing/soft-deleted per policy, `403` cross-tenant.

### 4.3 Metadata and lifecycle
- Keep `GET /api/v1/files/{id}`, `DELETE /api/v1/files/{id}`; ensure delete/purge policy reclaims blobs (or scheduled purge) and quota excludes soft-deleted.
- DB is source of truth for metadata; filesystem blob keyed by server-generated relative path only.

### 4.4 Suggested config keys/defaults
- `FLORA_STORAGE__LOCAL_PATH` (or `STORAGE_LOCAL_PATH` compat): canonical root, e.g. `/var/flora/storage`
- `FLORA_STORAGE__MAX_UPLOAD_BYTES`: `10485760` (10 MiB)
- `FLORA_STORAGE__ALLOWED_MIME_TYPES`: pdf/png/jpeg/webp/plain allowlist
- `FLORA_STORAGE__ORG_QUOTA_BYTES`, `FLORA_STORAGE__WORKSPACE_QUOTA_BYTES`: aggregate usage budgets (clarify semantics vs per-file)
- temp dir under root: `.tmp`

## 5. Implementation checklist (ordered, low-risk)

- [ ] **Contract & config:** define canonical upload/download routes, `max_upload_bytes`, allowlist, temp dir, quota semantics; document `FLORA_STORAGE__*` and proxy limits.
- [ ] **Storage primitives:** single hardened local backend (containment check, temp+atomic rename, streaming read/write helpers, cleanup); avoid `flora-core`/`flora-files` logic drift.
- [ ] **Core service/repo:** streamed upload flow persisting actual `size_bytes/hash/mime/name/path/workspace_id`; aggregate quota query; fix workspace-aware create/list; transactional/compensating cleanup.
- [ ] **API handlers:** Axum multipart upload (`DefaultBodyLimit`/`RequestBodyLimitLayer`) + streamed download with authz and safe headers; map 400/403/404/413/415 correctly.
- [ ] **Schema (if needed):** additive migration for `workspace_id` (+ index/FK) and any constraints required for quota/listing correctness.
- [ ] **Frontend (`apps/frontend`):** add `workspace_id` to `FormData`, align max size/errors, verify download auth behavior (anchor vs fetch->blob).
- [ ] **Tests (deterministic):** unit (filename/path allowlist/magic-byte/size/quota/checksum), API/integration (happy path, 413/415 spoof/traversal/authz, headers, orphan cleanup), DB-gated aggregate quota/listing, FE component/e2e contract.
- [ ] **Ops:** create root/.tmp perms, disk monitoring, orphan/temp purge, backup/exclude policy.

## 6. NVs (non-goals / deferred)

- **NV1:** No S3/object storage, presigned direct upload/download, CDN, or resumable/chunked upload protocol in v1.
- **NV2:** No in-upload AV/malware scanning, CDR, or media transcoding/thumbnailing pipeline (can be added as async hooks later).
- **NV3:** No HTTP range/partial downloads, public anonymous links, or complex preview rendering; inline preview only for explicitly safe types after hardening.
- **NV4:** No advanced abuse controls (per-user/IP rate limiting, anomaly detection, per-tenant dynamic throttling) beyond authz + size + quotas.
- **NV5:** No full-text indexing/search of file contents; metadata/search integration remains separate.
- **NV6:** DB RLS/policies are defense-in-depth only; application-level tenant checks remain mandatory.
- **NV7:** Existing dual storage abstractions (`flora-core::storage` vs `flora-files`) are not unified in this research; implementation should pick one canonical path to reduce risk.

## 7. Source notes

- Repo (read): `flora-api/src/routes/files.rs`, `flora-core/src/services/file_service.rs`, `flora-core/src/storage/local.rs`, `flora-core/src/config/storage.rs`, `flora-files/src/lib.rs`, `flora-api/src/state/app_state.rs`, `apps/frontend/src/lib/components/files/FileUpload.svelte`, `apps/frontend/src/lib/components/files/FilePreview.svelte`.
- Prior research: `docs/research/2026-11-16-015-binary-file-upload-download.md`, `docs/research/2026-11-16-015-files-impl-readiness.md`.
- Fetched docs: Axum `Multipart` (0.8.x docs.rs; default 2 MB, last extractor, `next_field/field.chunk` streaming), `tower_http::limit::RequestBodyLimitLayer` (docs.rs), OWASP File Upload Cheat Sheet (cheatsheetseries.owasp.org), `infer` crate docs (docs.rs; magic-number detection API). Multilingual synthesis (incl. zh-CN/ja/ko) is an applied translation of these controls to this architecture, not a claim of locale-specific primary standards beyond them.
- Recommendation: before coding, verify current pinned versions/APIs for `axum` (workspace), `tower-http`, `tokio-util`, `infer`, and proxy behavior in deployment.
