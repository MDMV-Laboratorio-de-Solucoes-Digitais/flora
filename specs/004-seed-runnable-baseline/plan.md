# Plan: 004 — Seed Runnable Baseline (Dev)

**Date:** 2026-11-15  
**Status:** Planned/executed as documentation + dev enablement slice (this cycle)

## Approach

1. **Consolidate evidence and direction**
   - Create `OBJETIVO.md`, `ROADMAP.md`, `docs/research/2026-11-15-flora-seed-80-20-research.md`.
   - Capture assessment 004 artifacts with `go` decision and bounded scope.

2. **Provide reproducible local dependency baseline**
   - Add `docker-compose.yml` (dev) for `postgres:16-alpine`, `valkey/valkey:8-alpine`, `getmeili/meilisearch:v1.12` with localhost-bound ports, volumes, restart policies, and healthchecks.
   - Align with `.env.example` defaults (document dev-only Meilisearch key).

3. **Document minimal run/smoke path**
   - Record steps: copy `.env.example` → `.env`, adjust keys/URLs if needed, `docker compose up -d`, `cargo run -p flora-api`, optional `dev-seed.sql`.
   - Note NV items (Docker execution, full integration/CI remote, OIDC provider).

4. **Preserve governance constraints**
   - No secrets, no production deploy/merge/release actions, no AGPL-incompatible or unauthorized RE reuse.

## Deliverables

- `OBJETIVO.md`, `ROADMAP.md`
- `docs/research/2026-11-15-flora-seed-80-20-research.md`
- `.specify/assessments/004-seed-runnable-baseline/{intake,research,decision}.md`
- `specs/004-seed-runnable-baseline/{spec,plan,tasks}.md`, `checklists/requirements.md`
- `docker-compose.yml` (dev)

## Verification (this cycle)

- Static/document review of created files and config consistency with `.env.example`/AGENTS.
- Runtime verification of `docker compose` and API+deps smoke: **NV** unless explicitly executed in an environment with Docker and required resources.
