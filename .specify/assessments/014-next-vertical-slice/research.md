# Research: 014 — WS session authority, realtime options, Seed/VPS operability

**Date:** 2026-11-16  
**Method:** local code/docs audit + multilingual web research (EN, pt-BR, es, de, fr, zh-CN prioritized; ja/ko/ru accessible indexed coverage limited — findings triangulated with upstream docs and cross-language consensus). No installs beyond read-only `npx skills find` query.

## 1) Current state (local evidence)

| Area | Evidence | Finding |
|---|---|---|
| HTTP session authority (baseline) | `docs/research/2026-11-16-012-session-authority-international.md`, `flora-api/src/extractors/auth.rs` (per research), PR #215 notes in `OBJETIVO.md`/`ROADMAP.md` | `authenticate_bearer` validates JWT + `SessionService::validate_session(jti)` in Postgres (active, not expired, user/org match, nil-org parity); generic 401; DB failure → 500 fail-closed. |
| WS handshake/auth | `flora-api/src/routes/websocket.rs:100-141,281` | `/ws` accepts upgrade then `extract_org_context` does **JWT decode only** from `Authorization: Bearer`; **no DB `jti`/session check**, no expiry/active/revocation enforcement beyond token validity, no token transport fallback. Binds `(user_id, org_id)` for socket lifetime. |
| WS protocol/runtime | `websocket.rs:22-94,159-329` | Tagged messages `Subscribe/Unsubscribe/SendMessage/Ping` ↔ `Subscribed/.../NewMessage/MessageSent/Pong/Error`; `SendMessage` checks channel-org ownership and persists; Valkey publish best-effort; **no subscription registry/fanout loop** evidenced. |
| Frontend WS client | `apps/frontend/src/lib/api/websocket.ts:23-97`, 013 research | Connects to `/api/v1/ws?token=` (path+transport mismatch), blind reconnect, no subscribe/send protocol, expects `workspace.user.removed` and `/api/v1/messages/sync` (missing). Org/token changes not rebound to socket lifecycle. |
| Seed/VPS audit | `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docker-compose.yml` context | Postgres 18 mandatory; Valkey operationally optional in state but present in compose; **Meilisearch effectively required by boot in current behavior** despite “optional” intent; DB pool hardcoded/duplicated concerns; no mem limits in compose; 2 vCPU/4 GB tight with PG+Valkey+Meili. |
| 013 decision | `.specify/assessments/013-next-vertical-slice/decision.md` | Chose REST persistence slice; explicitly deferred WS authority/protocol/fanout, sync, attachments, DM UX. |

## 2) Topic (1) — Secure DB-backed WebSocket session auth/revocation in Axum/Rust

