# 018 — DB-gated contract/integration testing and hardening for secure file upload/download (VPS, Rust/Axum/sqlx/Postgres)

Date: 2026-11-16  
Scope: 80/20 research + Flora 017 cross-check (EN, pt-BR, es, de, fr, zh-CN, ja).  
Status: research only (no source changes).

## 1) Executive summary (80/20)

- **Highest ROI:** add DB-gated contract tests for multipart upload/download/delete with real migrations on disposable PostgreSQL 18, asserting tenant/workspace isolation, path containment invariants, size/MIME/checksum semantics, and failure cleanup. Keep tests env-gated (`FLORA_DATABASE__POSTGRES_URL` or testcontainers) so cheap VPS CI can opt in.
- **Critical correctness gap:** quota in 017 is configured but not enforced as aggregate workspace/org usage; current checks only compare single-file size against quota ceilings. This is the main correctness/NV to close before claiming quota guarantees.
- **Critical robustness gaps:** multipart field-order dependence for `workspace_id`, metadata-only upload path bypasses binary storage invariants, storage trait methods (`put/get/delete/exists`) lack the same canonical containment as `put_stream/open`, and soft-delete removes bytes best-effort without transactional coupling to DB state.
- **VPS strategy:** prefer ephemeral per-test databases/schemas on a local disposable PG18 (Docker/testcontainers or native service with tmpfs) + `sqlx::migrate!`/`migrate()` + small deterministic fixtures; avoid shared long-lived test DBs and large binary payloads in-repo.

## 2) Multilingual evidence themes (cross-checked)

| Language | Keywords/themes reviewed | Practical takeaway for this slice |
|---|---|---|
| EN | Axum `Multipart`, streaming upload, `content_length_limit`, `tokio_util::io::ReaderStream`, sqlx test isolation, PostgreSQL advisory/transactional checks, file upload hardening | Stream to temp+fsync/rename, enforce limits before/while reading, validate tenant/workspace ownership in DB, test with real HTTP multipart and disposable DB. |
| pt-BR | upload seguro, validação de tipo/tamanho, isolamento multi-inquilino, quota por organização, limpeza em falha, testes de contrato com banco descartável | Não confiar só em `Content-Type`; checar autorização por workspace/organização; garantir rollback/limpeza de arquivo órfão; quotas devem ser transacionais/agregadas. |
| es | carga/descarga segura, recorrido de ruta (path traversal), autorización por inquilino, pruebas de integración con PostgreSQL efímero | Rechazar `..`, rutas absolutas y symlink escape; descargar solo si `file.org/workspace` pertenece al actor; usar migraciones reales en CI. |
| de | Dateiupload-Härtung, Mandantentrennung, Pfadnormalisierung, Integritätsprüfung (Prüfsumme), Aufräumen bei Fehlern | Kanonische Pfadprüfung gegen Storage-Root, Checksummen speichern/prüfen, Temp-Dateien und DB-Einträge bei Fehler konsistent bereinigen. |
| fr | téléversement sécurisé, contrôle d'accès locataire/espace de travail, quotas, tests d'intégration avec base jetable | Matrice de tests doit couvrir cas refusés (403/404/413/415) et cas limites (fichier vide, double champ, nom dangereux). |
| zh-CN | 文件上传下载安全、分片/流式处理、租户隔离、路径穿越、配额校验、一次性 PostgreSQL 测试环境 | 以流式落盘避免 OOM；配额需汇总统计并在事务中校验；合同测试覆盖跨租户访问与失败清理。 |
| ja | ファイルアップロード/ダウンロードの安全性、テナント分離、パス正規化、整合性(チェックサム)、使い捨てDBによる統合テスト | メタデータと実体の整合性、認可境界、サイズ上限超過時の中断/削除、決定的フィクスチャを重視。 |

> Note: recommendations below are synthesized from these themes and aligned to Flora’s current implementation/NVs; no external web fetch was performed in this session.

## 3) Flora 017 cross-check (evidence)

### Implemented (good 80/20 base)
- Routes: `flora-api/src/routes/files.rs`
  - `POST /files/upload` multipart streaming into service; member check via `ensure_member`; returns 201 attachment DTO.
  - `GET /files/download/{id}` and `/{id}/download` stream file with `Content-Type`, `Content-Length`, `nosniff`, sanitized attachment filename.
  - `GET /files/{id}` metadata, `DELETE /files/{id}` owner-only soft delete; tenant mismatch mapped to `FileNotFound`.
  - Legacy `POST /files` metadata-only create exists.
- Service: `flora-core/src/services/file_service.rs`
  - Sanitizes display name, normalizes/allowlists MIME, sniffs header bytes, enforces max-by-config-per-file ceiling, streams to storage, computes SHA-256, stores checksum/metadata, best-effort storage cleanup on DB create failure, best-effort byte removal on delete.
