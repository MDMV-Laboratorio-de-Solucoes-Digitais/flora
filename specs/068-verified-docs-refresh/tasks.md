# Tasks: 068 — refresh verificado de documentação

## Fase 1 — documentação

- **T001** `docs/research/index.md` — corrigir cabeçalho e tabela de contagens; anexar nota datada preservando dívida 047+. *(FR-001, SC-001)*
- **T002** `docs/PROGRESSO.md` — anexar estado verificado 2026-11-19 com contagens, gate e **[NV]**. *(FR-002, SC-001)*
- **T003** `OBJETIVO.md` — anexo datado com intervalo até 067. *(FR-003)*
- **T004** `ROADMAP.md` — anexo datado com intervalo até 067. *(FR-003)*
- **T005** `flora-api/src/routes/rbac.rs` — corrigir comentário `//!` obsoleto. *(FR-004, SC-003)*

## Fase 2 — rastreabilidade

- **T006** `.specify/memory/STATE.md` — apêndice de execução do ciclo. *(FR-005)*
- **T007** `specs/068-verified-docs-refresh/{analyze,converge}.md` — analyze READ-ONLY; converge append-only. *(FR-005)*
- **T008** `.specify/validations/068-verified-docs-refresh/validate.md` e `.specify/releases/068-verified-docs-refresh/release.md` — R4 e R7. *(FR-005, SC-002)*
- **T009** gates locais + remoto, PR, merge deliberado e higiene (Blocos 7–8). *(SC-002, SC-003)*

## Convenção de commit

Um arquivo por commit, mensagem conventional em pt-BR, paths explícitos (`git add <arquivo>`); nunca `git add -A`/`.`.
