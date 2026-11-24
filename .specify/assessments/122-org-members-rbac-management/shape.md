# Assess — shape: 122

**Data**: 2026-11-24

## Desenho da solução

1. **Camada de repositório (`flora-core`)**: `MembershipRepository::update_role` preserva `joined_at`/`metadata` e falha com `Error::MembershipNotFound` quando o usuário não pertence à organização.
2. **Camada HTTP RBAC (`flora-api/src/routes/rbac.rs`)**:
   - Rotas `/api/v1/roles/members` montadas antes de `/{id}` para evitar colisão de path (`Uuid` parser).
   - DTO `OrganizationMemberResponse` expõe apenas `user_id`, `email`, `display_name`, `role_id`, `role_name`, `joined_at` (nunca `password_hash`, `oidc_subject` ou `profile`).
   - Reuso de `require_org_reader` (`can_list_roles`) e `require_org_admin` (`can_manage_roles`) com tenant extraído exclusivamente da sessão (`auth.require_org()`).
   - Bloqueio explícito (`403 Forbidden`) para tentativa de alterar/revogar o próprio vínculo (`req.user_id == auth.user_id`).
3. **Frontend (`apps/frontend`)**: `MemberStore` segue o padrão `RoleStore` (`syncContext`, `loadSequence`, `pendingLoad`), e `MemberManager.svelte` exibe `Badge` `Você` para o próprio usuário sem oferecer ações destrutivas ou de auto-rebaixamento, além de sincronizar opções de papel com `roleStore.roles`.
4. **Verificação**: TDD RED→GREEN no backend (`contract::rbac::db::db_`, `rbac::tests`), lockstep `87` pisos DB, TDD RED→GREEN no frontend (`members.test.ts`), recompile `flora-api` + Playwright E2E Chromium real com `page.reload()` + Axe WCAG 2.2 AA e Gist.
