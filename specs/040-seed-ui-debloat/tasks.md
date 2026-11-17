# Tasks: 040 — Seed UI debloat + real board

- [x] T001 R1 research (10 subagents) + assessment/decision `go (scoped)` + frozen verification contract.
- [x] T002 Baseline frontend gates (check/lint/30 unit/build) captured before changes.
- [x] T003 Remove `apps/web/` (5.844 files/43 MB) + dead e2e (`tasks.spec.ts`, `files.spec.ts`); diff = only `D`.
- [x] T004 Post-removal gates identical to baseline (non-regression proven).
- [x] T005 Pure `kanban.svelte.ts` + `types.ts` (groupByStatus/isValidMove, canonical statuses) + `kanban.test.ts` (6 tests).
- [x] T006 Rewrite `KanbanBoard.svelte` as controlled component (props, no mock, no duplicated state).
- [x] T007 Wire `(app)/tasks/+page.svelte`: `changeStatus()` → PATCH, render board.
- [x] T008 Frontend gates green (check/lint/36 unit/build); format new files with Prettier.
- [x] T009 Rust gates green (fmt, check_gate_integrity.py); zero apps/web/coparse refs in code/config.
- [x] T010 Fresh-context author-blind audit against frozen contract.
- [x] T011 Validate + release + research + ROADMAP drift; commit.
