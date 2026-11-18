# Release: 053 — DB gate uncovered legs

**Ciclo:** 053 · **Data:** 2026-11-18T00:10Z · **Baseline:** `github/dev@22f16d6ac`
**Tipo:** CI gate; **nenhuma tag, release ou deploy executado.**

## Entregue

- `.github/workflows/rust.yml`: legs `tasks` (4) e `workspace` (2) com filtros
  `contract::tasks::db::db_` e `contract::workspace::db::db_` no job `db-tests`.
- `scripts/ci/check_gate_integrity.py`: `FLOORS` com 8 entradas, regex que aceita
  o caminho `db::db_`, mensagem atualizada para 8 pisos / 49 total.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.7` (elevação de cobertura de CI, sem API externa;
considera propostas 0.1.1–0.1.6 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001`/`FR-002` — dois legs novos, 6 contratos DB antes órfãos agora executam.
- `FR-003` — guardião valida 8 pisos sem reduzir os 6 originais.
- `FR-004` — 4 required checks, ADR-0003 e constituição inalterados.
- `SC-001`–`SC-004` — verificados (filtros, `--list`, guardião, gates).
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; sinal veio do ranking C25b (intake).

## Evidência

`.specify/validations/053-db-gate-uncovered-legs/validate.md` ·
`specs/053-db-gate-uncovered-legs/converge.md` ·
`specs/053-db-gate-uncovered-legs/analyze.md`.

## Rollback

`git revert` dos commits `fix(ci): cobre legs DB de tasks e workspace no gate` e
`fix(ci): espelha pisos 4 e 2 no guardiao determinístico` restaura os 6 legs
originais e o texto 6/43. Nenhum efeito externo; nenhuma tag/deploy envolvido.

## Não feito / NV

Fail-open de `db_state()` sem env (V1), piso de auth 12→13, ADR-0003, C5/C6:
fora de escopo. Resultado dos legs no runner e CI/PR ao vivo: **[NV]**.
Sem merge/auto-merge/force-push nesta fase.
