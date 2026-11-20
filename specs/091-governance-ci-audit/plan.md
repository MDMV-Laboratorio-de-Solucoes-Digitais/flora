# Implementation Plan: Auditoria viva de governança do CI (ciclo 091)

**Branch**: `feature/091-governance-ci-audit` @ `673c687ea` | **Date**: 2026-11-20 | **Spec**: [spec.md](./spec.md)

**Input**: `/specs/091-governance-ci-audit/spec.md` · Âncora: `.specify/feature.json`

## Summary

Auditoria **viva, documental e não mutante** da governança do CI após os ciclos
086–090. Confirma-se que a prática (required checks, proteção de branch, gate
autoritativo, deploy separado, explicações, omissão dos scripts SDD) coincide
com a ADR-0003 emendada em 2026-11-20 — "CI/CD GREEN basta para merge autônomo
em `dev/`". Nenhum gate, workflow, ADR, constituição, código ou credencial é
alterado; nenhum required check novo é criado.

## Technical Context

**Autoridade**: ADR-0003 + emenda 2026-11-20 (CI green basta, sem aprovador
humano). **Gate**: `scripts/ci/check_gate_integrity.py`, oráculo dinâmico do
ciclo 088 — 10 legs / 58 pisos, PASS (exit 0).

**Leitura ao vivo**: `gh` com PAT do SE (fine-grained, `Administration: read`,
restrito a `flora`); secret `FLORA_BRANCH_PROTECTION_TOKEN` já registrado no
GitHub. 4 required checks: `frontend-build`, `rust-build`, `db-tests`,
`dependency-review` (*strict*, `enforce_admins`, sem required reviews).

**Deploy**: ligação MDMV-Autonoma — decisão separada, fora do gate; health e
rollback documentados, estado ao vivo **[NV]**.

**Constraints**: nenhum required check novo; nenhum aprovador humano
reintroduzido; nenhum valor de credencial exposto; nenhum script SDD citado como
gate; nenhum toque em `validate/`, `release/` ou `.specify/memory/STATE.md`;
sem PR, merge, tag, deploy ou promoção neste ciclo de documentação.

## Constitution Check

**Gate**: **PASS**. A auditoria é somente leitura dos mecanismos existentes: o
gate autoritativo permanece `check_gate_integrity.py` (10 legs / 58 pisos) e a
proteção de branch não é alterada. A emenda 2026-11-20 é confirmada como regra
vigente; demais emendas constitucionais pendentes permanecem escaladas **[NV]**
a maintainer. Tabela de complexidade vazia — nenhuma mudança de processo.

## Arquivos tocados

| Arquivo | Mudança | Evidência |
|---|---|---|
| `.specify/assessments/091-governance-ci-audit/decision.md` | novo — verdict **go**, 2026-11-20 | baseline `673c687ea` |
| `specs/091-governance-ci-audit/spec.md` | template preenchido (FR-001..FR-005, SC-001..SC-005) | antes template |
| `specs/091-governance-ci-audit/plan.md` | novo | 2026-11-20 |
| `specs/091-governance-ci-audit/tasks.md` | novo — T001..T008 `[X]` | evidência por tarefa |

**Fora de escopo**: `.github/workflows/`, gate, ADRs, constituição, código,
`.specify/validations/`, `.specify/releases/`, `.specify/memory/STATE.md`, PRs,
merge, tags, deploy, produção e credenciais.

## Next Phase

Fechamento documental do ciclo 091. Nenhuma fase adicional; **[NV]** reconsulta
de CI/proteção ao vivo, deploy em produção, Forgejo e aprovações de maintainer
antes de qualquer merge ou promoção.
