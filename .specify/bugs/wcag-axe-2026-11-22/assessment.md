# Bug assessment — wcag-axe-2026-11-22

**Data**: 2026-11-22 · **Origem**: PR #366, job `e2e-browser`, varredura axe-core 4.14

## Achados reais (report-only; não desativados)

- **BUG-111-01 (critical)**: `button-name` — botão ícone `.size-8.w-8` sem texto discernível, sem `aria-label`/`aria-labelledby` (WCAG 4.1.2 / wcag2a).
- **BUG-111-02 (serious)**: `color-contrast` — `#7d818a` sobre `#f8fafc` = 3.73:1, esperado ≥4.5:1 (WCAG 1.4.3 / wcag2aa).
- **BUG-111-03 (critical)**: `document-title` — documento sem `<title>` não vazio (WCAG 2.4.2 / wcag2a).

## Decisão

A varredura passa a ser **informativa** (`annotations` no smoke existente), sem desativar regras e sem falhar o CI. Os três achados ficam registrados para correção de UI em ciclo dedicado. O teste isolado que rotacionava o jti do seed foi removido: ele revogava o token usado por `seed-flow.e2e.ts`.

## Evidência

Logs do job `e2e-browser` em https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45038205571 (acesso 2026-11-22). **[NV]**: repro em staging real.

## Fechamento (2026-11-22)

BUG-111-01/02/03 corrigidos no ciclo 112, PR #370 mesclado em `dev` (`544a075ca`); axe final sem achados em `/tasks` e `/channels` (run 45048150000). **[NV]**: staging real e modo escuro.