- Storage: `flora-core/src/storage/local.rs`
  - Rejects absolute/traversal relative paths, writes to `.tmp/<uuid>.part` then renames, enforces `max_bytes` while streaming, SHA-256 digest, canonical-root checks for `put_stream/open/absolute_path`.
- Migration: `20261116000002_add_workspace_id_to_files.sql` adds nullable `workspace_id`, FK to workspaces, indexes.
- Validation: `.specify/validations/017-secure-binary-files/validate.md` records NVs (no DB-gated file contract tests, no full frontend/e2e, no load/OOM/proxy/concurrent quota drills, no advanced magic-byte/AV/resumable/S3).

### Gaps / risks observed
- **Authorization granularity:** upload/download/delete require org membership; upload takes `workspace_id` but does not verify the workspace belongs to org or that user can access that workspace. Download allows any org member (owner-only not required by design?) while delete is owner-only — policy must be explicit and tested.
- **Multipart ordering/shape:** in `upload_file_multipart`, encountering `file` before `workspace_id` yields validation error; robust parsers should accept fields in any order or buffer metadata until file/EOF strategy is clear.
- **Metadata-only endpoint:** `POST /files` persists records and storage paths without writing bytes; can create dangling metadata and bypass streaming/checksum/content checks.
- **Quota semantics:** service compares `size_bytes` to configured quota ceilings but does **not** sum existing active bytes by workspace/org; concurrent uploads can exceed aggregate quota (matches NV). Retention/purge does not remove stored bytes.
- **Transactional consistency:** DB insert and filesystem write are not atomic; cleanup is best-effort. Soft-delete removes bytes then marks DB deleted (order/retry can diverge).
- **StorageProvider API consistency:** `LocalFileSystem::{put,get,delete,exists,full_path}` do not apply the stronger canonical containment used by `put_stream/open`; mixed usage could weaken guarantees.
- **Download headers/limits:** filename sanitization is ASCII-graphic-only (safe but lossy for Unicode); `Content-Length` from `i64` trusted; no range/resume; Axum/body/proxy limits and timeouts are deployment concerns (NV).
- **Tests:** `flora-tests/src/contract/files.rs` and `integration/files.rs` are placeholders only.

## 4) Proposed DB-gated test matrix (contract + integration)

### A. Disposable Postgres 18 strategy (CI/VPS-friendly)
- **Preferred:** `testcontainers-modules` (postgres:18-alpine) for hermetic CI where Docker is available; configure tmpfs for `/var/lib/postgresql/data` on cheap VPS.
- **Fallback:** env-gated real DB URL (`FLORA_DATABASE__POSTGRES_URL`) + create unique database/schema per test run (`flora_test_<uuid>`), apply `sqlx::migrate!`, drop after. Matches existing `--ignored` db-test pattern in `flora-tests/src/contract/auth.rs`.
- **Determinism:** fixed UUIDs where possible, deterministic file bytes fixtures (small in-memory buffers), stable MIME/sniff inputs, explicit clock-independent assertions, run migrations once per isolated DB.
- **Runtime hygiene:** `#[serial]` or isolated DB names for parallel tests; low pool size; cap payload sizes (e.g. 64KiB–2MiB fixtures) to fit 2vCPU/4GB.

### B. Contract tests (`flora-tests/src/contract/files.rs`)
| ID | Case | Setup | Expected |
|---|---|---|---|
| C1 | Multipart upload happy path | seeded org/workspace/member; multipart with `workspace_id`+`file` | 201, JSON attachment fields, download URL, DB row with checksum/size/workspace/org/path |
| C2 | Download happy path | uploaded file | 200, streamed body matches bytes, `Content-Type`, `Content-Length`, `nosniff`, attachment disposition |
| C3 | AuthZ tenant isolation | file in org A, member of org B requests get/download/delete | 404/403 per policy (must not leak existence across orgs) |
| C4 | Workspace mismatch/unauthorized workspace | upload or access with workspace not in org / user not allowed | 4xx (validation/forbidden/not found), no stored bytes/row |
| C5 | Missing/duplicate/ordered multipart fields | `file` before `workspace_id`, missing `workspace_id`, missing `file`, two `file` parts | 400 validation with stable field/message semantics; no orphan temp files |
| C6 | Validation limits | empty file, oversize file, disallowed MIME/ext/signature mismatch | 400/413/415 mapped errors, storage cleaned |
| C7 | Metadata-only legacy endpoint policy | `POST /files` | explicitly assert intended behavior (allowed metadata-only or rejected/flagged) and tenant scoping |
| C8 | Delete semantics | owner deletes, non-owner deletes, already deleted download/get | 204 for owner, forbidden for non-owner, 404 for deleted reads; bytes/metadata state as specified |

