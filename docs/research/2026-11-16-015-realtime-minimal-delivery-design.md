# 015 — Minimal, reliable realtime delivery for persisted chat (Flora)

**Date:** 2026-11-16  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (read-only research; no application source modified)  
**Scope:** Deep multilingual desk research (EN, pt-BR, es, de, fr, zh-CN, ja) on minimal, reliable realtime updates for persisted chat in **Rust (Axum) + Postgres + Svelte** on a **single cheap VPS (≈2 vCPU / 4 GB)**. Cross-checked with Flora's current implementation and prior research: `docs/research/2026-11-16-014-realtime-delivery-vps-options.md`, `docs/research/2026-11-16-014-ws-codebase-audit.md`.

## 1. Executive summary (80/20)

- **Source of truth:** Postgres commit is the only durable record. Push transports (WS/SSE/Valkey/NOTIFY) are best-effort notification planes and must never be the sole copy.
- **Reliability target for Seed:** *no silent loss after DB commit* + *eventual UI convergence after reconnect/reload* via deterministic catch-up (cursor/ID-based REST), with **idempotent dedupe** in the client/store. Exactly-once push is unnecessary and expensive.
- **Recommended for Flora 015 (single VPS):** keep REST as send/mutation and history authority; add **in-process fanout** (Tokio broadcast/per-channel registry) for connected clients, emit canonical events **after DB commit** from REST/WS mutation paths, and implement a **real cursor catch-up contract** (`after` by stable sequence/cursor). Defer Valkey fanout and Postgres `LISTEN/NOTIFY` as scale/debloat adapters behind a small internal event interface.
- **Transport choice:** **WebSocket is acceptable if finished minimally** (Flora already has `/ws` and `/api/v1/ws`, auth, tagged protocol, frontend client hooks) — but only with real subscription registry/fanout, aligned envelopes, and catch-up. **SSE+REST** is often simpler for one-way notifications and has browser reconnect/`Last-Event-ID` advantages, but would be a larger pivot given current WS investment; pick one coherent path for 015 rather than keeping half-states.
- **Defer now:** Redis/Valkey pub/sub backplane, Postgres `LISTEN/NOTIFY` as delivery, durable streams/outbox brokers, multi-instance guarantees. Keep contracts bus-agnostic so these remain future options.

## 2. Cross-check: Flora current implementation

