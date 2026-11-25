# Release: 121 — Seletor interativo de tema e contraste WCAG em `/settings`

**Ciclo**: 121 · **Data**: 2026-11-24 · **Baseline**: `github/dev@2ff91e229` · **Fecha**: `BF-04`

## Entregue

- `FR-001` / `T-001` ([#415](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/415)): utilitários puros (`apps/frontend/src/lib/utils/theme.ts`: `THEME_STORAGE_KEY = 'flora.theme'`, `parseThemePreference`, `resolveInitialTheme`, `syncThemeClass`, `applyThemePreference` compatíveis com SSR/storage restrito), store reativo Svelte 5 Runes (`src/lib/state/ThemeStore.svelte.ts`) e `4` testes unitários TDD (`tests/unit/theme.test.ts`, `RED -> GREEN`).
- `FR-002` + `FR-003` / `T-002` ([#416](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/416)): chaves `theme.*` em `src/lib/i18n/pt-BR.ts`, componente `ThemeSelector.svelte` (`Claro`/`Escuro`, `aria-pressed`, `role="group"`, `aria-live="polite"`) montado em `src/routes/(app)/settings/+page.svelte`, ajuste WCAG 2.2 AA de `:root --destructive` (`0 84.2% 60.2%` → `0 84.2% 40%`) em `src/routes/layout.css` (preservando `.dark`) e polimento LOW de `RoleManager.svelte` (`cancelEditing` limpando `errorMessage` e `$effect` limpando `errorMessage`/`successMessage` na troca/ausência de contexto).
- `FR-004` / `T-003` ([#417](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/417)): Passo 9 de `tests/e2e/seed-flow.e2e.ts` atualizado para alternar tema por clique real nos botões `Claro`/`Escuro`, verificar persistência em `localStorage['flora.theme']` após `page.reload()`, auditar Axe WCAG 2.2 AA em ambos os modos e gerar `18-settings-roles-dark.png` e `19-settings-roles-light.png`.
- `FR-004` / `T-004` ([#418](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/418)): governança, R1, SDD, R4, R7 (`0.13.0`), docs append-only fechando `BF-04`, ACR independente e promoção `dev -> main`.

## SemVer

MINOR `0.12.0` → `0.13.0`: nova funcionalidade visível ao usuário em `/settings` (seletor interativo de tema claro/escuro persistido entre reloads) acompanhada de correção de contraste WCAG `--destructive` no tema claro e polimento de estado em `RoleManager.svelte`, sem breaking change em APIs REST nem migrations SQL (`85` pisos DB preservados).

## Evidência

Portas locais verdes (`task-2015`, `task-2049`, `task-2063` sobre `github/dev@2ff91e229`): `svelte-check` (`0 errors and 0 warnings`), `prettier --check . && eslint .` (exit `0`), Vitest `server` (`Test Files 20 passed (20)`, `Tests 84 passed (84)` incl. `4/4` em `theme.test.ts`), `vite build` (`✓ built in 1m 2s`), `check_gate_integrity.py` (`gate-integrity: PASS`, `85` pisos intactos), `check_deploy_workflows.py` + `--self-test` (`PASS`), e Playwright E2E Chromium real + Postgres 18.7 (`4 passed (1.4m)`, **`14/14` auditorias WCAG 2.2 AA: `sem achados`**) com capturas publicadas em <https://gist.github.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf>. Evidências de CI remoto e ACR são registradas em `.specify/validations/121-settings-theme-selector/evidence.md`.

## Rollback

Novo commit de reversão removendo `utils/theme.ts`, `ThemeStore.svelte.ts`, `ThemeSelector.svelte`, `tests/unit/theme.test.ts` e restaurando `pt-BR.ts`, `settings/+page.svelte`, `layout.css`, `RoleManager.svelte` e `seed-flow.e2e.ts`; nenhuma migration SQL, dependência externa ou piso DB (`85`) foi alterado.

## [NV]

- CI remoto pós-push/PR em `dev`, ACR independente e pipeline de promoção/deploy manual em staging/produção via Coolify (`workflow_dispatch`).
