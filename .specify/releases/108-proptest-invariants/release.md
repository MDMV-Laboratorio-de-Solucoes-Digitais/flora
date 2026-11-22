# Release: 108 — proptest

**Ciclo**: 108 · **Data**: 2026-11-22 · **Baseline**: `d79238dc6`

## Entregue

Quatro propriedades em `flora-core`: round-trip de `Permission`, não-concessão por string arbitrária, limites de `Pagination` e seletor wildcard. Nenhuma dependência/gate/required check/migration novo.

## SemVer

**Proposta:** PATCH `0.7.9` → `0.7.10`. Apenas testes novos; nenhum comportamento de produção muda, logo não é MINOR nem MAJOR.

## Changelog (FR / T)

`FR-001` quatro proptests em `flora-core` · `FR-002` `prop_assert`/`prop_assume` sem `unwrap`/`expect`/`panic` · `FR-003` pisos preservados. Decisão F7 do assessment; bug report não aplicável.

## Evidência

R4 APROVADO · 55 testes · 500 casos/propriedade · fmt/check/clippy exit 0 · gate PASS.

## Rollback

Novo commit de reversão dos três módulos de teste, sem force-push.

## [NV]

Nenhuma mudança de produção.