| Area | Evidence (current) | Status / gap for 015 |
|---|---|---|
| WS routes & upgrade | `flora-api/src/routes/mod.rs`: registers `GET /ws` and `GET /api/v1/ws` to `websocket::websocket_handler`; CORS + rate-limit layers applied globally. | Path availability improved (both legacy/new). |
| WS auth | `flora-api/src/routes/websocket.rs`: `websocket_handler` resolves token from `Authorization: Bearer` or `?token` (`resolve_ws_token`), calls `crate::extractors::auth::verify_bearer_token` (DB-backed session path shared with HTTP extractor), rejects nil org, then upgrades. Unit tests cover token resolution/serialization. | **Handshake auth is materially stronger** than 014 audit (DB-backed verification). Still verify parity details (org scoping semantics, error/close behavior) match HTTP policy; query-token remains supported for browser constraints. |
| WS protocol & behavior | Client msgs: `Subscribe`, `Unsubscribe`, `SendMessage{channel_id,content,thread_id}`, `Ping`. Server msgs: `Subscribed`, `Unsubscribed`, `NewMessage{channel_id,message_id,sender_id,content,thread_id,created_at}`, `MessageSent{message_id,channel_id}`, `Pong`, `Error{code,message}`. `handle_subscribe/unsubscribe` only ack/log (no registry). `handle_send_message` checks non-empty content, validates channel ownership in-org (`PgChannelRepository::find_by_id`), persists (`PgMessageRepository::create`), best-effort publishes to Valkey (`state.messaging_service.publish` with `flora_messaging::Message` lacking message id/timestamps), acks sender only. No receive loop for inbound fanout. | **No real per-connection subscription/fanout/delivery to other clients.** Event envelope omits canonical DB fields (`is_edited`, `updated_at`, etc.) and diverges from REST/frontend expectations. |
| Messaging bus (Valkey) | `flora-messaging/src/lib.rs`: `MessagingService::new(ConnectionManager)`, `publish` does `PUBLISH org:{org_id}:channel:{channel_id}` of serialized `flora_messaging::Message {channel_id,organization_id,sender_id,content,thread_id}` with latency logging/warn targets (500 ms). No subscribe/listen/bridge API. `flora-core/src/config/messaging.rs`/`.env.example` expose valkey/topic/fanout/TTL/circuit-breaker knobs. | **Outbound publish primitive only; no working delivery plane.** Published payload not sufficient as canonical event without IDs/cursor. |
| REST channels/messages (source of truth path) | `flora-api/src/routes/channels.rs`: `GET/POST /api/v1/channels?workspace_id=...`, `GET/POST /api/v1/channels/{channel_id}/messages`, `PATCH/DELETE /api/v1/channels/{channel_id}/messages/{msg_id}`. Org/membership/workspace/channel checks enforced; edits/deletes limited to sender in same org/channel. `list_messages` takes `limit` (default 50, capped 100) and `before` param but calls `msg_repo.find_by_channel_id(..., Pagination{limit, offset:0})`, filters deleted, returns `Vec<MessageResponse>` ordered per repo query. `post/edit/delete` do **not** publish realtime events. `MessageResponse`: `{id,channel_id,sender_id,content,thread_id,is_edited,created_at,updated_at}`. | **DB-backed CRUD exists, but no push emission and no cursor-correct catch-up contract yet** (`before` accepted but not applied as shown). |
| Persistence model | `flora-core/src/models/message.rs`: `Message` has UUIDv7 `id`, `channel_id`, `organization_id`, `sender_id`, `content`, `thread_id`, `is_edited`, `is_deleted`, `created_at`, `updated_at`. `PgMessageRepository::find_by_channel_id` selects non-deleted `ORDER BY created_at DESC LIMIT $2` (no offset cursor logic). `create/update/soft_delete` operate by id. | UUIDv7 + timestamps are good building blocks for monotonic-ish cursors, but **repository/query API needs explicit after/before cursor semantics and stable ordering for sync**. |
| Frontend realtime client | `apps/frontend/src/lib/api/websocket.ts`: singleton `WsClient`, connects to `/api/v1/ws?token=...`, reconnect w/ exp backoff+jitter (max 8), auth-failure detection (close codes/reasons/envelope), `sendSubscribe/sendUnsubscribe`, `onEnvelope` updates `lastKnownMessageId` from `payload.id` or `payload.message.id`, handles `NewMessage` by calling `messageStore.addMessage(payload.message as never)` if `message.id` and `message.channel_id` present, ignores `Subscribed/Unsubscribed/Pong/MessageSent`, `syncMissedMessages()` is explicit no-op (no backend sync route), has back-compat `onMessage`. | **Contract drift with backend (`NewMessage` shape), no catch-up, no channel-aware subscription lifecycle, store merge is append-only dedupe.** |
| Frontend state/mappers/UI | `apps/frontend/src/lib/state/MessageStore.svelte.ts`: in-memory array, `setMessages`, `clear`, `addMessage` dedupes by `id` only (append). `apps/frontend/src/lib/api/mappers.ts`: maps REST `ApiMessage` to UI `Message {id,channelId,authorId,content,threadId,fileAttachments,createdAt,updatedAt}` (drops `is_edited`). `ChatLayout.svelte` loads via REST on channel change, sends via REST and appends result, renders top-level + thread replies from same store; no WS subscribe/unsubscribe or realtime merge integration. Root layout does not wire `wsClient` (unlike 014 audit note). | **Chat is effectively REST-driven today; realtime path not end-to-end.** Any 015 plan must define echo/dedupe, ordering, and edit/delete propagation semantics. |

**Net:** Flora has most primitives (Postgres persistence, UUIDv7 IDs, WS handshake/auth, partial protocol, REST CRUD, frontend client/store shells, Valkey publish stub) but **not a complete reliable loop**: missing canonical event emission + in-process fanout/subscription registry + cursor catch-up + frontend contract/alignment.

