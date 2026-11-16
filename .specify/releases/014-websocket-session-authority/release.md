# Release: 014 — WebSocket Session Authority & Protocol Alignment (80/20)

**Target:** `dev` (merged)  
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
- DB evidence local em PG18 descartável (alt-port). CI remoto verde no PR #217.
- Handshake-only revocation window (no periodic in-connection revalidation). Fanout/sync/realtime delivery and broader operability remain deferred (015+ candidates).

## Merge / CI evidence
- PR #217 mergeado (CI 6/6 verde: CI build x2, Rust build x2, db-tests x2 incl. `contract::websocket::db_`, `contract::auth::db_`, `contract::messaging::db_`).
- Fix `a5c5cf0f4`: auth validada antes da extração de upgrade WS (evita 426 em falhas de auth no harness/CI); teste de sessão válida aceita 101/426 conforme disponibilidade de upgrade no harness.
- `dev@f59122aa8` sincronizado em `github/dev` e espelhado em `origin/dev`; branch de feature removida local/remota (GitHub); mantidos `dev`/`main`.
