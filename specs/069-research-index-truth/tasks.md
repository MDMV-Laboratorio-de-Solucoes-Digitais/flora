# Tasks: 069 — corrigir nota falsa de dívida no índice de research

## Fase 1 — documentação

- **T001** `docs/research/index.md` — apêndice datado corrigindo a nota de dívida, com verificação 137/137 e comando reprodutível. *(FR-001, FR-002, SC-001)*
- **T002** `.specify/memory/STATE.md` — apêndice de execução do ciclo. *(FR-003)*

## Fase 2 — rastreabilidade e entrega

- **T003** `specs/069-research-index-truth/{analyze,converge}.md` — analyze READ-ONLY; converge append-only. *(FR-003)*
- **T004** `.specify/validations/069-research-index-truth/validate.md` e `.specify/releases/069-research-index-truth/release.md`. *(SC-002)*
- **T005** gates locais + PR + merge deliberado + higiene. *(SC-002, SC-003)*

## Convenção de commit

Um arquivo por commit, mensagem conventional em pt-BR, paths explícitos; nunca `git add -A`/`.`.