## 3. Design options (multilingual evidence, source-backed)

### 3.1 Axum + in-process WebSocket fanout (single process)

**Pattern:** after DB commit, publish event into process-local hub (`tokio::sync::broadcast` or per-channel registry of `mpsc` senders) and forward to sockets subscribed to that channel/org. Axum's canonical chat example demonstrates `broadcast::channel` state + per-socket sender/receiver tasks (EN: tokio-rs/axum chat example). Practical guides emphasize per-connection task split, backpressure/bounded buffers, and heartbeat/ping handling (EN: Axum WS docs/examples; EN/pt-BR dev guides; zh-CN Axum realtime/broadcast tutorials).

**Pros:** zero extra infra, lowest latency, simple on 2 vCPU/4 GB, easy to reason about for Seed. **Cons:** single-node/process only; connection state memory is fixed per idle socket; no replay — reconnect must catch up from DB. Bounded broadcast capacity + lag handling (`Lagged`) must be explicit to avoid silent drops under slow consumers.

**Fit for Flora 015:** **high** as the minimal push plane if WS is retained.

### 3.2 SSE notifications + REST mutations/catch-up

**Pattern:** client sends writes via REST (`POST/PATCH/DELETE`), receives server→client events over `text/event-stream`; browser `EventSource` auto-reconnects and can send `Last-Event-ID` for resume if server supports replay/cursor semantics (EN: MDN/EventSource behavior widely documented; Encore SSE-vs-WS article; es: We.Inc "SSE vs WebSocket"; fr comparisons emphasize one-way HTTP simplicity; zh-CN Axum SSE/broadcast practices).

**Pros:** simpler than bidirectional WS, HTTP/proxy-friendly, native reconnect story, fits "notify then fetch/catch-up" for persisted data. **Cons:** one-way only, `EventSource` cannot set custom auth headers (cookies/query params), HTTP/1.1 per-origin connection limits (HTTP/2 mitigates), replay requires server-side event id window or DB-cursor fallback.

**Fit for Flora 015:** **high conceptually** for correctness/operability; **medium adoption cost now** because Flora already has WS protocol/client/server scaffolding and needs bidirectional-ish send/subscribe semantics decided explicitly.

### 3.3 REST polling / long-poll catch-up (baseline/fallback)

**Pattern:** client periodically or long-polls `GET messages?after=<cursor>` from Postgres. No persistent push state. (EN comparisons: polling simplest but higher request/latency overhead for active chat; pt-BR/EN engineering practice: viable at low frequency, pair with push for UX.)

**Pros:** stateless, trivial reconnect, easy horizontal scaling, strongest debloat baseline. **Cons:** latency/battery/chattiness tradeoff; not "live" UX alone.

**Fit for Flora 015:** **mandatory as recovery/correctness path**, not sufficient alone for realtime feel.

### 3.4 Cursor/sequence, dedupe, idempotency, reconnect/resync semantics

- **Stable cursor:** prefer monotonic, totally-orderable cursor per channel for sync, e.g. `(created_at, id)` keyset or a channel-scoped sequence; UUIDv7 is time-sortable and useful but DB ordering must be defined canonically (and unique tie-breaker by `id`). REST sync should accept `after=<cursor>` and return events/messages in deterministic ascending order for application (EN: cursor/keyset pagination guidance in Axum/Rust and realtime chat recovery patterns; design principle echoed across multilingual sources).
- **Idempotent upsert:** clients must merge by immutable `message_id` (Flora store already dedupes by `id` but only appends) and handle duplicates/reordering from reconnect, replay, or send-ack races.
- **At-least-once to UI, not wire:** tolerate duplicate deliveries; never assume push is durable. On connect/reconnect/visibility regain/gap detection, fetch missed rows from Postgres after last applied cursor.
- **Mutation events:** define canonical event kinds (`message.created|updated|deleted`) carrying `id`, `channel_id`, `org_id`, cursor/seq, and changed fields/timestamps so edits/deletes converge too (currently WS only models create-ish flow).

