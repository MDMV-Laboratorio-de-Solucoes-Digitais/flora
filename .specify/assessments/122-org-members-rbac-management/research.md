# Assess — research: 122

**Data**: 2026-11-24

## Evidências coletadas no código (`github/dev@8f984adb7`)

1. `flora-api/src/routes/rbac.rs:33-34,286-325` já possui `POST /api/v1/roles/{id}/assign` e `POST /api/v1/roles/{id}/revoke`, porém:
   - `assign_role` faz `delete` + `create` (perdendo `joined_at`/`metadata` e permitindo inserir alguém que não era membro da org);
   - `revoke_role` ignora `Path(_id)` e aceita `req.user_id == auth.user_id` (risco de auto-lockout do único admin);
   - não há `GET /api/v1/roles/members` nem `POST /api/v1/roles/members` para listar membros e adicionar usuário ativo existente por e-mail;
   - `UserRoleRequest` não utiliza `#[serde(deny_unknown_fields)]`.
2. `flora-core/src/traits/repository.rs:89-111` e `flora-core/src/repositories/membership_repository.rs` possuem `find_by_organization_id`, `find_by_user_and_organization`, `create` e `delete`, além de `UserRepository::find_by_ids` e `find_by_email`, viabilizando listagem em lote sem N+1 e `update_role` atômico via `UPDATE ... WHERE user_id = $1 AND organization_id = $2 RETURNING ...` sem nenhuma migration SQL nova.
3. `dev-seed.sql:28-73,153-185` semeia `Acme Corporation` com `Alice Admin`, `Bob Builder` (`bob@acme.test`, `Member`) e `Carol Viewer` (`carol@acme.test`, `Viewer`), viabilizando teste E2E real sem mocks.
4. `scripts/ci/check_gate_integrity.py:42-54` e `.github/workflows/rust.yml:111-121` controlam `85` pisos DB (`rbac: 3`).
5. `apps/frontend/src/lib/api/roles.ts:29,41` ainda interpolava `${roleId}` diretamente (achado LOW de promoção do ciclo 121).
