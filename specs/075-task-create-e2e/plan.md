# Implementation Plan: 075 — E2E de criação de tarefa

**Entrada**: spec/clarify/research/decision (go). **Um arquivo por commit.**

1. **T001** Reforçar `apps/frontend/tests/e2e/seed-flow.e2e.ts` com guard (FR-003), descrição (FR-004) e card na coluna `To Do` (FR-005), preservando a seleção única de org (FR-001/FR-002). *(FR-001, FR-002, FR-003, FR-004, FR-005, SC-002)*
2. **T002** Validar descoberta com `playwright test --list` e types/lint no arquivo. *(SC-001, SC-002)*
3. **T003** Tentar execução completa em Postgres isolado (PGPORT 5434; 5433 já ocupado por outro cluster); se inviável, registrar **[NV]** sem tocar o DB compartilhado. *(SC-004)*
4. **T004** `analyze.md` READ-ONLY e `converge.md` append-only. *(FR-006)*
5. **T005** Gates locais, `validate.md` (R4), `release.md` (R7), PR via `gh` sem merge. *(SC-003, SC-004)*

## Rollback

Reverter os commits do PR em `dev/`; nenhum código, schema, required check ou produção é alterado.