### 3.5 Postgres `LISTEN/NOTIFY` (defer for 015)

**Evidence:** notifications are transactional on commit, async, **not durable**, **at-most-once** to active listeners, payload hard limit ~**8000 bytes** (send IDs/hints, fetch full data), require dedicated long-lived listener connections, and are incompatible with transaction-pooling setups like PgBouncer transaction mode; reconnect gaps need app-level recovery (EN: PostgreSQL docs semantics summarized; JusDB/pg-pubsub/MVP Factory/Neon/practical blogs; de: Rebase/Ugur Aslim note lightweight multi-instance coordination but no persistence/backlog and dedicated connections). Good for low-rate invalidation on small setups, poor fit as durable/high-fanout/high-frequency delivery.

**Fit for Flora 015:** **defer**. Useful later as Valkey-free invalidation signal (IDs only) behind same catch-up contract.

### 3.6 Valkey/Redis pub/sub (defer for 015)

**Evidence:** standard multi-instance fanout backplane (publish DB-committed event, every instance forwards to local sockets), but pub/sub is ephemeral/at-most-once with no backlog for disconnected subscribers; Redis Streams/NATS JetStream add replay/consumer offsets at higher complexity (EN: realtime system design guides, Redis vs Streams discussions; zh-CN Axum+Redis bridge examples; 014 notes). Valkey remains protocol-compatible for core pub/sub in many cases but is extra operational surface on 4 GB VPS.

**Fit for Flora 015:** **defer** (Flora has config/publish stub, not required for single-node correctness). Keep adapter seam in `flora-messaging` or app event publisher.

## 4. Concrete recommended plan for Flora 015

### 4.1 Principles (non-negotiable)

1. **DB commit = success boundary.** A message/mutation acknowledged to clients must be persisted in Postgres first (or atomically committed before fanout event is treated authoritative).
2. **One canonical event envelope.** Use REST/WS/frontend-shared shape keyed by persisted message row: at minimum `{event_id/cursor, type, org_id, channel_id, message:{id,channel_id,sender_id,content,thread_id,is_edited,is_deleted,created_at,updated_at}}`.
3. **Catch-up is mandatory.** Implement working sync: e.g. `GET /api/v1/channels/{channel_id}/messages?after=<cursor>&limit=...` (or dedicated sync endpoint) with deterministic keyset order, indexed query, and documented cursor encoding.
4. **Push is advisory acceleration.** On (re)connect, auth restore, subscribe, visibility change, or suspected gap, run catch-up before/alongside live stream consumption.
5. **Single-node first.** No required Valkey/extra daemon for baseline correctness in 015.

### 4.2 Minimal implementation slices (recommended order)

**Slice A — Correctness contract & catch-up (highest ROI)**
- Define cursor strategy for channel timeline: recommended **keyset on `(created_at, id)`** (or UUIDv7-aware equivalent) with strict ascending order for sync and a stable "last seen" token stored by frontend per channel.
- Extend message query/repository/API to support `after` (and keep/implement `before` properly) without ambiguous offset recovery; add supporting DB index if missing for channel timeline scans.
- Specify response semantics: include deleted/edited state transitions as needed for convergence, or document tombstone behavior.

**Slice B — Canonical event emission on all mutations**
- Publish/fanout events from REST `post_message`, `edit_message`, `delete_message` (and WS send path if retained) **after successful DB write**, using the same envelope builder.
- Include sender identity/org/channel/message id and cursor so clients can dedupe/idempotently apply.

**Slice C — In-process realtime hub (single VPS)**
- Add process-local subscription registry in `AppState`/dedicated module: track connections by authenticated `(user_id, org_id)` and subscribed `channel_id`s with bounded queues and cleanup on disconnect.
- On emitted event, deliver only to sockets authorized for `(org_id, channel_id)`; never trust client-submitted org.
- Keep `flora-messaging` Valkey publish optional/off (adapter) until multi-instance needed; do not depend on it for delivery in 015.

