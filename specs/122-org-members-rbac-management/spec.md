# Feature Spec: Gestão de membros e papéis RBAC em `/settings` (`122-org-members-rbac-management`)

**Ciclo**: 122 · **Item de backlog**: `BF-02` · **Status**: Em execução

## Requisitos Funcionais

- **FR-001 (Repositório e endpoints de membros RBAC)**: `MembershipRepository::update_role` deve atualizar `role_id` atomicamente apenas para vínculos `(user_id, organization_id)` existentes. `flora-api/src/routes/rbac.rs` deve expor `GET /api/v1/roles/members` (`OrgRead|OrgAdmin`, batch sem N+1, sem campos sensíveis) e `POST /api/v1/roles/members` (`RoleWrite|OrgAdmin`, usuário ativo existente + `role_id` da org ativa, `201 Created`, `409` duplicado, `404` ausente/inativo).
- **FR-002 (Hardening de `/roles/{id}/assign` e `/roles/{id}/revoke`)**: `assign_role` deve exigir vínculo existente (via `update_role`) e rejeitar auto-rebaixamento (`req.user_id == auth.user_id`) com `403 Forbidden`; `revoke_role` deve validar o papel alvo na org, exigir que o vínculo atual pertença ao papel informado e rejeitar auto-revogação com `403 Forbidden`. Ambos os payloads aceitam apenas campos conhecidos (`#[serde(deny_unknown_fields)]`).
- **FR-003 (Cliente API, Store Svelte 5 Runes e UI `/settings`)**: `apps/frontend` deve expor `OrganizationMemberItem`, `utils/members.ts`, `api/roles.ts` (com `encodeURIComponent(roleId)`), `MemberStore.svelte.ts` isolado por contexto e `MemberManager.svelte` em `/settings` (listagem, alteração de papel, remoção, formulário de adição de membro existente, selo `Você` sem auto-ações e i18n `pt-BR`).
- **FR-004 (Contratos DB, TDD, E2E com `reload()`, Briefing e Promoção)**: Adicionar `+2` contratos PG 18 (`rbac: 3 -> 5`, total `85 -> 87`), testes TDD backend/frontend (`RED -> GREEN`), estender Passo 9 de `seed-flow.e2e.ts` exercitando atribuição/persistência pós-`reload()`/revogação/readição com Axe WCAG 2.2 AA `sem achados`, publicar capturas em Gist e gerar Briefing `auth`/`rbac`, R4/R7 (`0.14.0`), ACR e escalonamento no PR `#410`.
