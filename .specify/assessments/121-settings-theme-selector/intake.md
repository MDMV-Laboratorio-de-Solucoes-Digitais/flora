# Assess — intake: 121

**Data**: 2026-11-24 · **Ciclo**: 121 · **Slug**: `121-settings-theme-selector`

## Demanda

- **`BF-04`** (fila de fechamento de `STATE.md` / `decision.md` de `120`): entregar seletor interativo de tema claro/escuro na rota `/settings`, integrado ao mecanismo existente `localStorage['flora.theme']` e classe `.dark` em `document.documentElement`.
- **Correção de acessibilidade acoplada**: corrigir contraste WCAG 2.2 AA do token `--destructive` no tema claro (`:root` em `apps/frontend/src/routes/layout.css`, atualmente `0 84.2% 60.2%` = `3.59:1` vs `--background`), onde botões `variant="destructive"` (como `Excluir` em `RoleManager.svelte`) aparecem diretamente em `/settings`.
- **Polimento LOW do ACR do PR `#409`**: em `RoleManager.svelte`, limpar `errorMessage` ao cancelar edição e limpar `errorMessage` + `successMessage` em troca ou ausência de contexto organizacional no `$effect`.
