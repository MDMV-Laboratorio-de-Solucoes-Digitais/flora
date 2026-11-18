# Implementation Plan: 065 — ADR floor alignment

**Data:** 2026-11-18T13:06Z · **Baseline:** `github/dev@aef13277a`

## Abordagem

1. Editar `docs/adr/0003:19` e `docs/adr/0006:10,52` com as cifras reais.
2. Validar: `grep`, guardião (51), fmt/clippy/deny/build/test, frontend.
3. Verificador isolado.

## Riscos

Nulo: apenas 3 linhas de documentação.

## Verificação

Grep, guardião, gates, verificador.

## Fora do plano

Constituição, código, CI, C5/C6, deploy.
