# Intake: 056 — Docs research index

**Data:** 2026-11-18T02:12Z · **Âncora:** `github/dev@94fcf19ca` · branch `feature/056-docs-research-index`

## Sinal

`docs/research/` contém **135 arquivos** sem índice; `docs/PROGRESSO.md`,
`docs/research/index.md` e `index.md` não existem (`test -f` → ausente).
`OBJETIVO.md`/`ROADMAP.md` registram a ausência como dívida desde ciclos anteriores.
Navegar o corpus exige `ls`/`grep` ad hoc; não há ponto de entrada verificado.

## Evidência (verificada por find/ls, 02:10Z)

- `specs` 46 dirs/196 arquivos; assessments 43/170; bugs 4/12; validations
  42/43; releases 42/42; research 0 dirs/135 arquivos; adr 6 arquivos.
- `ls docs/research | head/tail` → nomes `YYYY-MM-DD[-NNN]-slug.md`,
  de `2026-11-15-012-frontend-api-contract-audit.md` a
  `2026-11-17-046-auth-hardening.md`.
- Gate autoritativo PASS (8 pisos/50); nenhum ciclo em voo no ledger.

## 80/20

`(I×C)/(E×R)` = (2×5)/(3×1) = **3.33**, maior candidato autônomo. ADR drift
(5.0) exige aprovação de maintainer → escalado **[NV]**, não executado.
N3 1.0; dívidas de fase 0.67; C5/C6 não autônomos.

## Fronteiras

Escopo: 2 arquivos novos (`docs/PROGRESSO.md`, `docs/research/index.md`),
artefatos do ciclo e notas datadas. Fora: ADR-0003, constituição, código,
workflows, testes, C5/C6, deploy. Factory fonte `046e1f2a…`, nada transplantado.
