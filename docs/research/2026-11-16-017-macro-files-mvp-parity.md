# 017 — Macro files/documents/storage: MVP parity & 80/20 slice (multilingual research)

**Date:** 2026-11-16  
**Branch/context:** `dev` (read-only research; no app source modified, no commit)  
**Goal:** Inform cycle 017 (ROADMAP P0.1 / decision 016 slice B candidate) with evidence-based, debloated guidance for a **minimal “real binary files”** capability on cheap VPS (Rust + PostgreSQL + Svelte + **local disk**, Postgres-first).

## 1. Method, sources, and limitations

**Local cross-check (read):** `OBJETIVO.md`, `ROADMAP.md`, `.specify/assessments/016-next-vertical-slice/decision.md`, `docs/research/2026-11-16-016-macro-files-ops-parity.md`, `flora-api/src/routes/files.rs`, `flora-core/src/services/file_service.rs`, `flora-core/src/models/file.rs`, `flora-core/src/config/storage.rs` (via prior research), `apps/frontend/src/lib/components/files/{FileUpload.svelte,FilePreview.svelte}`, `apps/frontend/src/lib/types/models.ts`, `apps/frontend/src/lib/utils/files.ts`, `docker-compose.yml`/`.env.example` context from 016.

**Primary external evidence (EN):**
- Macro README: https://raw.githubusercontent.com/macro-inc/macro/main/README.md (features: File storage, Docs/CRDT, Canvas/CRM/Databases/Agents/Search, @-linking, channel-based permissions, unified memory/search)
- Macro “Running locally”: https://raw.githubusercontent.com/macro-inc/macro/main/docs/development/running-locally.md (local stack includes Postgres/Redis/LocalStack/OpenSearch/Kafka/FusionAuth; storage proxy/CloudFront/local S3 semantics mentioned; operational weight)
- Macro Drive product page: https://docs.macro.com/product/folders (UX: auto-organized file storage; files from email/chat/uploads auto-imported/tagged/searchable; @mention to share; in-app previews for many types; agent/MCP access and full-text search; folders+databases; tags/CRM linkage; skills/snippets stored as files)
- Macro pricing/FAQ: https://docs.macro.com/ (plan/storage limits context: 5 GB/100 GB/1 TB by tier) and repo metadata (AGPL, large Rust monorepo)

**Multilingual coverage (requested: EN, pt-BR, es, de, fr, zh-CN, ja):** targeted web searches were executed in pt-BR, es, zh-CN, ja (and EN) for Macro file/storage/upload/download/permissions/preview/self-host terms; **no new high-confidence, locale-primary first-party Macro file-storage documents were retrieved** in this environment (NV). Findings were therefore **triangulated** with: (a) primary EN Macro docs above, (b) prior multilingual Flora research already in-repo (`016-macro-files-ops-parity.md`, `015-macro-debloat-vps-prioritization.md` / `seed-vps-profile-audit.md`) covering pt-BR/es/de/fr SMB/VPS and zh-CN/ja realtime/stack constraints, and (c) Flora’s own roadmap/objective constraints. No direct de/fr locale-primary Macro file-doc contradiction found; treat non-EN operational specifics as NV until corroborated by primary localized sources.

**Not done (NV):** runtime benchmarks/load/OOM, browser e2e, backup/restore drills, inspection of Macro private backend implementation, exhaustive API contract comparison beyond public docs/README.

## 2. Macro: file/document/storage behavior & UX expectations (evidence)

| Area | What Macro provides (public evidence) | Implication / coupling |
|---|---|---|
| **File storage / Drive** | “Auto-organized file storage.” Files from **email, chat, and uploads** auto-imported/siphoned into one searchable system; attachments alongside docs/spreadsheets/files. [folders] | Value depends on **ingestion pipelines** (email/channel), indexing, tagging, and cross-surface graph — far beyond blob CRUD. |
| **Organization & views** | Folders for compatibility/sharing; databases side-by-side for property tracking/sorting; drag-to-upload UX; filter by type/tag/creator. [folders] | Rich metadata/property model and UI; Seed can start with flat/list + basic filters later. |
| **Sharing & permissions** | “Channel-based permissions. Anything you @mention in a channel is automatically shared with its members. Join a channel, gain access…”; @mention files in chat/docs/canvas/tasks; bidirectional @-linking graph. [README, folders] | Permissions are **graph/context-driven** in Macro; Flora Seed should use simpler **org/workspace (+optional owner) authz** first. |
| **Previews / open-in-app** | Many file types open in-app (PSD/AI/Figma/PPT/Word/PDF/video etc.); notes in Docs, sheets in Spreadsheets, whiteboards in Canvas. [folders] | High preview/render/transcode surface; defer for VPS MVP (download + basic type metadata is enough). |
| **Search & agents** | Files fully searchable; agents can read/search Drive (MCP/Claude/ChatGPT mentioned) with full-text search; unified search across workspace incl. attachment PDFs (parsed) per README. [README, folders] | Requires indexing/extraction pipelines (and in Macro’s broader stack, search infra). Seed should prefer metadata/filename (and later Postgres FTS) without parsing. |
| **Cross-object linkage** | Files show on CRM records; @mention in Tasks attaches to work; skills/snippets stored as files; bidirectional links across email/chat/docs/tasks/CRM/calls. [README, folders] | Deep product graph; not required for 017 minimal attachment/file slice. |
| **Docs/collaboration** | Collaborative markdown docs on **CRDTs** (Cloudflare durable objects), history/forking/offline reconciliation, agentic editing. [README] | Heavy real-time/state infra; explicitly out of 017 scope. |
| **Storage/infra footprint** | Local dev: Docker/Nix stack with **LocalStack** (S3/SQS/DynamoDB emulation), OpenSearch, Kafka, Redis, FusionAuth etc.; local storage proxy/public URL/signing semantics in docs; hosted tiers include storage quotas. [running-locally, pricing] | Cloud-parity/object-store/search/queue/idp patterns are **not** cheap-VPS defaults; Flora should use **local FS + Postgres metadata** and avoid mandatory external deps. |

