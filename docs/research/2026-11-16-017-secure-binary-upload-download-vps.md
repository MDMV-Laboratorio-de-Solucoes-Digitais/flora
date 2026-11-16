# 017 — Secure binary upload/download for Flora (cheap VPS, 80/20)

**Date:** 2026-11-16  
**Repo:** `flora` (read-only cross-check; research output only)  
**Scope:** secure binary file upload/download for a self-hosted SMB collaboration app (Rust + Axum 0.8 + PostgreSQL + local disk + Svelte 5) on ~2 vCPU/4 GB. Covers Axum multipart/streaming and body limits, path traversal/symlink containment, filename sanitization, MIME allowlists vs magic-byte sniffing, size limits/quotas, checksums/integrity, temp-file+atomic rename, streaming downloads with safe headers/authz, orphan cleanup/backup considerations, and reverse-proxy limits. Multilingual synthesis: **EN, pt-BR, es, de, fr, zh-CN, ja** (+ ko/ru notes where useful).  
**Cross-check (read):** `flora-api/src/routes/files.rs`, `flora-core/src/services/file_service.rs`, `flora-core/src/models/file.rs`, `flora-core/src/storage/local.rs`, `flora-core/src/traits/storage_provider.rs`, `flora-core/src/config/storage.rs`, `flora-api/src/state/app_state.rs`, `apps/frontend/src/lib/components/files/{FileUpload.svelte,FilePreview.svelte}`, `docs/research/2026-11-16-016-binary-file-upload-hardening.md`, `docs/research/2026-11-16-016-macro-files-ops-parity.md`, `.specify/assessments/016-next-vertical-slice/decision.md`.

## 1. Current repo state vs secure binary flow (evidence)

| Area | Current state | Security/operability gap |
|---|---|---|
| API contract | `POST /api/v1/files` JSON metadata (`workspace_id,name,file_type,size_bytes`); `GET /{id}` metadata; `DELETE /{id}` soft-delete owner-only (`flora-api/src/routes/files.rs`). No upload bytes, no download route. | FE expects multipart `POST /api/v1/files/upload` and `GET /api/v1/files/download/{id}` (`FileUpload.svelte`, `FilePreview.svelte`) — contract mismatch; cannot enforce server-observed size/type/hash; no safe download path. |
| Service/model | `FileService::create_file` sanitizes filename for separators/`..`, denylist-blocks a few executable MIME types, enforces per-request size against `org_quota_bytes`/`workspace_quota_bytes` as caps, generates `orgs/{org_id}/files/{file_id}`, inserts DB only (`flora-core/src/services/file_service.rs`, `models/file.rs`). | No blob I/O, no aggregate usage quota (`SUM(size_bytes)` active), no workspace linkage persisted/enforced in record, no checksum, client-declared size/MIME trusted, denylist-first (weak) not allowlist+sniff. |
| Storage | `LocalFileSystem::put/get` do `root.join(path)` and full-buffer `&[u8]`/`Vec<u8>` read/write; no canonical containment/symlink-escape check, no streaming, no temp+atomic write; `StorageProvider` trait is buffer-based; duplicate `flora-files` impl noted in prior research (`flora-core/src/storage/local.rs`, `traits/storage_provider.rs`). | Path traversal/symlink risks if path ever influenced; OOM risk on 4 GB if buffering; no atomicity/integrity guarantees. |
| Config/wiring | `StorageConfig` has local/S3, org/workspace quota, multipart/chunk flags but unwired; defaults include 10 GB org / 2 GB workspace quotas (`flora-core/src/config/storage.rs`). `AppState` injects config into `FileService` only; search/messaging already optional/degraded (`app_state.rs`). | Missing `max_upload_bytes`, allowlist, temp dir, quota semantics; large quota defaults unsafe for cheap VPS disk; multipart flags misleading. |
| Frontend | Upload sends `FormData { file }` only, client 50 MB UX check; download uses plain anchor to `/download/{id}` (`apps/frontend`). | Missing `workspace_id`/tenant context in upload contract; 50 MB conflicts with VPS-safe limits; anchor download depends on auth/session/cookie behavior. |
| Prior decisions | 016 chose operability slice C (profiles/optionality/pooling) over B; 015/016 hardening research already defined streaming/allowlist/containment direction and NVs (`decision.md`, 016 research docs). | 017 should produce an implementation-ready, security-first contract for slice B that fits the C-hardened, low-footprint posture. |

