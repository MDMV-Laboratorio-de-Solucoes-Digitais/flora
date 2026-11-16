# Feature Specification: 014 — WebSocket Session Authority & Protocol Alignment (80/20)

**Feature Branch**: `feat/014-websocket-session-authority`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/014-next-vertical-slice/{intake,research,problem,concept,decision}.md` (verdict: go, slice A)

## Goal
Enforce DB-backed session authority on `/ws` with parity to HTTP extractors, and align WS path/token transport and minimal frontend↔backend protocol/close behavior for safe, tenant-correct authenticated realtime foundation on Seed/VPS.

## Scope
- Shared bearer/session verification helper reused by HTTP extractors and WS handshake (JWT decode + `validate_session(jti)` + active/expiry/user/org checks; generic 401/500).
- WS handshake accepts token from supported transports (Authorization Bearer and browser fallback `?token=`), validates before upgrade, rejects nil-org as org-context-required for this slice.
- Route/path alignment for frontend (`/api/v1/ws`, keep `/ws` alias if low-risk).
- Minimal tagged envelope handling compatibility (`Subscribed/Unsubscribed/NewMessage/MessageSent/Pong/Error`) and safe client behavior (no broken `/messages/sync` dependency, no blind reconnect on auth/policy failures, bounded reconnect with jitter for retryable disconnects, disconnect/reconnect on auth/org changes).
- DB-gated WS/session contract tests (valid/revoked/expired/missing/mismatch/nil-org/cross-org channel guard where applicable) and CI `db-tests` wiring.
- Deterministic Rust/frontend verification and SDD evidence.

## Non-goals
- Full fanout/subscription registry, multi-client broadcast guarantees, cross-node delivery, Valkey/NOTIFY-driven push.
- `/api/v1/messages/sync`, history replay/catch-up, presence/typing/read receipts expansion.
- Attachments/binaries, DMs UX, search/notifications wiring.
- OIDC/JWKS e2e, browser Playwright e2e, `apps/web`, deploy/merge/release/secrets.

## NV
- Live browser e2e, real OIDC/JWKS, realtime multi-client delivery beyond implemented scope, default-port smoke conflicts.
