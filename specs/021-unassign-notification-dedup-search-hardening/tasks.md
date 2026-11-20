# Tasks: 021 — Task Unassign Contract, Assignment Dedup, PG Search Hardening

## Implementation
- [x] T001 Implement tri-state `UpdateTaskRequest.assignee_id` (`Option<Option<Uuid>>`) and explicit unassign merge semantics in `flora-api/src/routes/tasks.rs` (omit=no change, `null`=clear).
- [x] T002 Implement transition-based assignment notification rules (assign/reassign only, no unassign/self/no-op) with simple unread dedup guard in `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/notification_repository.rs`, and `flora-api/src/routes/tasks.rs`.
- [x] T003 Harden/document PG fallback search behavior as needed in `flora-api/src/routes/search.rs` (contract-compatible: empty type-token filtering) with explicit Meilisearch parity/precedence decision recorded in artifacts.
- [x] T004 Add/extend DB-gated contracts in `flora-tests/src/contract/{notifications.rs,search.rs}` for unassign transitions, dedup, omit-preserve semantics, and PG fallback type normalization/limit edge cases (disposable PG18, env-gated).
- [x] T005 Run verification (`cargo fmt/clippy/tests`, DB-gated env-gated), record `.specify/validations` and `.specify/releases` artifacts.

## PR/Release
- [ ] T006 Commit scoped changes, push branch, open PR to `dev`, ensure CI green (incl. `db-tests`), merge, sync `dev`/`origin/dev`, clean feature branch, update `OBJETIVO.md`/`ROADMAP.md` for 022.
