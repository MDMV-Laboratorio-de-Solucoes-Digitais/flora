# Implementation Plan: 057 — Dynamic floor total

**Data:** 2026-11-18T02:45Z · **Baseline:** `github/dev@97838cd9c`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | identificar literal e `FLOORS` | — |
| guardião | mensagem derivada de `FLOORS` | research |
| verification | saída idêntica + mutação + gates | guardião |

Ordem: research → guardião → verification. Sem paralelismo de código.

## Abordagem

1. `scripts/ci/check_gate_integrity.py:120`: trocar o literal por
   `f"all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)"`.
2. Verificar: guardião PASS com saída idêntica; cópia em `/tmp` com 9º piso
   (ex.: `"extra": 4`) imprime "nine ... (54 total)"; gates locais.

## Riscos

- Nenhum: apenas string de sucesso. A lógica de falha e a regex não mudam.
- ADRs seguem com 43 — dívida escalada, não editada.

## Verificação

`check_gate_integrity.py`, mutação em `/tmp`, fmt/clippy/deny/build/test e
frontend check/lint/unit/build.

## Fora do plano

ADRs, constituição, workflow, testes, C5/C6, deploy.
