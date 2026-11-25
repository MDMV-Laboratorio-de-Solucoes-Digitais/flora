# Assess — shape: 119

**Data**: 2026-11-23

## Desenho técnico

1. **Camada pura (`apps/frontend/src/lib/utils/roles.ts`)**: `ROLE_PERMISSION_OPTIONS` (27 strings canônicas do `Permission` sem `*`), `toRoleItem`, `canModifyRole`/`canDeleteRole`, `pickEditablePermissions`, `formatRolePermissions`, `validateRoleForm`.
2. **Cliente HTTP (`apps/frontend/src/lib/api/roles.ts`)**: `fetchRoles`, `createRole`, `updateRole` (`PATCH`), `deleteRole` (`DELETE`) sobre `ApiClient`.
3. **Store Svelte 5 (`apps/frontend/src/lib/state/RoleStore.svelte.ts`)**: estado `$state` com `syncContext`, `loadSequence`, `pendingLoad` (deduplicação), `upsert`, `remove`, `clear`.
4. **Componente & rota (`RoleManager.svelte` em `routes/(app)/settings/+page.svelte`)**: atualização manual, formulário dual criação/edição, botões `Editar` / `Excluir` exclusivos para `!role.isBuiltin` com `aria-label` compatível com WCAG 2.5.3, banner verde acessível no tema claro/escuro (`text-green-700 dark:text-green-400`) e efeito de carga único por `contextKey`.
5. **Verificação**: suíte Vitest (`tests/unit/roles.test.ts`), `svelte-check`, `eslint`/`prettier`, `vite build`, e Passo 9 em `tests/e2e/seed-flow.e2e.ts` auditando WCAG 2.2 AA em modos escuro e claro + `page.reload()`.
