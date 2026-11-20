# Release: 057 — Dynamic floor total

**Ciclo:** 057 · **Data:** 2026-11-18T02:48Z · **Baseline:** `github/dev@97838cd9c`
**Tipo:** gate determinístico; **nenhuma tag, release ou deploy executado.**

## Entregue

- `scripts/ci/check_gate_integrity.py:120`: mensagem de sucesso derivada de
  `FLOORS` (`len` e `sum`), sem literal duplicado.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.11` (correção de saída do gate, sem API
externa; considera propostas 0.1.1–0.1.10 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — total e contagem derivados de `FLOORS`.
- `FR-002` — regex e falhas inalteradas.
- `FR-003` — saída informacional equivalente (`8` em vez de `eight`).
- `FR-004` — 1 linha, 1 arquivo.
- `SC-001`–`SC-004` — verificados (saída, mutação, gates, diff).
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/057-dynamic-floor-total/validate.md` ·
`specs/057-dynamic-floor-total/converge.md` ·
`specs/057-dynamic-floor-total/analyze.md`.

## Rollback

`git revert` do commit `fix(ci): deriva total de pisos de FLOORS` restaura o
literal anterior. Nenhum efeito externo; nenhuma tag/deploy envolvido.

## Não feito / NV

ADR-0003/0006 (texto 43), root `index.md`, N3, C5/C6: fora de escopo ou
escalados. Estado remoto/CI: **[NV]**. Sem merge/auto-merge/force-push.
