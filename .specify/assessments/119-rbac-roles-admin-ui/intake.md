# Assess — intake: 119

**Data**: 2026-11-23 · **Baseline**: `github/dev@df60db4b8` (`github/main@4c3e01d35`) · **Slug**: `119-rbac-roles-admin-ui`

## Origem

Fechamento do item `BF-02` (`backlog_de_fechamento` em `.specify/memory/STATE.md`): o backend já expõe CRUD autenticado e escopado por organização para papéis (`GET/POST /api/v1/roles`, `GET/PATCH/DELETE /api/v1/roles/{id}` em `flora-api/src/routes/rbac.rs` + `contract/rbac.rs` `3/3`), porém o painel `/settings` no frontend canônico (`apps/frontend`) exibia apenas criação de organizações (`OrganizationManager`) e criação de workspaces (`WorkspaceCreate`), sem qualquer interface para visualizar papéis nativos, criar papéis personalizados, editá-los ou excluí-los.

## Alvo e escopo

- Frontend canônico Svelte 5 (`$lib/{types/models.ts,utils/roles.ts,api/roles.ts,state/RoleStore.svelte.ts,components/rbac/RoleManager.svelte,i18n/pt-BR.ts}`, `routes/(app)/settings/+page.svelte`), testes unitários (`tests/unit/roles.test.ts`) e E2E Chromium/WCAG (`tests/e2e/seed-flow.e2e.ts`).
- Herança validada (Bloco 0.1): retomada limpa do worktree `119-rbac-roles-admin-ui`, corrigindo loop de `$effect` quando a lista vem vazia/erro `403`, adicionando suporte de edição (`PATCH /api/v1/roles/{id}`), deduplicação de requests em `RoleStore` e tratamento de `409 Conflict` duplicado.
