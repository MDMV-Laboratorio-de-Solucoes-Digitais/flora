# Decision: 040 — Seed UI debloat + real board

**Verdict:** go (scoped)

## Chosen slice
Two 80/20 frontend moves that directly serve "alternativa self-hosted em VPS barata":
(S1) remove the imported Macro original client `apps/web` (5.844 files / 43 MB, zero
functional refs) plus two dead `.spec.ts` e2e files; (S2) turn the orphan `KanbanBoard`
into a controlled component fed by real task data, persisting status via the existing
`PATCH /api/v1/tasks/{id}` endpoint. Both behavior-preserving for the build and gated
by deterministic checks frozen before implementation (c040-10).

## Scope (in)
- `apps/web/` deletion + dead e2e deletion (only `D` entries).
- `kanban.svelte.ts` / `types.ts` pure module + `kanban.test.ts` (Vitest `server`).
- `KanbanBoard.svelte` controlled rewrite; `(app)/tasks/+page.svelte` `changeStatus()`.
- ROADMAP drift, validation, release, research doc.

## Non-goals / non-claims
No backend endpoint/migration, no RBAC/RLS/authz changes, no Tauri (ADR-0002 defers),
no FASE 7 Documents, no full Macro parity, no ADR/constitution edit, no deploy/tag/
release/secrets, no required-check/protection change. Do NOT claim team board,
persisted order, or status transition rules.

## NV
Client-component/e2e lanes not required CI gates; backend silently accepts invalid
status (canonical union mitigates); `apps/web` historical value unaudited; Forgejo
protections and remote CI result [NV]. ADR-0003 makes push/PR/merge a machine decision
under 4 green required checks — no human escalation needed for the merge itself.