## 2. Multilingual research synthesis (EN, pt-BR, es, de, fr, zh-CN, ja, ko, ru)

**Primary/official sources consulted:** Axum `extract::Multipart` and `extract::DefaultBodyLimit` (docs.rs 0.8.x), `tower_http::limit::RequestBodyLimitLayer` (docs.rs), `multer::Field` streaming (`chunk/bytes`) (docs.rs), `infer` magic-byte detection (docs.rs), `tempfile::NamedTempFile`/`persist` (docs.rs), `tokio_util::io::ReaderStream` (docs.rs), OWASP File Upload Cheat Sheet, nginx `client_max_body_size`, MDN `Content-Disposition` and `X-Content-Type-Options` usage context. Live web search returned limited hits in-session; synthesis below applies these primary controls across languages without inventing locale-specific standards.

| Lang | Applied guidance (stack-specific) |
|---|---|
| **EN** | Treat uploads as untrusted. Layer limits (proxy + Axum/Tower body limit + streamed byte cap). Require authz before read/write, allowlist types and verify with magic bytes (`infer`) not client metadata, generate server-side names/paths, stream multipart chunks to a temp file (no whole-file RAM), compute SHA-256 and actual size, canonicalize/contain paths to prevent traversal/symlink escape, atomic persist/rename, safe download headers (`Content-Disposition`, `X-Content-Type-Options: nosniff`, `Cache-Control: private, no-store`), stream responses, clean orphans. |
| **pt-BR** | Tratar todo upload como dado não confiável. Limitar em camadas (proxy/nginx `client_max_body_size` + limite de corpo Axum/Tower + teto de bytes no streaming), exigir autenticação/autorização por organização/workspace, usar lista de permissão e validar assinatura/magic bytes (não confiar em `Content-Type`/nome do cliente), gerar nome/caminho no servidor, gravar em arquivo temporário e mover atomicamente, calcular hash/tamanho real, impedir path traversal/symlink por canonicalização e verificação de raiz, registrar metadados no Postgres e servir download em streaming com cabeçalhos seguros. |
| **es** | Considerar cada carga como entrada no confiable. Aplicar límites en capas, control de acceso por organización/espacio, lista blanca de tipos verificada por firma/magic bytes, nombres y rutas generados por el servidor, escritura en streaming a archivo temporal con tope de bytes y movimiento atómico, cálculo de hash/tamaño real, confinamiento de ruta canónica para evitar *path traversal* y escapes por enlaces simbólicos, cuotas agregadas y descargas autorizadas en streaming con cabeceras seguras. |
| **de** | Jeden Upload als nicht vertrauenswürdig behandeln. Gestaffelte Limits (Proxy, Axum/Tower, Stream-Byte-Obergrenze), strikte Autorisierung, Allowlist plus Magic-Byte-Prüfung statt Client-`Content-Type`, serverseitige Pfade/Namen, Streaming in temporäre Dateien mit atomarem Persistieren, SHA-256 und tatsächliche Größe erfassen, kanonische Pfadprüfung gegen Path Traversal/Symlink-Escape, aggregierte Quoten, sichere Streaming-Downloads mit `nosniff`/`Content-Disposition`/privatem Caching. |
| **fr** | Traiter chaque téléversement comme non fiable. Appliquer des limites en couches, un contrôle d’accès strict, une liste blanche validée par signature/magic bytes, des chemins/noms générés côté serveur, l’écriture en streaming vers un fichier temporaire avec plafond et déplacement atomique, le calcul d’empreinte/taille réelle, le confinement de chemin canonique contre le *path traversal* et les liens symboliques, des quotas agrégés et des téléchargements en streaming avec en-têtes sûrs. |
| **zh-CN** | 将所有上传视为不可信输入。实施分层大小限制（反向代理 + Axum/Tower 请求体限制 + 流式字节上限），在读取/写入前完成组织与工作区鉴权；使用类型白名单并结合魔数/文件签名校验（不要信任客户端 `Content-Type`/文件名）；由服务端生成存储路径与文件名，流式写入临时文件并原子落盘，记录真实大小与 SHA-256；通过规范化与根目录包含校验防止路径遍历与符号链接逃逸；下载时按权限流式返回并设置安全响应头（`X-Content-Type-Options: nosniff`、`Content-Disposition`、私有缓存策略），定期清理孤儿临时文件。 |
| **ja** | すべてのアップロードを信頼しない前提とする。リバースプロキシ、Axum/Tower のボディ制限、ストリーミング時のバイト上限で多層的に制限し、組織/ワークスペース単位の認可を必須にする。クライアントの `Content-Type` やファイル名を信用せず、許可リストとマジックバイト検査を行う。保存先パスとファイル名はサーバー生成とし、一時ファイルへストリーミング書き込み後に原子的に配置、実サイズと SHA-256 を記録する。正規化とルート配下チェックでパストラバーサル/シンボリックリンク逃避を防ぎ、ダウンロードは認可後にストリーミング配信し安全なヘッダーを付与する。 |
| **ko** | 모든 업로드를 신뢰하지 않는 입력으로 처리한다. 프록시·Axum/Tower 본문 제한·스트리밍 바이트 상한의 다층 제한과 조직/워크스페이스 인가를 적용하고, 클라이언트 메타데이터 대신 허용 목록과 매직 바이트 검사를 사용한다. 서버가 경로/파일명을 생성해 임시 파일에 스트리밍 후 원자적 저장하며 실제 크기·SHA-256을 기록한다. 경로 정규화/루트 포함 검증으로 경로 탐색·심볼릭 링크 탈출을 막고, 다운로드는 인가 후 스트리밍과 안전 헤더(`nosniff`, `Content-Disposition`, 비공개 캐시)로 제공한다. |
| **ru** (useful) | Считать загрузки недоверенными: многоуровневые лимиты (прокси, Axum/Tower, лимит байтов при потоковой записи), строгая авторизация, allowlist + проверка сигнатур/magic bytes, серверные имена/пути, потоковая запись во временный файл и атомарное переименование, фиксация реального размера и SHA-256, защита от path traversal/symlink, агрегированные квоты, потоковая выдача с безопасными заголовками и очистка «осиротевших» файлов. |

