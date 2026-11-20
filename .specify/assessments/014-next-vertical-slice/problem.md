# Problem: 014 — WS session authority and protocol alignment gap

## Core problem
The realtime WebSocket path is not yet safe, authoritative, or client-usable for the Seed MVP: the server upgrades `/ws` using JWT decode only (no DB-backed session/JTI validation, no revocation/expiry enforcement against `sessions`), while the frontend connects to a different path with an incompatible token transport and message contract. As a result, revoked/expired/mismatched sessions can outlive HTTP authority on WS, browser clients cannot reliably authenticate/communicate, and tenant/session lifecycle events (org switch, logout, membership removal) lack defined realtime semantics.

## Why this is the 014 priority (80/20)
- **Security/tenancy (P0):** 012 established DB-backed HTTP session authority; leaving WS on signature-only auth creates an inconsistent trust boundary and a revocation window on the same org-scoped data plane.
- **Product unblock:** 013 made messages persist/read over REST, but a trustworthy realtime channel is the next step for a usable chat experience and for future fanout; protocol/path/token drift currently blocks that path entirely.
- **Feasibility:** a minimal fix can reuse existing `session_service`/extractor logic and existing WS persistence/ownership checks, avoiding new infra and limiting blast radius.
- **Sequencing:** operability tuning (C) and full fanout (B) are valuable but either cross-cutting or dependent on a correct authenticated protocol substrate.

## Evidence (verified)
- `flora-api/src/routes/websocket.rs`: `extract_org_context` reads `Authorization: Bearer` and decodes JWT only; no `validate_session(jti)`, no DB active/expiry/user/org checks; `401` empty body pre-upgrade.
- `apps/frontend/src/lib/api/websocket.ts`: connects to `/api/v1/ws?token=`, no `Subscribe/SendMessage` protocol, no `NewMessage` envelope handling, retries blindly, expects missing `/api/v1/messages/sync` and `workspace.user.removed`.
- `docs/research/2026-11-16-013-websocket-session-authority.md`: documents mismatch matrix, shared-helper design, revocation/close-code options, test strategy.
- HTTP baseline (012): DB-backed `jti` validation and generic 401 behavior already implemented for extractors — must be the single source of truth.

## Impact
- **Security:** revoked or expired sessions may remain usable on WS until token expiry/reconnect (policy undefined).
- **Correctness/UX:** frontend cannot interoperate with backend WS in real browser conditions; realtime features appear broken or misleading.
- **Tenant safety:** org-bound socket lifetime without revalidation/rebind on org switch risks stale scope; future fanout would amplify mistakes if not anchored to verified session.
- **Operability:** blind reconnect/sync loops and protocol errors add noise on small VPS deployments.

## Root causes
1. **Auth duplication/drift:** WS reimplements token parsing instead of calling shared DB-backed session verification.
2. **Transport mismatch:** browser-incompatible assumptions (header-only vs query/subprotocol/cookie realities) and wrong path.
3. **Protocol/schema drift:** tagged enum contract vs FE event model; missing lifecycle/error semantics.
4. **Undefined long-lived semantics:** no agreed revocation window, close codes, or rebind rules for org changes.

## Success criteria (014 target)
- WS handshake enforces the **same session authority as HTTP** (valid signature/claims + DB `jti` active, not expired, user/org consistent) via a shared/reused verification path.
- Path and token transport are aligned for supported clients (deterministic, documented), with generic pre-upgrade failures.
- Minimal FE↔BE protocol compatibility for connect/auth/error/close handling is established (and/or explicitly scoped), including safe behavior on auth failure (no infinite bad-token retry).
- Tenant isolation is preserved at handshake and on message actions; org-switch/token refresh causes socket rebind/disconnect per defined rule.
- Deterministic tests cover handshake accept/reject matrix and key tenant cases; revocation/expiry window is explicitly chosen and documented (implemented or NV with rationale).

## Non-goals (for this problem framing)
- Full broadcast/subscription registry, cross-node fanout, guaranteed delivery/sync history, presence/typing/receipts, rich event taxonomy beyond minimal alignment (see slice decision).