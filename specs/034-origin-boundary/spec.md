# Feature Specification: 034 — Origin boundary (ADR-0002 #2)

**Branch**: `feat/034-origin-boundary` · **Status**: In progress
**Assessment**: `.specify/assessments/034-origin-boundary/decision.md`

## Goal
Make API/WS URLs runtime-configurable and verify the bearer/`select-organization` contract from a non-browser origin against an explicit CORS allowlist, with a server-side WS `Origin` guard.

## Scope
- `origin_allowed()` + early 403 in `websocket.rs`; 3 pure unit tests + 2 DB-free oneshot tests.
- FE runtime config chain (`static/config.js`, `app.html`, `app.d.ts`, `client.ts`, `websocket.ts`, mock) + `tests/unit/resolver.test.ts`; `.env.example`.
- Isolated fresh-canonical T8 live proof and validation artifact.

## Non-goals / non-claims
No FASE 7 Documents, no frontend affordance cleanup, no cargo-vet promotion, no tag/release. Do not mark ADR-0002 #2 met until CI (including browser lane) is green and the ADR gate note records the policy decision and evidence. Never touch other-agent work (`/metrics` divergence, `:3000` process).

## NV
Tauri origin literals per-OS unproven; proxy must forward `Origin`; `?token=` leakage unchanged.
