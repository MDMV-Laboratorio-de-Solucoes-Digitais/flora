# Decision: 022 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**S1 — DB-enforced notification dedup + search fail-open degradation contract + CI DB-gated enforcement (disposable PostgreSQL 18, Seed/VPS).**  
Primary 80/20 for cycle 022: make unread notification dedup deterministic and race-safe at the database level, harden search behavior to fail-open to the Postgres FTS correctness baseline on Meilisearch query errors, and enforce relevant DB-gated contracts in CI.

## Justification (evidence)

- **Roadmap fit:** `OBJETIVO.md`/`ROADMAP.md` define 022 as Meilisearch paridade/contrato operacional explícito de degradação (mantendo fallback PG determinístico), endurecimento de idempotência/dedup com cobertura de concorrência/transições quando viável, e contratos DB-gated/CI verde (Meilisearch-gated quando viável).
- **Convergent R1:** `docs/research/2026-11-16-022-{meilisearch-indexing-hooks-reindex-parity-vps,meilisearch-pg-coherence-degradation-contract-vps,notification-idempotency-unassign-semantics,db-meilisearch-gated-test-harness,macro-search-notification-reference-flows,meilisearch-vps-resource-ops}.md` and audits (`ses_e1c0bc4e5`, `ses_e1c0bc4c8`) converge on: PG FTS as correctness baseline; Meilisearch mutation hooks/freshness as higher-risk/infra-sensitive without task-awaitable harness; partial-unique unread dedup + `ON CONFLICT DO NOTHING` as the strongest cheap deterministic fix for TOCTOU duplicates; keep U0 unassign semantics; CI DB-gated enforcement as high-value/low-cost.
- **Value/80/20:** eliminates the main assignment/reply unread-duplicate race with a small additive migration and conflict-safe insert, improves search resilience without new mandatory services, and increases deterministic regression protection in CI.

## Scope summary (in)

- Notification dedup/idempotency:
  - Additive migration creating partial unique index for unread notifications: `UNIQUE (user_id, event_type, target_id) WHERE is_read = false`.
  - Make `NotificationRepository::create` conflict-safe/idempotent for that arbiter (`INSERT ... ON CONFLICT ... WHERE is_read = false DO NOTHING RETURNING ...`, return `Option<Notification>`; suppression is not an error). Update trait/impl and callers to treat suppression as success.
  - Keep 021 transition semantics (notify on effective assign/reassign to non-actor; no notification on unassign/self/no-op) and best-effort, post-persist delivery (no tx-coupling/outbox).
- Search degradation/contract:
  - Preserve precedence: when Meilisearch service is configured, attempt Meilisearch search first; on Meilisearch query failure, log warning and fall back to PG FTS (`search_pg`) for the same request (fail-open to correctness baseline).
  - Keep PG fallback deterministic/tenant+soft-delete safe and contract-compatible.
  - Explicit contract: Meilisearch freshness/parity requires mutation-indexing hooks, async task guarantees, and rebuild tooling not implemented in 022 (NV/deferred); PG fallback remains authoritative for guaranteed multi-entity parity.
- Deterministic verification/CI:
  - Extend DB-gated contracts to assert conflict-safe unread dedup suppression/idempotent behavior and preserve existing search/notification invariants.
  - Add `contract::search::db_`, `contract::notifications::db_`, and `contract::files::db_` to `.github/workflows/rust.yml` `db-tests` job (disposable PostgreSQL 18, `--ignored --test-threads=1`).
- SDD artifacts, local verification, PR/CI green (incl. `db-tests`), merge/sync, planning updates for 023. No frontend changes.

## Key risks

1. **Migration/arbiter correctness:** partial unique index predicate must exactly match `ON CONFLICT` inference; wrong shape can fail inserts or miss dedup. Mitigate with targeted SQL/migration and DB-gated tests on disposable PG18.
2. **Behavioral change (dedup scope):** unread duplicate suppression now applies to same `(user_id,event_type,target_id)` across notification types (including Reply retries) — intended hardening, but must not suppress legitimate distinct events. Mitigate by scoping to unread-only + same target/type and preserving read/reassignment behavior.
3. **Meilisearch staleness remains:** fallback-on-error does not fix drift when Meilisearch responds with stale/empty-but-successful results. Mitigate with explicit deferred-NV contract and preserved PG fallback path/tests.

## NVs / non-goals

- NV: Meilisearch mutation-indexing hooks (task/file/message), delete-on-soft-delete propagation, async task waiting/completion handling, rebuild/backfill tooling, Meilisearch-gated deterministic CI/harness, cross-engine ranking/snippet parity.
- NV: outbox/tx-coupling, coalescing upsert/read-state policy changes, unassign-specific notification event/recipients, mentions/completion/realtime/push delivery, notification `target_url` exposure.
- NV: `pg_trgm`/i18n/CJK relevance, channel/workspace-level ACL in search, 2vCPU/4GB perf/load/OOM/restore benchmarks, frontend adoption/e2e/multi-client.
- Non-goals: new mandatory services, crate convergence, Meilisearch removal, multi-node infra.
