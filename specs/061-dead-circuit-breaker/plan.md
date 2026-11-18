# Implementation Plan: 061 — Dead circuit breaker

**Data:** 2026-11-18T04:07Z · **Baseline:** `github/dev@7ed232c46`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | mapear referências órfãs | — |
| code | excluir arquivo, limpar config/erro | research |
| verification | grep + gates + CI | code |

Ordem: research → code → verification.

## Abordagem

1. `git rm flora-core/src/utils/circuit_breaker.rs`.
2. `config/messaging.rs`: remover os dois campos e seus defaults.
3. `error.rs`: remover a variante `CircuitBreakerOpen` e o braço do match.
4. Validar: `grep -rn circuit_breaker` = 0; fmt/clippy/deny/build/test;
   frontend; CI do PR.

## Riscos

- Baixo: módulo nunca compilado; Figment sem `deny_unknown_fields`.
- Nenhum required check renomeado.

## Verificação

Grep, gates locais e CI ao vivo.

## Fora do plano

ADRs, constituição, CI, migrações, C5/C6, deploy.
