# Assess — decide: 121

**Data**: 2026-11-24

## Decisão

**Veredito: go**

Score 80/20 (Prompt v4.9): **1500 / 3125** (`impacto=4 × confiança=5 × fechamento=3 × (6−esforço=1)=5 × (6−risco=1)=5`), tipo `IMPLEMENTAÇÃO` + `CORREÇÃO` — fecha **`BF-04`** (seletor interativo de tema em `/settings`), resolve o contraste WCAG AA de `--destructive` no tema claro e conclui o polimento LOW pendente de `RoleManager.svelte` (ACR `#409`).

## Restrições

- Manter compatibilidade estrita com o bootstrap inline de `apps/frontend/src/app.html` (`localStorage['flora.theme']` e classe `.dark`).
- Nenhuma alteração em migrations SQL nem pisos DB (`85` pisos preservados); SemVer `MINOR` (`0.12.0 -> 0.13.0`).
- Zero binários/PNGs no repositório (`playwright-report`/`test-results` removidos antes do lint/commit; screenshots publicados em Gist público).
- R6 com 4 issues (`#415`–`#418`) sem `dark-factory:eligible`.
