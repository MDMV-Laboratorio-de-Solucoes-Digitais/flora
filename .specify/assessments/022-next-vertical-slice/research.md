# Research: 022 — Notification idempotency + search degradation contract (R1 synthesis)

**Sources:** 022 R1 research docs, codebase/frontend audits, 021 artifacts, `OBJETIVO.md`/`ROADMAP.md`.

## Convergence (80/20)
- **Notification dedup/idempotency:** 021 uses query-before-insert unread guard `(user_id,event_type,target_id)` — race-prone (TOCTOU), unread-only, no DB enforcement. Postgres partial unique index for active/unread rows + `INSERT ... ON CONFLICT DO NOTHING` provides atomic, deterministic dedup under concurrency/retries without heavy infra. Scope decision: unread dedup keyed by `(user_id,event_type,target_id)` matches current intent and allows future notification after read/reassignment; applying to all unread notification types is a safe general hardening (suppresses duplicate unread signals of same logical target/type).
- **Unassign semantics:** explicit `null` unassign is supported (021, tri-state PATCH) but intentionally emits no `Assignment` (U0). Research/product signals do not require adding unassign notification in 022; keep U0, treat unassign-specific event/recipient as NV.
- **Search Meilisearch/PG coherence:** PG FTS fallback is transactional correctness baseline (multi-entity). Meilisearch path is preferred when configured but mutation indexing hooks are not wired and adapter ignores async task completion, so freshness/completeness can drift. Full hook wiring + task-waitable APIs + rebuild + Meilisearch-gated deterministic CI is higher effort; minimal deterministic hardening is to fail-open to PG fallback on Meilisearch query errors and keep PG fallback contracts/tests authoritative. Explicit degradation/precedence contract (healthy Meilisearch preferred, error -> PG fallback, freshness/parity without hooks NV) satisfies "contrato operacional explícito de degradação/paridade" direction for 80/20.
- **Deterministic verification:** existing DB-gated harness (disposable PG18, env-gated, migrations, unique seeds, oneshot routers) is the cheapest reliable gate. CI `db-tests` currently runs only `auth/messaging/websocket` db_ suites; extending to `search/notifications/files` db_ suites enforces existing and 022 contracts deterministically in CI.
- **Frontend:** canonical `apps/frontend` does not consume search, task unassign/edit, or notification feeds; backend-only 022 slice requires no FE changes.

## Key risks/NVs
- Migration/index behavior: partial unique index must match `ON CONFLICT` target; for disposable PG18/CI regular index creation is sufficient (CONCURRENTLY not required).
- `NotificationRepository::create` becomes conflict-safe (may return no row) — API/trait/callers must treat suppression as success, not error.
- Meilisearch staleness when healthy but unindexed remains possible (hooks deferred); fallback-on-error does not guarantee freshness. Meilisearch-gated parity tests and rebuild tooling deferred.
- Best-effort post-persist delivery and tx-coupling/outbox remain NV.