## 3. 80/20 recommendations (concrete, VPS-safe)

1. **Stream end-to-end, never buffer whole files in RAM.** Use `Multipart` as the last extractor (body consumed) and `field.chunk()` loop; write to `tokio::fs::File` in `{storage_root}/.tmp` while counting bytes and updating SHA-256 hasher; reject immediately when streamed bytes exceed cap; avoid `field.bytes()`/`Vec<u8>` for bodies. For downloads use `tokio_util::io::ReaderStream` (or equivalent chunked stream) from a file handle, not `read_to_end`. (Axum multipart/multer streaming; tokio-util ReaderStream.)
2. **Layered size limits (fail early, enforce always).** (a) Reverse proxy: set nginx `client_max_body_size` (and timeouts) to match app cap. (b) Axum/Tower: apply `DefaultBodyLimit::max(...)` and/or `tower_http::limit::RequestBodyLimitLayer` per upload route (413). (c) Stream cap: hard-stop on accumulated bytes for chunked/unknown-length cases. Default cheap-VPS cap **10 MiB** (`10 * 1024 * 1024`) configurable (e.g. `FLORA_STORAGE__MAX_UPLOAD_BYTES` / `storage.max_upload_bytes`), exposed to FE for UX only.
3. **Allowlist-first type policy + magic-byte verification.** Accept minimal allowlist initially (recommend `application/pdf`, `image/png`, `image/jpeg`, `image/webp`, `text/plain`), detect via `infer` from initial bytes/buffered prefix and/or final content rules; require consistency with (sanitized) claimed type where practical; reject unsupported/mismatch as `415` (or `400`). Keep filename/MIME denylist checks only as defense-in-depth, not primary control. (OWASP allowlist/signature validation; infer.)
4. **Server-generated identity and path containment.** Never trust client path/name for storage. Persist display name separately (sanitized: length <= 255, strip control/NUL, reject separators/`..`, normalize unicode cautiously) and store bytes at server-generated key, e.g. `orgs/{org_id}/workspaces/{workspace_id}/files/{file_id}` under canonical `storage_root`. Canonicalize root at startup; resolve target and enforce it remains under canonical root (and temp dir under root); reject traversal and symlink escape; create dirs with restrictive perms; do not serve from webroot.
5. **Temp-file + atomic persist with integrity metadata.** Write to unique temp in `.tmp` (`tempfile::NamedTempFile`/`persist` or equivalent atomic rename semantics on same filesystem), `flush`/`sync` policy as practical, then move to final path and record DB metadata. Store authoritative `size_bytes` (actual), `checksum` (SHA-256 hex), detected `mime_type`, storage key, org/workspace/owner, timestamps; on any failure remove temp/final blob (compensating cleanup) to avoid orphans/leaks.
6. **Quota semantics: per-file cap + aggregate active-bytes budget.** Cheap-VPS safe: small per-file cap (10 MiB default) and realistic aggregate budgets (configurable, far below current 10 GB/2 GB defaults unless disk supports). Enforce `new_size + SUM(active size_bytes WHERE org/workspace AND NOT deleted)` before final commit (transactional/retry-aware) and exclude soft-deleted from active quota; reclaim on purge. Treat current `org_quota_bytes/workspace_quota_bytes` as needing semantic clarification in code/config.
7. **Streaming downloads with strict authz and safe headers.** Resolve file, verify tenant/org/workspace membership and read permission before opening blob; return consistent `404` for missing/soft-deleted/cross-tenant (or 403 per policy, but avoid oracle). Stream body, set `Content-Type` from stored/detected type, `Content-Length` when known, `Content-Disposition: attachment; filename*=UTF-8''...` (sanitized/ASCII fallback), `X-Content-Type-Options: nosniff`, and `Cache-Control: private, no-store` (MVP). No inline execution contexts.
8. **Orphan cleanup, observability, and backup basics.** Periodic job/CLI to purge stale `.tmp` files (age threshold) and soft-deleted blobs per retention; structured logs with file/org/workspace/size/hash (no content). Backups: include Postgres + storage root consistently; document restore order/integrity checks (hash/size sampling). Monitor disk usage on VPS.
9. **Frontend alignment (Svelte 5, minimal).** Send required `workspace_id` (and rely on existing auth/org context) in multipart; keep client size/type checks as UX hints only; handle 413/415/403/401 distinctly. Download: prefer `fetch` with credentials/auth -> blob/object URL when cookies/headers required, else anchor is fine; revoke object URLs. Align UI limit with server-configured cap.

