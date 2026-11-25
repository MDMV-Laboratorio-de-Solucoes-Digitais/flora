# Release: 119 — UI CRUD de administração de papéis RBAC em `/settings`

**Ciclo**: 119 · **Data**: 2026-11-24 · **Baseline**: `github/dev@42c38bf52`

## Entregue

- `FR-001` / `T-001` ([#405](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/405)): interface `RoleItem` (`apps/frontend/src/lib/types/models.ts`), utilitários puros (`src/lib/utils/roles.ts`: `ROLE_PERMISSION_OPTIONS` com as 27 permissões canônicas sem `*`, `toRoleItem`, `canModifyRole`, `canDeleteRole`, `pickEditablePermissions`, `formatRolePermissions`, `validateRoleForm`), cliente API (`src/lib/api/roles.ts`: `fetchRoles`, `createRole`, `updateRole`, `deleteRole`), store Svelte 5 `RoleStore.svelte.ts` (isolamento multi-contexto por `syncContext`/`loadSequence` e deduplicação `pendingLoad`) e `9` testes unitários em `tests/unit/roles.test.ts`.
- `FR-002` / `T-002` ([#406](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/406)): componente `RoleManager.svelte` (`apps/frontend/src/lib/components/rbac/RoleManager.svelte`) montado em `src/routes/(app)/settings/+page.svelte`, chaves `pt-BR` em `src/lib/i18n/pt-BR.ts` (listagem com badges `Nativo`/`Personalizado`, recarga manual, criação `POST`, edição `PATCH`, exclusão `DELETE`, bloqueio visual de nativos, carga única por contexto sem loop infinito, tratamento de `403`/`409`) e correção de contraste WCAG 2.2 AA em tema claro em `src/routes/layout.css` (`:root --primary` e `--ring`: `152 76% 39%` → `152 76% 25%`, `2.72:1` → `5.85:1`, mantendo `.dark` intacto).
- `FR-003` / `T-003` ([#407](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/407)): Passo 9 de `apps/frontend/tests/e2e/seed-flow.e2e.ts` cobrindo `/settings` em tema `.dark` e tema claro com Axe WCAG 2.2 AA, criação (`POST 201`), `page.reload()`, edição (`PATCH 200`), exclusão (`DELETE 204`) e geração de capturas `18-settings-roles-dark.png` e `19-settings-roles-light.png`.
- `FR-004` / `T-004` ([#408](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/408)): Briefing de Compreensão (`auth`/`rbac`), governança, roteiro/evidências/validação R4, release R7 (`0.11.0`), docs append-only e ACR independente.

## SemVer

MINOR `0.10.1` → `0.11.0`: nova funcionalidade visível ao usuário na interface web (`/settings` — administração CRUD completa de papéis RBAC personalizados e proteção visual de papéis nativos) acompanhada de ajuste de token `--primary`/`--ring` em `:root` para conformidade WCAG 2.2 AA, sem breaking change em APIs REST nem migrations SQL (`82` pisos DB preservados).

## Evidência

Portas locais verdes (`task-1679`, `task-1692` sobre `github/dev@42c38bf52`): `svelte-check` (`0 errors and 0 warnings`), `prettier --check . && eslint .` (exit `0`), Vitest `server` (`Test Files 19 passed (19)`, `Tests 80 passed (80)` incl. `9/9` em `roles.test.ts`), `vite build` (`✓ built in 15.92s`), `check_gate_integrity.py` (`gate-integrity: PASS`, `82` pisos), `check_deploy_workflows.py` + `--self-test` (PASS), e Playwright E2E Chromium real + Postgres 18.7 (`4 passed (59.9s)`, **`14/14` auditorias WCAG 2.2 AA: `sem achados`**) com capturas publicadas em <https://gist.github.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468>. Evidências de CI remoto e ACR são registradas em `.specify/validations/119-rbac-roles-admin-ui/evidence.md`.

## Rollback

Novo commit de reversão removendo `RoleManager.svelte`, `roles.ts`, `RoleStore.svelte.ts`, `utils/roles.ts`, `tests/unit/roles.test.ts` e restaurando `models.ts`, `settings/+page.svelte`, `layout.css`, `pt-BR.ts` e `seed-flow.e2e.ts`; nenhuma migration SQL nem piso DB (`82`) foi alterado, e zero dependência externa foi adicionada.

## [NV]

Execução remota em CI do PR em `dev` (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`, `e2e-browser`), ACR independente e deploy manual em staging/produção via Coolify (`workflow_dispatch`).
