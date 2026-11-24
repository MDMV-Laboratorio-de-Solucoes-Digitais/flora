# Governança 121 — Seletor interativo de tema e conformidade WCAG `--destructive` em `/settings`

**Data**: 2026-11-24 · **Ciclo**: 121 · **Fechamento**: `BF-04 -> fechado`

## Decisões Arquiteturais e de Acessibilidade

1. **Chave canônica única (`flora.theme`)**: `ThemeStore.svelte.ts` e `utils/theme.ts` compartilham a constante `THEME_STORAGE_KEY = 'flora.theme'` com o script inline de `apps/frontend/src/app.html`, aceitando `'light'` ou `'dark'` e tratando falhas de armazenamento em `try/catch`.
2. **Svelte 5 Runes e i18n pt-BR**: `ThemeSelector.svelte` usa apenas `$state`/event props (`onclick`) e strings em `apps/frontend/src/lib/i18n/pt-BR.ts`, com `role="group"`, `aria-pressed` e região `aria-live="polite"`.
3. **Correção WCAG 2.2 AA (`:root --destructive`)**: Ajuste pontual em `apps/frontend/src/routes/layout.css` (`0 84.2% 60.2%` → `0 84.2% 40%`) apenas no tema claro (`:root`), mantendo `.dark` (`0 90.6% 70.8%`) preservado.
4. **Polimento `RoleManager.svelte`**: Separação explícita de `cancelEditing()` (limpa formulário + `errorMessage`) e limpeza de `errorMessage`/`successMessage` no `$effect` quando muda ou limpa o contexto organizacional, preservando feedback de sucesso após mutações no mesmo contexto.
