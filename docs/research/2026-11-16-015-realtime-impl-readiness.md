# 015 — Realtime minimal delivery/catch-up implementation readiness (single-node VPS)

**Date:** 2026-11-16  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (research artifact only; no application source modified/committed)  
**Goal:** Concrete, low-risk implementation plan for minimal realtime delivery/catch-up of **persisted** channel messages on a **single cheap VPS (~2 vCPU / 4 GB)** using: Axum **in-process** fanout/broadcast, deterministic **REST cursor catch-up**, idempotent frontend upsert, reconnect/resubscribe.  
**Context inputs:** `docs/research/2026-11-16-015-realtime-minimal-delivery-design.md`, `docs/research/2026-11-16-014-ws-codebase-audit.md`, `docs/research/2026-11-16-014-realtime-delivery-vps-options.md`, `flora-api/src/routes/{websocket.rs,channels.rs,mod.rs}`, `flora-api/src/state/app_state.rs`, `flora-messaging/src/lib.rs`, `flora-core/src/{models/message.rs,models/mod.rs,traits/repository.rs,repositories/message_repository.rs}`, `apps/frontend/src/{lib/api/websocket.ts,lib/api/mappers.ts,lib/state/MessageStore.svelte.ts,lib/components/chat/ChatLayout.svelte,routes/(app)/+layout.svelte}`, `flora-tests/src/{contract/websocket.rs,integration/messaging.rs}`.

## 1. Readiness verdict (80/20)

- **Feasible as a minimal slice** if scoped to: REST remains mutation+history authority, in-process WS hub for acceleration only, canonical events emitted **after DB commit**, and mandatory cursor catch-up on (re)connect/subscribe/visibility/gap. No Valkey/NOTIFY/durable broker required for Seed correctness.
- **Main blockers today:** no subscription registry/fanout (`websocket.rs` acks only), REST mutations do not emit events (`channels.rs`), `list_messages` ignores `before` and uses offset+DESC without sync cursor (`channels.rs`, `message_repository.rs`), WS/REST/frontend envelopes drift (`WsServerMessage::NewMessage`, `websocket.ts` expects `payload.message`, `mappers.ts` drops `is_edited`), `MessageStore` append-only dedupe, `ChatLayout` REST-only/no subscribe lifecycle, `syncMissedMessages()` no-op.
- **Low-risk order:** (A) cursor/catch-up contract + repo/query/index, (B) canonical event builder/emission on all persisted mutations, (C) in-process hub + authz-scoped fanout, (D) frontend upsert/subscribe/reconnect-catch-up alignment, (E) limits/observability/tests.

## 2. Proposed canonical event and cursor contract

### 2.1 Cursor (deterministic, per-channel)

- **Primary cursor:** keyset tuple `("{created_at_rfc3339_nanos_or_millis}|{id_uuid}")` encoded as an opaque URL-safe string (base64url of the raw tuple) for API transport; server parses/validates. Rationale: `Message.created_at` + UUIDv7 `id` exist now (`flora-core/src/models/message.rs`), UUIDv7 is time-sortable but **`id` is the mandatory tie-breaker** for stable total order.
- **Ordering contract:**
  - Sync/catch-up responses: **ascending** by `(created_at ASC, id ASC)` so clients can advance `last_cursor` monotonically.
  - Existing UI/history loads may keep `before`/DESC semantics for “load older” pagination, but must be implemented correctly (keyset, not offset 0) and documented separately from `after` catch-up.
- **`after` semantics (catch-up):** return rows where `(created_at, id) > cursor(after)` for the same `channel_id` and `organization_id`, `is_deleted` handling per 2.3, limited (default 50, cap 100/200 decided in impl), in ascending order. Empty list => up-to-date.
- **Index (DB-gated migration):** add/support composite index for timeline scans, e.g. `(channel_id, created_at, id)` (or match chosen WHERE/order predicates) if not already present in migrations; validate with `EXPLAIN` in DB-gated tests/dev.

### 2.2 Canonical realtime envelope (REST/WS shared)

Single JSON shape (tagged or `{ "type", "event_id", "payload" }` — pick one and align both sides; prefer existing tagged `serde(tag="type", content="payload")` style in backend but include cursor inside payload/event wrapper consistently):

```jsonc
{
  "type": "MessageCreated" | "MessageUpdated" | "MessageDeleted",
  "event_id": "<cursor of this message state>",   // equals message cursor after mutation
  "org_id": "uuid",
  "channel_id": "uuid",
  "message": {
    "id": "uuid",
    "channel_id": "uuid",
    "sender_id": "uuid",
    "content": "string",
    "thread_id": "uuid|null",
    "is_edited": false,
    "is_deleted": false,
    "created_at": "rfc3339",
    "updated_at": "rfc3339"
  }
}
```

