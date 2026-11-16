# Intake: 014 — Next vertical slice (Flora Seed MVP)

**Date:** 2026-11-16  
**Base:** `dev @530d63fea` (cycle 013 merged, PR #216, CI green)  
**Mode:** assessment/research only (no application source changes, no commits)

## Problem statement
Cycle 013 delivered a persisted REST `workspace→channel→message` slice, but the realtime path remains unsafe and contract-broken for Seed/P0: `/ws` authenticates with JWT decode only (no DB-backed `jti`/session validation, unlike HTTP extractors fixed in 012), the frontend targets `/api/v1/ws?token=` while backend serves `/ws` with `Authorization` header only, and WS protocol/event shapes (`Subscribe/SendMessage/NewMessage` tagged envelopes) diverge from `WsClient` expectations (`workspace.user.removed`, `/messages/sync`, no subscribe/send handling). Session revocation/expiry/tenant/org-switch semantics are undefined for long-lived sockets. ROADMAP P0.1/P0.2 and existing 013 research (`docs/research/2026-11-16-013-websocket-session-authority.md`) call for WS authority+alignment and minimal reliable realtime for persisted messages.

Seed/VPS operability is a parallel concern (Meilisearch/Valkey optionality, pool/config wiring, memory limits, PG18 posture) per `docs/research/2026-11-16-seed-vps-profile-audit.md`, but current boot/compose behavior and core chat usability/security risks make WS authority/alignment the tighter 80/20 blocker.

## Candidates evaluated (pick ONE primary)
| Option | Summary | 80/20 value | Feasibility / risk |
|---|---|---|---|
| **A. WS session authority + protocol alignment** | Reuse HTTP session verification (`jti` in Postgres, active/expiry/user/org) at WS handshake; align path/token transport and minimal FE/BE protocol/close semantics; deterministic tests | **Highest now.** Closes P0 security/tenancy gap on realtime path, unblocks trustworthy browser WS use, prerequisite for fanout work (B). Small, localized diff (extractor helper + `routes/websocket.rs` + `apps/frontend/src/lib/api/websocket.ts` + layout/tests). | Medium: browser token transport tradeoff, revocation window decision, FE contract changes; contained by reusing 012 logic and generic errors. |
| B. Minimal realtime delivery/fanout for persisted messages | Subscription registry/broadcast loop, Valkey/Postgres-NOTIFY/SSE/polling choice, sync/replay, event taxonomy | High product value but depends on A (authz) and stable protocol; larger design/runtime surface for 2vCPU/4GB VPS | Higher: stateful connection fanout, backpressure, missing `/messages/sync`, multi-client semantics/testing complexity. |
| C. Seed/VPS operability/optional-services hardening | Make search/valkey truly optional, wire DB pool/config, compose mem limits/profiles, health/readiness, env contract | Important for “runs on cheap VPS” but mostly incremental infra/config hardening; does not by itself make chat/realtime secure or contract-correct | Medium-broad: touches boot/state/config/compose/docs; some items (SQLite claims, pool duplication) cross-cutting. |

## Chosen primary slice
**A — WS session authority + protocol alignment (go, scoped).**  
Rationale: reuse proven 012 DB-backed session authority on `/ws`, fix endpoint/transport/protocol mismatches so a browser client can establish an authenticated, tenant-correct socket and handle auth failure/close deterministically. Defer full fanout/sync redesign to follow-up (post-A) except minimal compatibility hooks required by tests.

## Constraints / guardrails
- Assessment artifact only: **no** application source edits, **no** commits/PRs/merges/releases/deploys, **no** secrets or paid services.
- Postgres-first, VPS-cheap (2 vCPU / 4 GB target), no new mandatory services/infra.
- Evidence-backed claims; mark **NV** where not executed/accessible.
- Multilingual research consulted where accessible (EN, pt-BR, es, de, fr, zh-CN; ja/ko/ru sources limited in indexed results—conclusions cross-checked against upstream Axum/Tokio/PostgreSQL/Redis docs and consensus).

## Inputs reviewed
`OBJETIVO.md`, `ROADMAP.md`, `.specify/assessments/013-next-vertical-slice/{decision,intake,research,problem,concept}.md`, `docs/research/2026-11-16-013-websocket-session-authority.md`, `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docs/research/2026-11-16-012-session-authority-international.md`, `flora-api/src/routes/websocket.rs`, `flora-api/src/extractors/auth.rs` (context from prior research), `apps/frontend/src/lib/api/websocket.ts`, external research (see `research.md`).