**Slice D — Finish one coherent client path (Svelte)**
- Align `WsClient` envelope parsing with backend canonical `NewMessage`/mutation events and map payloads through `mappers.ts` (preserve `is_edited`, handle update/delete/tombstone).
- Make `MessageStore` idempotent and order-stable (upsert by id, merge edits/deletes, sort/insert by cursor policy) instead of append-only-on-new.
- Implement subscription lifecycle per active channel in chat view, reconnect flow: connect -> (re)subscribe active channels -> run REST catch-up from last cursor -> resume live events.
- Keep sends/mutations over REST if simpler/reliable, or reconcile WS `MessageSent` echo with REST response to avoid duplicates (dedupe by id covers this if consistent).
- Replace `syncMissedMessages()` no-op with real catch-up call(s); stop relying on nonexistent legacy sync contract.

**Slice E — Operability & safety**
- Connection/message limits, heartbeat/ping interval and idle timeout compatible with reverse proxies, bounded per-socket outbound buffers, slow-consumer disconnect policy.
- Observability: connect/disconnect counts, auth failures, subscription counts, fanout lag/drops, catch-up volume.
- Preserve HTTP-session authority semantics and org/channel authorization on subscribe and emit.

### 4.3 Suggested acceptance criteria for Flora 015

- A message persisted by REST/WS is visible to another online client in-org/channel without refresh when realtime path is healthy, and **always** after reconnect/reload via catch-up.
- Duplicate deliveries/reconnects do not create duplicate UI rows; edited/deleted states converge.
- No cross-org/cross-channel leakage in subscriptions or fanout.
- Revoked/expired sessions cannot maintain or establish realtime delivery (fail closed, bounded reconnect behavior).
- Baseline runs on single VPS with **Postgres + app only** (Valkey optional/not required).
- Frontend/backend contracts (path, auth transport, envelope, cursor fields) are consistent and tested.

### 4.4 Explicit deferrals (YAGNI for 015)

- **Valkey/Redis pub/sub or Streams backplane:** only when multi-instance/horizontal fanout is required.
- **Postgres `LISTEN/NOTIFY` delivery plane:** only if intentionally replacing Valkey as low-rate invalidation signal with strict 8 KB/at-most-once caveats and dedicated listener connection.
- **Outbox + durable log (Kafka/NATS JetStream):** only for stronger replay/SLO demands beyond Seed.
- **Typing/presence/read receipts:** separate protocol/product scope once message delivery is reliable.

## 5. Multilingual research notes (EN, pt-BR, es, de, fr, zh-CN, ja)