## 4. Proposed API/storage contract (low-risk, implementable)

### 4.1 Upload
- **Endpoint:** `POST /api/v1/files/upload` (Axum 0.8 multipart, extractor last). Route-level `DefaultBodyLimit`/`RequestBodyLimitLayer`.
- **Authn/Authz:** authenticated user + resolved `organization_id` + membership; verify `workspace_id` belongs to org and user has write access (per policy) **before** persisting bytes/record.
- **Request:** `multipart/form-data`
  - `workspace_id` (required, UUID)
  - `file` (required, binary; one file per request for 80/20)
  - `name` (optional hint, never trusted for path)
- **Server behavior:** stream to `.tmp`, enforce streamed cap, sniff/allowlist MIME, compute SHA-256 and actual bytes, sanitize display name, check aggregate quota, atomic move to server key, insert/commit metadata (compensating delete on DB failure).
- **Response 201 (JSON):** `{ id, fileName, mimeType, byteSize, rustFsUrl: "/api/v1/files/download/{id}", uploadedAt, checksum }` (compatible with FE `FileAttachment` expectations; checksum additive).
- **Errors:** `400` invalid/missing fields, `401/403` authz, `413` size/quota exceeded, `415` unsupported/invalid type, `409` quota/race conflict (optional), `500` internal.

