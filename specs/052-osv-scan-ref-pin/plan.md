# Implementation Plan: 052 — OSV scan ref pin

**Data:** 2026-11-17T23:39Z · **Baseline:** `github/dev@01d822205`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | refs upstream + doc oficial + inputs do tag | — |
| workflow | trocar `@v2` por `@v2.7.1` | research |
| verification | gates + grep do ref | workflow |

Ordem: research → workflow → verification. Sem paralelismo de código.

## Abordagem

1. Em `.github/workflows/osv-scan.yml:14`, substituir `reusable.yml@v2` por
   `reusable.yml@v2.7.1`. Nenhuma outra linha do workflow muda.
2. Registrar notes datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).
3. Rodar gates e colar saídas em `validate.md`.

## Riscos

- Risco: tag inexistente → mitigado por duas fontes primárias independentes.
- Risco: incompatibilidade de inputs → mitigado pela leitura do workflow no tag.
- Risco: drift dos required checks → mitigado por `check_gate_integrity.py`.

## Verificação

`grep -n 'reusable.yml@' .github/workflows/osv-scan.yml`,
`python3 scripts/ci/check_gate_integrity.py`, fmt/clippy/deny/build/test,
check/lint/test:unit/build do frontend.

## Fora do plano

ADR-0003, constituição, RBAC/RLS, outros workflows, merge, deploy, auto-merge.
