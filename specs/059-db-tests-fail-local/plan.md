# Implementation Plan: 059 — DB tests fail locally

**Data:** 2026-11-18T03:26Z · **Baseline:** `github/dev@2730fd2dd`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | 9 helpers / 50 chamadores | — |
| contratos | `bail!` nos 9 `else` | research |
| verification | sem env / com DB / `--list` / gates | contratos |

Ordem: research → contratos → verification.

## Abordagem

1. Substituir, nos 9 blocos `else` de `flora-tests/src/contract/*.rs`,
   `eprintln!(…); return Ok(None);` (ou só `return Ok(None);`) por
   `anyhow::bail!("FLORA_DATABASE__POSTGRES_URL/DATABASE_URL is required for DB contract tests")`.
2. Validar: sem env → exit != 0; com DB isolado → 50 passed; `--list` → 50;
   fmt/clippy/deny/build/test e frontend.

## Riscos

- Lints: `bail!` é macro `anyhow` legítimo; `panic`/`expect` não usados.
- Testes padrão (sem `--ignored`) não executam estes contratos — sem impacto.
- CI com env válida continua verde (verificar ao vivo no PR).

## Verificação

Comandos com `env -u` e com DB `flora_059_verify`, gates locais.

## Fora do plano

CI, gate, ADRs, constituição, C5/C6, deploy.
