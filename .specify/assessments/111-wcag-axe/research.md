# Assess — research: 111

**Data**: 2026-11-22 · **Cobertura idiomática**: pt-BR (primária) + en (documentação técnica)

## Evidência local

- `apps/frontend/package.json`: `@playwright/test` 1.60, Vitest 4, sem `axe-core`.
- `apps/frontend/tests/e2e/*.e2e.ts`: smoke autenticado via `playwright/.auth/alice.json`.
- `.github/workflows/rust.yml:149`: job `e2e-browser` é advisory; required checks são `frontend-build`, `rust-build`, `db-tests`, `dependency-review`.
- Assessment `avaliacao-arquitetura-multiagente-flora/decision.md:29,41`: F9 ADAPT, FR-06/FR-08 WCAG 2.2 AA, FR-07 revisão humana permanece.

## Fontes primárias

- axe-core / @axe-core/playwright — https://github.com/dequelabs/axe-core-npm (documentação do pacote, acesso em 2026-11-22).
- WCAG 2.2 — https://www.w3.org/TR/WCAG22/ (Recomendação W3C, 2023-10-05).

## Triangulação

pt-BR: Assessment e constituição do Flora. en: W3C e Deque. Nenhuma afirmação carregada rely apenas em fonte secundária.

## [NV]

Comportamento em produção/staging real; jobs remotos após o merge.
