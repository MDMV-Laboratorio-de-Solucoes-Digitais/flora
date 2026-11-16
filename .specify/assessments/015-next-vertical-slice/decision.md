# Decision: 015 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**A — Minimal realtime delivery + deterministic catch-up for persisted channel messages (single-node, Postgres-authoritative).**  
Primary 80/20 for cycle 015: ship a reliable recovery path via **cursor-correct message catch-up** backed by Postgres and a **minimal in-process WS subscription/fanout** for same-node delivery, so committed messages become visible to other authorized clients in near-realtime and always recoverable on reconnect/reload. **Do not** require Valkey/Postgres LISTEN/NOTIFY, multi-node delivery, or full realtime feature parity.

## Justification (evidence)

- **P0 usability/correctness:** 013 persisted REST messages and 014 scoped WS session authority/protocol alignment, but delivery remains incomplete: no subscription registry/fanout, REST mutations emit no realtime events, `list_messages` is not cursor-deterministic (`before` accepted but offset/0 used, in-handler delete filter), and frontend lacked working sync/catch-up (`flora-api/src/routes/channels.rs`, `websocket.rs`, `docs/research/2026-11-16-014-ws-codebase-audit.md`, `014-realtime-delivery-vps-options.md`, ROADMAP P0.2/4).
- **80/20 leverage:** Postgres is already source of truth; adding stable cursor reads + in-process broadcast reuses existing WS/auth/DTO foundations with **no new mandatory infra**, fitting 2 vCPU/4 GB Seed. Push treated as best-effort with DB catch-up matches robust practice (NOTIFY/pubsub loss, reconnect gaps).
- **Sequencing:** completes the core communication slice before higher-complexity work (files binary I/O security surface, operability/boot refactors) and before scale adapters (Valkey/NOTIFY/Streams).
- **Research consensus (EN/es/de/fr/zh-CN/ja; ko/ru partial NV):** keyset/cursor by stable monotonic key (UUIDv7/`created_at,id`) for "after cursor" feeds; in-process fanout viable single-node; LISTEN/NOTIFY useful but non-durable/connection-sensitive; SSE strong for notify-then-fetch but adds transport churn now (`research.md`).

## Scope (in) — file-by-file implementation checklist

> Planned targets (confirm exact symbols/paths at implementation time; keep diff minimal).

### Backend — catch-up contract (Postgres-authoritative)
- `flora-core/src/traits/*message_repository*` & `flora-core/src/repositories/*message_repository*` (Postgres impl): add/support cursor query (e.g. `after`/`before` or keyset params) with deterministic ordering and documented deleted/edited behavior; ensure/select supporting index if missing (verify `messages` indexes/migration need).
- `flora-api/src/routes/channels.rs`: wire query params to cursor semantics (honor `before`, add `after` and/or explicit sync response metadata/cursor), remove non-deterministic offset-only recovery behavior, keep org/workspace/channel authorization intact; on successful `post_message` (and minimally `edit_message`/`delete_message` if included) emit canonical realtime event after commit/save.
- `flora-api/src/routes/mod.rs`: only if exposing explicit `/api/v1/messages/sync` alias (prefer extending existing list endpoint/contract first).

### Backend — in-process realtime delivery
- `flora-api/src/state/app_state.rs` (+ state module/types): add shared realtime hub/broadcaster handle (process-local, no external service).
- `flora-api/src/routes/websocket.rs` (or extracted `realtime` module): per-connection subscription set (org/channel-scoped), register on `Subscribe`/remove on `Unsubscribe`/cleanup on disconnect, forward canonical `NewMessage` events from hub to matching subscribers with bounded buffering/backpressure policy; define sender-echo/dedupe semantics; keep DB-backed auth/tenant checks and 014 transport/protocol decisions.
- `flora-api/src/routes/channels.rs` (and WS send path if applicable): publish canonical payload aligned to REST `MessageResponse` fields to hub (and retain best-effort Valkey publish if configured, non-authoritative).

### Frontend — catch-up + realtime consumer
- `apps/frontend/src/lib/api/messages.ts` (or equivalent API module): request messages with cursor/after support and parse deterministic ordering/cursor metadata.
- `apps/frontend/src/lib/api/websocket.ts`: subscribe/unsubscribe per active channel, handle canonical tagged envelopes, surface reconnect and gap signals.
- `apps/frontend/src/lib/api/mappers.ts` & `apps/frontend/src/lib/state/MessageStore.svelte.ts`: consistent mapping and idempotent upsert/merge by message `id`.
- `apps/frontend/src/lib/components/chat/ChatLayout.svelte` (and channel views): initial REST load, subscribe active channel, on reconnect/event trigger incremental catch-up, avoid duplicate render on REST-send vs WS delivery.
- `apps/frontend/src/routes/+layout.svelte` / `(app)/+layout.svelte` (as needed): WS lifecycle aligned with auth/org/token changes per 014.

