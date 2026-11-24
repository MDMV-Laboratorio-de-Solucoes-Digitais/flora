# Briefing de Compreensão — 122 / #420–#423

**Data**: 2026-11-24 · **Domínio crítico**: auth / rbac
**Issues**: [#420](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/420)–[#423](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/423) · **Autor**: orquestrador

## Intenção

Completar o gerenciamento RBAC multi-tenant de organizações (`BF-02 -> fechado`) adicionando listagem e adição controlada de membros existentes (`GET/POST /api/v1/roles/members`), atualização atômica de papel (`MembershipRepository::update_role`), proteção contra auto-rebaixamento e auto-revogação (`assign_role`/`revoke_role`) e a UI acessível `MemberManager.svelte` em `/settings`.

## Invariantes

- **Antes**:
  - `POST /api/v1/roles/{id}/assign` e `/revoke` existiam no backend, mas sem listagem/adição de membros, `assign_role` fazia `delete` + `create` (perdendo `joined_at`/`metadata` e permitindo adicionar não membros), `revoke_role` ignorava `{id}`, e ambos permitiam ao administrador rebaixar ou remover a si próprio.
- **Depois**:
  - **Isolamento exclusivo na sessão (`auth.require_org()`)**: todos os endpoints de membros derivam `organization_id` exclusivamente da sessão JWT + `jti` ativa no PostgreSQL, nunca de payload ou query param.
  - **Autorização granular**: `GET /api/v1/roles/members` exige membro ativo com `OrgRead` ou `OrgAdmin`; `POST /api/v1/roles/members`, `/assign` e `/revoke` exigem `RoleWrite` ou `OrgAdmin`.
  - **Proteção contra auto-lockout e vínculo cruzado**: `assign_role` e `revoke_role` rejeitam `req.user_id == auth.user_id` com `403 Forbidden`; `assign_role` exige vínculo pré-existente via `UPDATE ... WHERE user_id = $1 AND organization_id = $2`; `revoke_role` valida o papel alvo na mesma organização e exige `membership.role_id == role_id`.
  - **Proteção contra vazamento de PII/credenciais e N+1**: `GET /api/v1/roles/members` resolve usuários em lote via `UserRepository::find_by_ids` e retorna apenas `OrganizationMemberResponse` (`user_id`, `email`, `display_name`, `role_id`, `role_name`, `joined_at`), omitindo `password_hash`, `oidc_subject` e `profile`.
  - **Payloads fechados**: `AddMemberRequest` e `UserRoleRequest` usam `#[serde(deny_unknown_fields)]`.

## Decisões implícitas

1. **`add_member` opera sobre usuários ativos já registrados (`is_active == true`)**: retorna `404 UserNotFound` para e-mail inexistente ou inativo e `409 DuplicateEntry` para usuário já membro da organização; criação de contas externas segue exclusivamente pelo fluxo OIDC ou bootstrap/seed.
2. **UI espelha restrições sem relaxar servidor (`MemberManager.svelte`)**: o card do próprio usuário exibe apenas o selo `Você`, sem controles de alteração ou remoção; todas as mutações passam por autorização server-side e isolamento de contexto em `MemberStore.svelte.ts`.
3. **Elevação em lockstep dos contratos DB**: `rbac` passa de `3` para `5` (`85 -> 87` total) em `check_gate_integrity.py` e `.github/workflows/rust.yml`.

## Comando de verificação

```bash
export FLORA_DATABASE__POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:5433/flora'
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources && cargo test --workspace && cargo test -p flora-tests contract::rbac::db::db_ -- --ignored --test-threads=1 && python3 scripts/ci/check_gate_integrity.py
cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build && bun run test:e2e
```

## Pergunta-teste ao SE

**Pergunta**: Se um administrador tentar rebaixar ou revogar o próprio usuário via UI ou `POST /api/v1/roles/{id}/assign|revoke`, ou atribuir um papel de outra organização a um membro da organização atual, o servidor aceita a mutação?
**Resposta esperada**: Não. `can_change_member_role` / `can_revoke_member` retornam `false` para `target_user_id == actor_user_id` (`403 Forbidden`), e `role_repo.find_by_id_and_organization(id, org_id)` rejeita papéis de outro tenant com `404 RoleNotFound`.

## [NV]

- Execução em staging/produção externos com Coolify (`workflow_dispatch`, fora do gate de PR).

## Atualização pós-ACR Rodada 1 (`#424`, 2026-11-24)

- Ao revogar um membro (`POST /api/v1/roles/{id}/revoke`), `revoke_role` agora usa `MembershipRepository::delete_with_role` atômico e revoga imediatamente todas as sessões ativas desse usuário na organização via `SessionRepository::revoke_for_user_and_organization` (`UPDATE sessions SET is_active = false`), invalidando JWTs pré-existentes e refresh tokens na organização (mantendo eventuais sessões em outras organizações intactas).
- `flora-api/src/routes/search.rs` deixou de descartar o `Option<Membership>` e agora retorna `403 Forbidden` caso o usuário não seja membro ativo da organização; `validate_member_email` passou a retornar `422 Error::Validation`.
- Revalidação local `[EV]` completa após a remediação: `contract::rbac::db` **`5/5`**, `contract::search::db` **`8/8`**, `89/89` Vitest e Playwright E2E **`4 passed` (`15/15` WCAG `sem achados`)**.
