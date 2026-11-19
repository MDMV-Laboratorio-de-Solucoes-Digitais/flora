# Quality Checklist: 070 — contrato RBAC real

> Revisão de **qualidade de requisitos**, não registro de implementação.

## Requisitos

- [x] FR-001 — `db_state()` fail-closed com `bail!` sem env.
- [x] FR-002 — list escopo por sessão, sem vazamento cross-org.
- [x] FR-003 — create exige `RoleWrite`/`OrgAdmin` e grava no org da sessão.
- [x] FR-004 — built-in imutável; custom mutável.
- [x] FR-005 — rastreabilidade em STATE/artefatos, 1 arquivo por commit.

## Critérios de sucesso

- [x] SC-001 — 3 testes `db_rbac_*` com DB real; sem env falha fechada.
- [x] SC-002 — gates listados em `plan.md` são os do alvo (ADR-0003); nenhum piso alterado.
- [x] SC-003 — fronteira de paths explícita, verificável por `git diff --name-only`.

## Portarias

- [x] R1 `go (scoped)` antes do BLOCO 6; R3 zero perguntas; R4 validate antes de release; R7 release com SemVer/changelog/rollback.
- [x] TDD: teste escrito antes da execução; falha observada e corrigida na fonte, nunca relaxada.
- [x] `analyze.md` READ-ONLY; `converge.md` append-only.
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`.
