# Assess — shape: 121

**Data**: 2026-11-24

## Desenho da solução

1. **Camada pura (`apps/frontend/src/lib/utils/theme.ts`)**: funções injetáveis (`parseThemePreference`, `resolveInitialTheme`, `applyThemePreference`) com tratamento `try/catch` em leitura/escrita de `localStorage` e compatibilidade SSR (sem acessar `window`/`document` fora de guardas).
2. **Estado Svelte 5 (`apps/frontend/src/lib/state/ThemeStore.svelte.ts`)**: `$state<ThemePreference>('light')`, `syncFromEnvironment()` sincronizando com `flora.theme` / `.dark` no cliente e `setTheme(next)` atualizando estado reativo, `documentElement.classList.toggle('dark', ...)` e `localStorage['flora.theme']`.
3. **UI (`apps/frontend/src/lib/components/settings/ThemeSelector.svelte`)**: card em `/settings` com botões `Claro` e `Escuro` (`aria-pressed`), região `aria-live="polite"` anunciando o estado atual em `pt-BR`.
4. **WCAG e Polimento (`layout.css`, `RoleManager.svelte`)**: `:root --destructive: 0 84.2% 40%` (`> 6.2:1` em `--background`/`--card`, `> 5.2:1` em `bg-destructive/10`) e limpeza determinística de `errorMessage`/`successMessage` sem apagar confirmação imediata de criação/edição.
5. **Verificação**: TDD unitário (`tests/unit/theme.test.ts`), `svelte-check`, `prettier`/`eslint`, `vite build`, `check_gate_integrity.py` (`85` pisos intactos) e Playwright E2E real com `@axe-core/playwright` em `.dark` e claro + `page.reload()`.
