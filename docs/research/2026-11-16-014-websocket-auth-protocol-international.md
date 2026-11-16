# 014 — WebSocket auth & session authority (Axum/Tower/Rust, multilingual research)

**Date:** 2026-11-16  
**Repo:** `MDMV/projetos/Arquivados/flora` (dev after 013)  
**Scope:** Deep multilingual internet research (EN, pt-BR, es, de, fr, zh-CN, ja, ko requested) on WebSocket authentication and session authority for Axum/Tower/Rust services: DB-backed token/session validation at handshake and/or during connection lifetime, revocation/expiry handling for long-lived connections, close codes, tenant isolation, protocol alignment with browser clients, and cheap-VPS performance implications. Cross-checked with current implementation and `docs/research/2026-11-16-013-websocket-session-authority.md`. **No application source modified.**

## 1. Cross-check: current implementation vs 013

| Area | Current state (evidence) | 013 finding | Delta / confirmation |
|---|---|---|---|
| Route & upgrade | `GET /ws` in `flora-api/src/routes/mod.rs:47`; `websocket_handler` does auth before upgrade (`routes/websocket.rs:100-119`). | Same. | Confirmed. |
| WS auth mechanism | `extract_org_context` (`websocket.rs:122-140`) reads `Authorization: Bearer` only, `decode_token` only, parses `sub/org`, rejects nil org. **No** `SessionService::validate_session(jti)`, no `is_active/expires_at/user/org` DB checks. | Handshake-only JWT, no DB authority. | Confirmed gap remains. |
| HTTP auth baseline | `extractors/auth.rs::authenticate_bearer` (`:33-116`) enforces DB-backed `jti` lookup, active, not expired, user/org match; generic 401 `"Invalid or expired session"`; DB error → 500 `"Authentication service unavailable"`. | Reuse this policy. | Confirmed; helper extraction still needed. |
| Session store | `SessionService::validate_session/revoke_*` (`flora-core/src/services/session_service.rs`), `PgSessionRepository::find_by_jti` (`flora-core/src/repositories/session_repository.rs`). Active+expiry filtering exists at query level for `find_active_by_user_id` but `find_by_jti` returns row; extractor enforces authority checks. | DB authority available. | Confirmed. |
| Post-upgrade protocol | Tagged `WsClientMessage`/`WsServerMessage` (`Subscribe`, `Unsubscribe`, `SendMessage`, `Ping` / `Subscribed`, `Unsubscribed`, `NewMessage`, `MessageSent`, `Pong`, `Error{code,message}`) over text JSON (`websocket.rs:22-94,294-325`). `Subscribe/Unsubscribe` ack-only. | Same. | Confirmed. |
| Tenant isolation | `SendMessage` enforces channel-org ownership via `PgChannelRepository::find_by_id` (`:174-219`). No per-connection subscription registry/broadcast fanout in handler. | Must extend to future subscribe/fanout. | Confirmed. |
| Revocation/expiry in lifetime | No periodic revalidation, no jti-bound recheck, no session-invalid close path. | Handshake-only residual window unless added. | Confirmed. |
| Frontend | `apps/frontend/src/lib/api/websocket.ts` connects to `/api/v1/ws` with `?token=` (`:23-27`), flat `{type,id}` handling, expects `workspace.user.removed`, calls missing `/api/v1/messages/sync` (`:78-83`), blind reconnect, no send/subscribe/ping protocol. `+layout.svelte` toggles connect/disconnect on auth but not on org/token rotation. | Blocking protocol/path drift. | Confirmed. |

## 2. International, source-backed findings

### 2.1 Protocol & browser constraints (RFC/MDN, multilingual)

