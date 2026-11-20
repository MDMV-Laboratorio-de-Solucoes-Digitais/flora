# Research: 021 — Unassign contract/notification hardening + search contract/precedence

**Sources:** R1 research docs, codebase/frontend audits, 020 artifacts, `OBJETIVO.md`/`ROADMAP.md`.

## Convergence (80/20)
- **Unassign contract:** current `UpdateTaskRequest.assignee_id: Option<Uuid>` + merge `req.assignee_id.or(existing.assignee_id)` cannot distinguish omit vs explicit null, so unassign is impossible. Tri-state `Option<Option<Uuid>>` (serde double-option: absent=`None`, `null`=`Some(None)`, value=`Some(Some(id))`) enables merge-patch semantics: omit=no change, null=clear, value=set. DB already supports nullable `assignee_id`.
- **Notification semantics:** emit `Assignment` only on effective transition to `Some(new)` where `new != old` and `new != actor` (create/assign/reassign). Unassign (`Some -> None`) and no-op/self produce no `Assignment` in MVP (no dedicated unassign event type). Simple best-effort dedup: before insert, skip if unread Assignment already exists for same `(user_id,target_id)` (query-before-insert, no migration; race residual NV).
- **Search/precedence:** PG fallback now covers `message|task|file` with deterministic merge; Meilisearch path exists but mutation indexing hooks for tasks/files/messages are not wired and Meilisearch-only mode can drift/miss parity. 80/20: keep precedence unchanged (`search_service` => Meilisearch path else PG fallback), harden/type-normalize and test PG fallback edge cases/limits, and record explicit decision that Meilisearch task/file parity requires indexing wiring + deterministic tests (deferred). No Meilisearch-gated infra in this cycle.
- **Frontend:** canonical `apps/frontend` has no task update/unassign UI and no search/notification consumption; backend contract hardening is compatible with no FE change (FE can adopt `assignee_id:null` later).
- **Tests:** DB-gated PG18 contracts can deterministically verify unassign/assign/reassign/self/no-op/dedup behavior and search normalization/limits/tenant/soft-delete edges using existing harness patterns.

## Key risks/NVs
- `assignee_id: null` changes meaning for PATCH from "ignored/no-op (current collapse)" to "clear" (merge-patch) — behavioral contract evolution; must be explicit and tested. Omit remains no-op.
- Dedup via query-before-insert is race-prone and unread-only; no uniqueness constraint/tx-coupling/outbox.
- Meilisearch indexing parity and cross-engine ranking/snippet equivalence unverified/deferred; 2vCPU/4GB perf, `pg_trgm`/i18n/CJK, ACL, mentions/completion/realtime delivery remain NV.
