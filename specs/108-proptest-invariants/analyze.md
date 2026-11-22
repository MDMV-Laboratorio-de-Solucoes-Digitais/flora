# Analyze: 108 — proptest (READ-ONLY)

**Data**: 2026-11-22 · **Baseline**: `d79238dc6`

## Diagnóstico

`proptest 1.5`/1.11 já era dependência de `flora-core`, mas nenhum `proptest!` existia. Invariantes puros escolhidos: round-trip de `Permission`, não-concessão por string arbitrária, limites de `Pagination` e equivalência do seletor wildcard a `trim() == "*"`.

## Checagem

`fr=3/3 sc=3/3 critical=0 go=True`. Nenhum `#[ignore]` novo: pisos 10 legs / 77 preservados.

## Correções

`prop_assert!` não aceita struct literal (interpretado como formato) — extraído `let resolved`; estratégia `prop_filter` não enxerga variáveis geradas no corpo — trocada por `prop_assume!`; clippy exigiu `map_or`.

## [NV]

Nenhuma mudança de produção; cobertura é evidência local.
