# Implementation Plan: 054 — DB env fail-closed

**Data:** 2026-11-18T00:30Z · **Baseline:** `github/dev@dd982cadc`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | contagem de 9/50 falhas abertas e lacuna do guardião | — |
| workflow | duas expansões `:?` em `run_leg` | research |
| guardião | exigir env no bloco `db-tests` | workflow |
| verification | mutações + DB real + gates | guardião |

Ordem: research → workflow → guardião → verification. Sem paralelismo de código.

## Abordagem

1. `rust.yml`, dentro de `run_leg`, após `local …` e antes do `echo`:
   `: "${FLORA_DATABASE__POSTGRES_URL:?FLORA_DATABASE__POSTGRES_URL must be set for db-tests}"`
   e `: "${DATABASE_URL:?DATABASE_URL must be set for db-tests}"`.
2. `check_gate_integrity.py`: no loop de `REQUIRED`, quando `check == "db-tests"`,
   verificar cada uma das chaves `FLORA_DATABASE__POSTGRES_URL:` e `DATABASE_URL:`
   no `block` do job; registrar falha se ausente.
3. Verificar: extrair o `run:` do step, executar sem env (deve falhar antes do
   cargo) e com DB real isolado (8 legs verdes); mutação temporária do workflow
   em `/tmp` para provar que o guardião fica RED; gates locais.

## Riscos

- Falso positivo: um leg futuro sem DB no mesmo job exigirá env — aceitável.
- Guardião é checagem de texto; overrides step-level não são detectados.
- Nenhum check renomeado → proteção de branch permanece válida.

## Verificação

`check_gate_integrity.py`, mutação sem env, 8 legs com Postgres isolado,
fmt/clippy/deny/build/test e frontend check/lint/unit/build.

## Fora do plano

ADR-0003, constituição, testes Rust, piso de auth, C5/C6, deploy.