**UX expectations distilled for a minimal viable file slice (SMB):** upload a file, see it attached/listed with name/type/size/owner/time, download it reliably, be blocked when over limits or unauthorized, and not lose bytes/metadata consistency. Everything else (auto-import, previews, parsing, graph sharing, CRDT, S3/CDN) is additive.

## 3. Flora current state (cross-check)

| Layer | Observed | Gap vs 017 needs |
|---|---|---|
| **API** `flora-api/src/routes/files.rs` | `POST /api/v1/files` (JSON metadata: name/type/size/workspace_id), `GET /{id}` metadata, `DELETE /{id}` soft-delete (owner-only). Auth+org membership checks. | **No binary upload (multipart/stream), no download/stream endpoint, no content retrieval, no range/headers semantics.** Response/contract mismatches frontend (`FileResponse` vs `FileAttachment`). Workspace tenancy not reflected in response/path model. |
| **Service** `flora-core/src/services/file_service.rs` | Validates filename traversal chars and blocks a few executable MIME types; checks **per-request** size against `org_quota_bytes`/`workspace_quota_bytes`; generates `storage_path=orgs/{org}/files/{id}`; persists DB record; list/soft-delete/restore/purge. | **No actual byte write/read/delete from disk, no checksum computation, no aggregate/used-bytes quota accounting, no workspace association stored/enforced, no multipart/chunk handling, no content sniffing/storage sandbox verification at IO layer.** |
| **Model** `flora-core/src/models/file.rs` | Fields include `organization_id`, `owner_id`, `file_type`, `name`, `size_bytes(i64)`, `storage_path`, `checksum`, `metadata`, `is_deleted`, timestamps. | **No `workspace_id` (NV if schema differs) — blocks clean workspace-scoped listing/linking.** `CreateFileInput` carries `workspace_id` but service discards it. |
| **Config/storage** | Local backend default path `/var/flora/storage`, quotas (10GB org / 2GB ws), multipart/chunk flags present but not realized. | Need **truthful local-disk IO**, safe path joins/containment, size limits wired to request/FS, and operable defaults for VPS. |
| **Frontend** `apps/frontend` file components | `FileUpload.svelte` posts **multipart FormData** to `/api/v1/files/upload` and expects `FileAttachment`; `FilePreview.svelte` downloads from `/api/v1/files/download/{id}` using `file.fileName`; client-side 50MB check; store/types exist. | **Contract drift:** endpoints, request/response shapes, and fields (`fileName/mimeType/byteSize/uploadedAt/rustFsUrl`) do not match backend. Wiring to core flows (messages/tasks/workspace context) appears partial. |
| **Ops/profile** | 016 hardened Seed posture (Postgres-first, optional search/messaging, compose profiles/limits/healthchecks) but 017 must preserve cheap-VPS simplicity. | File IO must be **streaming/bounded** (avoid buffering whole uploads in RAM), respect local volume, and keep no new mandatory services. |

## 4. Prioritized keep / replace / defer (017-focused)

### KEEP (build now, Seed/VPS-aligned)
1. **Real binary blob path on local disk (P0):** authenticated multipart upload → stream to temp/final path under configured storage root → persist metadata (+ computed checksum if cheap) → authenticated download/stream with correct headers.
2. **Tenant & path safety:** org membership + org ownership checks on read/write/delete; workspace association validated and stored if schema supports (or explicitly scoped) — never trust client paths; enforce storage-root containment and safe filenames/keys.
3. **Simple, truthful limits:** enforce max upload size and coherent quota policy (at minimum per-file + documented/configured caps; prefer aggregate-used accounting within 017 if feasible, else explicit NV/guardrail) with clear 413/403/400 errors.
4. **Minimal contract alignment:** one stable API+DTO contract consumed by `apps/frontend` canonical file components for the happy path (upload, metadata/get/list as needed, download link).
5. **Postgres as source of truth for metadata; no new mandatory infra.**

