# Release: 013 — Channel & Message REST Slice (80/20)

**Target:** `dev` (PR to merge)  
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
- Remote CI pending; DB smoke used disposable PG18 alt port locally.
- WS authority/protocol, realtime delivery, attachments, OIDC/browser e2e remain deferred (014+ candidates per R1 research).

## Merge plan
1. Push branch, open PR to `dev`, wait for CI green (incl. db-tests with messaging).
2. Merge (non-blind), fast-forward local `dev`, push `origin/dev`, delete feature branch, keep only `dev`/`main`.
3. Mark T015 complete and update OBJETIVO/ROADMAP with next cycle definition.
