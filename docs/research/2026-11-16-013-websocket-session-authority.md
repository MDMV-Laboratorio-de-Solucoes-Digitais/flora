# WebSocket `/ws` authentication & session authority — cycle 013+ research

**Date:** 2026-11-16  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (dev, post PR #215)  
**Scope:** audit + minimal design for DB-backed session authority on the realtime upgrade path. No source changes.

## 1. Current protocol, path, and auth behavior

| Surface | Behavior (evidence) |
|---|---|
| Route/path | `GET /ws` registered in `flora-api/src/routes/mod.rs:47` via `websocket::websocket_handler`. Rate-limit + CORS + shared `AppState` layers apply (`mod.rs:49-57`). |
| Upgrade/auth | `websocket_handler` extracts `(user_id, org_id)` **before** upgrade (`websocket.rs:100-119`). `extract_org_context` (`:122-140`) reads `Authorization: Bearer <jwt>` only, decodes with `flora_core::utils::jwt::decode_token`, parses `claims.sub`/`claims.organization_id`, rejects nil org as `OrganizationContextRequired`. No DB/session lookup, no `jti` check, no expiry/active check beyond JWT decode. |
| Failure mode | Any auth/context failure returns bare `401` with empty body (`:110-114`); no WS close frame (HTTP rejection). |
| Post-upgrade protocol | Client→server tagged enum `WsClientMessage` (`Subscribe`, `Unsubscribe`, `SendMessage{channel_id,content,thread_id}`, `Ping`) over text JSON (`:22-47,294-325`). Server→client `WsServerMessage` (`Subscribed`, `Unsubscribed`, `NewMessage{channel_id,message_id,sender_id,content,thread_id,created_at}`, `MessageSent`, `Pong`, `Error{code,message}`) (`:49-94`). Non-text frames ignored; parse errors emit `Error/PARSE_ERROR` (`:269-277`). |
| Tenant enforcement | `SendMessage` validates channel ownership via `PgChannelRepository::find_by_id` and requires `channel.organization_id == org_id` (`:174-219`); persists to Postgres, best-effort Valkey publish to `org:{org_id}:channel:{channel_id}` (`:223-250`). `Subscribe/Unsubscribe` are acknowledgements only (no server-side subscription registry/broadcast loop shown). |
| Session authority (HTTP baseline) | `flora-api/src/extractors/auth.rs::authenticate_bearer` (`:33-115`) enforces: valid JWT sig/claims, `session_service.validate_session(&claims.jti)` DB lookup, `is_active`, `expires_at > Utc::now()`, user/org match (incl. nil-org pre-selection). Generic 401 `"Invalid or expired session"`; DB failure → 500 `"Authentication service unavailable"`; tracing only for details. |

## 2. Frontend mismatches (blocking for 013)

| Frontend (`apps/frontend/src/lib/api/websocket.ts`, `+layout.svelte`, contract audit 012) | Backend reality | Impact |
|---|---|---|
| Connects to `new URL('/api/v1/ws', API_BASE_URL)` and passes token as `?token=` query param (`ws:23-27`). | Route is `/ws` (`routes/mod.rs:47`), auth reads `Authorization` header only (`websocket.rs:125-132`). Browsers cannot set custom WS headers. | Connection will 401/miss route in real browser usage. |
| Expects events like `workspace.user.removed` and flat `{type,id}` (`ws:89-96`, spec `002` T023). | Server emits tagged `{"type":"NewMessage","payload":{...}}` etc.; no `workspace.user.removed` emission path in audited WS handler. | Session invalidation/realtime UX cannot work as coded. |
| `syncMissedMessages()` calls `/api/v1/messages/sync?last_known_message_id=...` (`ws:78-83`). | No matching handler found in `flora-api`. | Reconnect recovery is dead-end. |
| No subscribe/send/ping client protocol, no handling of `Error`/close semantics; `scheduleReconnect` retries blindly on close/error. | Server requires explicit `Subscribe`/`SendMessage` messages and can return structured errors. | Protocol drift; no auth-failure differentiation from transient disconnect. |
| `+layout.svelte` connects on `authState.isAuthenticated` and disconnects on logout/invalidated, but never re-authenticates on org switch/token refresh. | Org context is bound at handshake (`user_id, org_id` captured for socket lifetime, `:119,281`). | Stale org scope across org changes unless socket is cycled with new token. |

## 3. Minimal DB-backed session validation design

**Goal:** make WS upgrade enforce the same session authority as HTTP without duplicating policy.

- **Extract shared helper (preferred, minimal-risk):** refactor `authenticate_bearer(parts: &Parts, state: &AppState)` in `flora-api/src/extractors/auth.rs` into a reusable core, e.g. `verify_bearer_token(token: &str, state: &AppState) -> Result<VerifiedSession, (StatusCode, &'static str)>` (or keep existing fn and add a thin token-based wrapper). HTTP extractors continue to read header → call core; WS reads token → calls same core.
- **WS token acquisition (explicit, browser-compatible):** accept token from (a) `Authorization: Bearer` header when present (non-browser/SSR/proxy cases) else (b) `Sec-WebSocket-Protocol` negotiation or (c) a query param **only if** documented as fallback; prefer subprotocol/first-message-less patterns carefully. *Minimal*: support `?token=` and/or `access_token` **in addition** to header, but treat all sources identically and never log token values. Frontend must be aligned to `/ws` (or add `/api/v1/ws` alias) and chosen transport.
- **Handshake flow:** `websocket_handler` → resolve raw token → `verify_bearer_token` → on `Err((status,_))` return HTTP rejection (current) with generic body; on `Ok(v)` upgrade and pass `v.user_id`, `v.organization_id`, and `v.claims.jti` (needed for revocation/reauth hooks) into `handle_socket`.
- **Reuse, not reimplement:** keep JWT decode, `SessionService::validate_session(jti)`, active/expiry/user/org checks, and error mapping identical to HTTP (`extractors/auth.rs:56-109`). This guarantees nil-org parity and future policy changes apply once.
- **State dependency:** `AppState` already carries `session_service` (`extractors tests`/state usage); no new infra required.

## 4. Revocation/expiry semantics for long-lived connections

- **At connect:** DB-backed check is mandatory (closes 012 gap noted in `docs/research/2026-11-16-012-session-authority-international.md:57`).
- **During lifetime (design choice):**
  - *Minimal/VPS-safe default:* **no continuous revalidation loop**; rely on bounded JWT/session TTL and require clients to reconnect on close/401-equivalent signals. Revocation takes effect at next handshake (and on any operation that independently rechecks, if added later).
  - *Stronger (recommended for 013+ hardening):* periodic revalidation (e.g. every 60–300s, jittered) calling `session_service.validate_session(jti)`; on missing/inactive/expired/mismatch → send `Error{code:"SESSION_INVALID",...}` (generic) then close with policy/auth close code. Also revalidate on privileged actions (`SendMessage`, subscribe to new channel) to cap abuse window.
  - *Immediate revocation push (out of minimal):* subscribe server-side to a revocation/pubsub signal keyed by `jti/user/org` (Valkey already in stack) to force-close; higher complexity.
- **Expiry:** honor `session.expires_at` (DB) as authority; do not trust client clocks. If revalidation disabled, natural expiry = token TTL + reconnect behavior; document residual window.
- **Org switch / logout / membership removal:** must trigger client disconnect+reconnect with fresh token (and server-side force-close if push path implemented). `workspace.user.removed` requires a defined event source and mapping to session/org invalidation.

## 5. Close codes, errors, tenant isolation

- **HTTP upgrade rejection (pre-upgrade):** keep `401 Unauthorized` (and `500` only if auth backend unavailable, mirroring HTTP extractor semantics) with empty/generic body; avoids leaking session state. Optionally `400` for malformed token transport (generic).
- **Post-upgrade close codes (RFC 6455 / Axum ws):**
  - `1008` Policy Violation — authz/tenant violations, revoked/expired session detected mid-stream.
  - `1001` Going Away / `1000` Normal — server shutdown or client-initiated.
  - `1011` Internal Error — unexpected server failure (rare; prefer app-level `Error` + controlled close).
  - Avoid custom codes; convey app detail via `WsServerMessage::Error{code,message}` before close when possible, using **generic messages** for auth failures (`"Invalid or expired session"` / `"SESSION_INVALID"`).
- **Tenant isolation invariants:**
  - Bind authoritative `user_id, org_id, jti` at handshake from verified session (never trust client-supplied org/channel claims).
  - Keep existing `validate_channel_ownership` on `SendMessage` (`websocket.rs:205-219`); extend to `Subscribe` before any real fanout is implemented (current subscribe is a no-op ack but future-proofing matters).
  - Any broadcast/subscription registry must key by `org_id` and enforce channel-org match; never cross-org deliver even on bugs/mis-subscribe.
  - Nil-org sessions: decide explicitly—either deny WS upgrade with org-context-required semantics or allow only non-tenant-scoped ops; align with HTTP `AuthExtractor::require_org` behavior and product needs.

## 6. Performance & security tradeoffs (VPS)

| Option | Cost | Revocation latency | Notes |
|---|---|---|---|
| Handshake-only DB check (minimal) | 1 indexed `jti` read/connection | Until reconnect/expiry | Best for low churn, short TTLs; simplest. |
| Handshake + periodic recheck | +1 read/interval/conn | <= interval | Good balance; add jitter, timeout, and cheap `find_by_jti` path/index. |
| Handshake + per-message recheck | +1 read/message | Near-immediate | Highest DB load; reserve for sensitive actions. |
| Valkey revocation push | pubsub + state | Immediate | Operational complexity; fits existing Valkey but needs correctness guarantees. |

Security: never accept unsigned claims; never put tokens in logs/metrics; prefer header/subprotocol over query params when feasible (query strings leak via access logs/proxies) but support browser constraints deliberately; rate-limit upgrades/connections; bound message size and per-conn send rate (partly outside audited code). DB unavailability on handshake should **fail closed** (500) consistent with HTTP extractor.

## 7. Deterministic test strategy

- **Unit (no DB):** token transport parsing precedence; rejection on missing/malformed bearer; close/error mapping helpers; message (de)serialization compatibility (`Subscribe`/`NewMessage` envelope) and protocol-version fixtures.
- **Integration (disposable Postgres, deterministic seed/jti):** mirror `authenticate_bearer` coverage for WS handshake — valid session accepted; revoked (`is_active=false`), expired (`expires_at<=now`), missing row, user mismatch, org mismatch all rejected with generic 401; nil-org session parity case explicit. (Reuse patterns from extractor tests in `flora-api/src/extractors/auth.rs:351-395` but with real DB-backed sessions.)
- **Integration (WS):** upgrade success yields usable socket; `SendMessage` to foreign-org channel returns `Error/FORBIDDEN`; same-org channel persists/publishes as today. If periodic revalidation added, simulate revocation after connect and assert `SESSION_INVALID` + close `1008` within bounded time (fake clock/jitter control or injectable interval).
- **Frontend/unit contract:** `WsClient` connects to correct path, attaches token via chosen mechanism, handles tagged payloads, distinguishes auth-policy close from retryable close, and clears/retries per state (`lockSession`/`invalidate` alignment).
- **Smoke/manual (env-dependent):** real Valkey + multi-client fanout once broadcast path exists; `scripts/dev-smoke-session.sql` style deterministic fixtures.

## 8. Out-of-scope (013+ follow-ups unless explicitly pulled in)

- Full realtime fanout implementation correctness (current `Subscribe` is ack-only; no per-connection channel registry/delivery loop evidenced in handler).
- Implementing missing `/api/v1/messages/sync` REST fallback and aligning event taxonomy (`workspace.user.removed`, `NewMessage` envelope/id fields) beyond defining contract requirements.
- OIDC/JWKS end-to-end callback tests (noted in 012).
- Distributed revocation bus, presence/typing/read receipts, message edit/delete realtime semantics, horizontal multi-node WS affinity, compression, and load/perf benchmarking.
- Refresh-token rotation/transparent reauth over an already-open socket (requires product + protocol decisions).

## 9. Recommended 013 execution order

1. Shared `verify_bearer_token` helper + WS handshake DB validation (parity with HTTP).  
2. Align path/token transport and minimal frontend contract for connect/auth failure.  
3. Add deterministic handshake authz tests (DB) + channel-tenant WS tests.  
4. Decide/document revocation window (handshake-only vs periodic) and implement chosen close/error semantics.  
5. Defer fanout/sync/event-schema expansion to follow-up slices with explicit specs.