# Release: 043 — Manifesto sem drift de tenancy

**Ciclo:** 043 · **Data:** 2026-11-17 · **Baseline:** `dev@2a68f024` · **Tipo:** docs.

## Entregue

- `/llms.txt` e comentário do roteador não anunciam mais `X-Organization-ID` como auth.
- Novo teste de ausência (flora_api 28→29 testes).
- Decisão 80/20, validação, research doc.

## Evidência

`.specify/validations/043-manifest-drift/validate.md` ·
`docs/research/2026-11-17-043-manifest-tenancy-drift.md`.

## Não feito

Nenhum comportamento de runtime, RLS/role, seed, ADR, required checks, deploy/tag.

## Follow-ups

D3 (seed permission drift) e RLS/role least-privilege — decisões do dono/SE.
