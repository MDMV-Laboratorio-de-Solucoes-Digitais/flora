# Assess — research: 119

**Data**: 2026-11-23

## Fatos verificados no repositório

- **`flora-api/src/routes/rbac.rs`**:
  - `GET /api/v1/roles`: exige `auth.require_org()`, membership ativa e permissão `OrgRead` ou `OrgAdmin` (`Permission::OrgAdmin` cobre `OrganizationAdmin` do `dev-seed.sql`). Retorna `Vec<RoleResponse>` (`id`, `name`, `description`, `permissions`, `is_builtin`) sem vazar `organization_id` nem timestamps.
  - `POST /api/v1/roles`: exige `RoleWrite` ou `OrgAdmin`, valida nome não vazio e cada string via `Permission::from_str` (strings canônicas kebab-case em `flora-core/src/models/role.rs`, total `27` permissões canônicas). Retorna `201 Created`.
  - `PATCH /api/v1/roles/{id}`: exige `RoleWrite` ou `OrgAdmin`, proíbe editar papéis nativos (`is_builtin == true` -> `403 Forbidden`), atualiza `name`, `description` e `permissions`. Retorna `200 OK`.
  - `DELETE /api/v1/roles/{id}`: exige `RoleWrite` ou `OrgAdmin`, proíbe excluir papéis nativos (`is_builtin == true` -> `403 Forbidden`). Retorna `204 No Content`.
- **`dev-seed.sql`**: semeia 3 papéis nativos em `Acme Corporation` (`Admin` para Alice com `RoleWrite`/`OrganizationAdmin`, `Member` para Bob com apenas `RoleRead`, `Viewer`), todos com `is_builtin = true`.
- **`apps/frontend/src/lib/i18n/index.ts`**: `CODE_TO_KEY['conflict']` e `STATUS_TO_KEY[409]` apontam para `'error.conflict'` (`"Este workspace já foi configurado..."`), portanto o painel RBAC deve mapear `409`/`conflict` para uma mensagem específica `'roles.duplicateName'` em pt-BR.
