# Implementation Plan: 050 — Rate limit paths

**Data:** 2026-11-17T16:10Z · **Baseline:** `github/dev@b3bfda505`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | paths reais vs casados | — |
| classifier | enum + função pura | research |
| middleware | usar o classificador | classifier |
| tests | 4 casos puros | classifier |
| verification | gates | middleware, tests |

Ordem: research → classifier → middleware → tests → verification.

## Abordagem

1. Definir `RateLimitClass` (derive `Debug, Clone, Copy, PartialEq, Eq`) e
   `classify_endpoint(path: &str) -> RateLimitClass` com `ends_with` nos sufixos reais.
2. No middleware, substituir a cadeia `if/else` por `match classify_endpoint(path)`,
   mantendo `(key, limit, window_secs)` idênticos.
3. Acrescentar 4 testes unitários puros.
4. Rodar gates.

## Riscos

- Risco: lints estritos (`unused_results`, pedantic) → mitigado com enum simples e testes.
- Risco: mudar limites → mitigado preservando os literais.

## Verificação

`cargo test -p flora-api`, fmt/clippy/deny/build/test workspace, `check_gate_integrity.py`.

## Fora do plano

Rotas, limites, state distribuído, PRs anteriores, constituição, ADRs, merge/deploy.
