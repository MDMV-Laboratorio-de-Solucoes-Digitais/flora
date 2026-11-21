# Implementation Plan: 103 — RBAC na criação de workspace

**Branch**: `feature/103-workspace-create-rbac` @ `14cb336173e01d63d6edb161a7b1ad09f520683b` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/103-workspace-create-rbac/decision.md` (go) e reprodução em staging local zerado pelo roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` (item 11)

## Summary

Fechar a lacuna de autorização em `POST /api/v1/workspaces`: `create_workspace` passa a carregar o papel da membership com `PgRoleRepository::find_by_id_and_organization` e a exigir `WorkspaceWrite || WorkspaceAdmin`, o mesmo par de `update_workspace`, antes de chamar o serviço de criação. A negação usa `Error::Forbidden("Insufficient permissions")` → 403 com corpo constante. `org_id`, membership, `RoleNotFound`, `CreateWorkspaceInput`, `WorkspaceResponse`, `201`, log de sucesso e todos os outros handlers permanecem idênticos. Nenhuma rota, flag, schema, migração, required check/gate ou permissão nova.

## Technical Context

**Root cause**: `flora-api/src/routes/workspace.rs:126-137` obtém `org_id` e verifica membership, mas `:139-153` grava e responde 201 sem jamais consultar o papel. `SomenteLeituraQA` (`permissions=[]`) é, portanto, tratado como qualquer membro. Update (`:208-225`) e delete (`:256-271`) já demonstram o padrão: `PgRoleRepository::find_by_id_and_organization` (`flora-core/src/repositories/role_repository.rs:40-56`), `has_permission` (`flora-core/src/models/role.rs:66-75`) e `Error::Forbidden("Insufficient permissions")`.

**Target Path**: `flora-api/src/routes/workspace.rs` (bloco de autorização em `create_workspace`) e `flora-tests/src/contract/workspace.rs` (helper com permissões, POST autenticado e cenários), além dos artefatos do ciclo 103. Somente leitura: `flora-api/src/routes/mod.rs`, `flora-core/src/models/role.rs`, `flora-core/src/repositories/role_repository.rs`, `flora-core/src/services/workspace_service.rs`, `flora-core/src/error.rs`, `flora-tests/src/contract/rbac.rs`, `apps/frontend/src/lib/components/workspace/WorkspaceCreate.svelte` e o roteiro.

**Testing**: com `WorkspaceWrite`/`WorkspaceAdmin`/`*` → 201 e `WorkspaceResponse` inalterado; com `permissions=[]` ("NegadoQA") → 403 `{"error":"forbidden","message":"Insufficient permissions"}` e nenhum workspace gravado; papel de outra org → `RoleNotFound`/fail-closed sem gravação; sem membership → 403 `Not a member`; contratos atuais de listagem/isolamento (`workspace.rs:134-216`) permanecem PASS. Gates do alvo no handoff **[NV]**.

**Constraints**: usar o par de permissões do update (`WorkspaceWrite || WorkspaceAdmin`), não o `WorkspaceAdmin` exclusivo do delete; manter a ordem membership → papel → criação; `has_permission` já cobre `*` e aliases legados (`role.rs:66-110`), sem migração; erro 403 genérico (`error.rs:291-294`), sem nomes/IDs no corpo ou log; sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição; sem commits.

## Steps

