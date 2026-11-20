# Concept: 015 — Minimal reliable realtime delivery + deterministic catch-up

## Chosen concept (Slice A, scoped go)

Implement a **Postgres-authoritative, single-node realtime slice** for persisted channel messages:

1. **Deterministic catch-up (correctness path):** make message listing cursor-correct and idempotent so clients can fetch "messages after cursor" reliably (stable ordering, explicit `after`/`before` semantics, indexed query, deleted/edited handling defined). Provide a documented sync contract (either extend `GET /channels/{id}/messages` with cursor params and response cursor metadata, or add a thin `/api/v1/messages/sync` alias with same semantics) so frontend recovery is real.
2. **Minimal in-process fanout (delivery path):** add an in-app broadcast hub/registry keyed by `org_id` + `channel_id` that tracks active WS subscriptions on the local process and forwards canonical message events to matching sockets. No external broker required.
3. **Unified event emission from mutations:** publish a canonical `NewMessage` (and optionally minimal `MessageUpdated`/`MessageDeleted` if cheap; otherwise defer taxonomy) when a message is persisted via REST and/or WS, sourced from the saved DB record/DTO so payloads match REST `MessageResponse` fields.
4. **Frontend minimal alignment:** WS client subscribes/unsubscribes per active channel, handles canonical envelopes, performs initial load + incremental catch-up on (re)connect/event gap, merges idempotently by `id`, and uses bounded reconnect with auth-failure stop (respecting 014 auth behavior). REST remains send/load fallback.

**Design stance:** push is **best-effort notification/delivery**; **DB cursor catch-up is the recovery guarantee**. This matches LISTEN/NOTIFY/SSE/WS realities and 014 options research.

## Why not LISTEN/NOTIFY or Valkey as primary for 015

- **Valkey pub/sub:** already a publish-only optional primitive; adding a reliable consumer+bridge+ops is extra surface and still at-most-once without replay. Defer as scale adapter.
- **Postgres LISTEN/NOTIFY:** attractive no-new-service signal but requires dedicated listener connection, payload/queue limits, and careful missed-notify recovery; for single-process in-node fanout we already have the commit point in-process, so direct in-process broadcast after DB commit is simpler and lower risk. LISTEN/NOTIFY can be a later invalidation adapter if multi-instance is needed.
- **SSE:** viable alternative for server→client only, but backend/frontend already have WS protocol foundations; 015 can deliver minimal value by completing WS registry/fanout + REST catch-up without introducing a second transport.

## Alternatives considered

| Option | Pros | Cons | Verdict |
|---|---|---|---|
| Polling-only catch-up | simplest, stateless | latency/battery/request overhead; weak realtime UX | Keep as implicit fallback via catch-up, not primary |
| SSE notifications + REST catch-up | auto-reconnect/Last-Event-ID ergonomics | adds new transport path; diverges from current WS investment | Good follow-up; not 015 primary |
| In-process WS fanout + cursor catch-up | reuses existing WS/auth/DTOs; zero new infra; immediate UX | single-node only; needs registry/backpressure discipline | **Chosen** |
| LISTEN/NOTIFY bridge | no Valkey, multi-instance-ready signal | dedicated conn, missed-notify handling, extra complexity now | Defer |
| Valkey/Streams durable fanout | scale/durability path | overkill for Seed, ops/RAM | Defer (P2/scale) |

## Scope boundaries (015)

- **In:** cursor-correct message read/sync semantics for a channel; server-side per-connection subscription tracking and in-process event dispatch for subscribed channels; emit canonical events on persisted message mutations in-scope; minimal frontend consumer/merge/reconnect-catch-up behavior; deterministic tests/docs.
- **Out (unless trivial):** multi-node fanout, durable delivery guarantees, presence/typing/read receipts, rich edit/delete realtime UX, attachment/file realtime, full text-search/notification wiring, compose/runtime operability refactors, binary file upload/download, `apps/web`.

## Debloat / VPS notes

- **No new mandatory services.** In-memory hub only; Valkey remains optional/unused-for-correctness.
- Bound buffers and subscriptions (e.g. cap channels per socket, drop/lag strategy) to protect 4 GB VPS.
- Keep event payloads minimal (IDs + canonical fields) and REST as fetch source on large catch-up.
- Prefer one canonical message DTO/envelope mapping shared conceptually across REST, WS, and frontend to reduce drift.
- Postgres: add/confirm supporting index for cursor query (likely `messages(channel_id, created_at, id)` or leverage UUIDv7 ordering) — verify before adding migration; avoid speculative schema churn.

