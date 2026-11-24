# Feature Spec: Seletor interativo de tema e contraste WCAG em `/settings` (`121-settings-theme-selector`)

**Ciclo**: 121 · **Item de backlog**: `BF-04` · **Status**: Em execução

## Requisitos Funcionais

- **FR-001 (Estado e persistência de tema)**: `apps/frontend/src/lib/utils/theme.ts` e `apps/frontend/src/lib/state/ThemeStore.svelte.ts` devem resolver o tema inicial (`'light' | 'dark'`) a partir de `localStorage['flora.theme']` (fallback para classe `.dark` no elemento raiz ou `'light'`), alternar `.dark` em `document.documentElement.classList` e persistir `'light'` ou `'dark'` em `flora.theme` com resiliência a ambientes SSR ou `localStorage` bloqueado.
- **FR-002 (UI acessível em `/settings` e dicionário `pt-BR`)**: `ThemeSelector.svelte` montado em `apps/frontend/src/routes/(app)/settings/+page.svelte` deve exibir controles `Claro` e `Escuro` em `pt-BR` com `role="group"`, `aria-pressed` coerente, ícones decorativos (`aria-hidden="true"`) e feedback `aria-live="polite"`.
- **FR-003 (Contraste WCAG 2.2 AA e polimento `RoleManager`)**: `apps/frontend/src/routes/layout.css` deve ajustar `:root --destructive` para `0 84.2% 40%` mantendo `.dark` intacto; `RoleManager.svelte` deve limpar `errorMessage` ao cancelar edição e limpar `errorMessage`/`successMessage` ao trocar ou perder contexto organizacional.
- **FR-004 (Cobertura TDD, E2E com `reload()`, ACR e promoção)**: TDD unitário em `tests/unit/theme.test.ts` (`RED -> GREEN`), Passo 9 de `tests/e2e/seed-flow.e2e.ts` exercitando os botões interativos `Claro`/`Escuro` e `page.reload()` com Axe WCAG 2.2 AA `sem achados` e screenshots publicados em Gist, além de artefatos R1/SDD/R4/R7 (`0.13.0`), ACR independente e promoção `dev -> main` escalada ao SWE.

## Critérios de Aceitação

- **SC-001**: Vitest `server` executa todos os casos de `theme.test.ts` após falha inicial (`RED -> GREEN`).
- **SC-002**: `bun run check`, `bun run lint` e `bun run build` saem com código `0`.
- **SC-003**: `bun run test:e2e` valida clique em `Claro` -> `page.reload()` (tema claro mantido) -> `Escuro` -> `Claro`, com `0` violações WCAG 2.2 AA em `/settings` escuro e claro.
- **SC-004**: `check_gate_integrity.py` e `check_deploy_workflows.py` passam preservando `85` pisos DB.
