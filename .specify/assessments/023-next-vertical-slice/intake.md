# Intake: 023 — Next vertical slice (post-022)

**Date:** 2026-11-16  
**Baseline:** `dev@023bd2467` (022 merged, PR #225/#226, CI green).  
**Objective:** evolve Flora as a viable, performant, self-hosted Macro-like alternative for SMBs on cheap VPS (Rust + PostgreSQL, Svelte 5; Tauri later) through 80/20 debloat and verified vertical slices.

## Inputs
- `OBJETIVO.md`/`ROADMAP.md`: 023 candidate = minimal Meilisearch indexing/freshness harness **or** explicit operational capability contract + deterministic rebuild, preserving authoritative PG fallback.
- Constitution: **Seed = Flora + PostgreSQL**; Meilisearch belongs to the **Grove** tier. Debloat/vendor-unlock is a core principle.
- 022 residuals: Meilisearch mutation hooks, delete-on-soft-delete, async task waiting, rebuild/backfill, Meilisearch-gated CI, cross-engine parity — all deferred.
- R1 research (8 background agents, multilingual/primary sources): `docs/research/2026-11-16-023-{meilisearch-task-await-determinism,meilisearch-rebuild-backfill-pg,search-debloat-pg-only-vps,outbox-lite-indexing-pg,meilisearch-gated-ci-harness,macro-backfill-notification-reference}.md`; codebase and frontend/ops audits in session reports.

## Candidates (80/20)
1. **S1 — PG-authoritative search contract (recommended):** make Postgres FTS the sole authoritative backend for `GET /api/v1/search`; keep Meilisearch dormant/non-authoritative opt-in; add an explicit `source` capability field, deterministic contracts (including a Meili-configured-but-unreachable fixture), config alignment, and cheap PG FTS quality hardening (separator normalization). No new services, hooks, outbox, or CI Meili job.
2. **S2 — Minimal fail-open mutation hooks + type/snippet normalization + lazy `setup_index`:** improves freshness if Meilisearch is enabled, but requires task-await handling, a rebuild tool, and contract normalization to avoid silent zero-results; higher effort/risk and conflicts with the Seed debloat goal.
3. **S3 — Outbox-lite + Meilisearch-gated CI + blue-green rebuild:** strongest durability/parity, but requires transaction refactor of pool-bound repositories and extra services; defer to 024+.

## Constraints
- Seed/VPS (~2 vCPU/4 GB): no new mandatory services, Postgres-first, cheap deterministic tests, safe migrations.
- Strict lints (no unwrap/expect/panic, pedantic clippy), no blind merge, CI green, explicit-path staging, only `dev`/`main`.
- NV: hooks/rebuild/outbox, cross-engine parity, `pg_trgm`/i18n/CJK relevance, search ACL, perf/load/restore, frontend adoption/e2e.
