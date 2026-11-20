# Feature Specification: 040 — Seed UI debloat + real board

**Branch**: `feat/040-seed-ui-debloat` · **Status**: In progress (working tree)
**Assessment**: `.specify/assessments/040-seed-ui-debloat/decision.md`

## Goal
Serve the two highest-confidence frontend levers of the "Macro alternativa VPS barata"
objective: (a) remove the imported Macro original client (`apps/web`, 5.844 files / 43 MB)
that has zero functional references, and (b) turn the orphan `KanbanBoard` into a real,
controlled component backed by the existing task API (no new backend endpoint).

## Scope
- Delete `apps/web/` entirely and the two dead Playwright files (`tasks.spec.ts`,
  `files.spec.ts` — `testMatch: '**/*.e2e.ts'` never executed them). No build/workspace
  config references either; pure I/O debloat on cheap VPS checkouts.
- New pure module `kanban.svelte.ts` (`groupByStatus`, `isValidMove`, canonical statuses
  `Todo|InProgress|Done`) with unit tests in Vitest `server` project (the required CI gate).
- `KanbanBoard.svelte`: controlled component with props `tasks` + `onStatusChange`;
  no local duplicated state, no hardcoded mock tasks/tags; `Archived` omitted.
- `(app)/tasks/+page.svelte`: `changeStatus()` → `PATCH /api/v1/tasks/{id}` with
  `{ status }` (backend already accepts `Todo|InProgress|Done|Archived`); render board
  instead of the inline `<ul>`; keep empty/loading states.
- Governance drift closed (ROADMAP); validation + release artifacts.

## Non-goals / non-claims
- No new backend endpoint, no migration, no change to RBAC/RLS/tenancy/authz.
- No Tauri (ADR-0002 defers), no FASE 7 Documents, no paridade Macro completa, no
  `apps/web` UX-historical-value claim (NV). No deploy/tag/release/secrets.
- No modification to required checks/protections/CODEOWNERS or ADRs/constitution.
- Do NOT claim the board has persisted column order, team-wide visibility, or
  transition rules — none exist (GET /tasks is assignee-scoped, order by created_at).

## Verification (frozen before implementation: c040-10-verification-contract)
Frontend gates `check`/`lint`/`test:unit --project server`/`build`; Rust gates
`fmt`/`check_gate_integrity.py`; removal diff = only `D` for apps/web + dead e2e;
zero `apps/web`/`@coparse` refs in code/config; fresh-context author-blind audit.

## NV
- Board status changes rely on PATCH with silent fallback for invalid status (backend
  does not 400) — mitigated by the canonical union in the pure module.
- Client-component (`project client`) and browser e2e are not required CI gates here.
- `apps/web` historical reference value not audited (NV).
- Forgejo protections / remote CI live result [NV].