### Consensus (high-confidence, cross-language)
- **Authenticate before upgrade** (or at minimum before accepting app traffic) and **fail closed**; reuse a single verification path with HTTP to avoid policy drift. Axum `WebSocketUpgrade` handler/extractor can reject with HTTP status pre-upgrade (e.g. `ws-barbican` `AuthenticatedWs`, Axum guides). [EN: mojoauth.com, rustysocks.io, toolsku.com; es: rustango.com/es, spacehowen.com; pt-BR: rustlang.com.br; fr: rustango.com/fr; de: rustango.com/de]
- **DB-backed authority:** JWT signature/claims are necessary but insufficient for immediate revocation; check server-side session state keyed by `jti`/session id (active, not expired, user/org/tenant match). Store/blacklist in Postgres or shared store (Redis/Valkey) for multi-replica; in-memory only is single-process. [EN: messages.solutions, dev.to/bittokks, 0xshadow.dev, sheroz/axum-rest-api-sample, hstm/fastify-uws-auth; es/fr/de: Rustango JtiStore guidance; pt-BR: rustlang.com.br]
- **Browser token transport:** browsers cannot set arbitrary WS headers. Prefer **cookie session (HttpOnly/SameSite/Origin checks)** for browser apps, or **short-lived one-shot/ticket** exchanged over authenticated HTTP then passed via `Sec-WebSocket-Protocol`/first message/query fallback with strict logging/TTL controls; avoid long-lived tokens in URLs (leak via logs/history/referrer/proxies). [EN: rustysocks.io “headers only, never URL” vs pragmatic fallbacks in ws-barbican/kirino-session examples; messages.solutions; project-orchestrator PR #46 pattern: validate cookie pre-upgrade; pt-BR: rustlang.com.br WebSocket auth notes]
- **Revocation/expiry for long-lived sockets:** define explicit window. Options: handshake-only (revocation on reconnect/TTL) — lowest cost; handshake + periodic revalidation (jittered, e.g. 60–300s) or on privileged actions — bounded abuse window; push invalidation via pubsub keyed by `jti/user/org` — immediate but complex. On invalidation send generic app error then close `1008` (policy) / reauth semantics. [EN: messages.solutions, docs/research/2026-11-16-013-websocket-session-authority.md, RustySocks/Axum patterns]
- **Tenant isolation:** bind verified `user_id/org_id/jti` at handshake; never trust client org/channel claims; enforce channel-org membership on subscribe/send and future fanout; nil-org handling must match HTTP policy. [local websocket.rs:205-219, HTTP extractor parity]
- **Operational safety:** generic auth errors (no token/JTI leakage), no token logging, rate-limit upgrades/connections/messages, bound payload sizes, constant-time/strict claim validation where relevant; DB unavailable on auth → fail closed (500/deny) consistent with HTTP.

### Axum/Rust implementation patterns (practical)
- Shared verifier: `fn verify_bearer_token(token: &str, state: &AppState) -> Result<VerifiedSession, AuthReject>` reused by HTTP extractors and WS handler (preferred for Flora; aligns with 012).
- Extractors/middleware crates/patterns: `AuthenticatedWs` (ws-barbican) authenticates pre-upgrade; custom `FromRequestParts`/handler parsing `Authorization`, optional query/subprotocol; `tower` layers for rate limiting/CORS already in stack.
- One-shot WS ticket (kirino-session style) reduces replay risk of URL tokens but adds endpoint/state; **likely over-scope for 014 minimal** vs reusing existing access token + DB session check.

## 3) Topic (2) — Minimal realtime delivery for chat on 2 vCPU / 4 GB VPS

### Tradeoffs (evidence-backed)

| Mechanism | Pros (small VPS/chat) | Cons / limits | Fit for Flora Seed |
|---|---|---|---|
| **Polling (HTTP, conditional/ETag or cursor)** | Simplest, stateless, proxy-friendly, no persistent WS state, easy tenancy/auth via existing HTTP middleware | Extra requests/latency, wasted cycles when idle; acceptable at minute→few-second cadence for low volume | **Strong fallback/minimal correctness path** if fanout deferred; easy to make DB-session-authoritative per request. |
| **SSE (server→client)** | Standard HTTP, auto-reconnect/event IDs, low complexity vs WS, good for notifications/feeds, 1-way fits “new message” push; efficient in HTTP/2 | One-way (client sends via POST), browser connection limits on HTTP/1.1, needs buffering/heartbeat tuning, still holds connections | Viable **minimal push** alternative; less protocol drift than current custom WS if redesigned, but requires new endpoint/client. |
| **WebSockets (current direction)** | Bidirectional, low per-message overhead after handshake, natural for chat/presence/typing | Stateful connections/tasks/memory, reconnect/auth complexity, proxy quirks, must implement registry/fanout/backpressure | **Keep as target** (already scaffolded) but fix auth/protocol first (slice A); full fanout is slice-B scope. |
| **Postgres LISTEN/NOTIFY** | No extra infra, transactional with writes (notify on commit), great “doorbell” with table-as-truth, fits Postgres-first; fine p50 (<1ms) at moderate rates | Payload < ~8kB, duplicate collapse per tx, fire-and-forget (missed while offline), p99/tail spikes at higher fanout/rate, writer throughput cost (~20–45% in tested bands), needs dedicated LISTEN connection (PgBouncer tx mode issue), stalled listener risk on queue | **Good doorbell for small VPS single-node** if paired with DB catch-up; avoid as payload bus; watch pool/connection budget. (fabwaseem.com 2026 benchmark; freeCodeCamp 2026; redisson.pro; aaronhsyong2/pg-stack-benchmark) |
| **Valkey/Redis Pub/Sub** | Steadier tail latency, offloads DB writer path, simple fanout primitive, already optional in stack | Extra service/memory, at-most-once (miss while disconnected), adds ops surface | Useful when enabled, but **must remain optional** for Seed/VPS; not a correctness source of truth. (NodeRings/Valkey footprint ~3–4 MiB idle but real cap/planning matters) |

### VPS guidance (2 vCPU / 4 GB)
- Prefer **Postgres-first, table-as-truth + notify/doorbell or short polling** for correctness; treat push as optimization. (multiple sources, Seed audit)
- Keep **one dedicated LISTEN connection** if used; never hold tx open on listener; monitor `pg_notification_queue_usage`.
- Budget pressure: Postgres + app dominate; Meilisearch can be 300–700+ MB; Valkey small if capped (`maxmemory`) but optional; strict `mem_limit`, tuned `shared_buffers/work_mem/max_connections`. (noded.cloud, NodeRings, laraue.com, cloudjobs.io, micro-VPS patterns)

## 4) Topic (3) — Seed/VPS operability for Rust+Postgres stacks

- **Optionality reality:** make search/valkey truly optional at boot/runtime with graceful degradation; document required vs optional per profile. (Seed audit action #1,#4,#5)
- **Resource controls:** compose profiles/overlays, `mem_limit`/reservations, restart policies, bind internal services to localhost/private net, persist PG18 volume correctly (`/var/lib/postgresql`), set `--maxmemory` for Valkey, cap Meilisearch indexing memory/threads. (Seed audit, docker compose best practices, laraue.com, cpanelfree.com, NodeRings)
- **DB/pool tuning:** wire configured pool params (currently hardcoded concerns), reduce default connections for small VPS, avoid duplicate pool+migration init; tune PG for 1–4 GB (`shared_buffers`, `effective_cache_size`, `work_mem`, `max_connections`, WAL/checkpoint basics). (Seed audit, cloudjobs.io, micro-VPS/dev.to)
- **Env/health contract:** consistent `FLORA_*`/`__` keys, strong JWT secrets, readiness distinguishing required (DB) vs degraded/disabled (Valkey/Meili), avoid mixed naming. (Seed audit, `.env.example`/config loader behavior)
- **Backup/restore minimum:** logical `pg_dump` off-host + restore drill on fresh PG18, persist storage volume, treat Valkey/Meili as rebuildable where possible. (Seed audit)
- **Security/ops:** no exposed DB/cache/search ports publicly, TLS termination via proxy, rate limits, log rotation, avoid swap reliance for DB-heavy workloads.

## 5) Skills consultation (`npx skills find/use`)
- Queried `npx skills find "websocket authentication"` (read-only discovery). Relevant matches surfaced: `patricio0312rev/skills@websocket-realtime-builder`, `martinholovsky/claude-skills-generator@websocket`, `mukul975/anthropic-cybersecurity-skills@testing-websocket-api-security`, `actionbook/rust-skills@domain-web`. **No full skill install performed** (per constraint); insights incorporated via external research and local patterns instead.

## 6) Multilingual coverage note
- **EN/pt-BR/es/de/fr/zh-CN:** directly accessible sources reviewed (Axum/Rust auth, JWT/session revocation, Postgres/Redis realtime, VPS tuning).  
- **ja/ko/ru:** limited directly relevant indexed results in this run; no contradictory signals found. Guidance aligns with language-agnostic upstream behavior (RFC 6455, Axum/Tokio, PostgreSQL LISTEN/NOTIFY semantics, Redis pubsub/streams) and the cross-language consensus above. **NV** for language-specific operational nuances not surfaced.