- **Authority:** `message` fields come from persisted DB row after commit (or the committed mutation result) — never trust client-supplied ids/org/channel except validated inputs already enforced by REST/WS authz.
- **Edit/delete:** emit `MessageUpdated` / `MessageDeleted` (or tombstone form) so clients converge; `MessageDeleted` should carry `is_deleted=true` and latest `updated_at` (and optionally content policy decision — recommend omit/blank only if product policy requires, but keep contract explicit).
- **Sender ack:** keep `MessageSent { message_id, channel_id, cursor? }` as delivery ack for WS send path compatibility, but clients must dedupe/upsert by `message.id` from canonical event/REST response.

### 2.3 REST catch-up API contract

- **Option (recommended, lowest surface):** extend existing `GET /api/v1/channels/{channel_id}/messages` query:
  - `after=<cursor>` (new): catch-up, ascending, includes rows needed for convergence (see deleted policy).
  - `before=<cursor>` (fix): older-page pagination, descending or return-then-client-reverse with documented order; must use keyset, not `offset: 0`.
  - `limit` retained/capped.
- **Deleted visibility:** for catch-up convergence prefer returning tombstones/deleted state for messages that transitioned to deleted **after** cursor when feasible; if repo currently filters `is_deleted=false`, add a sync-specific query/path or flag. Minimum: document behavior and ensure delete events are fanned out to online subscribers and clients refetch/mark deleted on reconnect window policy.
- **Response:** either `Vec<MessageResponse>` (compatible) plus `X-Next-Cursor`/metadata, or `{ items: [...], next_cursor: "..." }` (breaking) — prefer **backward-compatible** addition (`next_cursor` nullable field in wrapper or parallel endpoint `/messages/sync`) only if necessary; choose one in impl RFC/PR and update frontend mapper/client atomically.

### 2.4 WS control/auth contract (minimal changes)

- Keep `GET /ws` and `/api/v1/ws`, token via header or `?token` (`resolve_ws_token`) and DB-backed verification (`verify_bearer_token`) — already correct baseline.
- `Subscribe { channel_id }`: server must **authorize** `(org_id from session, channel_id)` membership/visibility before registering; ack only after registration (or with explicit failure).
- `Unsubscribe`: remove registration.
- On reconnect: client re-subscribes active channel(s) and runs REST catch-up from per-channel last cursor (not global `lastKnownMessageId` heuristic alone).
- Heartbeat: handle `Ping`/`Pong`; consider server ping/idle timeout compatible with reverse proxies (configurable), bounded write buffers, drop slow clients (log + metrics).

## 3. File-by-file change plan