### Tests & docs
- `flora-api` unit tests: cursor param parsing/normalization, envelope/event serialization, subscription routing helpers (pure where possible).
- `flora-tests` DB-gated contracts/integration (PG18 disposable): deterministic catch-up ordering and tenant isolation, cursor progression/no-duplicate/no-silent-gap within committed set, mutation visibility semantics.
- `apps/frontend/tests/unit/*`: websocket client classification/merge behavior and cursor fetch integration points (mocked).
- `.github/workflows/rust.yml`: include new env-gated/`--ignored` DB tests in `db-tests` job if added.
- `docs/dev-smoke.md`: manual steps for two-client realtime + disconnect/reconnect catch-up (NV if not executed).

## Deterministic verification plan

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings` (document scoped fallback only if unavoidable/preexisting)
3. `cargo test -p flora-core --lib`
4. `cargo test -p flora-api`
5. `cargo test -p flora-tests --lib`
6. DB-gated (Postgres 18, disposable/alt-port ok):  
   `FLORA_DATABASE__POSTGRES_URL=... cargo test -p flora-tests <catchup_delivery_contract_filter> -- --ignored --test-threads=1`
7. `cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build`
8. **NV (not claimed):** live multi-browser/client e2e, real network/proxy/TLS, Valkey/Meilisearch boot permutations, 2 vCPU/4 GB load/concurrency sizing, 5432-default-port smoke, OIDC/JWKS, `apps/web`, deploy/merge/release.

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Cursor semantics wrong (duplicates/gaps/reordering) | Silent missed or duplicated messages | Use stable keyset order (UUIDv7/id and/or `created_at,id`), idempotent merge, DB-gated determinism tests, define deleted/edit visibility. |
| In-process fanout only (single node) | No cross-instance delivery | Explicit non-goal; keep event/hub interface adapter-friendly for future Valkey/LISTEN-NOTIFY. |
| Backpressure/memory on 4 GB VPS | Dropped events or resource pressure | Bounded per-conn/channel buffers and subscription caps, drop-oldest-with-catch-up signal or equivalent, rely on DB recovery. |
| Sender echo / REST+WS duplication | UI duplicates | Canonical event IDs + client dedupe by `id`; define single emission rule per committed mutation. |
| REST/WS DTO drift | Broken clients | One canonical payload shape aligned to `MessageResponse`; mapper/unit tests. |
| 014 not fully merged/aligned | Integration friction | Treat 014 auth/path/protocol alignment as base; implement only compatible deltas and note dependency. |
| Index/migration need for cursor query | Perf regression or schema churn | Verify existing indexes/query plans; add minimal migration only if evidenced necessary. |

## Non-goals (015)

- Binary file upload/download/streaming, storage writes, attachment handling, quota enforcement changes beyond existing metadata behavior (**slice B**).
- Seed/VPS operability refactors: Meilisearch optional boot decoupling, compose profiles/resource limits, DB pool wiring/init dedup, broad env contract cleanup (**slice C**), except incidental doc notes.
- Multi-node/cross-process fanout, durable exactly-once delivery, Valkey consumer bridge, Postgres LISTEN/NOTIFY listener, outbox/streams.
- Presence/typing/read receipts, threads UX expansion, reactions, message search/notification wiring, full edit/delete realtime taxonomy if non-trivial.
- New `/messages/sync` subsystem beyond a thin alias to the same cursor list semantics (if needed).
- Browser Playwright/e2e, OIDC/JWKS end-to-end, `apps/web`, deploy/production rollout, merges/releases, secrets/services.

## Debloat / VPS notes

- **No new mandatory services or daemons.** Correctness = Postgres + deterministic catch-up; realtime = in-process best-effort.
- Keep payloads small (IDs/canonical fields), enforce bounded fanout state, and prefer simple backoff/limits over feature-rich brokers.
- Preserve Valkey as optional side-channel only; do not couple correctness to it.
- Favor extending existing routes/repos over new crates/abstractions; interface seams only where they replace future adapters cheaply.

## NVs

- NV-1: 014 merge/runtime state and final WS auth/protocol details not executed/verified here; checklist assumes 014 intent as baseline.
- NV-2: No load/memory measurements on target VPS; buffer/subscription limits require tuning.
- NV-3: Multilingual research coverage partial for **ko/ru/pt-BR** (triangulated, no conflicts found); primary recommendations rest on PostgreSQL/Axum/RFC/Docker/MDN evidence.
- NV-4: Existing `messages` schema/index adequacy for chosen cursor strategy to be confirmed at implementation time.
- NV-5: `apps/web` and browser e2e remain out of scope/unverified.