### 4.2 Download
- **Endpoint:** `GET /api/v1/files/download/{id}`
- **Authz:** load metadata, enforce tenant/workspace access before open/stream.
- **Response 200:** streamed body + `Content-Type`, `Content-Length` (if known), `Content-Disposition: attachment; filename*=UTF-8''<safe>`, `X-Content-Type-Options: nosniff`, `Cache-Control: private, no-store`.
- **Errors:** `404` missing/deleted/not accessible per policy, `403` explicit forbidden (if preferred), `500` I/O.

### 4.3 Metadata/lifecycle and storage model
- Keep `GET /api/v1/files/{id}`, `DELETE /api/v1/files/{id}` semantics; delete marks soft-deleted and schedules/requires blob purge for space/quota reclaim.
- **DB as source of truth:** `id, organization_id, workspace_id, owner_id, name(display), file_type(detected), size_bytes(actual), storage_path(server key), checksum(sha256), metadata, is_deleted, timestamps` (+ index on org/workspace/active for quota/listing) — additive migration likely needed for `workspace_id` linkage.
- **Filesystem layout:** `<canonical_root>/.tmp/*` and `<canonical_root>/orgs/{org_id}/workspaces/{workspace_id}/files/{file_id}` (or equivalent contained key). One canonical storage implementation to avoid core/files drift.

### 4.4 Suggested config (cheap VPS defaults)
- `storage.backend = "local"` (v1)
- `storage.local_path` canonical root (e.g. `/var/flora/storage`, persistent volume)
- `storage.max_upload_bytes = 10485760` (10 MiB)
- `storage.allowed_mime_types = ["application/pdf","image/png","image/jpeg","image/webp","text/plain"]`
- `storage.temp_dir` under root (`.tmp`) or derived
- aggregate quotas sized to disk (replace large hardcoded defaults with explicit, conservative values and clear semantics)
- reverse proxy aligned: e.g. nginx `client_max_body_size 10m;` and adequate timeouts/body buffering settings

## 5. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| RAM exhaustion / slowloris-like large bodies on 2 vCPU/4 GB | OOM, latency, instability | Stream-to-disk, strict proxy+Axum+stream caps, timeouts, no full-buffer APIs, small per-file default (10 MiB). |
| Path traversal / symlink escape / arbitrary overwrite | Data leak, RCE-adjacent persistence issues | Server-generated keys only, canonical root containment checks, no client path resolution, safe temp dir, restrictive perms. (OWASP storage/filename guidance.) |
| Content-type spoofing / polyglot uploads | XSS/content confusion, malware delivery | Allowlist + magic-byte sniffing (`infer`) + safe download headers (`nosniff`, attachment) + never execute from storage. |
| Missing/weak authz on upload/download | Cross-tenant data exposure | Enforce org/workspace membership per request before I/O; consistent 404/403; avoid ID oracle leaks. |
| Quota bypass or disk exhaustion | Service outage on small VPS | Aggregate active-bytes accounting (DB), per-file cap, reclaim on purge, disk monitoring, orphan cleanup. |
| Partial writes / crashes leave orphans or corrupt state | Storage leak, integrity confusion | Temp+atomic rename, compensating cleanup, checksums/size recorded, periodic `.tmp` purge, backup/restore checks. |
| Proxy/app limit mismatch or chunked bypass | Unexpected 413/DoS or oversized acceptance | Align nginx/Caddy/Traefik limits with Axum/Tower and streamed cap; test chunked uploads. |
| FE/BE contract drift (current mismatch) | Broken uploads/downloads, insecure assumptions | Single contract (multipart fields, response shape, error codes), config-driven UI cap, auth-compatible download flow. |

## 6. NVs (non-goals / deferred for 017)