| File | Planned change | Risk / notes |
|---|---|---|
| `flora-core/src/models/mod.rs` | Add cursor DTO/helpers (opaque cursor encode/decode, `MessageCursor { created_at, id }`) and/or pagination extensions (`after`, `before`) without breaking existing offset users; keep `Page` compatible. | Low if additive; avoid breaking `Pagination.resolved_limit()` semantics. |
| `flora-core/src/models/message.rs` | No schema change required; optionally helper `cursor()`/ordering key. Ensure `is_deleted` included in canonical event payload mapping decisions. | Low. |
| `flora-core/src/traits/repository.rs` | Extend `MessageRepository` with cursor-aware query, e.g. `find_channel_messages_after(channel_id, after_cursor, limit)` and/or replace/augment `find_by_channel_id` signature; consider `list_for_sync` returning tombstones if needed. | Medium: trait change ripples to impls/tests/mocks; prefer additive method to reduce blast radius. |
| `flora-core/src/repositories/message_repository.rs` | Implement keyset queries (`WHERE channel_id=$1 AND (created_at,id) > ($2,$3) ORDER BY created_at ASC, id ASC LIMIT $n`), fix `before` support, stop hardcoding `offset 0` for recovery paths, define deleted filtering per contract. | Medium: SQL correctness + perf; needs DB-gated validation. |
| `flora-core/src/migrations/sql/*` (new migration) | Add index for channel timeline keyset/catch-up (and `before` if used) and any constraint needed for deterministic ordering. | Low/medium; coordinate naming/order with existing migrations. |
| `flora-api/src/routes/channels.rs` | Parse `after`/`before` query params (cursor strings, not UUID-only unless cursor == id-only), return deterministic results/metadata, emit canonical events **after** successful create/update/soft-delete (post-commit in app flow), reuse shared envelope builder, preserve org/channel/sender authz checks. | Medium: central correctness path; avoid emitting on failed writes. |
| `flora-api/src/routes/websocket.rs` | Introduce per-connection state (authed user/org, subscribed channel set), handle `Subscribe/Unsubscribe` by mutating registry, split recv/send tasks or `tokio::select!` with outbound queue fed by hub, on `SendMessage` persist then emit canonical event via same publisher/hub (and/or rely on shared mutation service) and ack sender; remove/replace Valkey-only publish coupling for baseline delivery. Enforce subscribe authz (channel belongs to org / visible). | **Highest risk** (concurrency, task lifetime, backpressure). Keep bounded channels (`broadcast` or `mpsc` per socket), handle lag/drop explicitly. |
| `flora-api/src/state/app_state.rs` | Add in-process realtime hub/broker field (e.g. `Arc<RealtimeHub>` with `DashMap/RwLock<HashMap<ChannelId, subscribers>>` or sharded map) and optional event publisher abstraction; keep `messaging_service`/`valkey` optional for future adapter, but **not** in critical path. | Medium: shared-state contention; keep API simple (`publish(event)`, `subscribe`, `unsubscribe`, `disconnect`). |
| `flora-api/src/routes/mod.rs` | No route shape change required unless adding `/messages/sync`; ensure WS routes remain. | Low. |
| `flora-messaging/src/lib.rs` | Either (a) leave as optional Valkey adapter and extend with canonical event struct/serialization later, or (b) add a small `RealtimeEvent`/publisher trait seam consumed by API; do **not** require Valkey for 015. | Low if additive; avoid breaking existing publish callsites. |
| `apps/frontend/src/lib/api/websocket.ts` | Align envelope types to canonical contract, track per-channel subscriptions and last cursor (not only global `lastKnownMessageId`), implement `syncMissedMessages(channelId)` via REST catch-up, on connect/(re)subscribe/visibilitychange trigger catch-up then live apply, keep auth-failure stop behavior, bounded reconnect (existing). | Medium: reconnect races; serialize catch-up per channel. |
| `apps/frontend/src/lib/api/mappers.ts` | Map `is_edited`, `is_deleted` (if exposed), normalize nullable fields; support mapping from realtime payloads and REST responses identically. | Low. |
| `apps/frontend/src/lib/state/MessageStore.svelte.ts` | Replace append-only `addMessage` with idempotent **upsert/merge** by `id` (update content/edit/delete/updatedAt), maintain deterministic order by `(createdAt,id)`/cursor policy, optionally per-channel store or filtering helpers for active view. | Medium: Svelte 5 reactivity + ordering correctness. |
| `apps/frontend/src/lib/components/chat/ChatLayout.svelte` | Subscribe/unsubscribe on `channelId` change (and cleanup), integrate WS client lifecycle for active channel, on REST send/edit/delete reconcile with upsert (dedupe by id), show convergence-safe state (deleted/edited), trigger catch-up on load/reconnect hooks as needed. | Medium: avoid double-subscribe/leaks. |
| `apps/frontend/src/routes/(app)/+layout.svelte` | Keep global connect keyed by auth/org (already present); optionally expose reconnect/visibility hooks; ensure disconnect on logout/session lock. | Low. |
| `apps/frontend/src/lib/api/client.ts` / types/models (if present) | Add cursor/query helpers and response typing for catch-up; no secrets. | Low. |
| `flora-tests/src/contract/websocket.rs` | Extend DB-gated tests for: authorized subscribe receives fanout for same org/channel only, revoked/expired session rejected, cross-org isolation (if harnessable), invalid cursor handling. | Medium (DB-gated). |
| `flora-tests/src/integration/messaging.rs` | Replace placeholders with DB-gated end-to-end: REST create -> persisted -> catch-up via `after` returns message; edit/delete convergence; duplicate-safe behavior contract (where testable). | Medium. |
| `flora-api` / `flora-core` unit tests | Envelope serialization, cursor encode/decode roundtrip, keyset comparator, authz decisions for subscribe, hub registration/deregistration (pure/unit where possible). | Low/medium. |
| `docs/research/*` / `docs/agents/*` (if needed) | Document final contract/cursor encoding and ops limits for VPS/proxy timeouts. | Low. |

## 4. Migration and compatibility notes