| Lang | Themes corroborated (selected) | Representative sources consulted |
|---|---|---|
| **EN** | Axum WS patterns (`broadcast`, split sender/receiver tasks), SSE reconnect/`Last-Event-ID`, WS/SSE/poll tradeoffs, Postgres LISTEN/NOTIFY limits (8 KB, non-durable, dedicated conn), Redis pub/sub lossy vs Streams replay, cursor catch-up/idempotent merge | tokio-rs/axum chat/WS examples & docs; Encore "SSE vs WebSockets"; JusDB LISTEN/NOTIFY guide; Ably/WebSocket.org/roadmap comparisons; realtime system design notes |
| **pt-BR** | Em VPS pequena, polling simples serve baixa frequência mas chat pede push + recuperação por cursor; persistir no banco e tratar push como otimização (triangulated with Flora 014 synthesis and EN practice) | 014 synthesis (pt-BR ecosystem note) + EN/implementation patterns cross-check |
| **es** | SSE unidireccional HTTP con reconexión automática/EventSource encaja para notificaciones/feeds; WebSocket para bidireccionalidad real; costes de heartbeats/proxies/reconexión manual en WS | We.Inc, "SSE vs WebSocket: elige el stream adecuado" (https://we.inc/es/guides/sse-vs-websocket) |
| **de** | `LISTEN/NOTIFY` als leichtgewichtiges Pub/Sub ohne Zusatzinfra in kleinen Setups, aber **nicht persistent** (kein Backlog), dedizierte Listener-Verbindungen, 8 KB-Payload, Recovery applikationsseitig nötig; Redis für höhere Last/dauerhafte Queues | Rebase Docs (de) Echtzeit/`LISTEN/NOTIFY`; Ugur Aslim (de) multi-tenant realtime events (as referenced in 014) |
| **fr** | Piles temps réel auto-hébergées privilégient simplicité (DB + WS/SSE) avec auth/sécurité/reconnexion explicites; passage multi-instance impose backplane/pubsub et gestion d’état | ForgeChat/VPSConnect (fr README ecosystem signal in 014), Encore comparison framing applicable to HTTP stream semantics |
| **zh-CN** | Axum 生态中 SSE/WS 成熟；SSE 适合单向事件流与 keep-alive/广播，集群可桥接 Redis Pub/Sub；WS 适合双向但连接/心跳/扩展更复杂，小规模常见“单机 Hub + 可选 Redis 桥接” | 祺洛/掘金 Axum 实时广播实践 (referenced in 014); axum-realtime-kit ecosystem patterns |
| **ja** | Axum+WebSocket チャットでは接続管理・再接続・認証・ブロードキャスト境界が課題；単一ノードは in-process broadcast で成立しやすいが保証/複数台には Pub/Sub・永続化・再取得設計が必要、履歴は DB 分離が一般的 | nukopy/engawa 等 Rust+Axum+WS 実装知見 (referenced in 014) |

> Evidence quality note: public benchmarks and VPS-specific multilingual measurements are uneven; recommendations prioritize official docs and repeatedly corroborated engineering guidance (Postgres semantics, browser SSE behavior, Tokio/Axum patterns) over micro-benchmarks. Revalidate against Flora workload (concurrent sockets, channels, msg/s, proxy timeouts) before SLO claims.

## 6. References

### Local
- `docs/research/2026-11-16-014-realtime-delivery-vps-options.md`
- `docs/research/2026-11-16-014-ws-codebase-audit.md`
- `flora-api/src/routes/websocket.rs`, `flora-api/src/routes/channels.rs`, `flora-api/src/routes/mod.rs`
- `flora-messaging/src/lib.rs`, `flora-core/src/config/messaging.rs`, `.env.example`
- `flora-core/src/models/message.rs`, `flora-core/src/repositories/message_repository.rs`
- `apps/frontend/src/lib/api/websocket.ts`, `apps/frontend/src/lib/api/mappers.ts`, `apps/frontend/src/lib/state/MessageStore.svelte.ts`, `apps/frontend/src/lib/components/chat/ChatLayout.svelte`

### External (selected)
1. tokio-rs/axum chat example (`broadcast` fanout, per-socket tasks): https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs
2. Axum WebSocket docs/extract: https://docs.rs/axum/latest/axum/extract/ws/
3. Encore, "SSE vs WebSockets: Which One Should Your Backend Use": https://encore.dev/articles/sse-vs-websockets
4. We.Inc (es), "SSE vs WebSocket: elige el stream adecuado": https://we.inc/es/guides/sse-vs-websocket
5. JusDB, "PostgreSQL LISTEN/NOTIFY: Real-Time Application Events Without Kafka": https://www.jusdb.com/blog/postgresql-listen-notify-realtime-events
6. PostgreSQL NOTIFY docs (payload/behavior): https://www.postgresql.org/docs/current/sql-notify.html
7. Ably, "WebSockets vs Server-Sent Events": https://ably.com/blog/websockets-vs-sse
8. Ithy, "SSE vs WebSockets for Rust & Axum": https://ithy.com/article/sse-vs-websockets-axum-rust-cf77ux8k
9. Rebase Docs (de), "Echtzeit über mehrere Instanzen" / LISTEN/NOTIFY context: https://rebase.pro/de/docs/backend/realtime-transports/
10. 祺洛 (zh-CN), Axum 事件广播（SSE/WebSocket）实践: https://www.qiluo.vip/tutorial/digging-deeper/event-broadcasting.html
11. nukopy/engawa (ja), Rust+Axum WebSocket chat architecture reference: https://github.com/nukopy/engawa

*This document is research output for planning cycle 015; implementation choices should be validated with targeted integration tests (reconnect, duplicate delivery, auth revocation, channel isolation, catch-up correctness) on the target VPS profile.*