- **NV1:** No S3/presigned direct upload-download, CDN, resumable/multipart chunk protocol, or multi-node shared storage.
- **NV2:** No malware/AV scanning, CDR, OCR, transcoding, thumbnails, or inline preview rendering pipeline.
- **NV3:** No HTTP range downloads, public/anonymous share links, signed expiring URLs, or dedupe/object GC beyond basic purge/orphan cleanup.
- **NV4:** No advanced abuse controls (per-IP/user rate limiting, anomaly detection) beyond authz, size/quotas, and timeouts.
- **NV5:** No full-text/content indexing/search of file bodies.
- **NV6:** No unification of `flora-core` vs `flora-files` abstractions in this doc; implementation must pick one canonical hardened path.
- **NV7:** Runtime/load/OOM benchmarks, proxy-specific tuning per provider, and backup-restore drill execution not performed here (verification required in implementation cycle).
- **NV8:** DB RLS not a substitute for mandatory application-level tenant checks.

## 7. Recommended implementation order (next cycle, 80/20)
1. Config/contract: add `max_upload_bytes`, allowlist, temp dir, clarified quota semantics; align proxy docs/defaults.
2. Canonical storage: contained, streaming temp-write/atomic-persist/read-stream APIs; single backend.
3. Core/service + schema: workspace-aware metadata, aggregate quota checks, actual size/hash/MIME persistence (additive migration as needed).
4. API: multipart upload + streaming download routes with limits/authz/safe headers/error mapping.
5. Frontend: add `workspace_id`, error handling, config-aligned limits, auth-safe download.
6. Deterministic tests: unit (sanitize/containment/sniff/cap/hash), integration (413/415/traversal/cross-tenant/orphan cleanup/headers), DB-gated quota/listing.
7. Ops: volume/perms, disk monitoring, temp/purge job, backup/restore runbook.

## 8. Source notes (primary/official)
- **Axum 0.8 (docs.rs):** `extract::Multipart` — body-consuming, must be last extractor; `next_field()`/`field.chunk()` streaming; default body size limit considerations (commonly 2 MB for body-limited extractors / `DefaultBodyLimit`), `DefaultBodyLimit::max/disable` for configuring extractor limits.
- **Tower HTTP (docs.rs):** `tower_http::limit::RequestBodyLimitLayer` — rejects oversized bodies (413) based on content length / limit enforcement.
- **Multer (docs.rs):** `Field::chunk/bytes/text` streaming/collection semantics underlying Axum multipart.
- **Infer (docs.rs):** magic-number based type detection (`get`, matcher types) for allowlist/signature checks.
- **Tempfile (docs.rs):** `NamedTempFile` / `persist` / `persist_noclobber` for safe temp handling and atomic-ish persistence patterns (same-filesystem constraints apply).
- **Tokio-util (docs.rs):** `io::ReaderStream` for streaming `AsyncRead` (file) to HTTP response bodies.
- **OWASP Cheat Sheet Series:** File Upload Cheat Sheet — allowlists, do not trust client filename/Content-Type, server-generated names, store outside webroot, validate/sanitize, size limits, permissions, authz, scanning/processing cautions. https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html
- **nginx (nginx.org):** `client_max_body_size` (default 1m; 413 on exceed) — must align with app limits. https://nginx.org/en/docs/http/ngx_http_core_module.html#client_max_body_size
- **MDN:** `Content-Disposition` (attachment/filename/filename*) for safe download naming. https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Disposition
- **Repo evidence:** files listed in cross-check section; prior research `docs/research/2026-11-16-016-binary-file-upload-hardening.md`, `016-macro-files-ops-parity.md`, `.specify/assessments/016-next-vertical-slice/decision.md`.
- **Limitation:** in-session web search yielded limited results; ko/ru and locale phrasing are applied translations of the above primary controls, not claims of additional locale-specific primary standards. Verify pinned crate versions (`axum`, `tower-http`, `tokio`, `tokio-util`, `infer`, `tempfile`) and deployment proxy behavior before implementation.