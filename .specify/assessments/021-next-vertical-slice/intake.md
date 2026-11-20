# Intake: 021 — Next vertical slice (post-020)

**Date:** 2026-11-16  
**Baseline:** `dev@0dcde353a` (020 PG FTS tasks/files fallback + task assignment notification merged, PR #223, CI green).  
**Objective:** advance Flora as a viable, performant, self-hosted Macro-like alternative for SMBs on cheap VPS (Rust + PostgreSQL, Svelte 5; Tauri later) via 80/20 debloat and verified vertical slices.

## Inputs
- `OBJETIVO.md` / `ROADMAP.md`: 021 candidate = consolidar paridade/confiabilidade de busca/notificações — paridade mínima `task/file` no modo Meilisearch (ou decisão explícita de precedence/contrato) e endurecimento determinístico do fallback PG; evoluir semântica de atribuição para unassign/reassign quando contrato suportar (ou travar contrato atual) com dedup simples; ampliar contratos DB-gated determinísticos; CI verde.
- `.specify/assessments/020-next-vertical-slice/decision.md`, `specs/020-pg-search-tasks-files-assignment-notification/*`, `.specify/validations/020-pg-search-tasks-files-assignment-notification/validate.md`: 020 delivered multi-entity PG fallback and best-effort assignment notification on explicit provided transition; NVs include Meilisearch parity, unassign contract (`PATCH` cannot clear), dedup/idempotency/tx-coupling, search ACL/relevance/i18n.
- 021 R1 research (completed): `docs/research/2026-11-16-021-meilisearch-task-file-search-parity-vps.md`, `docs/research/2026-11-16-021-task-unassign-contract-notification-semantics.md`, `docs/research/2026-11-16-021-macro-search-parity-unassign-notifications.md`, `docs/research/2026-11-16-021-db-gated-tests-unassign-search-parity.md`, `docs/research/2026-11-16-021-optional-meilisearch-pg-coherence-vps.md`.
- 021 audits (completed): codebase readiness (`ses_e1d515825ffeRc6tFzKjrFXEzD`), frontend readiness (`ses_e1d515824ffewqOPRTcuB2Liu6`, canonical frontend has no task unassign/update UI or search/notification consumption; backend-only contract hardening feasible with no FE change required).

## Candidates (80/20)
1. **S1 — Explicit unassign contract + assignment notification correctness/dedup + PG search hardening (recommended):** tri-state `assignee_id` in task PATCH (`omit`=no change, `null`=clear/unassign, `value`=set) with deterministic transitions (assign/reassign notify new non-actor assignee; unassign/no-op/self do not notify), simple best-effort dedup guard (avoid duplicate unread Assignment for same user+target), and PG fallback search edge-case/type/limit hardening with DB-gated contracts; document explicit Meilisearch precedence/parity decision (no indexing hooks added) as NV.
2. **S2 — Meilisearch indexing parity wiring (messages/tasks/files hooks, delete-on-soft-delete, reindex):** higher surface across mutation paths, consistency/ops burden on 2vCPU/4GB, and hard to test deterministically without Meilisearch-gated infra.
3. **S3 — Frontend search/notification/task-edit UI:** decoupled from backend contract fixes and exceeds Seed 80/20 given current FE gaps.

## Constraints
- Seed/VPS (~2 vCPU/4 GB): no new mandatory services, Postgres-first, cheap deterministic tests, additive/safe contract evolution, conservative write paths.
- AGPL-safe, preserve existing contracts where feasible, no blind merge, CI green required, keep only `dev`/`main`, stage explicit paths.
- NV: Meilisearch mutation-indexing parity and Meilisearch-gated tests; `pg_trgm`/i18n/CJK relevance and channel/workspace ACL in search; tx-coupling/outbox/push/realtime delivery, mentions/completion, race-hard idempotency/uniqueness constraints beyond simple guard; 2vCPU/4GB load/OOM/restore benchmarks; browser e2e/multi-client.
