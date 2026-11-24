# Implementation Plan: `121-settings-theme-selector`

**Baseline**: `github/dev@2ff91e229` · **SemVer**: MINOR (`0.12.0 -> 0.13.0`)

## Etapas

1. **T-001 (`#415`)**: Escrever testes TDD RED (`apps/frontend/tests/unit/theme.test.ts`), verificar falha inicial, implementar `apps/frontend/src/lib/utils/theme.ts` e `apps/frontend/src/lib/state/ThemeStore.svelte.ts` (Svelte 5 Runes) e confirmar GREEN.
2. **T-002 (`#416`)**: Adicionar chaves `theme.*` em `apps/frontend/src/lib/i18n/pt-BR.ts`, criar `apps/frontend/src/lib/components/settings/ThemeSelector.svelte`, montar em `apps/frontend/src/routes/(app)/settings/+page.svelte`, ajustar `:root --destructive: 0 84.2% 40%` em `apps/frontend/src/routes/layout.css` e aplicar polimento em `apps/frontend/src/lib/components/rbac/RoleManager.svelte`.
3. **T-003 (`#417`)**: Atualizar Passo 9 de `apps/frontend/tests/e2e/seed-flow.e2e.ts` clicando em `Claro` e `Escuro`, validando `localStorage['flora.theme']`, classe `.dark`, persistência pós-`page.reload()`, auditoria WCAG 2.2 AA e capturas para Gist público.
4. **T-004 (`#418`)**: Registrar governança (`docs/governance/2026-11-24-121-settings-theme-selector.md`), R4 (`test-roadmap.md`, `evidence.md`, `validate.md`), R7 (`release.md`), docs append-only fechando `BF-04`, ACR independente (`research`/`pro`), CI remoto, merge em `dev` e atualização/escalonamento do PR de promoção para `main`.
