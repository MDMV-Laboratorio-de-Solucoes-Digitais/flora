# Tasks: 070 — contrato RBAC real

## Fase 1 — teste (TDD)

- **T001** `flora-tests/src/contract/rbac.rs` — substituir stub por `mod db` com `db_state()` fail-closed, seed, helpers e 3 testes `db_rbac_*`. *(FR-001, FR-002, FR-003, FR-004, SC-001)*
- **T002** executar com DB descartável e sem env; corrigir até PASS/fail-closed. *(SC-001)*

## Fase 2 — rastreabilidade e entrega

- **T003** `.specify/memory/STATE.md` — apêndice de execução. *(FR-005)*
- **T004** `specs/070-rbac-contract/{analyze,converge}.md`. *(FR-005)*
- **T005** `.specify/validations/070-rbac-contract/validate.md` e `.specify/releases/070-rbac-contract/release.md`. *(SC-002)*
- **T006** gates + PR + merge + higiene + remoção do DB descartável. *(SC-002, SC-003)*

## Convenção de commit

Um arquivo por commit, conventional em pt-BR, paths explícitos; nunca `git add -A`/`.`.
