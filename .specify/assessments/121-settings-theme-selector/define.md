# Assess — define: 121

**Data**: 2026-11-24

## Escopo fechado (`BF-04`)

- Implementar utilitários puros de tema (`apps/frontend/src/lib/utils/theme.ts`) e store reativo Svelte 5 Runes (`ThemeStore.svelte.ts`) usando a chave canônica `flora.theme` (`'light' | 'dark'`).
- Implementar componente acessível `ThemeSelector.svelte` em `/settings` (`Claro` / `Escuro` com `aria-pressed`, `role="group"`, `aria-live="polite"` e ícones `Sun`/`Moon` `aria-hidden="true"`) usando dicionário `pt-BR` (`apps/frontend/src/lib/i18n/pt-BR.ts`).
- Corrigir `:root --destructive` em `apps/frontend/src/routes/layout.css` (`0 84.2% 60.2%` → `0 84.2% 40%`) garantindo contraste WCAG 2.2 AA no tema claro preservando `.dark` intacto.
- Aplicar polimento LOW em `RoleManager.svelte`: `cancelEditing()` limpa `errorMessage`, e `$effect` limpa `errorMessage`/`successMessage` na troca/ausência de contexto organizacional.
- Atualizar Passo 9 de `apps/frontend/tests/e2e/seed-flow.e2e.ts` para clicar nos botões `Claro` e `Escuro`, validar persistência após `page.reload()`, auditar Axe WCAG 2.2 AA em ambos os temas e capturar screenshots para Gist.
- Fora de escopo: modo automático `system` com listener de `matchMedia` dinâmico no runtime (bootstrap atual em `app.html` opera com `'dark'` explícito ou padrão claro), mudanças no backend Rust ou migrations SQL.
