# Assess — research: 121

**Data**: 2026-11-24

## Evidências coletadas no código (`github/dev@2ff91e229`)

1. `apps/frontend/src/app.html:16-22` já aplica `document.documentElement.classList.add('dark')` no boot inicial quando `window.localStorage.getItem('flora.theme') === 'dark'`, com `try/catch` para contextos restritos.
2. Busca por `flora.theme` em `apps/frontend` mostra apenas `app.html` e manipulação manual nos testes E2E (`seed-flow.e2e.ts:44,396`); não existe ainda store nem controle interativo na UI.
3. `apps/frontend/src/routes/layout.css:56` define `:root --destructive: 0 84.2% 60.2%` (~`#EF4444`), que produz contraste `3.59:1` sobre `--background` (`210 40% 98%`) e `3.76:1` sobre `--card` (`0 0% 100%`), abaixo de `4.5:1` (WCAG 2.2 AA). Em `.dark` (`layout.css:78`), `--destructive: 0 90.6% 70.8%` já atende contraste sobre fundo escuro. Ajustar `:root --destructive: 0 84.2% 40%` eleva o contraste para `6.24:1` sobre `--background` e `6.53:1` sobre `--card`.
4. `apps/frontend/src/lib/components/rbac/RoleManager.svelte:40-45,133-146,247`: `resetForm()` não limpa `errorMessage`, e o `$effect` de mudança/ausência de contexto organizacional não limpa `errorMessage`/`successMessage`.
5. `apps/frontend/src/routes/(app)/settings/+page.svelte` compõe `<OrganizationManager />`, `<WorkspaceCreate />` e `<RoleManager />` usando cartões `<section class="space-y-4 rounded-lg border p-6 shadow-sm">` e dicionário `pt-BR`.
