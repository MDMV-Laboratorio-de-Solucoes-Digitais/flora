# Implementation Plan: 053 — DB gate uncovered legs

**Data:** 2026-11-18T00:04Z · **Baseline:** `github/dev@22f16d6ac`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | `--list` dos filtros + contagem de ignorados | — |
| workflow | dois `run_leg` no job `db-tests` | research |
| guardião | `FLOORS`, regex e mensagem 8/49 | workflow |
| verification | gates locais + `--remote` + CI ao vivo | guardião |

Ordem: research → workflow → guardião → verification. Sem paralelismo de código.

## Abordagem

1. Em `rust.yml`, após `run_leg files … 6`, acrescentar:
   `run_leg tasks contract::tasks::db::db_ 4` e
   `run_leg workspace contract::workspace::db::db_ 2`.
2. Em `check_gate_integrity.py`, adicionar `"tasks": 4, "workspace": 2` a
   `FLOORS`; trocar a regex por `contract::(?:\w+::)?db_...` de forma a aceitar
   tanto `contract::auth::db_` quanto `contract::tasks::db::db_`;
   atualizar a mensagem para "all eight db contract floors intact (49 total)".
3. Rodar gates na worktree; observar CI do PR ao vivo antes do merge.

## Riscos

- Risco: leg RED no runner → corrigir na branch, revalidar até verde.
- Risco: regex muito larga afrouxar validação → manter âncora `^` e `\b` no piso.
- Risco: documentação (ADR-0003/bug docs) citar 43 → registrado como dívida, fora de escopo.

## Verificação

`cargo test --list`, `check_gate_integrity.py`, fmt/clippy/deny/build/test,
frontend check/lint/unit/build e checks do PR ao vivo.

## Fora do plano

ADR-0003, constituição, nomes de checks, proteção de branch, V1, C5/C6, deploy.
