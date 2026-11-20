# 016 — Macro files/document handling & ops parity (multilingual, evidence-based)

**Date:** 2026-11-16  
**Scope:** Deep research on [macro-inc/macro](https://github.com/macro-inc/macro) **file/document handling** and **self-hosting/operational footprint** (storage, dependencies, infra requirements), compared with a cheap-VPS **Rust + PostgreSQL + Svelte + local-disk** alternative in minimal **80/20** form. Focus: debloating heavy/proprietary/cloud dependencies and practical SMB/PME needs.  
**Task constraint:** no application source modified, no commit.

## 1. Method, evidence, and limitations

**Local cross-check (read):** `OBJETIVO.md`, `ROADMAP.md`, `docs/research/2026-11-16-015-macro-debloat-vps-prioritization.md`, `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docker-compose.yml`, `.env.example`, `Cargo.toml`, `flora-core/src/config/{mod,storage}.rs`, `flora-api/src/routes/files.rs`, `flora-core/src/{models,file_service}.rs`, `flora-api/src/state/*` (via grep).  
**External evidence (fetched):** Macro `README.md`, `docs/development/running-locally.md`, `CONTRIBUTING.md`, GitHub repo metadata API (`macro-inc/macro`, AGPL-3.0, Rust-primary, ~2.96 GB repo size snapshot, 2026-11-16).  
**Multilingual pass (requested EN, pt-BR, es, de, fr, zh-CN, ja):** direct web queries were attempted in pt-BR/es/de/fr/zh-CN/ja terms but the search tool returned no usable hits in this environment (**NV** for new first-party localized sources). To satisfy depth responsibly, findings were triangulated with multilingual/regional evidence already consolidated in `015` (pt-BR/es/de/fr hosting/collab sizing context, zh-CN/ja realtime/architecture signals cited in prior Flora research) and re-validated against primary EN Macro docs and current Flora repo state. No runtime benchmarks performed.

## 2. Cross-check anchors (Flora)

| Anchor | Relevance to files/ops (016) |
|---|---|
| `OBJETIVO.md` §2/3.4/5 | MVP P0 includes **arquivos básicos** (upload/download real, metadados/permissões mínimas, quotas coerentes, storage local por padrão) and operability (compose, migrations, health/readiness, backup/restore). Gaps: binário/upload/download real e quota/escopo; Seed/VPS profile/opcionalidade/debloat; NV for standard-port smoke and optional-service decoupling in practice. |
| `ROADMAP.md` §4 P0.1/P0.2 | Cycle 016 candidate **B:** real binary upload/download on local disk (multipart/streaming, path safety, limits, checksums, quotas/tenant) aligned to frontend. Candidate **C:** Seed/VPS operability (Meilisearch/Valkey optional in boot/config, limits/healthchecks/profiles, `FLORA_*`/pool alignment, backup/restore mínimo). |
| `015` debloat map | Keep core files basic in monolith/Postgres+local FS; replace OpenSearch/Kafka/FusionAuth/LocalStack/cloud-shaped mandatory stack; defer email sync/calls/agents/CRM/databases/canvas/CRDT-heavy parity. 2 vCPU/4 GB envelope: lean `api+postgres` preferred. |
| `seed-vps-profile-audit` | Postgres 18 mandatory (`uuidv7`), Valkey runtime-degraded-optional but Meilisearch boot-coupled/fail-hard, pool hardcoded/duplicated init, compose unbounded, env drift, local storage default. **SQLite fallback not operative** despite config surface. |

## 3. Flora current state — files and ops (repo evidence)

### 3.1 File/document handling (partial, metadata-first)
- **API surface:** `flora-api/src/routes/files.rs` exposes `POST /api/v1/files`, `GET /api/v1/files/{id}`, `DELETE /api/v1/files/{id}` (soft-delete) with auth + org membership checks and owner-only delete. `POST` records **metadata only** (`name`, `file_type`, `size_bytes`, `workspace_id`) and returns created record; **no multipart/body-bytes upload, no download/stream endpoint, no content retrieval**.
- **Domain/service:** `flora-core/src/models/file.rs` stores `organization_id`, `owner_id`, MIME-like `file_type`, `name`, `size_bytes`, `storage_path`, `checksum` (optional), `metadata`, `is_deleted`, timestamps. `flora-core/src/services/file_service.rs` validates filename traversal chars and blocks a small executable/script MIME allow-risk set, enforces **per-request size against org/workspace quota caps** (`org_quota_bytes`, `workspace_quota_bytes` from config), generates `storage_path = orgs/{org_id}/files/{file_id}`, persists DB record; supports get/list/soft-delete/restore/purge. **No actual byte write/read, no checksum computation, no aggregate quota accounting evidenced in this path, no multipart/chunk handling, no content-type sniffing/storage policy beyond basic checks.**
- **Config:** `flora-core/src/config/storage.rs` defaults `backend=local`, `local_path=/var/flora/storage`, S3 fields optional, `org_quota_bytes` 10 GB, `workspace_quota_bytes` 2 GB, `multipart_enabled=true`, `chunk_size_bytes` 5 MB — but implementation does not yet realize multipart/S3/local byte I/O. `.env.example` documents `FLORA_STORAGE__LOCAL_PATH` only.
- **Search/wiring:** files exist as domain concept and search route mentions files in comment, but P0 wiring/indexing/notification completeness remains NV/partial per roadmap/objective.

### 3.2 Operational/infra footprint (current)
- **Compose (`docker-compose.yml`):** `postgres:18-alpine` (required for `uuidv7`), `valkey/valkey:8-alpine` (AOF), `getmeili/meilisearch:v1.12` (dev master key, analytics off), all bound to `127.0.0.1`, named volumes, no CPU/memory limits, no profiles/overlays. Startup intent in comments: `docker compose up -d` then run API/migrate.
- **Env/config:** `Config::load()` uses `FLORA_*` with `__` nesting (`flora-core/src/config/mod.rs`); `.env.example` mixes prefixed app/db/messaging/search/storage vars with non-prefixed `OIDC_*`/`ZITADEL_*` and `FLORA_OIDC__REDIRECT_URI` (drift risk). Meilisearch commented optional in `.env.example` but boot behavior currently coupled/fail-prone per audit; Valkey degrades gracefully at runtime.
- **Runtime topology:** `flora-api` is the real entrypoint (Axum, migrations on startup); modular monolith crates; Dockerfiles exist for API/frontend but no minimal VPS-tuned compose profile evidenced.

## 4. Macro: file/document handling (evidence)

| Capability (Macro) | Public evidence | Ops implication |
|---|---|---|
| **File storage / attachments** | README lists **“File storage”** block: “Auto-imported from email and channels, fully searchable.” Docs link: product/folders. [M1] | Requires ingestion pipelines (email/channel), parsing/indexing hooks, permissions, searchable metadata/content — beyond simple blob CRUD. |
| **Collaborative docs** | README **“@linked markdown docs powered by CRDTs”**: native markdown, live collaboration with CRDTs and Cloudflare durable objects, history/forking, offline editing/reconciliation, @-links, embeds, mobile, agentic editing in CRDT system. [M1] | Rich doc state/sync layer; collab runtime and conflict model materially heavier than append/versioned markdown on single VPS. |
| **Databases / canvas / CRM objects** | Typed tables/relations/views/agent queries; 2D canvas with embedded links; CRM records tied to emails/files/tasks/calls/discussions. [M1] | Structured/object storage and graph/link semantics increase schema, indexing, and UI/backend coupling (not core 80/20 for Seed files). |
| **Email/PR/call artifacts** | Multi-account email with attachment handling/search, PR links, recorded calls/transcripts/summaries. [M1] | External sync, media/transcript storage, and provider integrations drive background workers, secrets, bandwidth/CPU, and larger storage classes. |
| **Search over files/content** | Unified search across workspace including file attachment PDFs (parsed) and cross-surface graph. [M1] | Implies extraction/index pipelines and often dedicated search infra in Macro’s broader stack (see ops). |

**80/20 read:** Macro’s file/document value is tightly coupled to **ingestion + parsing + collab + graph + search + agents**. For cheap-VPS SMB Seed, Flora should target **durable tenant-scoped blob + metadata + basic permissions/quotas + download** and defer CRDT/auto-import/parsing pipelines.

## 5. Macro: self-hosting / operational footprint (evidence)

| Area | Evidence (EN primary) | Footprint signal |
|---|---|---|
| **Local/dev stack & dependencies** | `running-locally.md`: Nix + Docker/Compose; local stack brings up **Postgres, Redis, LocalStack, OpenSearch, Kafka, FusionAuth** plus many Rust/aux services; stubs for many integrations; first builds/wasm/images can be heavy. Also local Mailpit/auth/dev proxy flows. [M2] | Multi-service, stateful, ops-heavy for a small VPS/operator; designed for rich dev parity, not minimal production. |
| **Service sprawl / architecture** | README/product breadth + repo scale (Rust-primary, very large monorepo snapshot) and prior 015 synthesis (many deployables/services/workers, AWS-shaped components, queue/search/idp stacks). [M1][M3][M4 in 015] | High fixed RAM/process/debug overhead vs modular monolith. |
| **Storage/search/messaging/idp shape (as documented/synthesized)** | LocalStack (S3/SQS/DynamoDB emulation), OpenSearch, Kafka, Redis, FusionAuth in local flows; object/static/CDN patterns in broader architecture discussions (015). [M2][015] | Cloud-parity components are poor defaults for 2 vCPU/4 GB single-node. |
| **Self-host practicality** | README/FAQ positioning in ecosystem: AGPL self-host possible but production-grade one-command simplicity not implied by dev stack; third-party analyses in 015 reported non-trivial setup and higher resource/ops burden for small teams; hosted comparator exists. [M1][015][M3/M7/M10 in 015] | “Can run” ≠ “cheap VPS friendly”; Flora must optimize for operator simplicity. |

**Multilingual/regional interpretation (transparent):** direct new localized Macro teardowns were NV this pass; however 015 triangulated pt-BR/es/de/fr SMB/VPS hosting context and zh-CN/ja realtime/stack practice indicating that **2 vCPU/4 GB** is viable for **focused** self-hosted apps when DB/search/cache/brokers are constrained or minimized — consistent with EN Macro footprint being materially above that baseline. [015:R1–R6, F4]

## 6. 80/20 recommendations — cheap VPS Rust+Postgres+Svelte+local-disk

### 6.1 Target profile (Seed)
- **Topology:** single modular monolith API + Postgres 18 + local filesystem volume; optional Valkey/search only as explicit profiles when justified. Svelte frontend static/served separately or behind proxy.
- **Resource posture (planning, NV benchmark):** reserve ~512 MB OS; aim `api+postgres` first (~1.5–2.0 GB typical envelope), avoid mandatory Meilisearch/JVM/broker/idp; keep DB connections modest (tune pool from config), add compose limits/health/readiness semantics. [seed-vps-profile-audit, 015]

### 6.2 Files/documents — minimal 80/20 (debloated)

| Need (SMB) | Do now | Replace/simplify vs Macro | Defer/drop |
|---|---|---|---|
| Store & retrieve files | **Real binary upload + download/stream** on local disk, tenant/path-safe storage keys, DB metadata as source of truth, optional checksum on write, basic MIME/size validation, authenticated org/workspace-scoped access | Local FS default (no S3/LocalStack mandatory), no email/channel auto-import, no PDF parsing pipeline | S3/CDN, resumable/multipart complexity unless needed, virus scan pipeline, e-sign/version-heavy DMS, media transcoding |
| Quotas/safety | Enforce **effective** limits (per-file + tracked org/workspace usage or conservative checks) aligned to config; clear 413/403 errors; storage path sandboxing | Simple quota accounting in Postgres | Billing-grade metering, dedupe/object GC sophistication beyond periodic cleanup |
| “Documents” | Start with **file attachments + optional lightweight markdown/text notes** stored simply (DB or files) with basic version metadata later | No CRDT/DO collab runtime, no canvas/DB parity | Real-time collaborative CRDT docs, offline CRDT sync, embeds/graph-heavy doc platform |
| Discovery | Filename/metadata search in **Postgres FTS/trgm** for core objects when wired | No OpenSearch/attachment parsing required for Seed | Full-text extraction of PDFs/emails, semantic/agent retrieval |

### 6.3 Ops/dependencies — minimal 80/20

| Area | Recommendation | Rationale |
|---|---|---|
| Postgres | **Mandatory, single source of truth**; PG18+ for current migration path; backups (`pg_dump`) + restore drills; persistent volume (`/var/lib/postgresql`) | Aligns with `uuidv7` and Flora architecture [compose, dev-smoke, audit] |
| Search | **Default off or truly optional** (Postgres FTS first); never fail Seed boot because external search missing | Current boot coupling is a debloat blocker [audit, 015] |
| Cache/pubsub (Valkey/Redis) | **Optional profile**; keep in-memory/DB-safe fallback; cap memory/AOF policy when enabled | Useful later, not required for correctness on single-node Seed [audit, 015] |
| Queues/workers | **In-process + Postgres-backed jobs/outbox** only when needed | Avoid Kafka/SQS/LocalStack complexity [015, M2] |
| Auth/IdP | **Embedded/session core**; OIDC as optional integration | Avoid FusionAuth-class mandatory dependency [015, M2] |
| Compose/config | Provide **profiles/overlays** (core, +realtime, +search), `mem_limit`/reservations, localhost bindings, healthchecks, and consistent `FLORA_*__*` contract; wire DB pool config and avoid duplicate init | Directly addresses audit findings and 4 GB realism [audit] |
| Storage ops | Persistent mount for `FLORA_STORAGE__LOCAL_PATH`, permissions, backup of DB+files together (consistency-aware as feasible) | Local-disk default requires operator discipline [storage config, audit] |

## 7. Implications for cycle 016 (B vs C)

**Decision lens (80/20 for “runs on cheap VPS”):** deliver one **verifiable, safe, low-footprint** slice that unblocks real SMB use without increasing mandatory services.

### Option B — Basic real binary file upload/download (product slice)
- **Pros:** closes a concrete P0 gap in `OBJETIVO`/`ROADMAP` (real upload/download, quotas/tenant-safe storage), makes files demoable end-to-end, leverages existing model/service/route scaffolding.
- **Costs/risks:** touches API contract (multipart/streaming or equivalent), storage I/O, limits/checksums, potential large-body handling and proxy/compose timeouts; can add subtle security/perf issues if rushed. On 4 GB, naive buffering can hurt; must stream and cap sizes.
- **Best when:** user-visible “files actually work” is the top Seed blocker after messaging/realtime progress.

### Option C — Seed/VPS operability hardening (ops/debloat slice)
- **Pros:** directly addresses highest deployment-risk findings: make **Meilisearch/Valkey optional by boot/profile**, add resource caps/profiles/health semantics, align `FLORA_*`/pool/init, document backup/restore — increasing probability that Flora starts and stays up on 2 vCPU/4 GB.
- **Costs/risks:** less visible feature progress; if files remain metadata-only, core collaboration utility is still incomplete.
- **Best when:** “can’t reliably run/configure on cheap VPS” is the dominant blocker for adoption/testing.

### Evidence-based recommendation
1. **Prioritize C-like operability fixes as prerequisites or as the cycle’s spine** if a choice must be made: Macro’s and Flora’s current risks show **dependency/boot/config bloat** can negate feature work on cheap VPS (Meilisearch coupling, unbounded compose, pool/init issues). A Seed that cannot boot leanly cannot benefit from file bytes. [audit, 015, ROADMAP P0.2]
2. **Scope B narrowly if selected in 016:** implement **local-disk, streaming, single-request upload/download with strict size/path/auth/quota checks and checksum**, explicitly deferring multipart/resumability/S3/preview/indexing pipelines. Treat as “basic file blob parity,” not Macro document-suite parity. [OBJETIVO, ROADMAP P0.1]
3. **Recommended sequencing (pragmatic 016):** do **minimum C gates required for safe B** in the same slice where feasible — e.g. ensure storage path/limits are configurable and optional services don’t block the profile used for file testing — then deliver **thin B**. If capacity is constrained, pick **C first**, **B next** (016→017), because C reduces operational risk for every subsequent slice.
4. **Acceptance evidence for 016:** documented profile/compose behavior (what runs on 4 GB), boot success without non-required services, and for files a verifiable upload→persist→download (or equivalent byte path) with tenant isolation and quota/limit enforcement; mark e2e/browser/NV gaps explicitly per constitution.

## 8. Keep / Replace / Defer summary (files & ops)

- **Keep/build:** Postgres-backed metadata, modular monolith, tenant-scoped authz, local-disk storage, basic quotas/limits, simple health/backup story.
- **Replace/simplify:** Macro’s OpenSearch/Kafka/FusionAuth/LocalStack/CRDT-cloud collab/email-sync/media pipelines → Postgres FTS/jobs, embedded auth, local FS, simple notes/versioning, minimal realtime/catch-up patterns.
- **Defer:** attachment auto-import+parsing, collaborative CRDT docs, canvas/databases/CRM/email/calls/agents parity, S3/CDN/multi-node object semantics, advanced DMS workflows.

## 9. Sources

**Primary/external**
- [M1] Macro README (features, files/docs/CRDT, AGPL): https://raw.githubusercontent.com/macro-inc/macro/main/README.md
- [M2] Macro running locally (Nix/Docker, Postgres/Redis/LocalStack/OpenSearch/Kafka/FusionAuth, stubs): https://raw.githubusercontent.com/macro-inc/macro/main/docs/development/running-locally.md
- [M3] Macro repo metadata: https://api.github.com/repos/macro-inc/macro
- [M4] Macro CONTRIBUTING: https://raw.githubusercontent.com/macro-inc/macro/main/CONTRIBUTING.md

**Flora internal (cross-check)**
- [F1] `OBJETIVO.md`
- [F2] `ROADMAP.md`
- [F3] `docs/research/2026-11-16-015-macro-debloat-vps-prioritization.md` (multilingual synthesis, debloat map, VPS envelope)
- [F4] `docs/research/2026-11-16-seed-vps-profile-audit.md`
- [F5] `docker-compose.yml`, `.env.example`, `Cargo.toml`
- [F6] `flora-core/src/config/mod.rs`, `flora-core/src/config/storage.rs`
- [F7] `flora-api/src/routes/files.rs`, `flora-core/src/models/file.rs`, `flora-core/src/services/file_service.rs`

**Limitations:** no new localized primary Macro self-host/file-storage docs retrieved in pt-BR/es/de/fr/zh-CN/ja during this run (search tool NV); 015 cited regional context retained for triangulation. No load/perf benchmarks executed. Macro upstream may drift.