- **DB:** additive migration for index(es) only; no destructive data changes. Safe on single VPS; run in deploy window as usual.
- **API compatibility:** prefer additive query params (`after`, optional `include_deleted`/sync mode) and keep existing response fields in `MessageResponse`; if adding metadata, use non-breaking envelope evolution or versioned sync endpoint. `before` currently accepted but ineffective — fixing it is behavior-correcting; document as bugfix.
- **WS compatibility:** backend `NewMessage` shape and frontend `parse/onEnvelope` must change in lockstep (015 implementation PR should be atomic across api+frontend). Preserve tolerant parsing during rollout if needed (accept old/new payload keys briefly), but remove drift quickly.
- **Valkey:** remains optional (`messaging_service` may be `None`); in-process hub provides correctness on single node. Do not gate baseline delivery on Valkey availability.
- **Auth/session:** continue DB-backed handshake/session authority; fail closed on invalid/revoked/expired/nil-org sessions (align with existing `extract_org_context` and tests).
- **Frontend state:** store becomes merge-based; ensure initial REST load + catch-up do not regress ordering for threaded replies (threads currently derived client-side in `ChatLayout.svelte`).

## 5. Deterministic test strategy

### 5.1 Unit (fast, no DB)

- **Backend (`flora-api`, `flora-core`):** cursor encode/decode roundtrip and ordering; comparator `(created_at,id)`; query-param parsing/validation (`after`/`before` malformed -> 4xx); canonical event builder maps persisted `Message` -> envelope (created/updated/deleted); hub register/subscribe/unsubscribe/drop semantics with bounded channels (mock sockets/channels); `websocket.rs` existing serialization/token-resolution tests updated.
- **Frontend (`apps/frontend`, Vitest server/project as CI):** `parseWsEnvelope`/`buildWsUrl` (existing) + new cases for canonical payloads; idempotent upsert merge (create/update/delete, out-of-order, duplicate); cursor advancement rules; `WsClient` reconnect decisions and auth-failure stop (mock `WebSocket`, timers via fake timers for determinism).

### 5.2 Integration/contract (DB-gated, deterministic)

- **Env gate:** `FLORA_DATABASE__POSTGRES_URL` or `DATABASE_URL` (follow existing pattern in `flora-tests/src/contract/websocket.rs`), run migrations before tests.
- **Cases:**
  - `list_messages?after=` returns committed messages in ascending keyset order, excludes/handles pre-cursor rows, respects limit, enforces org/channel authz.
  - `before` pagination correctness (no offset-0 bug) and stable boundaries.
  - REST create/edit/delete persists and (once wired) produces expected canonical event payloads; catch-up converges edited/deleted states per contract.
  - WS handshake: valid session accepted, revoked/expired/nil-org rejected (extend existing DB-gated tests).
  - Subscribe authz: cannot subscribe to other-org/unauthorized channel; fanout delivered only to authorized subscribers in same process/hub.
  - Reconnect recovery: simulate gap by skipping live delivery in harness or by applying only REST catch-up; final store/state equals DB truth.
- **Determinism controls:** fixed/controlled timestamps only where API allows, otherwise assert ordering/cursor monotonicity and set equality by ids+versions; unique test data via UUIDv7 labels; avoid sleeps—use bounded awaits/poll with timeouts.

### 5.3 E2E/manual smoke (single-node VPS profile)

- Two browser sessions, same org/channel: send/edit/delete -> peer converges live or after forced reconnect.
- Kill/restart WS path or drop network: reload/reconnect triggers catch-up, no duplicates, no silent loss after commit.
- Proxy timeout sanity: heartbeat/idle settings documented; observe reconnect counters.

## 6. NVs (non-functional/operational notes)

- **Single-node scope:** in-process hub state is per-process memory; restart drops transient subscriptions (acceptable) — correctness relies on catch-up from Postgres.
- **Backpressure/limits:** bound outbound queues, max subscriptions per socket, max payload/message size, heartbeat interval/idle timeout aligned with reverse proxy; on lag (`broadcast::RecvError::Lagged`) force resync path or disconnect+catch-up.
- **Observability:** counters/gauges for connections, auth failures, subscriptions, fanout deliveries/drops/lag, catch-up requests/rows; keep tracing structured (org/channel ids as appropriate, no secrets/tokens).
- **Security:** never trust client org/channel; authorize every subscribe and every emitted delivery target; keep query-token support only as constrained transport (prefer header when possible) and avoid logging tokens.
- **Performance (2 vCPU/4 GB):** keyset index critical; avoid N+1 in fanout; cap catch-up page size; prefer one REST catch-up per reconnect/channel activation batch.
- **Deferred (YAGNI):** Valkey/Redis pub-sub or Streams, Postgres `LISTEN/NOTIFY` delivery, outbox/durable log, multi-instance coordination, presence/typing/read receipts, global ordering across channels.

