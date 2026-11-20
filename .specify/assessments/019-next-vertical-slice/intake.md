# Intake: 019 — Next vertical slice (post-018)

**Date:** 2026-11-16  
**Baseline:** `dev@3c8eddbfd` (018 file hardening + DB-gated contracts merged, PR #221, CI green).  
**Objective:** advance Flora as a viable, performant, self-hosted Macro-like alternative for SMBs on cheap VPS (Rust + PostgreSQL, Svelte 5; Tauri later) via 80/20 debloat and verified vertical slices.

## Inputs
- `OBJETIVO.md` / `ROADMAP.md`: 019 candidate = "wiring essencial busca/notificações (Postgres FTS fallback + 1 evento in-app)" (Seed/Postgres-first, verificável, sem novos serviços obrigatórios).
- `.specify/assessments/018-next-vertical-slice/decision.md`, `specs/018-file-contracts-hardening/*`: 018 delivered file authz/workspace∈org hardening and DB-gated file contracts; residual NVs include aggregate/concurrent quota/purge consistency and broader file edge coverage.
- 018 research informing 019: `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`, `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`, `docs/research/2026-11-16-018-macro-search-notifications-parity.md`.
- 019 R1 research (completed): `docs/research/2026-11-16-019-postgres-fts-search-vps.md`, `docs/research/2026-11-16-019-inapp-notification-wiring-vps.md`, `docs/research/2026-11-16-019-macro-mutation-search-notification-flows.md`.
- 019 audits (completed): search/notifications readiness (`ses_e1d793fa1ffeg18iLwVe1BQrV1`), frontend readiness (`ses_e1d78fb75ffezHWpPzuDQ60xJS`, no FE change required for same backend contracts), mutation hook-point map (`ses_e1d77e0a0ffe9WZE984FSoaAv7`, recommends reply-in-thread).
- Pending 019 research (in-flight, non-blocking): `docs/research/2026-11-16-019-db-gated-tests-search-notifications.md`, `docs/research/2026-11-16-019-pg18-fts-notifications-vps-ops.md`.

## Candidates (80/20)
1. **S1 — PG FTS fallback for messages + reply-in-thread notification (recommended):** additive `messages.search_tsv` (generated/stored, `simple`, GIN partial) and PG search path with graceful fallback when Meilisearch unavailable/disabled, preserving `/api/v1/search` contract; emit `NotificationType::Reply` to parent message author on threaded reply (post-commit/best-effort, no self/cross-tenant notify); DB-gated tests on disposable PG18.
2. **S2 — Broader search (messages+tasks+files) and/or different notification (task assignment/completion):** higher schema/query/mutation surface and recipient/policy ambiguity for one cycle.
3. **S3 — Outbox/worker-backed indexing + push/Valkey delivery:** violates Seed/VPS 80/20 and adds mandatory/complex infra.

## Constraints
- Seed/VPS (~2 vCPU/4 GB): no new mandatory services, Postgres-first, cheap deterministic tests, conservative write/read paths.
- AGPL-safe, additive migrations preferred, no blind merge, CI green required, keep only `dev`/`main`, stage explicit paths.
- NV: zh-CN/ja FTS quality, channel-level ACL in search, relevance tuning/perf/load on 2vCPU/4GB, outbox/workers/push, multi-node/realtime delivery, browser e2e.
