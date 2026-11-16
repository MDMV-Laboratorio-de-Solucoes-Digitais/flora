# Intake: 020 — Next vertical slice (post-019)

**Date:** 2026-11-16  
**Baseline:** `dev@dde602b3b` (019 PG FTS messages fallback + reply notification merged, PR #222, CI green).  
**Objective:** advance Flora as a viable, performant, self-hosted Macro-like alternative for SMBs on cheap VPS (Rust + PostgreSQL, Svelte 5; Tauri later) via 80/20 debloat and verified vertical slices.

## Inputs
- `OBJETIVO.md` / `ROADMAP.md`: 020 candidate = extend PG FTS (fallback/auto) to tasks and file metadata + 1 deterministic in-app task-assignment notification, Seed/Postgres-first, verifiable, no new mandatory services.
- `.specify/assessments/019-next-vertical-slice/decision.md`, `specs/019-postgres-search-reply-notification/*`, `.specify/validations/019-postgres-search-reply-notification/validate.md`: 019 delivered messages FTS fallback and reply notification with DB-gated contracts.
- 019/018 research informing 020: `docs/research/2026-11-16-019-postgres-fts-search-vps.md`, `docs/research/2026-11-16-019-inapp-notification-wiring-vps.md`, `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`.
- 020 R1 research (completed): `docs/research/2026-11-16-020-pg-fts-tasks-files-vps.md`, `docs/research/2026-11-16-020-task-assignment-notification-vps.md`, `docs/research/2026-11-16-020-macro-tasks-files-assignment-flows.md`, `docs/research/2026-11-16-020-db-gated-tests-search-notifications-assignment.md`, `docs/research/2026-11-16-020-pg18-multi-entity-fts-vps-ops.md`.
- 020 audits (completed): codebase readiness (`ses_e1d674de4ffevX4wonv9WQ0JAS`, search/task/notification hooks, schemas, tests), frontend readiness (`ses_e1d670690ffeDzKAP9DWM7qZlR`, no FE change required for contract-compatible backend delivery).

## Candidates (80/20)
1. **S1 — PG FTS fallback for tasks/files metadata + task-assignment notification (recommended):** additive generated `search_tsv`+GIN for `tasks(title,description)` and `files(name,file_type)` integrated into PG fallback/multi-entity search contract; emit `NotificationType::Assignment` on effective assignee transition (create/update, `new != old && new != actor`, best-effort post-persist) via existing repo/API; DB-gated tests on disposable PG18.
2. **S2 — Broader parity (Meilisearch indexing parity, mentions/completion/outbox/push, CJK/trgm tuning):** higher surface, infra/coupling, and contract ambiguity for one cycle.
3. **S3 — Frontend search/notification UI or multi-node realtime delivery:** depends on backend parity and exceeds Seed/VPS 80/20.

## Constraints
- Seed/VPS (~2 vCPU/4 GB): no new mandatory services, Postgres-first, cheap deterministic tests, additive migrations/indexes, conservative query/write paths.
- AGPL-safe, preserve existing contracts/Meilisearch path where applicable, no blind merge, CI green required, keep only `dev`/`main`, stage explicit paths.
- NV: relevance/i18n/CJK and channel-level ACL in search; Meilisearch parity for tasks/files; unassign contract/tx-coupling/idempotency/dedup/outbox/workers/push/realtime delivery; 2vCPU/4GB load/OOM/restore benchmarks; browser e2e/multi-client.
