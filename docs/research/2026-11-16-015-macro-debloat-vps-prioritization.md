# 015 — Macro debloat & VPS prioritization (multilingual research)

**Date:** 2026-11-16  
**Scope:** Evidence-based analysis of [macro-inc/macro](https://github.com/macro-inc/macro) architecture/features and 80/20 prioritization for a self-hosted Flora alternative (Rust + PostgreSQL + Svelte, AGPL-aligned direction) runnable on a cheap VPS (**target: 2 vCPU / 4 GB RAM / SSD**, single-node, low concurrency) for SMBs/PMEs.  
**Method:** Repo inspection + existing Flora research cross-check + multilingual web research (EN, pt-BR, es, de, fr, zh-CN, ja queries/sources where available). No runtime benchmarks, no application source changes. Recommendations are hypotheses informed by public evidence and repo state; items without direct measurement are marked **NV**.

## 1. Cross-check with Flora sources (truth anchors)

| Source | Key takeaways relevant to 015 |
|---|---|
| `OBJETIVO.md` (2026-11-15/16) | North star: viable, performant, self-hosted Macro alternative for PMEs on cheap VPS; debloat, Postgres-first, optional components, verifiable vertical slices. MVP P0: identity/tenancy, communication (channels/messages + minimal realtime), light tasks, basic files, Postgres FTS, minimal in-app notifications, operability. P2 defers email client sync, calls/transcription, CRM/databases/canvas, agents/memory/routines/sandbox, deep integrations, Macro UX parity. Current gaps: tenancy/session authority consistency (HTTP improved through cycle 012; WS NV), realtime fanout/delivery, files binary/quota, search/notification wiring, optionality/debloat, `apps/web` drift. |
| `ROADMAP.md` (2026-11-16) | Cycles 011–013 merged (contract/session authority HTTP, REST workspace→channel→message slice). Cycle 014 candidates: DB-backed WS session/protocol alignment, minimal reliable realtime for persisted messages (or lean VPS alternative), make Meilisearch/Valkey truly optional by profile/boot with limits/healthchecks/backup, consolidate e2e slice, basic binary files + essential wiring. Seed criterion: consistent auth/tenancy + verifiable core e2e in reproducible dev, operable on modest VPS. |
| `docs/research/2026-11-15-macro-alternative-80-20-research.md` | Established Macro scope/stack heaviness and initial keep/replace/defer map (OpenSearch→Postgres FTS, Kafka→Postgres jobs/outbox, FusionAuth→embedded/OIDC optional, LocalStack/AWS-shaped→local FS, Workers/DO→unified runtime, microservices→modular monolith, defer email sync/calls/agents/CRM/databases/canvas/PR depth). Cheap VPS realism: ~2 vCPU/4 GB recommended profile. |
| `docs/research/2026-11-16-seed-vps-profile-audit.md` | Repo reality: PG18 mandatory path (`uuidv7`), Valkey runtime-degraded-optional but Meilisearch currently boot-coupled/fail-hard in `AppState` (contradicts "optional"), DB pool hardcoded/duplicated init paths, compose unbounded, env naming drift (`FLORA_*__*` vs non-prefixed OIDC vars), local storage default. Current stack can pressure 4 GB. |
| `docs/research/2026-11-16-014-seed-vps-operability-international.md` | Low-risk ops: `core/realtime/search` profiles, resource caps/reservations, localhost bindings, PG tuning for 4 GB (`shared_buffers` ~256–512MB, conservative `work_mem`, modest connections), Valkey `maxmemory` only when enabled, Meilisearch constrained/disabled when possible, `pg_dump` + storage backups + restore drills, readiness distinguishing required vs degraded. |
| `docs/research/2026-11-16-014-realtime-delivery-vps-options.md` | Flora has partial WS + Valkey publish but no complete fanout/delivery/recovery contract; frontend path/protocol drift (`/api/v1/ws`, token query, missing `/messages/sync`). 80/20: Postgres source-of-truth + deterministic cursor catch-up + minimal push (SSE notifications or finished single-node WS) + in-process fanout; defer Valkey multi-instance until needed; keep DB recovery for lossy channels. |

**Alignment verdict:** Prior research already points to the right Seed shape. 015 should sharpen *feature-level* keep/replace/defer against Macro’s full product surface and translate it into cycle sequencing and VPS tradeoffs.

## 2. Macro: architecture and feature surface (public evidence)

### 2.1 Product scope (README/docs, EN)
Macro positions itself as an all-in-one team workspace/“operating system”: **email (multi-account, unified inbox, shared inboxes), messages/channels/DMs, tasks, collaborative markdown docs (CRDT), canvas/2D board, agents with shared team memory/MCP/tools, calls (recorded/transcripts/summaries), file storage, pull requests, CRM (contacts/companies/custom props/enrichment), databases (typed tables/relations/views/agent queries), calendar/scheduling/booking** — all `@`-linked in a bidirectional graph with unified search/inbox and channel-based permissions. Frontend is static SPA (historically SolidJS in repo/docs signals) and backend/services in Rust; desktop shell/Tauri references appear in ecosystem descriptions. License: **AGPL-3.0** (full open source, not open core, per README; commercial/self-host contacts listed). [M1][M2][M3]

### 2.2 Implementation/operational shape (docs, DeepWiki/grok-wiki summaries, self-host analyses)
- **Monorepo at scale:** Cargo workspace with many Rust services/workers/lambdas/crates; Bun/TS/JS frontend/tooling; Nix/`just` dev flow; Pulumi/AWS-oriented infra stacks in repo. Representative service names in local/runtime docs include `authentication_service`, `connection_gateway(_service)`, `document_storage_service`, `email_service`, `search_processing_service`, `static_file_service`, `notification_service`, `contacts_service`, `document_cognition_service`, etc. [M4][M5][M6]
- **Local stack (development-oriented, not production runbook):** requires Nix + Docker/Compose; `just run_local --no-doppler`/setup flows bring up infrastructure including **PostgreSQL, Redis, LocalStack (S3/SQS/DynamoDB emulation), OpenSearch, Kafka, FusionAuth** (and additional service containers/images). Docs state stubbed config “do not work against real services” for many integrations. [M2][M3][M7][M8]
- **Data/processing dependencies:** Postgres as core DB in many paths; Redis for caching/auth/rate/state; OpenSearch for search; Kafka/SQS/queues for event/search/email/worker pipelines; DynamoDB-style metadata/connection tables in cloud/local-E2E patterns; S3/object storage and CDN/static-file flows; FusionAuth for auth; CRDT/collab and AI/agent/call features imply additional runtimes/providers. `search_processing_service` documents Kafka consumer semantics (commit-after-handoff loss window, retries, poison-record handling) and backfill/read-replica options. [M4][M5][M9]
- **Self-host reality (third-party analyses, EN):** AIBrew/GenZTech/RepoCloud/OpenSourceAlternatives characterize setup as non-trivial (no one-command production installer), resource/ops heavy for small teams (multi-service distributed system), and note unresolved/nuanced questions for fully air-gapped collab (README mentions CRDTs/Cloudflare Durable Objects in collab context — **self-host substitute NV from reviewed sources**). Hosted pricing reported ~$40/seat/mo (teams) as economic comparator. [M3][M7][M10][M11]

### 2.3 Multilingual signal (availability/interpretation)
- **EN:** strongest primary evidence (README, running-locally, service docs, architecture wikis, cost/self-host analyses). Confirms “feature-rich but operationally heavy” profile. [M1–M11]
- **pt-BR/es/de/fr:** direct high-quality Macro-specific self-host benchmarks were sparse in automated retrieval; findings triangulated via international analyses plus regional VPS/collab hosting context showing that suites with search/DB/cache/auth/media stacks typically need careful sizing above minimal 1–2 GB footprints, while 2 vCPU/4 GB plans are common entry VPS tiers (e.g. Kimsufi/Hostinger/Hetzner-class offerings and self-host guides). [R1][R2][R3][R4]
- **zh-CN/ja:** existing Flora research (`014-realtime-delivery-vps-options.md`) incorporated Axum SSE/WS and chat-architecture practices (zh-CN/ja project/docs signals) relevant to realtime debloat choices; Macro-specific localized teardowns were **NV** in this pass but do not contradict EN architecture evidence. [F4]

## 3. 80/20 feature prioritization for Flora (SMB on 2 vCPU/4 GB)

Prioritization lens: **user value for PME collaboration × implementation/ops cost on 4 GB × dependency weight × ability to verify in a vertical slice**.

| Tier | Capabilities | Macro reference | Recommendation for Flora | Rationale / VPS impact |
|---|---|---|---|---|
| **P0 Keep/Build now (Seed)** | Orgs/workspaces/users, secure auth/session + tenant isolation, channels + persistent messages, light tasks, basic file upload/download/metadata/quotas, basic in-app notifications, basic search over core objects, minimal reliable realtime/catch-up, health/readiness/backup basics | Messaging, tasks, file storage, search, notifications, auth/permissions primitives | **Keep/build, Postgres-first** | Highest daily SMB value; implementable in modular monolith; aligns with `OBJETIVO`/`ROADMAP` and cycles 011–013 foundation. |
| **P1 Consolidate (post-stable Seed)** | Lightweight markdown notes/docs (no heavy CRDT initially), finer sharing/permissions, import/export, digest/optional email notifications, basic observability/limits, optional OIDC hardening as profile | Docs, permissions, email-adjacent workflows | **Replace/simplify** | Useful, but can be staged without distributed infra. Prefer append-only/versioned markdown or simple conflict strategy before CRDT/DO-class complexity. |
| **P1.5 Optional profiles (demand-driven)** | Better search relevance, shared pubsub acceleration, SSO/IdP integration, object storage/S3, mobile/Tauri client polish | Search infra, Redis, auth, storage, clients | **Optional modules/profiles** | Enable per-customer need; never mandatory for Seed boot. |
| **P2 Defer** | Full email client/multi-account sync, PR/deep code surfaces, calendar+booking depth, generic databases/CRM parity, canvas, full call stack (recording/transcripts/media) | Email, PRs, calendar, databases, CRM, canvas, calls | **Defer** | High integration/compute/storage/ops cost (OAuth sync workers, media pipelines, SFU/turn-like concerns, complex schemas/UI). Poor 80/20 on 4 GB. |
| **P2/P3 Defer or Drop-for-Seed** | Agents with unified memory/routines/sandbox/MCP deep toolchain, AI-heavy indexing/enrichment, multi-runtime collab workers (Durable Objects/Cloudflare-specific), Kafka/OpenSearch/FusionAuth/LocalStack-shaped mandatory stack | Agents, cognition, collab/search pipelines | **Defer/drop as mandatory deps** | Expensive in runtime, secrets, external providers, and operator complexity; conflicts with “cheap VPS, no proprietary mandatory deps”. BYO/external hooks only behind explicit opt-in later. |

**80/20 cut line:** a useful Seed for SMBs is **identity + channels/messages + tasks + files + basic search/notifications + operability**, not “Macro parity”.

## 4. Debloat map: Macro dependency/capability → Flora direction

| Macro area (evidence) | Why heavy/proprietary-costly for cheap VPS | Flora 015 recommendation | Tradeoff |
|---|---|---|---|
| **42+ deployable services / microservice + worker/lambda sprawl** [M4][M5][M6] | Memory/process overhead, complex deploy/debug, many failure modes | **Modular monolith (Cargo workspace, clear crate boundaries)** | Less independent scaling, but drastically simpler ops and lower fixed RAM. |
| **OpenSearch** [M2][M4][M5][M7] | JVM + index memory/disk tuning; extra service to back up/monitor | **Postgres FTS (`tsvector`/`pg_trgm`) default; external engine optional profile only** | Less advanced relevance/features initially; huge debloat win. |
| **Kafka (+ SQS/event pipelines)** [M4][M5][M9] | Broker ops, partitions/lag/retention, consumer semantics complexity | **In-process events + Postgres outbox/jobs (`FOR UPDATE SKIP LOCKED`) or simple notify/poll patterns** | Lower throughput ceiling; sufficient for Seed/SMB. |
| **Redis/Valkey as mandatory glue** [M4][M5][M7] | Extra stateful service; useful but not always required at Seed scale | **Optional profile:** in-memory/local fanout + DB catch-up first; enable Valkey when shared pubsub/rate/session distribution is needed | Single-node focus now; must keep correctness in Postgres. |
| **FusionAuth** [M2][M5][M7] | Separate IdP service/config surface | **Embedded auth/session/JWT core; OIDC/IdP optional** | Enterprise SSO deferred; simpler default install. |
| **LocalStack + AWS-shaped services (S3/SQS/DynamoDB/CloudFront paths)** [M4][M5][M7] | Cloud-parity complexity not needed for single VPS | **Local filesystem default + Postgres; S3-compatible/object storage optional** | Need disciplined backup/permissions; no cloud elasticity by default. |
| **CRDT collab via Loro + Cloudflare Durable Objects/runtime split (as reported)** [M1][M3][M10] | Specialized realtime/state infra and uncertain self-host parity | **Defer heavy CRDT; use simple doc/version model first; realtime via WS/SSE + DB** | Concurrent rich editing parity postponed. |
| **Email sync, calls, transcription, enrichment** [M1][M4] | OAuth/token lifecycle, background sync, media CPU/bandwidth/storage, third-party APIs | **Defer**; design extension points only | Major surface removed from Seed critical path. |
| **Agents/shared memory/MCP/tooling/cognition services** [M1][M4][M5] | Model costs, prompt/tool safety, indexing/memory pipelines, extra workers | **No mandatory AI in core; optional BYO hooks later** | Keeps VPS cheap/offline-friendly and reduces security/ops burden. |
| **Nix/just/Pulumi-heavy dev+deploy assumptions** [M2][M4][M5] | High contributor/operator barrier for SMB self-hosters | **Plain Docker Compose profiles + env docs + `cargo`/`bun` dev path** | May diverge from upstream ergonomics; better for target users. |

## 5. Practical operational tradeoffs on 2 vCPU / 4 GB (SMB VPS)

### 5.1 Resource envelope (planning estimates, **NV benchmarked**)
Based on `seed-vps-profile-audit` and `014-seed-vps-operability`: reserve ~512 MB for OS/cache/headroom on 4 GB. Conservative starting caps: **API 256–512 MB**, **Postgres 768–1024 MB**, **Valkey 64–128 MB (only if enabled)**, **Meilisearch 384–768 MB (avoid for lean Seed)**. A lean `api+postgres` profile is the realistic “cheap VPS” target; current compose-like stacks can become tight under spikes. [F1][F2][R4]

### 5.2 Mandatory choices for Seed
- **Postgres is the correctness backbone:** mandatory state store, migrations at startup, backup/restore priority #1 (`pg_dump` off-host + periodic restore drill into PG18). [F1][F2]
- **Valkey/Meilisearch must be truly optional:** gate boot/runtime/features and expose degraded/disabled readiness; do not fail startup for optional search/cache in Seed profile. (Directly addresses current audit finding.) [F1][F2]
- **DB pool/concurrency discipline:** wire pool config, avoid duplicate pool+migrate init, keep connections modest for 4 GB; set conservative PG params (`shared_buffers`, `work_mem`, `maintenance_work_mem`, `effective_cache_size` as hint). [F1][F2]
- **Realtime:** prefer “DB commit + cursor catch-up + minimal push” over broker-first designs; single-node in-process fanout acceptable before Valkey/`LISTEN/NOTIFY` scale adapters. [F4]

### 5.3 Observed market/context tradeoff (multilingual VPS context)
Entry EU/FR VPS offers around 2 vCPU/4 GB are widely available at low monthly cost tiers (e.g. comparison listings show 4 GB-class plans as common baseline), but operators still must budget RAM for DB/search/cache and avoid stacking JVM/search/broker services by default. SMB self-host stacks (Nextcloud/Mattermost/Gitea/n8n-class guidance in es/de/fr/pt-facing hosting content and FR/EN blogs) repeatedly fit 2 vCPU/4 GB for focused workloads when components are limited and storage/backup are planned. [R1][R2][R3][R4]

### 5.4 What to avoid on 4 GB
- Always-on OpenSearch + Kafka + Redis + IdP + multiple workers.
- Unbounded containers, large connection pools, or duplicate service initialization.
- Treating realtime/queues as source of truth instead of Postgres.
- Requiring Cloudflare/external proprietary runtimes for core collaboration.

## 6. Implications for cycles 015+

| Cycle focus | Recommended 015+ implication | Depends on / evidence |
|---|---|---|
| **015 (recommended): Seed profile & optionality closure** | Make **Meilisearch and Valkey profile-optional** at boot/runtime with readiness states; add VPS compose overlays (`core`, `realtime`, `search`) with localhost bindings, restart/health semantics, and resource caps; wire DB pool config and remove duplicate init; align `.env`/`FLORA_*__*` contract and secrets guidance. | `ROADMAP` P0.3, `seed-vps-profile-audit`, `014-seed-vps-operability` [F1][F2][F3] |
| **015/016: Reliable realtime slice completion** | Finish one minimal delivery contract for `workspace→channel→message`: session-authoritative WS or SSE notifications, in-process fanout (single node), deterministic cursor sync endpoint/query, frontend protocol/path/auth alignment, idempotent merge/reconnect, org/channel authz at subscribe+emit. Keep Valkey publish as optional adapter, not correctness dependency. | `ROADMAP` P0.1–P0.2/P0.4, `014-realtime-delivery-vps-options`, `013-ws-session-authority` [F4][F5] |
| **016/017: Files & core wiring** | Implement real binary upload/download, storage path/permissions/quotas, and minimal backup story for DB+storage; wire Postgres FTS indexing and core mutation→in-app notification events only for P0 objects. | `OBJETIVO` gaps (files/search/notifications), `ROADMAP` P0.5 |
| **017+: Product consolidation (P1)** | Lightweight markdown notes, sharing/import-export polish, optional digest/OIDC profile, observability basics; revisit `apps/web` disposition and doc/README drift. | `ROADMAP` P1, `OBJETIVO` P1 |
| **Explicit non-goals for near cycles** | No email sync client, calls, CRM/databases/canvas parity, agents/memory, Kafka/OpenSearch/FusionAuth mandatory stack, multi-region/cloud-parity infra. | Macro scope contrast [M1–M11], Seed guardrails |

**Sequencing principle:** close **operability + correctness contracts** (optional deps, pool/limits, backups, authz, catch-up realtime) before adding feature breadth. Each cycle should ship a verifiable slice with explicit NV items.

## 7. Risk notes and limitations
- Macro’s upstream architecture evolves quickly; service names/infra details above reflect reviewed public snapshots/docs and may drift (**NV** for current private/prod topology).
- Self-hosted collaboration parity (especially CRDT/Cloudflare-related paths) is not fully verifiable from public docs alone; treat as a risk input, not a settled blocker claim. [M3][M10]
- No Flora load/perf benchmarks on 2 vCPU/4 GB were executed here; memory caps/tuning are conservative starting points requiring validation.
- Multilingual coverage was uneven by source quality (strong EN, partial es/de/fr/pt-BR via hosting/collab context, zh-CN/ja mainly through prior realtime research); conclusions were triangulated against primary Macro/Flora evidence.

## 8. Sources

### Macro / primary public
- [M1] Macro README: https://raw.githubusercontent.com/macro-inc/macro/main/README.md
- [M2] Running locally: https://github.com/macro-inc/macro/blob/main/docs/development/running-locally.md (and repo docs references)
- [M3] AIBrew, “Self-Host Macro…” (2026-08-14): https://www.aibrew.io/blog/macro-workspace-self-host-guide
- [M4] DeepWiki/grok-wiki architecture & service topology snapshots (public mirrors consulted): https://deepwiki.com/macro-inc/macro , https://grok-wiki.com/public/docs/macro-inc-macro-bb988e1a448e/
- [M5] Service/runtime details (search processing, compose/topology excerpts via public docs/mirrors): https://upd.dev/macro-inc/macro/src/branch/main/services/search_processing_service
- [M6] Macro site/product pages: https://macro.com/ , https://macro.com/posts/why-macro-is-open-source
- [M7] GenZTech setup analysis: https://genztech.blog/p/macro-workspace-self-host-setup/
- [M8] grok-wiki install/backend pages (Docker/Nix/LocalStack/OpenSearch/Kafka/FusionAuth): https://grok-wiki.com/public/docs/macro-inc-macro-bb988e1a448e/pages/02-installation.md , https://grok-wiki.com/public/docs/macro-inc-macro-bb988e1a448e/pages/12-run-backend-services-locally.md
- [M9] Kafka/search pipeline behavior (commit-after-handoff, retries): https://upd.dev/macro-inc/macro/src/branch/main/services/search_processing_service
- [M10] OpenSourceAlternatives / RepoCloud / AISignal summaries (self-host/collab notes): https://www.opensourcealternatives.to/item/macro , https://www.repocloud.io/details/Macro/ , https://www.aisignal.dev/repo/macro-inc/macro
- [M11] Macro vs Huly / licensing & cost context: https://www.aibrew.io/blog/macro-workspace-vs-huly

### Flora internal (cross-check)
- [F1] `docs/research/2026-11-16-seed-vps-profile-audit.md`
- [F2] `docs/research/2026-11-16-014-seed-vps-operability-international.md`
- [F3] `docs/research/2026-11-15-macro-alternative-80-20-research.md`
- [F4] `docs/research/2026-11-16-014-realtime-delivery-vps-options.md`
- [F5] `docs/research/2026-11-16-013-websocket-session-authority.md` (referenced by F4/current state)
- [F6] `OBJETIVO.md`, `ROADMAP.md`

### VPS / SMB hosting context (multilingual signals)
- [R1] Kimsufi (es) suite colaborativa autoalojada / requisitos orientativos: https://www.kimsufi.com/es-es/use-cases/uc-collaboration-server/
- [R2] OuiHeberg (fr/en) comparatifs VPS 2 vCPU/4 GB market context (2026): https://www.ouiheberg.com/fr/comparatifs/meilleur-vps-france-2026 , https://www.ouiheberg.com/en/comparisons/best-vps-france-2026
- [R3] Hostinger (es) Mattermost/OpenCloud VPS sizing examples (4 GB tiers): https://www.hostinger.com/es/applications/mattermost , https://www.hostinger.com/es/applications/opencloud
- [R4] BacLoud self-host app VPS sizing table (EN, indicative): https://www.bacloud.com/en/blog/309/10-self-hosted-applications-you-can-run-on-a-vps.html
- [R5] Korben (fr) self-hosting stack sizing/practices (5€/mois class VPS context): https://korben.info/self-hosting-guide-auto-hebergement.html
- [R6] Virtua Cloud (fr) RAM/stack guidance for self-hosted apps: https://www.virtua.cloud/learn/fr/concepts/self-host-app-stacks-vps
- [R7] PostgreSQL docs/wiki (tuning/backup) as cited in F2: https://www.postgresql.org/docs/current/runtime-config-resource.html , https://www.postgresql.org/docs/current/backup-dump.html
- [R8] Docker Compose deploy/healthcheck specs as cited in F2: https://docs.docker.com/reference/compose-file/deploy/ , https://docs.docker.com/reference/compose-file/services/#healthcheck