## File-by-file implementation checklist (planned)

> Confirm exact signatures/APIs at implementation; keep diff minimal and crate-local where possible.

### Backend — persistence/catch-up contract
- `flora-core/src/repositories/*message*` (and `traits`): add cursor-aware query (e.g. `find_by_channel_after/before` or extend list) with stable ordering and deleted-row policy suitable for sync.
- `flora-api/src/routes/channels.rs`: honor `before`/add `after` (or `cursor`) query params; return deterministic ordering and optional cursor metadata/headers if needed; ensure tenant/channel checks unchanged; on `post_message`/`edit_message`/`delete_message` emit canonical event to hub after successful DB mutation (and keep Valkey publish best-effort if present).
- `flora-api/src/routes/mod.rs`: only if adding explicit `/messages/sync` route (prefer extending existing list first).

### Backend — realtime hub/fanout
- `flora-api/src/routes/websocket.rs` (or extracted module e.g. `realtime.rs`): define canonical event payload aligned to `MessageResponse`; maintain `DashMap`/`RwLock`/`tokio::sync` structures for connection subscriptions (scoped to org/channel) with cleanup on disconnect; on `Subscribe` register and send ack (+ optional immediate catch-up hint), on `Unsubscribe` remove; on hub event forward `NewMessage` to matching local subscribers (exclude or allow sender-echo but define dedupe).
- `flora-api/src/state/app_state.rs` (and state module): hold shared realtime hub/broadcaster handle (in-process, no external dep).
- `flora-messaging/src/lib.rs`: no required changes for correctness; optional future adapter only.

### Frontend — minimal reliable client
- `apps/frontend/src/lib/api/websocket.ts`: connect to aligned path/transport (per 014), manage per-channel subscribe/unsubscribe, parse canonical envelopes, expose event stream/callbacks, trigger catch-up on reconnect/gap, bounded backoff+jitter and no blind retry on auth failure.
- `apps/frontend/src/lib/api/messages.ts` (or equivalent): support cursor/after params and/or sync call matching backend contract.
- `apps/frontend/src/lib/api/mappers.ts`: map REST/WS message payloads consistently (id, ids, content, timestamps, edited flag) with idempotent merge key.
- `apps/frontend/src/lib/state/MessageStore.svelte.ts`: upsert/dedupe by `id`, ordered merge, handle create/update/delete minimally if emitted.
- `apps/frontend/src/lib/components/chat/ChatLayout.svelte` (and channel route/page): subscribe to active channel, load initial history, merge realtime/catch-up results, avoid duplicate append on REST send vs WS echo (define rule).
- `apps/frontend/src/routes/+layout.svelte` / `(app)/+layout.svelte`: ensure WS lifecycle on auth/org/token changes per 014 integration.

### Tests & docs
- `flora-tests/src/contract/messaging/*` or `integration/*` (DB-gated, PG18): cursor catch-up determinism (after/before, ordering, no duplicates, tenant isolation), mutation emits/visibility semantics as feasible.
- `flora-api/src/routes/websocket.rs` / `channels.rs` unit tests: envelope serde, cursor param parsing, hub subscription routing (pure/unit where possible).
- `apps/frontend/tests/unit/*`: WS client envelope handling, reconnect/catch-up trigger classification, idempotent store merge.
- `.github/workflows/rust.yml`: include DB-gated messaging delivery/catch-up tests in `db-tests` job if env-gated/ignored pattern used.
- `docs/dev-smoke.md`: manual realtime + reconnect catch-up steps (NV if not executed).

## Deterministic verification plan

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings` (or scoped, documented fallback only if preexisting blockers)
3. `cargo test -p flora-core --lib`
4. `cargo test -p flora-api` (new/affected unit tests)
5. `cargo test -p flora-tests --lib`
6. **DB-gated (disposable Postgres 18, alt-port ok):** `FLORA_DATABASE__POSTGRES_URL=... cargo test -p flora-tests <catchup/realtime contract filters> -- --ignored --test-threads=1`
7. `cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build`
8. **NV:** multi-client/browser e2e, real Valkey/Meilisearch-disabled boot permutations, load/concurrency on 2 vCPU/4 GB, default-port 5432 conflicts absent, `apps/web`, production TLS/proxy behavior.
