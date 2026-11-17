# Implementation Plan: 049 — Local logger

**Data:** 2026-11-17T15:06Z · **Baseline:** `github/dev@55363b3ca`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | mapa de call sites e ausência de rota | — |
| logger | reescrita local-only | research |
| contract | alinhar contrato 002 | logger |
| test | provar ausência de rede e teto do buffer | logger |
| verification | gates | logger, contract, test |

Ordem: research → logger → contract + test → verification.

## Abordagem

1. `logger.ts`: remover `ApiClient`, `flush()`, intervalo, `beforeunload`, `sendBeacon`;
   manter `LogEntry`, API pública e buffer com teto 200; acrescentar `snapshot()`.
2. `specs/002-.../contracts/logger-contract.md`: remover `flush()` e a seção de
   ingestão backend; documentar comportamento local e `snapshot()`.
3. `apps/frontend/src/lib/utils/logger.spec.ts`: teste no projeto `server`.
4. Rodar gates.

## Riscos

- Risco: quebrar call sites → mitigado mantendo assinaturas.
- Risco: regra no-console → mitigado sem nenhum `console.*`.

## Verificação

`rg`, vitest `--project server`, check/lint/build, `check_gate_integrity.py`.

## Fora do plano

Backend, OTel, dependências novas, PR #262, constituição, ADRs, merge/tag/deploy.
