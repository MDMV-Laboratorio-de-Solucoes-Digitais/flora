# Intake: 015 — Next vertical slice

**Date:** 2026-11-16  
**Cycle:** 015 (post-014, PR #217 in progress/expected merge per prompt)  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (dev)  
**Scope:** deep 80/20, self-hosted Macro alternative for SMBs on a cheap single-node VPS (~2 vCPU / 4 GB RAM). No application source modified; no commits.

## Request

Perform extensive multilingual internet research (EN, pt-BR, es, de, fr, zh-CN, ja, ko, ru as accessible) on highly reliable sources for:

1. **A — Minimal reliable realtime delivery for persisted chat** on single-node 2 vCPU/4 GB VPS (Postgres LISTEN/NOTIFY, SSE, polling, in-process WS fanout) and deterministic catch-up.
2. **B — Simple self-hosted binary file upload/download/storage/quotas** in Rust + Postgres + local disk (safe paths, streaming, limits).
3. **C — Seed operability quick wins** (optional Meilisearch/Valkey, compose resource limits/healthchecks).

Deliver `.specify/assessments/015-next-vertical-slice/{intake,research,problem,concept,decision}.md` with a clear **go/scope verdict** for cycle 015 choosing **ONE** primary 80/20 slice (A, B, or C), evidence, risks/NVs, debloat notes, file-by-file implementation checklist, deterministic verification plan, and explicit non-goals.

## Evidence reviewed (in-repo)

- Direction: `OBJETIVO.md`, `ROADMAP.md` (P0: realtime mínimo confiável, arquivos básicos reais, Seed/VPS operabilidade; 014 em andamento para WS session authority/protocol).
- Prior cycle: `.specify/assessments/014-next-vertical-slice/decision.md` — **go (scoped) slice A** (WS session authority + protocol alignment); explicitly deferred fanout/registry/sync, `/messages/sync`/catch-up protocol, file upload/download, and broad Seed/VPS refactors.
- Research: `docs/research/2026-11-16-014-realtime-delivery-vps-options.md`, `docs/research/2026-11-16-014-ws-codebase-audit.md`, `docs/research/2026-11-16-014-websocket-auth-protocol-international.md`, `docs/research/2026-11-16-014-seed-vps-operability-international.md` (plus 013 WS authority and Seed VPS audit).
- Code paths: `flora-api/src/routes/{channels.rs,websocket.rs,files.rs,mod.rs}`, `flora-api/src/state/app_state.rs` (implied), `flora-core/src/services/file_service.rs`, `flora-messaging/src/lib.rs`, `flora-core/src/config/{app,storage,messaging,search,database}.rs` (via prior docs), `docker-compose.yml`, `apps/frontend/src/lib/api/websocket.ts` and chat/mappers stores (via audits).

## Current state snapshot (key facts)

| Area | State |
|---|---|
| **Persisted chat (REST)** | `GET/POST /api/v1/channels?workspace_id`, `GET/POST/PATCH/DELETE /api/v1/channels/{channel_id}/messages` with org/workspace/channel checks; `MessageResponse` includes `id/channel_id/sender_id/content/thread_id/is_edited/created_at/updated_at`. List takes `limit` and `before` params but implementation uses offset pagination (`Pagination{limit,offset:0}`) and filters deleted in-handler; no deterministic `after/cursor` catch-up, no `/api/v1/messages/sync`. Mutations do **not** publish realtime events. |
| **Realtime (WS)** | `GET /ws` with DB-backed handshake auth helpers (014 target), tagged envelopes `Subscribe/Unsubscribe/SendMessage/Ping` and `Subscribed/Unsubscribed/NewMessage/MessageSent/Pong/Error`. `SendMessage` validates channel org, persists message, best-effort publishes to Valkey if configured, but **no subscription registry, no server-side fanout/broadcast to other clients, `Subscribe` ack-only, no consumer bridge**. `flora-messaging` is publish-only primitive. |
| **Frontend** | `WsClient` previously targeted `/api/v1/ws` with query token and divergent event shapes / missing `/messages/sync`; chat uses REST for load/send (audit). 014 planned protocol/path alignment; delivery/catch-up correctness remains unresolved. |
| **Files** | `POST /api/v1/files` records metadata only (name/type/size/workspace) via `FileService::create_file` (quota/MIME/path-traversal checks, storage_path `orgs/{org}/files/{id}`) — **no actual binary upload/streaming, no storage write, no download/read endpoint, no content endpoint**. `GET/DELETE` operate on metadata/soft-delete. |
| **Seed/VPS operability** | Compose: PG18, Valkey, Meilisearch with localhost binds and healthchecks but **no resource limits/profiles**; `AppState`/search boot coupling makes Meilisearch effectively required despite optional intent; DB pool/config/init duplication concerns (per 014 audit); Valkey degrades gracefully. |

## Constraints & assumptions

- **No app source changes / no commits** in this assessment task.
- Target deployment: cheap single-node VPS (2 vCPU/4 GB), low concurrency, Postgres-first, minimal mandatory services.
- 014 implementation/merge status is external; recommendations must remain valid whether 014 is merged or in-flight (build on its intended WS-auth/protocol foundation).
- Multilingual research: accessible high-signal sources retrieved in EN and selected es/de/fr/zh-CN/ja materials; pt-BR/ko/ru coverage partial via corroborating sources and language-targeted queries (see `research.md` NVs). No live runtime measurements.
