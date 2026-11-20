# Research: 015 — Next vertical slice

**Date:** 2026-11-16  
**Method:** desk research (web, multilingual queries EN/pt-BR/es/de/fr/zh-CN/ja/ko/ru as accessible) + cross-check against in-repo 014 research/docs and current code paths. Sources prioritized: official docs (PostgreSQL, RFC/MDN/Axum, Docker, Meilisearch/Valkey) and corroborated engineering practice. No runtime benchmarks.

## A. Minimal reliable realtime delivery & deterministic catch-up (single-node VPS)

### Findings

1. **Source of truth vs delivery plane (consensus).** Persisted chat must treat Postgres as the commit/source-of-truth and any push channel (WS/SSE/NOTIFY/pubsub) as best-effort signaling; reconnect/recovery requires deterministic DB read. (014 options doc; PostgreSQL NOTIFY semantics: no backlog/persistence, 8 KB payload cap, listeners must be connected at publish time; `pg_notify`/queue bounded; dedicated LISTEN connection, not compatible with transaction-pooling patterns. EN: PostgreSQL docs, JusDB/Recall/Pedro Alonso analyses; es/de/zh-CN/ja engineering posts corroborate.)
2. **Postgres LISTEN/NOTIFY fit.** Useful as **low-rate invalidation/wakeup** on single/small multi-instance setups without extra broker, but **not durable delivery** and sensitive to missed notifications on reconnect/listener gaps and payload/queue limits. Practical pattern: `NOTIFY` with IDs only + client/app catch-up query from DB; fallback polling to heal missed signals. (EN: DBOS/Recall/JusDB; de: Rebase/Ugur Aslim coordination use; zh-CN: lightweight notification patterns.)
3. **SSE vs WS vs polling.** For server→client notifications SSE gives browser auto-reconnect and `Last-Event-ID` resume hints, simple HTTP/proxy semantics, and fits "notify then fetch" for chat updates when sends stay on REST; WS is better for bidirectional low-latency but requires manual reconnect/resume, heartbeats, backpressure, and subscription/fanout state. Long-polling/polling simplest for correctness/fallback but adds request overhead/latency. (EN/es: SSE vs WS comparisons, MDN/RFC guidance; zh-CN: Axum SSE/WS ecosystem maturity; ja: Axum WS chat emphasizes connection/broadcast/recovery separation.)
4. **Single-node in-process fanout (80/20).** `tokio::sync::broadcast` / per-channel registries in one process provide zero-extra-infra fanout for connected sockets; memory/CPU bounded by connection count and buffer/backpressure. Sufficient for Seed if limits enforced; does not solve reconnect loss without catch-up. (Axum/Tokio patterns; 014 audit shows publish path but missing registry/fanout.)
5. **Deterministic catch-up/cursor pagination.** Offset pagination is unstable under concurrent inserts/deletes and poor for "everything after cursor". Stable cursor should be monotonic/unique-ordered (e.g. `(created_at, id)` or UUIDv7 `id` when time-ordered) with indexed `WHERE (created_at,id) > (...) ORDER BY (created_at,id) ASC` (or UUIDv7 id ordering) and idempotent client merge by `message_id`. API should accept `after` (and/or `before`) cursor/token; server returns items + next cursor/high-water mark. UUIDv7 (native PG18 in Flora) is favorable for append-ordered IDs but tie-breaker/semantics must be explicit. (EN: cursor/keyset pagination guides, Prismatic/Stripe-like patterns conceptually; Supabase/Realtime discussions note UUIDv7 ordering utility; Milan Jovanovic/cursor-under-the-hood.)
6. **Axum SSE specifics.** `axum::response::sse::{Sse, Event, KeepAlive}` supports `id`/`event`/`retry`/data and keep-alive comments; `Last-Event-ID` can be read from request headers on reconnect for replay-from-cursor if server retains window or replays via DB query. (Axum docs/examples, irurueta/docs.rs, tokio-rs/axum.)

### Flora cross-check (A)

