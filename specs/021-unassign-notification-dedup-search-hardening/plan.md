# Plan: 021 — Task Unassign Contract, Assignment Dedup, PG Search Hardening

## Inputs
- Assessment: `.specify/assessments/021-next-vertical-slice/{intake,research,problem,concept,decision}.md`
- Research: `docs/research/2026-11-16-021-{meilisearch-task-file-search-parity-vps,task-unassign-contract-notification-semantics,macro-search-parity-unassign-notifications,db-gated-tests-unassign-search-parity,optional-meilisearch-pg-coherence-vps}.md`
- Audits: `ses_e1d515825ffeRc6tFzKjrFXEzD` (codebase), `ses_e1d515824ffewqOPRTcuB2Liu6` (frontend)
- 020 baseline: `specs/020-pg-search-tasks-files-assignment-notification/`, `flora-api/src/routes/{tasks.rs,search.rs}`, `flora-tests/src/contract/{tasks.rs,notifications.rs,search.rs}`

## Design
1. **Tri-state assignee contract:** change `UpdateTaskRequest.assignee_id` in `flora-api/src/routes/tasks.rs` to `Option<Option<Uuid>>` with serde semantics (missing => `None`, `null` => `Some(None)`, value => `Some(Some(id))`). Compute effective new assignee as `match req.assignee_id { None => existing.assignee_id, Some(inner) => inner }` so omit preserves and explicit null clears. Preserve create behavior.
2. **Transition-based notification + dedup:** refactor task create/update assignment handling to detect effective `old -> new` transitions. Notify only when `Some(new)` differs from old and `new != actor`; unassign/no-op/self produce no `Assignment`. Add a simple unread dedup guard in notification creation path (trait + Pg impl query-before-insert for same `user_id`, `event_type=Assignment`, `target_id`, `is_read=false`) and use it from assignment notifier (best-effort). No DB migration.
3. **Search hardening/decision:** keep Meilisearch precedence and PG fallback structure unchanged; add minimal normalization/hardening only if required by tests (e.g. whitespace/type token handling) and extend DB-gated contracts for edge cases. Document Meilisearch task/file parity as deferred/explicit decision in artifacts.
4. **Tests (DB-gated, disposable PG18):** extend `flora-tests/src/contract/tasks.rs` and/or `notifications.rs` for unassign/assign/reassign/self/no-op/dedup visibility; extend `flora-tests/src/contract/search.rs` for PG fallback hardening cases. Keep env-gated behavior.
5. **SDD/artifacts:** assessment (done), spec/plan/tasks, validation/release, and planning updates (`OBJETIVO.md`/`ROADMAP.md`) post-merge.

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib`
- `cargo test -p flora-api`
- `cargo test -p flora-tests --lib`
- DB-gated tests when `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` available (CI `db-tests`)

## Risks/NVs
- PATCH contract evolution for `assignee_id:null` (now explicit clear); omit remains no-op. Mitigated by tests.
- Dedup is query-before-insert, unread-only, race-prone (no uniqueness/tx-coupling).
- Meilisearch task/file indexing parity and Meilisearch-gated tests deferred; PG fallback authoritative for Seed multi-entity parity when Meilisearch unavailable.