## 7. Multilingual research synthesis (EN, pt-BR, es, de, fr, zh-CN, ja)

| Lang | Corroborated guidance for Flora 015 | Evidence anchors |
|---|---|---|
| **EN** | Split WS recv/send tasks + `tokio::sync::broadcast` hub is the canonical single-node Axum pattern; push is best-effort so recovery must come from DB cursor/idempotent merge. SSE offers HTTP-native reconnect/`Last-Event-ID` for one-way streams but requires replay/cursor design; EventSource has no custom auth headers. Postgres `NOTIFY` is transactional-on-commit, async, non-durable, payload-limited (~8000 bytes), needs dedicated listener and app-level recovery. | Axum chat example + `axum::extract::ws` docs (fetched), MDN SSE (fetched), PostgreSQL `NOTIFY` docs (fetched), 014/015 docs refs [1–6]. |
| **pt-BR** | Em VPS pequena, polling puro aumenta latência/custo; melhor “persistir no Postgres + notificação em tempo real + recuperação determinística por cursor” com idempotência no cliente; manter infraestrutura mínima (sem broker obrigatório) no Seed. | 014/015 synthesis, EN patterns cross-check (docs locais + MDN/PG semantics). |
| **es** | SSE es adecuado para flujo servidor→cliente con reconexión automática (`EventSource`), mientras WS aporta bidireccionalidad con mayor complejidad (heartbeats, reconexión manual, estado). En chat persistido, reconectar debe disparar catch-up por cursor e inserción idempotente por `message_id`. | MDN SSE (fetched), We.Inc es comparison cited in 015 [ref 4], Axum WS pattern. |
| **de** | `LISTEN/NOTIFY` ist leichtgewichtig (commit-sichtbar) aber **nicht persistent**, ohne Backlog, mit Payload-Grenze und Bedarf an dauerhafter Listener-Verbindung/Recovery — daher für 015 nur später als Invalidierungssignal, nicht als Zustellgarantie. Zuverlässigkeit über DB-Catch-up. | PostgreSQL `NOTIFY` docs (fetched), Rebase/Ugur notes cited in 014/015 [refs 12,13,18]. |
| **fr** | Pour une pile temps réel auto-hébergée simple, préférer DB comme source de vérité + push minimal + reprise après reconnexion par curseur; la montée multi-instance impose bus/pubsub et gestion explicite des pertes/duplications (au-moins-une fois côté UI). | 014/015 synthesis, Encore/SSE semantics, ForgeChat/VPS patterns cited in 014. |
| **zh-CN** | Axum 单机场景常用 in-process broadcast/hub + 有界队列；断线重连后必须按游标补拉（after/cursor）并按消息 ID 幂等合并/更新，避免丢失与重复；集群化再引入 Redis Pub/Sub 桥接。 | Axum chat example (fetched), 014/015 zh-CN refs (祺洛/axum-realtime-kit ecosystem notes). |
| **ja** | Rust/Axum の WebSocket 実装では接続管理・認証・購読境界・再接続時の取りこぼし補完が要点。履歴/正本は DB、配信は通知として扱い、再接続時はカーソルで差分取得し ID で冪等に反映する設計が堅実。 | 014/015 ja refs (engawa 等), Axum WS/docs + PG/SSE semantics triangulation. |

## 8. Implementation readiness checklist (go/no-go)

- [ ] Cursor encoding/semantics agreed (`after` ascending keyset, tie-breaker `id`) and DB index planned.
- [ ] Canonical event envelope frozen (create/update/delete + `event_id/cursor` + full message state).
- [ ] Mutation emission points enumerated (REST create/edit/delete, optional WS send) — all post-commit.
- [ ] Hub design bounded (queues, lag policy, authz on subscribe/deliver).
- [ ] Frontend upsert/store ordering + reconnect/catch-up flow specified and testable.
- [ ] DB-gated test env documented (`FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`) and new cases scoped.
- [ ] Deferred items explicitly out of scope for 015 (Valkey fanout, NOTIFY, durable broker, multi-node).

**Primary sources fetched/consulted:** Axum WS extract docs (`docs.rs/axum`), Axum chat example (`tokio-rs/axum`), PostgreSQL `NOTIFY` documentation, MDN “Using server-sent events”; plus repository-local research docs 014/015 for multilingual synthesis and Flora-specific evidence. 
