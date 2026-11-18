# Implementation Plan: 055 — Auth floor thirteen

**Data:** 2026-11-18T01:35Z · **Baseline:** `github/dev@d5d7ef5d`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | contar 13 e localizar os 12 | — |
| workflow | `run_leg auth … 13` | research |
| guardião | `FLOORS["auth"]=13`, texto 50 | workflow |
| verification | `--list`, DB real, mutação, gates | guardião |

Ordem: research → workflow → guardião → verification. Sem paralelismo de código.

## Abordagem

1. `rust.yml:107`: `12` → `13`.
2. `check_gate_integrity.py:43`: `"auth": 12` → `13`; `:120`: `49 total` → `50 total`.
3. Verificar: guardião PASS; `--list` = 13; leg auth com DB isolado = 13
   passed; mutação (alterar só um dos dois) → guardião RED; gates locais.

## Riscos

- Remove tolerância a 1 falha: qualquer falha intermitente fica RED. Mitigado
  pela evidência de estabilidade e por `--test-threads=1`.
- Edição em lockstep: mudar só um arquivo quebra o guardião — usado como prova.
- ADR-0003 segue com 43: dívida documental, fora de escopo.

## Verificação

`check_gate_integrity.py`, `--list`, leg auth com Postgres isolado, mutação,
fmt/clippy/deny/build/test e frontend check/lint/unit/build.

## Fora do plano

ADR-0003, constituição, outros pisos, testes Rust, C5/C6, deploy.
