# Assess — define: 122

**Data**: 2026-11-24

## Escopo fechado (`BF-02 -> fechado`)

- **Backend Rust (`flora-core` + `flora-api`)**:
  - Adicionar `MembershipRepository::update_role(user_id, organization_id, role_id)` atômico (`UPDATE memberships ... RETURNING ...`).
  - Montar `GET /api/v1/roles/members` (autoridade `OrgRead | OrgAdmin`, resposta DTO sem campos sensíveis e carregamento em lote via `find_by_ids`) e `POST /api/v1/roles/members` (autoridade `RoleWrite | OrgAdmin`, adiciona usuário ativo existente por e-mail e `role_id` da mesma organização, `201 Created`, `404` se usuário/papel não existir ou usuário inativo, `409` se já for membro).
  - Endurecer `assign_role` (bloqueia auto-rebaixamento com `403`, usa `update_role` atômico apenas em membros existentes) e `revoke_role` (valida papel na organização, bloqueia auto-revogação com `403`, exige correspondência `membership.role_id == id` retornando `409 Conflict` / `Error::InvalidInput` quando divergente).
  - Adicionar 2 contratos PostgreSQL 18 (`contract::rbac::db::db_rbac_members_*`) e atualizar trava lockstep `rbac: 3 -> 5` (`85 -> 87`).
- **Frontend Svelte 5 (`apps/frontend`)**:
  - `OrganizationMemberItem`, `utils/members.ts`, `api/roles.ts` (incluindo `encodeURIComponent(roleId)`), `MemberStore.svelte.ts`, testes unitários TDD (`tests/unit/members.test.ts`), `MemberManager.svelte` + `pt-BR.ts` montado em `/settings` e extensão do Passo 9 E2E (`seed-flow.e2e.ts`) + WCAG 2.2 AA + Gist.
- **Fora de escopo**: envio de e-mail transacional SMTP para usuários sem conta (`POST /users/invite` fora de escopo sem usuário pré-cadastrado; a adição opera sobre usuários ativos já registrados, como colaboradores federados OIDC ou seed/bootstrap).