### C. Integration tests (`flora-tests/src/integration/files.rs`)
| ID | Case | Focus |
|---|---|---|
| I1 | Upload→persist→download roundtrip | checksum/size/path consistency between DB, storage, HTTP response |
| I2 | Failure cleanup | induce DB failure after `put_stream` (or simulate via service boundary/fault injection seam) → no orphan final file; induce stream abort/oversize → no `.tmp` residue |
| I3 | Aggregate quota (target behavior) | sequential and concurrent uploads near workspace/org quota; assert transactional enforcement once implemented |
| I4 | Cross-workspace listing/access | list/get/download scoped to `(org, workspace)`; no leakage |
| I5 | Soft-delete/purge lifecycle | delete hides from reads, purge policy removes DB rows and (desired) storage bytes; retention bounds |
| I6 | Unicode/special filenames and content types | sanitized storage/display/download headers; no path injection |

### D. Minimal test harness additions (research recommendation)
- Reuse `flora-tests` app-state/router builders (factor from auth pattern) with temp `storage.local_path`.
- Add helpers: multipart body builder, seed org/workspace/membership/user, response snapshot/assert helpers.
- Gate DB tests behind env var/feature to keep default `cargo test` cheap.

## 5) Minimal hardening fixes (recommended, smallest safe set)

1. **Enforce workspace ownership and access on upload/download/list/delete:** resolve workspace by `(workspace_id, org_id)` and require user permission/membership for that workspace; reject early with 404/403.
2. **Harden multipart parsing:** accept `workspace_id`/`file` in any order (two-pass collect small fields then stream file, or require `workspace_id` query param only) and reject ambiguous multi-file payloads deterministically.
3. **Unify storage containment:** make `StorageProvider` path operations and `full_path` use the same sanitize+canonical-root checks as streaming APIs; consider symlink-safe checks on open.
4. **Make quota aggregate and race-safe:** compute active usage in DB (`SUM(size_bytes)` filtered by org/workspace, not deleted) inside a transaction/locking strategy (advisory lock per workspace/org or serializable/check constraint approach) before commit.
5. **Clarify metadata-only endpoint:** either disable/hide in production, or mark clearly and forbid unscoped/dangling records (require workspace existence, no fake storage path/checksum semantics).
6. **Consistency on delete/purge:** define source of truth (DB-first vs storage-first), retry/compensation for orphan cleanup, and make purge remove bytes when policy allows.
7. **Operational limits:** document/enforce proxy/body size/timeouts aligned with `max_upload_bytes`; keep download streaming and bounded buffers.

## 6) Risks / NVs (current + residual)

- **NV (from 017, still open):** DB-gated file-specific contract tests, full frontend lint/test/build/e2e against real API, 2vCPU/4GB load/OOM, proxy limit enforcement, restore/backup drills, concurrent aggregate quota correctness, advanced magic-byte library (`infer`), AV scanning, resumable/S3/presigned uploads, content indexing/previews.
- **Security residuals:** org-member-wide download may be broader than intended; sniffing is heuristic (not a full content inspector/AV); Unicode filename handling in headers can be improved (RFC 5987) without weakening safety.
- **Data integrity residuals:** best-effort cleanup can leave orphans under crashes; no checksum verification endpoint/on-read audit path.
- **VPS operational residuals:** disk-full, slow fs, container tmpfs sizing, and proxy buffering can change failure modes; CI Docker availability varies.

## 7) 80/20 implementation order

1. Add DB-gated contract harness + C1–C4, C6, C8 on disposable PG18.
2. Fix workspace ownership/authz + multipart ordering/validation (C4–C5).
3. Implement aggregate quota check + I3 concurrency coverage.
4. Unify containment + cleanup/purge consistency + document proxy limits.

## 8) References (local)

- `flora-api/src/routes/files.rs`
- `flora-core/src/services/file_service.rs`
- `flora-core/src/storage/local.rs`
- `flora-core/src/migrations/sql/20240818000001_init_schema.sql`
- `flora-core/src/migrations/sql/20261116000002_add_workspace_id_to_files.sql`
- `flora-tests/src/contract/files.rs`, `flora-tests/src/integration/files.rs`, `flora-tests/src/contract/auth.rs`
- `.specify/validations/017-secure-binary-files/validate.md`
- Prior research: `docs/research/2026-11-16-015-binary-file-upload-download.md`, `docs/research/2026-11-16-016-binary-file-upload-hardening.md`, `docs/research/2026-11-16-017-secure-binary-upload-download-vps.md`