1. **Causa (T001)** — fixar em intake/research a cadeia do item 11 (`docs/governance/2026-11-21-roteiro-staging-flora.md:184-188`): papel `permissions=[]` → `create_workspace` sem consulta de papel (`workspace.rs:120-155`) → 201; registrar o padrão de update/delete (`:208-225,256-271`), `find_by_id_and_organization` (`role_repository.rs:40-56`), `has_permission` (`role.rs:66-110`) e o 403 constante (`error.rs:291-294`). Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).
2. **Consulta de papel (T002)** — em `create_workspace` (`workspace.rs:132-147`), manter a membership e inserir, antes do `CreateWorkspaceInput`, `let role = PgRoleRepository::new((*state.db_pool).clone()).find_by_id_and_organization(membership.role_id, org_id).await?.ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;`, espelhando `:216-220`. Nenhum import novo (`PgRoleRepository`/`RoleRepository` já em `:17-18`); nenhuma mudança em `org_id`, `Not a member` ou no serviço.
3. **Decisão de permissão (T003)** — aplicar `if !role.has_permission(Permission::WorkspaceWrite) && !role.has_permission(Permission::WorkspaceAdmin) { return Err(Error::Forbidden("Insufficient permissions".to_string())); }` (`:221-225`) antes da escrita (`:139-147`). Sem log novo e sem distinção na resposta; `201`, `WorkspaceResponse` e `tracing::info!` de sucesso (`:149-153`) intactos.
4. **Helper de contrato (T004)** — em `flora-tests/src/contract/workspace.rs`, tornar `seed_member` (`:54-103`) aditivo com parâmetro `permissions: &[Permission]` no padrão de `flora-tests/src/contract/rbac.rs:55-81` (`Role::new` + `add_permission`), preservando os chamadores atuais (`:136-216`), e acrescentar helper `authed_post` equiparável a `authed_get` (`:116-132`) com corpo JSON e header Bearer.
5. **Contratos positivos (T005)** — cobrir 201 e `WorkspaceResponse` (`id`, `organization_id`, `name`, `description`, `created_at`, `updated_at`) para papéis com `WorkspaceWrite`, `WorkspaceAdmin` e `*`; confirmar que o `organization_id` do workspace criado é o da sessão e que o log/resposta de sucesso não mudou (FR-002, SC-001).
6. **Contratos negativos e regressão (T006)** — cobrir `permissions=[]` ("NegadoQA") → 403 com corpo exato `{"error":"forbidden","message":"Insufficient permissions"}` e nenhum workspace na listagem; papel de outra org → `RoleNotFound`/fail-closed sem gravação; sem membership → 403 `Not a member`; falha de DB na consulta de papel → sem criação. Preservar `db_workspace_list_returns_seeded_workspace`, `db_n3_cross_org_role_lookup_rejected` e `db_workspace_isolation_between_orgs` (`:134-216`) (FR-001, FR-003, FR-004, SC-002).
7. **Validação documental (T007)** — confrontar os contratos com o item 11 do roteiro (`:187-188`) e registrar gates do alvo no handoff **[NV]**; nenhuma requisição ou escrita ao vivo em staging/produção neste artefato.
8. **Fecho (T008)** — rastreabilidade T001..T008 em decision/spec/plan/tasks, verificação da fronteira proibida e confirmação de que nenhum `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*` ou `AGENTS.md` foi tocado, que nenhum gate/workflow/ADR/changelog mudou e que não houve commit.

## Constitution Check

Sem violações: nenhum required check, gate, contrato de escrita de workspace, membership, regra RBAC, schema, workflow, ADR ou constituição é alterado. A verificação reaproveita repositório e enum de permissões existentes e é escopada ao org da sessão (`role_repository.rs:40-56`), não autorizando papéis de outras organizações. O erro é o mesmo de update/delete e o corpo HTTP é constante (`error.rs:291-294`), sem exposição de dados. Contrato de sucesso, formato de resposta e os demais handlers seguem idênticos; o diff fica restrito ao handler de criação, aos contratos e aos artefatos do ciclo 103.

## Project Structure

```text
.specify/assessments/103-workspace-create-rbac/{intake,research,define,shape,decision}.md
specs/103-workspace-create-rbac/
├── spec.md      # FR-001..FR-005 / SC-001..SC-003
├── plan.md      # T001..T008
└── tasks.md     # fila T001..T008 (a executar)
flora-api/src/routes/workspace.rs                    # create_workspace: papel + WorkspaceWrite|WorkspaceAdmin antes da criação
flora-api/src/routes/mod.rs                          # montagem da rota (reuso, inalterado)
flora-core/src/repositories/role_repository.rs       # find_by_id_and_organization (reuso, inalterado)
flora-core/src/models/role.rs                        # has_permission + Permission (reuso, inalterado)
flora-core/src/services/workspace_service.rs          # criação (reuso, inalterado)
flora-core/src/error.rs                              # Forbidden -> 403 / RoleNotFound -> 404 (reuso, inalterado)
flora-tests/src/contract/workspace.rs                # helper com permissões, POST e cenários de RBAC
flora-tests/src/contract/rbac.rs                     # padrão de seed com permissões (reuso, inalterado)
docs/governance/2026-11-21-roteiro-staging-flora.md  # item 11 (leitura)
```

## Complexity Tracking

Nenhuma. É um bloco de autorização idêntico ao já existente em `update_workspace`, um helper de seed com permissões e exercícios de contrato — todos verificáveis por inspeção e testes DB, sem nova dependência, migração, rota, flag ou superfície pública. A repetição do bloco nos três handlers é intencional: extrair um helper comum mudaria update/delete e ampliaria a fronteira deste ciclo, sem ganho de correção.
