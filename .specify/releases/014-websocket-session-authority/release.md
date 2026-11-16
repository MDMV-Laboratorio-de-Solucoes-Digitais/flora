# Release: 014 — WebSocket Session Authority & Protocol Alignment (80/20)

**Target:** `dev` (PR to merge)  
**Branch:** `feat/014-websocket-session-authority`  
**Date:** 2026-11-16

## Summary
Delivers slice A for cycle 014:
- DB-backed WebSocket handshake session authority (parity with HTTP `jti`/session validation) with generic failures and nil-org rejection
- WS path/transport alignment (`/api/v1/ws`, Bearer header preferred + `?token=` fallback) and safe minimal tagged-envelope client handling
- Frontend lifecycle/reconnect hardening (no `/messages/sync` dependency, no blind retry on auth failure, auth/org-aware connect/disconnect)
- DB-gated websocket handshake/session contract tests and CI `db-tests` coverage

## Evidence
See `.specify/validations/014-websocket-session-authority/validate.md`.

## Risks / NV
- Remote CI pending; DB evidence on disposable PG18 alt port locally.
- Handshake-only revocation window (no periodic in-connection revalidation). Fanout/sync/realtime delivery and broader operability remain deferred (015+ candidates).

## Merge plan
1. Push branch, open PR to `dev`, wait for CI green (incl. `db-tests` with websocket/auth/messaging).
2. Merge (non-blind), fast-forward local `dev`, push `origin/dev`, delete feature branch, keep only `dev`/`main`.
3. Mark T014 complete and update OBJETIVO/ROADMAP with next cycle definition.