### REPLACE / SIMPLIFY (vs Macro)
1. **Local FS default** instead of S3/LocalStack/CDN/object pipelines; optional S3 deferred.
2. **Explicit org/workspace ACL checks** instead of Macro’s channel/@mention graph-driven sharing.
3. **Metadata/filename-first discovery** (and Postgres later) instead of OpenSearch + attachment/PDF parsing + unified cross-surface indexing.
4. **Simple attachments/files list/download** instead of Drive auto-import from email/channels, in-app previews/editors, canvas/CRM/database embedding.
5. **Postgres-backed/simple app logic** instead of Kafka/queue-heavy ingestion and CRDT/collab document runtime.

### DEFER (post-017 / P1-P2 unless proven necessary)
1. Auto-import from email/channels/messages, tagging/classification, agentic file organization/search, MCP file tools.
2. Rich previews/transcoding/OCR/PDF parsing/full-text extraction, version history/forking, collaborative docs/CRDT.
3. S3/CDN/signed public URLs/multi-region/object lifecycle, resumable/multipart-upload protocol sophistication beyond minimal need, dedupe/GC sophistication.
4. Sharing links, fine-grained per-file ACL graphs, CRM/database/task deep embedding parity.
5. Antivirus/malware scanning pipeline (note as security NV; keep strict type/size/path controls).

## 5. 017 acceptance criteria suggestions (smallest viable, verifiable)

**A. Binary storage correctness (backend/core)**
- AC1: `POST` upload endpoint accepts authenticated binary payload (multipart form-data preferred for frontend compatibility) and **persists bytes to local disk** under `FLORA_STORAGE__LOCAL_PATH` within an org-scoped, path-safe location; DB record points to stored object.
- AC2: `GET` download endpoint streams stored bytes for an authorized member of the same org, returns appropriate `Content-Type`, `Content-Length` (when known), and safe `Content-Disposition` filename handling; 404/403/410 behaviors are explicit.
- AC3: On metadata/list/get responses, `size_bytes`, `file_type`/MIME, name, ids, and timestamps are consistent with stored content; checksum is populated if implemented (NV-acceptable if explicitly deferred with integrity risk noted).
- AC4: Upload enforces configured/request max size and rejects oversized payloads without persisting partial corrupt state (or cleans up). Filename/path traversal and storage-root escape are prevented.

**B. Tenancy, authz, and quotas**
- AC5: Cross-organization access is forbidden for metadata and bytes; non-members cannot upload/download; delete remains owner-or-policy as defined (document choice).
- AC6: Workspace context is validated and not ignored silently (store and/or enforce association per chosen minimal schema/API contract). Quota behavior is deterministic and documented (aggregate accounting preferred; if per-request-only remains, state NV explicitly).

**C. Frontend/API contract (canonical `apps/frontend`)**
- AC7: Happy-path UI can upload a real file and download/open it via the aligned endpoint/DTO contract (no reliance on stubbed `rustFsUrl` semantics unless mapped intentionally).
- AC8: User-visible errors for too-large/unauthorized/failed uploads are handled minimally (no silent success).

**D. Verification & operability (Seed/VPS)**
- AC9: Deterministic automated tests cover core service/route behavior (path safety, authz, size/limit, upload→download or storage IO via temp dir) and run in CI-appropriate profile; DB-gated cases added only where valuable and feasible.
- AC10: Docs/config note local storage path/volume, limits, and any NVs (no previews, no S3, no aggregate quota if deferred). Implementation avoids whole-file buffering patterns unsuitable for 4 GB hosts where practical.

**Suggested out-of-scope for 017:** previews/renderers, email/chat auto-import, search indexing of contents, sharing links/ACL graph, S3, resumable chunk protocol, malware scanning, CRDT docs.

## 6. Risks and NVs

- **Security:** malicious filenames/types/content; path traversal/symlink issues; missing malware scan (defer but acknowledge). Mitigate with strict validation, containment, safe headers, and size caps.
- **Resource:** buffering large uploads/downloads can pressure 2 vCPU/4 GB — prefer streaming and conservative limits.
- **Contract/schema drift:** `workspace_id` absent in `File` model and FE/BE DTO mismatch can force migration/contract decisions; choose smallest compatible change and document it.
- **Quota correctness:** config defines quotas but current enforcement is per-request only; aggregate tracking may need schema/query work (scope carefully).
- **Multilingual evidence NV:** no new locale-primary Macro file-storage docs (pt-BR/es/de/fr/zh-CN/ja) retrieved this run; recommendations rely on EN primary + in-repo triangulation.
- **Operational NV:** no load/OOM or backup+restore drill executed for file blobs; compose volume/backup guidance should include storage dir alongside Postgres.

## 7. Bottom line

For 017, treat Macro’s file system as a **rich, ingestion/search/graph/agent-coupled Drive** and implement only its **durable core** for Flora: **secure tenant-scoped binary upload/download on local disk with honest metadata, limits, and a working frontend contract**. Defer everything that depends on parsing pipelines, CRDT/collab, cloud object infra, or graph permissions. This is the highest-leverage 80/20 step toward OBJETIVO/ROADMAP P0 “arquivos básicos reais” while staying operable on cheap VPS.