- **Handshake is HTTP Upgrade; auth happens at handshake boundary.** RFC 6455 defines opening handshake with client headers including optional cookies/origin context and server 101 response; after upgrade, application protocol/authz is application-defined over frames. (RFC 6455, IETF, Dec 2011, §§1.3, 4.1, 10: https://datatracker.ietf.org/doc/html/rfc6455)
- **Browser clients cannot set arbitrary WebSocket HTTP headers (e.g. `Authorization`) in `new WebSocket(...)`.** This forces token transport choices: cookie (if applicable), `Sec-WebSocket-Protocol` negotiation, query param (with leakage caveats), or an initial auth message after connect-before-trust. MDN server/client guidance is consistent across locales reviewed (EN/pt-BR/es/de/fr/zh-CN/ja pages on WebSocket server writing/API): handshake is HTTP GET+Upgrade, `Origin` is sent by browsers and useful for checks, and application auth is a server responsibility.  
  Sources: MDN EN https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers ; pt-BR https://developer.mozilla.org/pt-BR/docs/Web/API/WebSockets_API/Writing_WebSocket_servers ; es https://developer.mozilla.org/es/docs/Web/API/WebSockets_API/Writing_WebSocket_servers ; de https://developer.mozilla.org/de/docs/Web/API/WebSockets_API/Writing_WebSocket_servers ; fr https://developer.mozilla.org/fr/docs/Web/API/WebSockets_API/Writing_WebSocket_servers ; zh-CN https://developer.mozilla.org/zh-CN/docs/Web/API/WebSockets_API/Writing_WebSocket_servers ; ja https://developer.mozilla.org/ja/docs/Web/API/WebSockets_API/Writing_WebSocket_servers
- **ko (requested):** MDN ko page not fetched in this run; RFC 6455 and OWASP guidance remain language-neutral authoritative baselines for Korean teams as well. Treat transport/auth semantics as RFC/OWASP-driven, not locale-specific.

### 2.2 Security, session authority, revocation (OWASP)

OWASP WebSocket Security Cheat Sheet (https://cheatsheetseries.owasp.org/cheatsheets/WebSocket_Security_Cheat_Sheet.html) recommends:
- **Always `wss://`** in production; validate **`Origin`** (allowlist) to mitigate Cross-Site WebSocket Hijacking (CSWSH); browsers send cookies on handshake so cookie-based auth needs CSWSH defenses (SameSite, CSRF tokens where applicable).
- **No built-in WS auth:** validate authN/authZ at handshake and **authorize each action/message** (don’t assume connection == unlimited access). Close on session expiry/logout and re-validate periodically for long-lived connections.
- **Token transport:** for browser token clients prefer sending token in first message over WSS (avoid URL query tokens leaking to logs); accept only auth message until validated, close on auth failure/timeout, limit unauthenticated connections.
- **Token/session refresh & revocation:** rotate tokens for long-lived connections; close all user sockets on logout; maintain session→connection mapping; re-validate sessions periodically (example cadence given: ~30 min) and on sensitive actions.
- **DoS/resource controls:** per-user/per-IP connection limits, idle timeouts, ping/pong heartbeats, message size/rate limits, backpressure awareness (cheap-VPS relevant).
- **Close/error hygiene:** close with appropriate control frames/status (e.g. policy violation semantics like 1008 for rejected/expired auth in app flows) and avoid leaking sensitive details in logs/messages.

### 2.3 Close codes & lifetime semantics (RFC 6455)

- **1008 Policy Violation** and **1001 Going Away** / **1000 Normal Closure** / **1011 Internal Error** are standard close reasons in RFC 6455 §7.4.1; app-specific auth/session failures should map to policy-appropriate closes (commonly 1008) and/or an application error envelope before close.
- Ping/pong control frames (§5.5.2/5.5.3) support liveness/heartbeats; idle/dead connection cleanup is important for long-lived VPS deployments.

### 2.4 Axum/Tower/Rust implementation notes

- Axum `extract::ws` provides `WebSocketUpgrade` and `on_failed_upgrade` hook; handshake/upgrade is an HTTP extraction/response path where `State`, headers, and custom extractors can run before upgrade (Axum docs: https://docs.rs/axum/latest/axum/extract/ws/index.html). This fits **handshake-time DB-backed validation** using existing `AppState` services.
- Tower middleware/layers apply around the HTTP route; rate limiting/backpressure/timeout layers are appropriate pre-upgrade controls (cheap VPS).
- Rust/async: avoid blocking DB/Valkey in socket tasks; bound per-connection work, add jitter to periodic checks, and fail closed on auth-store unavailability (consistent with `authenticate_bearer` → 500).

## 3. Source-backed recommendations (Flora)

1. **Single session-authority core:** extract a reusable verifier (e.g. `verify_bearer_token(token, &AppState) -> Result<VerifiedIdentity{user_id,org_id,jti,claims}, AuthRejection>`) from `authenticate_bearer` and reuse for HTTP extractors and WS. Keep identical checks: JWT sig/claims, `session_service.validate_session(jti)`, `is_active`, `expires_at > Utc::now()`, `session.user_id == claims.sub`, `session.organization_id == claims.org`, generic auth errors, DB outage → 500/fail-closed. (OWASP: server-side validation; parity prevents drift.)
2. **Handshake must be DB-backed:** reject revoked/expired/missing/mismatched sessions at connect time (closes 012/013 gap). (OWASP session expiration/logout handling; prior 012/013.)
3. **Browser-compatible token transport, defense in depth:**
   - Accept `Authorization: Bearer` when present (SSR/proxies/tools).
   - For browsers, prefer **`Sec-WebSocket-Protocol`-based token carriage** or **first authenticated message** pattern so tokens avoid URL logs; if supporting `?token=`/`access_token` for compatibility, treat as fallback, never log, document leakage risk, and plan deprecation. (OWASP: prefer first message over WSS vs URL; browser header limitation.)
   - Add `/api/v1/ws` alias or move frontend to `/ws` (explicit alignment) and enforce `Origin` allowlist per deployment config.
4. **Bind and propagate verified identity:** pass `user_id, org_id, jti, session_id` into socket task; never trust client-supplied tenant IDs. Maintain `jti/user -> connection` mapping if/when server-side invalidation push is implemented.
5. **Revocation/expiry for long-lived sockets (VPS-aware):**
   - **Baseline (MVP, cheap):** handshake-only DB check + bounded token/session TTL + client reconnect on auth close/expiry; document residual revocation window (= until reconnect/expiry). 
   - **Hardened (recommended):** jittered periodic revalidation (e.g. 60–300s, configurable) of `jti` via `validate_session` + recheck on privileged ops (`SendMessage`, real `Subscribe`); on invalid → send generic `Error{code:"SESSION_INVALID"}` then close **1008**. (OWASP periodic revalidation; RFC 6455 1008.)
   - **Immediate revocation (later):** Valkey/pubsub signal keyed by `jti/user/org` to force-close; requires reliable mapping and fanout semantics.
6. **Tenant isolation invariants:** enforce org/channel ownership on every stateful operation (including future real subscriptions/fanout), key pubsub/registries by `org_id`, and decide nil-org policy explicitly (deny WS or restrict to non-tenant ops) consistent with `require_org` semantics.
7. **Protocol contract:** converge frontend/backend on one envelope and path; define explicit error/close taxonomy for `AUTH_REQUIRED`, `SESSION_INVALID`, `FORBIDDEN`, `PARSE_ERROR`; distinguish non-retryable auth/policy closes from transient reconnectable failures.
8. **Operational controls:** `wss://` only behind TLS termination/proxy correctly configured for Upgrade; connection/message limits, idle timeout, heartbeat/ping strategy, and observability without PII/token leakage.

## 4. Concrete Rust/Axum design options for Flora

### Option A — Minimal parity (recommended 014 implementation slice)
- **Change shape:** refactor `flora-api/src/extractors/auth.rs` to expose token-based verifier reused by `UserContext/OrgContext/AuthExtractor` and `routes/websocket.rs`.
- **WS flow:** resolve token from `Authorization` or negotiated browser mechanism (compat: query param) → `verify_bearer_token` → `401/500` HTTP rejection on failure → `on_upgrade(|socket| handle_socket(socket, state, VerifiedIdentity{...}))`.
- **Lifetime:** no periodic DB loop; rely on TTL + reconnect. Emit/handle deterministic auth close on reconnect-required conditions if detected.
- **Pros:** smallest diff, one DB read/connection, matches HTTP authority, low VPS cost. **Cons:** revocation/expiry latency until reconnect.

### Option B — Handshake + periodic/privileged revalidation
- Option A + store `jti` in connection state; spawn/select! timer (jittered) and/or revalidate before `SendMessage`/future subscribe side-effects.
- On failure: `send_ws_response(Error{code:"SESSION_INVALID", message:"Invalid or expired session"})` then `socket.close()` with 1008-equivalent application close (axum/tungstenite close frame support as needed).
- **Pros:** bounded revocation window, stronger for 013+ hardening. **Cons:** +N reads/interval/conn, needs careful cancellation/backoff to avoid thundering herd on VPS.

### Option C — Push revocation/invalidation bus (follow-up)
- Subscribe connection tasks to Valkey channel(s) for `session.revoked:{jti|user_id|org_id}` and force-close; combine with A/B.
- **Pros:** near-immediate revocation, aligns with future realtime events. **Cons:** correctness/ordering, multi-node fanout, mapping lifecycle complexity.

### Token transport options (browser reality)

| Option | Security/ops | Browser support | Flora fit |
|---|---|---|---|
| `Authorization` header | Best (no URL leak) | Not settable by browser WS API | Keep for non-browser/proxy clients |
| `Sec-WebSocket-Protocol` carries token | Avoids query string; visible in protocol negotiation | Yes (subprotocol arg) | Strong browser option if server/proxies preserve it |
| `?token=` query param | Log/proxy/referer leakage risk | Yes (current frontend) | Compatibility fallback only; never log; migrate away (OWASP) |
| First WS auth message after upgrade | Token out of URL/handshake headers; requires unauth window control | Yes | Good with strict pre-auth gating/timeouts/limits |
| Cookie-based session on same-site | Browser-native; needs CSWSH defenses (Origin+SameSite+CSRF strategy) | Yes | Only if cookie auth architecture exists/desired |

### Suggested close/error mapping

| Condition | Pre-upgrade | Post-upgrade |
|---|---|---|
| Missing/invalid/revoked/expired/mismatched session | `401` (generic body) | `Error{SESSION_INVALID}` + close `1008` |
| DB/auth backend unavailable | `500` (generic) | `Error{AUTH_UNAVAILABLE}` or controlled close `1011` (fail closed) |
| Tenant/channel violation | n/a | `Error{FORBIDDEN}` (and optionally `1008` for repeated abuse) |
| Malformed payload | n/a | `Error{PARSE_ERROR}` (no close required) |
| Normal/logout/navigation | n/a | client close / server `1000` or `1001` as appropriate |

## 5. Migration/compat strategy for frontend protocol

1. **Path & transport alignment:** change `WsClient` to backend-supported path (`/ws` or add `/api/v1/ws` alias server-side) and to negotiated token transport (prefer subprotocol/first-message; keep temporary query-param compatibility behind a flag if needed).
2. **Envelope alignment:** parse tagged server messages `{type, payload}` (`Subscribed`, `Unsubscribed`, `NewMessage{...}`, `MessageSent`, `Pong`, `Error{code,message}`) instead of flat `{type,id}` assumptions; update stores/events accordingly.
3. **Reconnect semantics:** classify closes/errors—`SESSION_INVALID`/policy auth close => stop blind retry and route to re-auth/session invalidation flow; transient/network/`1001` => bounded backoff reconnect. Reconnect must use fresh token and re-establish org-bound subscriptions.
4. **Session/org lifecycle:** on logout, `lockSession`/invalidate, token refresh, org switch, or membership removal, disconnect and reconnect with fresh credentials; only trust server-confirmed invalidation events (`workspace.user.removed` or explicit auth-invalid event) once backend emits them.
5. **Feature gating / rollout:** ship behind a compatibility flag: Phase 1 connect+auth parity, Phase 2 send/subscribe protocol, Phase 3 sync/fanout semantics. Keep REST/message APIs as source of truth until `/api/v1/messages/sync` (or equivalent recovery contract) is specified/implemented.
6. **Rollback:** retain ability to disable WS realtime client-side without breaking core REST flows.

## 6. Deterministic test strategy

- **Unit (fast, no DB):** token extraction precedence/normalization; verifier mapping for each rejection; close/error code selection; serde round-trip fixtures for `WsClientMessage`/`WsServerMessage`; frontend parser/reconnect classifier (auth-policy vs transient).
- **Integration (disposable Postgres, seeded deterministic fixtures):** handshake accepts active valid session; rejects missing/revoked/expired/user-mismatch/org-mismatch/nil-org (as specified) with generic 401 parity to HTTP; DB failure path fails closed (500). Reuse extractor fixture style in `flora-api/src/extractors/auth.rs` with real session rows.
- **WS integration:** successful upgrade with verified identity; `SendMessage` forbidden on foreign-org channel and allowed/persisted/published on same-org channel (existing behavior preserved); parse error returns `PARSE_ERROR`.
- **Lifetime/revocation (if Option B):** injectable clock/short interval + jitter control; revoke/invalidate session after connect and assert `SESSION_INVALID` + close 1008 within bounded deterministic window; recheck on privileged action path.
- **Contract/compat:** golden protocol fixtures shared between Rust tests and frontend TS tests; path/transport selection tests; no token in logs/assertions.
- **Non-functional (targeted):** connection/message limits and heartbeat/idle timeout behavior under constrained VPS-like settings; avoid flaky timing by deterministic schedulers/timeouts.

## 7. NVs (non-goals / validation notes)

- **NV-1 (out of scope now):** full realtime fanout correctness, subscription registry, delivery guarantees, presence/typing/read receipts, multi-node affinity, and missing REST sync endpoint implementation.
- **NV-2 (risk):** query-param token support is a compatibility risk (access logs/proxies/referrers) — acceptable only as temporary, documented fallback.
- **NV-3 (risk):** handshake-only validation leaves a revocation window until TTL/reconnect; acceptable only with explicit product/security sign-off.
- **NV-4 (validation):** any chosen design must be validated against `cargo test -p flora-api`/workspace gates and frontend unit/server checks in CI context when implemented; this research deliverable performs no runtime certification.
- **NV-5 (sources/currency):** recommendations grounded in RFC 6455 (2011, stable), OWASP WebSocket Security Cheat Sheet (current page content reviewed 2026-11-16), Axum 0.8.9 ws docs, MDN multilingual pages (EN/pt-BR/es/de/fr/zh-CN/ja reviewed; ko not fetched but covered by RFC/OWASP), and in-repo 013/current code cross-check.

## Source index
- RFC 6455: The WebSocket Protocol — https://datatracker.ietf.org/doc/html/rfc6455
- OWASP Cheat Sheet Series: WebSocket Security — https://cheatsheetseries.owasp.org/cheatsheets/WebSocket_Security_Cheat_Sheet.html
- Axum 0.8.9 WebSocket extractor docs — https://docs.rs/axum/latest/axum/extract/ws/index.html
- MDN: Writing WebSocket servers — EN https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers, pt-BR https://developer.mozilla.org/pt-BR/docs/Web/API/WebSockets_API/Writing_WebSocket_servers, es https://developer.mozilla.org/es/docs/Web/API/WebSockets_API/Writing_WebSocket_servers, de https://developer.mozilla.org/de/docs/Web/API/WebSockets_API/Writing_WebSocket_servers, fr https://developer.mozilla.org/fr/docs/Web/API/WebSockets_API/Writing_WebSocket_servers, zh-CN https://developer.mozilla.org/zh-CN/docs/Web/API/WebSockets_API/Writing_WebSocket_servers, ja https://developer.mozilla.org/ja/docs/Web/API/WebSockets_API/Writing_WebSocket_servers
- Repo: `flora-api/src/routes/websocket.rs`, `flora-api/src/extractors/auth.rs`, `flora-api/src/routes/mod.rs`, `flora-core/src/services/session_service.rs`, `flora-core/src/repositories/session_repository.rs`, `apps/frontend/src/lib/api/websocket.ts`, `apps/frontend/src/routes/+layout.svelte`, `docs/research/2026-11-16-013-websocket-session-authority.md`
