# Release: 013 — Channel & Message REST Slice (80/20)

**Target:** `dev` (merged)  
**Branch:** `feat/013-channel-message-rest-slice`  
**Date:** 2026-11-16

## Summary
Delivers the 013 REST-first workspace→channel→message slice on existing Postgres primitives:
- Channel listing by workspace and tenant-safe checks (`GET /api/v1/channels?workspace_id=...`)
- Message persistence with `thread_id` and path-tenancy enforcement on edit/delete
- Canonical frontend flow wired to real IDs/DTO mapping (list/create channels, post/list messages, threaded replies) without realtime dependency
- DB-gated messaging contract tests + CI coverage extension; ChannelType sqlx mapping fix for schema compatibility

## Evidence
See `.specify/validations/013-channel-message-rest-slice/validate.md` for commands and results.

## Risks / NV
- DB evidence local em PG18 descartável (porta alt 15434; 5432 ocupada externamente). CI remoto verde no PR #216.
- WS authority/protocol, realtime delivery, attachments, OIDC/browser e2e remain deferred (014+ candidates per R1 research).

## Merge / CI evidence
- PR #216 mergeado (CI 6/6 verde: CI build x2, Rust build x2, db-tests x2 incl. `contract::messaging::db_`).
- `dev@530d63fea` sincronizado em `github/dev` e espelhado em `origin/dev`; branch de feature removida local/remota (GitHub); mantidos `dev`/`main`.
