# Decision: 021 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**S1 — Task unassign contract + assignment notification correctness/dedup + PG search hardening with explicit Meilisearch precedence decision (disposable PostgreSQL 18, Seed/VPS).**  
Primary 80/20 for cycle 021: make task assignment semantics explicit and verifiable (tri-state PATCH `assignee_id` supporting clear/unassign, deterministic transition-based `Assignment` notification, simple dedup guard) and harden/test PG fallback search edge behavior, while explicitly deciding/recording Meilisearch multi-entity parity as deferred (no indexing-hook wiring in this cycle).

## Justification (evidence)

- **Roadmap fit:** `OBJETIVO.md`/`ROADMAP.md` define 021 as paridade/confiabilidade de busca/notificações: Meilisearch task/file parity ou decisão explícita de precedence/contrato, endurecimento determinístico do fallback PG, semântica de atribuição/unassign quando contrato suportar com dedup simples, e contratos DB-gated.
- **Convergent R1:** `docs/research/2026-11-16-021-{meilisearch-task-file-search-parity-vps,task-unassign-contract-notification-semantics,macro-search-parity-unassign-notifications,db-gated-tests-unassign-search-parity,optional-meilisearch-pg-coherence-vps}.md` and audits (`ses_e1d515825ffeRc6tFzKjrFXEzD`, `ses_e1d515824ffewqOPRTcuB2Liu6`) converge on tri-state unassign, transition-only notification (no notify on unassign/self/no-op), cheap query-before-insert dedup, PG fallback as Seed correctness baseline, and Meilisearch indexing wiring as higher-risk/infra-sensitive work.
- **Value/80/20:** fixes a real correctness/contract gap in core task assignment with deterministic DB-gated evidence, reduces obvious duplicate notification noise, and locks search fallback behavior without adding mandatory services or destabilizing optional Meilisearch.

## Scope summary (in)

- Task API contract: `UpdateTaskRequest.assignee_id` becomes tri-state (`Option<Option<Uuid>>`, serde double-option semantics) so PATCH can distinguish omit (no change), `null` (clear/unassign), and value (set/assign). Create remains value/absent as today.
- Assignment notification: `flora-api/src/routes/tasks.rs` computes effective transition from `old_assignee_id -> new_assignee_id`; notify assignee only when `Some(new)` and `new != old` and `new != actor` (create/assign/reassign). No notification on unassign (`Some -> None`), self, or no-op. Best-effort insert with simple dedup guard (skip if unread `Assignment` exists for same `user_id` and `target_id`).
- Search hardening: preserve/clarify PG fallback behavior (type token normalization including singular/plural/CSV forms already supported, bounds/limit, tenant/soft-delete invariants) and add/extend DB-gated contracts for edge cases; keep Meilisearch path/precedence unchanged in code.
- Explicit precedence/contract decision (documented): Meilisearch-enabled mode takes precedence when configured (current behavior) but **task/file mutation-indexing parity is not guaranteed in 021**; Postgres fallback is authoritative for guaranteed multi-entity parity when Meilisearch is unavailable. Full Meilisearch parity (hooks, delete-on-soft-delete, reindex, deterministic Meilisearch-gated tests) deferred.
- DB-gated contract tests on disposable PG18 for unassign/assign/reassign/self/no-op/dedup and search edge behavior; SDD artifacts, local verification, PR/CI green, merge/sync, planning updates.

## Key risks

1. **PATCH contract evolution:** `assignee_id: null` becomes explicit clear (merge-patch semantics) instead of legacy collapsed/no-op; mitigate with tri-state parsing, clear server-side merge rules, and DB-gated regression tests (omit remains no-op).
2. **Dedup/consistency limits:** query-before-insert is best-effort and race-prone without uniqueness constraint/tx-coupling; mitigate by guarding obvious unread duplicates only and documenting NVs.
3. **Meilisearch parity gap remains:** optional Meilisearch mode may miss task/file freshness/completeness without indexing hooks; mitigate by explicit precedence/contract decision, keeping PG fallback path intact/tested, and deferring wiring/reindex/tests.

## NVs / non-goals

- NV: Meilisearch mutation-indexing hooks/parity for tasks/files/messages, Meilisearch-gated deterministic tests, cross-engine ranking/snippet equivalence, `pg_trgm`/i18n/CJK relevance, channel/workspace-level ACL in search.
- NV: uniqueness constraint/transactional outbox/strong idempotency for notifications, unassign-specific notification type, assignee membership policy changes, mentions/completion/realtime/push delivery.
- NV: frontend task-edit/unassign UI and search/notification UI, 2vCPU/4GB perf/load/OOM/restore benchmarks, browser e2e/multi-client.
- Non-goals: new mandatory services, crate convergence, Meilisearch removal, multi-node infra.
