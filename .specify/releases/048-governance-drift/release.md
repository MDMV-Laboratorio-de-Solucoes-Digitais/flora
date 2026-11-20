# Release: 048 — Governance drift

**Ciclo:** 048 · **Data:** 2026-11-17T14:38Z · **Baseline:** `github/dev@4ae58ccd2`
**Tipo:** documentação de governança; **nenhuma tag, release ou deploy executado.**

## Entregue

- `AGENTS.md`: 5 membros reais + pacote raiz (6 manifests) e toolchain `1.98.1`.
- Notas datadas e append-only em `OBJETIVO.md`/`ROADMAP.md`.
- Ciclo SDD completo: assess (5), spec/clarify/plan/checklist/tasks/analyze, converge, validate R4, release R7.

## SemVer

**Proposta:** patch `0.1.0` → `0.1.2` (documentação; sem mudança de runtime/API).
Nenhuma tag criada. A numeração considera a proposta `0.1.1` do ciclo 047, ainda não aplicada.

## Changelog (IDs)

- `FR-001` — inventário de crates corrigido para 5 membros + raiz.
- `FR-002` — toolchain declarada `1.98.1`.
- `FR-003` — nenhuma regra/gate/permissão enfraquecida.
- `FR-004` — notas de estado verificado anexadas.
- `T001`–`T006` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em aberto; os 4 existentes estão `applied`/`verified`.

## Evidência

`.specify/validations/048-governance-drift/validate.md` ·
`specs/048-governance-drift/converge.md` ·
`specs/048-governance-drift/analyze.md`.

## Rollback

`git revert` dos commits do ciclo restaura as duas linhas de `AGENTS.md` e remove as
notas anexadas. Nenhum efeito externo a desfazer: sem tag, publicação, deploy ou merge.

## Não feito / NV

CI ao vivo, aprovações e merges: **[NV]**. Sem merge, auto-merge, force-push ou
alteração de required checks. PR #262 não foi modificado.
