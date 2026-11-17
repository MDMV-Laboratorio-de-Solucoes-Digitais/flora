# Implementation Plan: 051 — Dead invite UI

**Data:** 2026-11-17T17:30Z · **Baseline:** `github/dev@b3bfda505`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | call sites e ausência de rota | — |
| frontend | remover componente e import/uso | research |
| docs | marcar drift em spec 002 | research |
| verification | `rg` + gates | frontend, docs |

Ordem: research → frontend + docs → verification. Sem paralelismo de código.

## Abordagem

1. Excluir `apps/frontend/src/lib/components/users/UserManager.svelte`.
2. Em `settings/+page.svelte`: remover import (`:4`) e o `<div><UserManager /></div>` (`:14-16`), mantendo o grid.
3. Em `specs/002-.../spec.md:34`: anexar ao final da linha a marcação de não implementado.
4. Rodar gates.

## Riscos

- Risco: quebrar layout → mitigado preservando o grid e os dois componentes restantes.
- Risco: referência esquecida → mitigado por `rg`.

## Verificação

`rg`, `bun run check/lint/test:unit/build`, gates Rust e `check_gate_integrity.py`.

## Fora do plano

Backend, convites, RBAC, PR #265, constituição, ADRs, merge/deploy.
