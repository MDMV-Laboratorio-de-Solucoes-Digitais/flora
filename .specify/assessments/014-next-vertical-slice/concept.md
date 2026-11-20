# Concept: 014 — WS session authority + protocol alignment (scoped)

## Chosen approach (80/20, Postgres-first, VPS-cheap)
Unify realtime authentication with HTTP’s DB-backed session authority and make the WS contract minimally interoperable for the browser client, without introducing new mandatory services or full fanout infrastructure.

## 1) Shared session verification (backend core)
- Extract/reuse a token-based verification helper from `flora-api/src/extractors/auth.rs` (e.g. `verify_bearer_token` / `VerifiedSession { user_id, organization_id, jti, claims }`) that performs: JWT decode/claim validation, `SessionService::validate_session(jti)` DB lookup, `is_active`, `expires_at > now`, `user_id`/`organization_id` match (including nil-org parity).
- Keep **one policy**: generic 401 on auth failure, 500 on auth-backend/DB unavailability (fail closed), no sensitive detail leakage/logging.
- `websocket_handler` resolves token, calls shared verifier **before** `ws.on_upgrade`, and passes verified identity (and `jti`) into `handle_socket`.

## 2) Token transport and endpoint alignment
- **Canonical path:** decide and align `/ws` vs `/api/v1/ws` (prefer aliasing/route parity so FE and docs agree; minimal server change).
- **Transport (browser-compatible, explicit):** accept `Authorization: Bearer` when present **and** a documented fallback for browsers (query `token=`/`access_token` or `Sec-WebSocket-Protocol` bearer negotiation) — all parsed uniformly, never logged. Document tradeoff (URL/query leakage risk) and prefer short-lived tokens; consider cookie path only if existing session cookie infra supports it (NV if not present). Frontend must use the chosen mechanism consistently.
- Reject missing/malformed transport generically (401 pre-upgrade).

## 3) Minimal protocol/behavior alignment (FE↔BE)
- Align `WsClient` with backend tagged envelope (`{"type":"...","payload":...}`) for at least: `Pong`, `Error{code,message}`, `MessageSent`/`NewMessage` shapes as emitted today (or define a minimal, versioned mapping). Remove/guard assumptions about unsupported events (`workspace.user.removed`) and missing REST sync (`/messages/sync`) — replace with safe no-op or explicit NV handling.
- **Auth-failure behavior:** distinguish policy/unauthorized close (or repeated 401 on connect) from transient network failure; on auth invalidation trigger `authState.invalidate()/lockSession()` flow as appropriate and **stop blind reconnect loops** with bad credentials. Bounded backoff/jitter for retryable cases.
- **Org/token lifecycle:** on org switch, token refresh, or logout, disconnect and reconnect with fresh verified context (socket binds org at handshake). Document nil-org session behavior (deny WS or restrict) consistent with HTTP.

## 4) Revocation/expiry semantics (explicit, minimal)
- **014 minimum:** handshake-time DB authority (closes primary gap). Revocation/expiry takes effect on next connect/handshake and on any action that rechecks (optional).
- **Recommended bounded hardening (include if low-risk):** periodic revalidation of `jti` (configurable interval with jitter) and/or recheck on `SendMessage`/subscribe; on failure send generic `Error` (e.g. `SESSION_INVALID`) and close with `1008`. Record chosen window in decision/spec; if not implemented, mark NV with explicit residual risk.
- Do not trust client clocks; DB `expires_at`/`is_active` are authoritative.

## 5) Tenant isolation invariants
- Retain/require `validate_channel_ownership` for `SendMessage`; enforce same check before any real `Subscribe` side effects/fanout (future-proof, cheap).
- Never deliver/cross-route by mismatched org; any future registry keyed by org/channel.
- Keep `Subscribe` as ack-only in 014 unless trivially safe — **no fanout registry required**.

## 6) Deterministic verification (design)
- **Unit:** token extraction precedence/transport parsing, serialization/deserialization fixtures for WS envelopes, close/error mapping helpers, reconnect classification logic (pure/isolated).
- **DB-gated integration (Postgres 18 disposable, env-gated/ignored pattern):** handshake matrix — valid session accepted; revoked/inactive, expired, missing session, user mismatch, org mismatch rejected generically; nil-org case explicit. Reuse `scripts/dev-smoke-session.sql`-style deterministic fixtures.
- **WS integration:** upgrade with valid verified session succeeds; cross-org channel `SendMessage` returns `FORBIDDEN`; same-org persists as today. If periodic revalidation added, deterministic revocation-after-connect test with controlled interval.
- **Frontend unit:** correct URL/transport, envelope handling, auth-failure stops/locks appropriately, bounded reconnect behavior.
- **CI:** extend existing Rust `db-tests` job/pattern for new DB-gated WS/session cases (when implemented).

## 7) Minimal realtime delivery note (informs sequencing, B deferred)
- Correctness should rely on REST/persisted state; realtime is best-effort push. For small VPS, **Postgres table-as-truth + optional NOTIFY doorbell or simple polling/SSE** are viable follow-ups once auth/protocol are sound; avoid making Valkey mandatory. Full registry/fanout/sync/replay is **not** in 014.

## Alternatives considered
- **B first (fanout):** amplifies unauthenticated/misaligned protocol risks; larger stateful complexity on 4 GB VPS.
- **C first (operability):** improves runtime envelope but leaves P0 auth/contract gap open.
- **One-shot WS ticket service:** stronger URL-token hygiene but extra endpoint/state; defer unless needed.
- **Cookie-only WS auth:** clean for browsers if aligned with CORS/CSWS/Origin, but depends on current cookie/session issuance model — evaluate during implementation (NV here).

## VPS/debloat alignment
- No new services; reuse Postgres + existing session tables/services. Optional periodic checks must be low-frequency/jittered to limit DB load per connection. Keep implementation small (shared helper + narrow FE client changes) over introducing generic frameworks.