# Tasks: 075 — E2E de criação de tarefa

- **T001** `apps/frontend/tests/e2e/seed-flow.e2e.ts` — guard (FR-003), descrição (FR-004), coluna `To Do` (FR-005), sessão única (FR-001/FR-002). *(FR-001, FR-002, FR-003, FR-004, FR-005, SC-002)*
- **T002** `playwright test --list` + `bun run check`/lint/Prettier no arquivo. *(SC-001, SC-002)*
- **T003** Execução em Postgres isolado (PGPORT 5434) ou `[NV]` sem tocar DB compartilhado. *(SC-004)*
- **T004** `analyze.md` READ-ONLY + `converge.md` append-only. *(FR-006)*
- **T005** Gates + `validate.md` (R4) + `release.md` (R7) + PR via `gh` sem merge. *(SC-003, SC-004)*

Convenção: um arquivo por commit; nunca `git add -A`/`.`.
