# Feature Specification: 015 — Minimal Realtime Delivery & Catch-up (80/20)

**Feature Branch**: `feat/015-realtime-minimal-delivery`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/015-next-vertical-slice/{intake,research,problem,concept,decision}.md` (verdict: go, slice A)

## Goal
Deliver a minimal, reliable realtime path for persisted channel messages on a single-node cheap VPS: Postgres-authoritative mutations, deterministic cursor catch-up, and in-process WebSocket subscription/fanout with canonical events and frontend idempotent convergence.

## Scope
- Deterministic message listing/catch-up contract (`after` cursor; stable ordering) for channel messages; preserve/clarify existing list behavior with safe compatibility.
- Canonical persisted-message events emitted after committed REST mutations (post/edit/delete) and aligned for WS delivery.
- In-process realtime hub in app state (single-node): authenticated channel subscriptions, bounded fanout/dedupe-safe envelopes, org/channel isolation, no new mandatory infra.
- WebSocket handler: register/unregister connections and channel subscriptions, deliver canonical events to authorized subscribers, keep DB-backed handshake/tenant guards.
- Frontend: WS envelope/subscribe/reconnect alignment, idempotent message upsert (create/edit/delete), REST catch-up on connect/channel change/reconnect, chat flow uses realtime+catch-up coherently.
- Deterministic verification (fmt/check/clippy, unit/API/DB-gated contracts where applicable, frontend checks) and SDD evidence.

## Non-goals
- Binary file upload/download (slice B), Seed/VPS operability/compose refactors (slice C).
- Multi-node/cross-process delivery, Valkey/Redis as required plane, Postgres LISTEN/NOTIFY delivery.
- Full `/messages/sync` subsystem expansion beyond what 015 catch-up needs.
- Attachments/DMs/search-notifications wiring, OIDC/JWKS browser e2e, `apps/web`, load/concurrency evidence.
- Post-handshake WS session revalidation.

## NV
- Load/concurrency on 2vCPU/4GB, browser/multi-client e2e, default-port smoke conflicts, revocation window after WS handshake.