- `list_messages` ignores `params.before` semantics for cursoring and queries with `offset: 0` then in-memory filters deleted — **not deterministic catch-up**; deleted/edited visibility and ordering need a defined contract.
- No sync endpoint; frontend expects `/api/v1/messages/sync` (014 audit).
- REST create/edit/delete in `channels.rs` do not publish events; WS send persists and may publish to Valkey but no in-process subscriber/fanout to WS clients; `Subscribe` ack-only.
- 014 (planned) establishes DB-backed WS auth/protocol alignment — prerequisite largely addressed; delivery/catch-up remains the gap.

## B. Binary file upload/download/storage/quotas (Rust + Postgres + local disk)

### Findings

1. **Axum multipart/streaming.** `axum::extract::Multipart` (`multipart` feature) parses `multipart/form-data` field-by-field; can read chunks/bytes/streams per field to avoid buffering whole file in memory. Default body limits apply (historically small defaults in Axum extractors; configurable via `DefaultBodyLimit`/`RequestBodyLimitLayer`/`ContentLengthLimit`). Large uploads need explicit limits, timeouts, and streaming to disk/object store. (Axum docs.rs/source, tokio-rs examples/discussions, imfeld.dev, users.rust-lang.org.)
2. **Safe local storage.** Never trust client filename/path: sanitize to basename, reject `..`, `/`, `\`, NUL/control chars, normalize extension/MIME policy, generate server-side storage keys (e.g. `orgs/{org}/files/{file_id}` as Flora already models) and resolve within a fixed root using canonicalization/`strip_prefix` checks to prevent traversal/symlink escape. Set restrictive permissions (`0o600/0o700` where applicable). (OWASP file upload/storage guidance; repo `FileService::validate_filename` partial.)
3. **Quotas/limits.** Enforce max upload bytes (request/content-length and streaming-enforced), per-file/per-org/workspace quotas atomically enough for best-effort (DB-tracked usage + check before/around commit), and MIME allow/block lists. Streaming hash/size verification prevents declared-vs-actual mismatches.
4. **Download.** Serve via streaming body/file with `Content-Type`, `Content-Disposition` (attachment; safe filename), `Content-Length` when known, range support optional; always re-check org/ownership/soft-delete before read.
5. **VPS constraints.** Disk I/O and buffering dominate; prefer chunked streaming + temp-file-then-rename within same filesystem, bounded concurrency, and cleanup on failure. (Rust/Axum upload guides, EN; pt-BR query surfaced general Axum multipart/upload materials but fewer deep primary sources — recommendations grounded in Axum/OWASP.)

### Flora cross-check (B)

- `POST /api/v1/files` accepts JSON metadata (`size_bytes` client-declared) and creates DB record only; no bytes received, no disk write/read, no download endpoint, storage path generated but unused. `file_service` enforces filename/MIME basics and quota comparisons against declared size; actual content, checksum, and storage lifecycle missing.
- Config has `max_upload_bytes`/`allowed_file_types` (app) and storage quota/path fields (per prior audit) — partially wired.

## C. Seed operability quick wins (optional services, compose limits/healthchecks)

### Findings

1. **Docker Compose limits/profiles.** Compose `deploy.resources`/service-level constraints (or runtime equivalents) and profiles/optional dependencies reduce memory contention on 4 GB VPS. Keep stateful services bound to localhost/private net; use healthchecks and `depends_on` with `condition: service_healthy` for required deps; treat optional deps as degradable. (Docker docs/virtua/last9/SSDNodes/Econumo; corroborated EN sources.)
2. **PostgreSQL small-RAM tuning.** Conservative starting points for shared 4 GB (API+OS+optional services): `shared_buffers` ~256–512 MB, `work_mem` 4–8 MB (multiplies by sorts/connections), `maintenance_work_mem` 64–128 MB, `effective_cache_size` hint 2–3 GB, `max_connections` modest (20–40) with disciplined pooling. Avoid overcommit; validate. (PostgreSQL docs/wiki.)
3. **Optional services.** Valkey can be optional/degraded (cache/pubsub) with `maxmemory`/eviction when enabled; Meilisearch memory/indexing can be constrained at launch but is a significant RAM consumer — disabling when not needed is high leverage if boot decoupled. (Valkey memory docs; Meilisearch self-host config.)
4. **Backups/restore.** `pg_dump`/`pg_restore` (+ off-host copies, periodic restore drills) for critical DB; local storage volume in backup scope; Valkey/Meilisearch data often rebuildable depending on role. (PostgreSQL backup docs.)

### Flora cross-check (C)

- `docker-compose.yml`: PG18/Valkey/Meilisearch all present with healthchecks and localhost binds, **no mem/CPU limits, no profiles**.
- 014 audit: search init hard-fails at boot despite optional intent; duplicate pool/migrate/init and hardcoded pool sizing concerns; env contract drift.
- Highest-value quick wins (code+compose) are clear but require touching runtime wiring/config and compose — medium churn across boot path.

## Synthesis (80/20 for 015)

| Candidate | User value for persisted chat MVP | VPS/complexity | Dependency on 014 | Risk | 80/20 |
|---|---:|---:|---:|---:|---:|
| **A. Realtime delivery + deterministic catch-up** | **High** (makes persisted messages reliably visible/live) | Low–medium (no new mandatory infra; in-process + REST cursor) | Builds on WS auth/protocol foundation | Medium (contract/fanout correctness) | **Best** |
| **B. Binary file upload/download** | Medium–high (P0 files) but currently metadata-only; broader surface (multipart, streaming IO, security) | Medium (disk/limits/attack surface) | Independent | Medium–high (security/IO) | Good, but larger |
| **C. Seed/VPS operability hardening** | Medium (operability/cost) | Medium (boot/config/compose refactor) | Independent | Medium (boot regressions) | Good, less direct product unlock |

**Recommendation:** choose **A** for 015, scoped to single-node, Postgres-authoritative **cursor catch-up + minimal in-process realtime fanout/notification** compatible with existing WS, with no mandatory Valkey/NOTIFY dependency and explicit degraded behavior.

## Source index (selected, representative)

- PostgreSQL: `LISTEN`/`NOTIFY`, resource config, backup/dump docs — https://www.postgresql.org/docs/current/
- Axum: WebSocket/SSE extractors & examples, multipart/body limits — https://docs.rs/axum/latest/axum/ , https://github.com/tokio-rs/axum
- RFC 6455 (WebSocket protocol/close semantics): https://datatracker.ietf.org/doc/html/rfc6455 ; MDN WebSocket/SSE docs (multilingual pages reviewed in 014)
- Docker: Compose `deploy.resources`, healthchecks — https://docs.docker.com/reference/compose-file/
- Cursor/keyset pagination practice: https://milanjovanovic.tech/blog/understanding-cursor-pagination-and-why-its-so-fast-deep-dive , https://prismatic.io/blog/how-do-i-implement-cursor-based-pagination/ , https://www.rafeuddaraj.me/blog/cursor-pagination-under-the-hood
- LISTEN/NOTIFY tradeoffs & SSE/WS patterns: https://dbos.dev/blog/postgres-listen-notify-scalability , https://spin.atomicobject.com/postgres-listen-notify-events , https://www.jusdb.com/blog/postgresql-listen-notify-realtime-events
- Streaming uploads in Axum: https://imfeld.dev/writing/process_streaming_uploads_with_axum , https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
- Prior in-repo: `docs/research/2026-11-16-014-realtime-delivery-vps-options.md`, `...014-ws-codebase-audit.md`, `...014-websocket-auth-protocol-international.md`, `...014-seed-vps-operability-international.md`, `...seed-vps-profile-audit.md`

## NVs (research)

- NV-R1: **ko/ru/pt-BR** primary high-quality, VPS-specific sources were thinner in accessible results; conclusions triangulated from EN official docs and corroborating es/de/fr/zh-CN/ja materials. No conflicting guidance identified.
- NV-R2: No load tests or memory measurements for 2 vCPU/4 GB; connection/fanout/buffer limits must be validated.
- NV-R3: 014 code merge state not verified at runtime in this task; implementation checklist assumes 014's intended WS auth/verifier/path alignment as base (call out if absent).
- NV-R4: Exact `MessageRepository` query/index support for cursor-by-id/created_at to be confirmed during implementation.
