# Problem: 015 — Next vertical slice

## Problem statement

After 013 (persisted REST channels/messages) and 014 (scoped WS session authority + protocol alignment), Flora can **store** chat messages in Postgres but still lacks a **minimal reliable realtime delivery and deterministic catch-up** path for persisted channel messages on a single-node Seed VPS. As a result, connected clients do not receive each other's committed messages reliably, reconnects/reloads cannot deterministically recover missed updates, and the frontend/chat experience remains effectively REST-only or contract-drifted despite partial WS plumbing.

This is a P0 usability/correctness gap for the "Comunicação" core of the Macro-alternative MVP: the source of truth exists, but the delivery/recovery plane is incomplete.

## Evidence (current gaps)

1. **No deterministic catch-up contract.** `GET /api/v1/channels/{channel_id}/messages` accepts `limit`/`before` but `flora-api/src/routes/channels.rs::list_messages` queries with `Pagination{limit, offset:0}` and ignores cursor semantics; deleted filtering is in-handler. No `after` cursor, no stable high-water-mark sync, no `/api/v1/messages/sync` endpoint, yet prior frontend logic expected sync/recovery (`docs/research/2026-11-16-014-ws-codebase-audit.md`).
2. **No server-side fanout/subscription delivery.** `flora-api/src/routes/websocket.rs::handle_socket` acks `Subscribe/Unsubscribe` only (no registry). `SendMessage` validates tenant/channel, persists via `PgMessageRepository`, and best-effort publishes to Valkey if configured, but sends only `MessageSent` to sender. There is no in-process subscriber map and no Valkey consumer/WS bridge in API state; `flora-messaging` is publish-only. REST create/edit/delete in `channels.rs` emit no realtime events.
3. **Protocol/contract drift risk remains for delivery.** 014 scoped auth/path/envelope alignment; `NewMessage` payload shape/naming and event emission paths still must align with REST DTOs/idempotent client merge and with actual mutation sources (REST and/or WS) to avoid duplicates/loss.
4. **VPS constraint.** Seed target (2 vCPU/4 GB, single node) cannot afford unnecessary always-on brokers or stateful complexity; correctness must not depend on lossy push without DB catch-up.

## Why this matters now (80/20)

- **Direct user value:** makes the existing persisted `workspace → canal → mensagem` slice feel usable in near-realtime and recoverable after disconnect/reload — core SMB collaboration need.
- **Sequencing:** 014/013 provided persistence and authenticated WS foundation; delivery+catch-up is the next enabling step before richer realtime features, notifications wiring, or multi-instance scaling.
- **Debloat fit:** can be solved Postgres-authoritatively with in-process single-node fanout (no mandatory Valkey/NOTIFY infra), keeping REST as correctness path.

## Non-problems for this slice (deferred)

- Binary file upload/download/storage (metadata-only today) — candidate 015B.
- Seed/VPS operability hardening (Meilisearch boot coupling, compose limits/profiles, pool/config) — candidate 015C.
- Full presence/typing/read receipts, message history replay protocol beyond deterministic catch-up, edit/delete event taxonomy expansion beyond minimal needed, multi-node/cross-region delivery guarantees, `/messages/sync` as a separate complex subsystem unless trivially equivalent to cursor list.
- Browser e2e, OIDC/JWKS, deploy/merge/release.

## Success criteria (problem resolved when)

1. A message committed in Postgres is retrievable by any authorized same-org/channel client via deterministic cursor-based catch-up (after reconnect/reload) without manual intervention.
2. On a single running node, an authorized subscribed client receives newly persisted channel messages (or an invalidation sufficient to trigger catch-up) with bounded latency and **no cross-org/channel leakage**.
3. Client/server handling is idempotent/dedup-safe by message ID and resilient to reconnect (bounded backoff, auth-failure stops as per 014).
4. No new mandatory services beyond Postgres (+ app) are required; push remains best-effort with DB as correctness backbone.
5. Behavior is covered by deterministic automated tests (unit + DB-gated contracts where applicable) and a documented verification plan.
