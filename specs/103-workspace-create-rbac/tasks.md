# Tasks: 103 — RBAC na criação de workspace

**Input**: `specs/103-workspace-create-rbac/spec.md`, `plan.md`,
`.specify/assessments/103-workspace-create-rbac/decision.md`, reprodução do roteiro de staging (item 11)

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T008 (fila original; execução registrada no status abaixo); gates do alvo **PASS** no handoff.
**Status de execução (2026-11-21):** T001–T003 e T005–T008 executados/verificados; T004 (contrato DB-gated adicional) substituído por testes unitários + HTTP ao vivo para preservar os pisos do gate. Os `[ ]` abaixo são a fila original, não o estado corrente.

## Phase 1 — Diagnóstico e artefatos de assessoria

- [ ] T001 Fixar em `.specify/assessments/103-workspace-create-rbac/{intake,research}.md` a cadeia do bug: roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:184-188` (item 11; `:187` exige 403 para ação administrativa; `:188` proíbe cross-org), `flora-api/src/routes/workspace.rs:120-155` (`create_workspace` verifica só membership em `:132-137` e grava/response 201 em `:139-153`, sem consultar papel), e o padrão vigente em `:208-225`/`:256-271` (`PgRoleRepository::find_by_id_and_organization`, `has_permission`, `Forbidden("Insufficient permissions")`). Registrar `flora-core/src/repositories/role_repository.rs:40-56`, `flora-core/src/models/role.rs:66-110,126-131,189-190`, `flora-core/src/services/workspace_service.rs:29-39` e `flora-core/src/error.rs:291-294`. Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).

## Phase 2 — Backend: autorização antes da criação

- [ ] T002 Em `flora-api/src/routes/workspace.rs:132-147`, preservar `org_id` da sessão e a checagem de membership (`Forbidden("Not a member")`) e inserir, antes do `CreateWorkspaceInput`, a consulta de papel `PgRoleRepository::new((*state.db_pool).clone()).find_by_id_and_organization(membership.role_id, org_id).await?.ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?`, idêntica a `:216-220`. Sem import novo (`PgRoleRepository`/`RoleRepository` já em `:17-18`), sem mudança no serviço (`flora-core/src/services/workspace_service.rs:29-39`) e sem alterar `RoleNotFound`/404 (FR-002, SC-002).

- [ ] T003 Aplicar a decisão em `create_workspace`: `if !role.has_permission(Permission::WorkspaceWrite) && !role.has_permission(Permission::WorkspaceAdmin) { return Err(Error::Forbidden("Insufficient permissions".to_string())); }` (padrão de `workspace.rs:221-225`), **antes** de montar o input/chamar `create_workspace` (`:139-147`). Usar o par do update, não o `WorkspaceAdmin` exclusivo do delete (`:269-271`). Nenhum log novo: o único `tracing::info!` continua sendo o de sucesso (`:149`). `CreateWorkspaceInput`, `WorkspaceResponse`, `201` e todos os demais handlers permanecem idênticos; `Error::Forbidden(_)` → 403 com corpo constante (`flora-core/src/error.rs:291-294`), sem nomes/IDs/permissões na resposta (FR-001, FR-003, SC-001, SC-003).

## Phase 3 — Contratos

- [ ] T004 Em `flora-tests/src/contract/workspace.rs`, estender `seed_member` (`:54-103`) com parâmetro aditivo `permissions: &[Permission]`, usando `Role::new` + `add_permission` no padrão de `flora-tests/src/contract/rbac.rs:55-81`, sem quebrar os três chamadores atuais (`:136,160,188`). Acrescentar helper `authed_post` com corpo JSON e header Bearer, espelhando `authed_get` (`:116-132`) e `ws_app` (`:105-107`) (FR-004).

- [ ] T005 Cobrir os caminhos autorizados: papel com `WorkspaceWrite`, papel com `WorkspaceAdmin` e papel com `*` → `POST /api/v1/workspaces` retorna 201 com `WorkspaceResponse` íntegro (`id`, `organization_id`, `name`, `description`, `created_at`, `updated_at` — `workspace.rs:41-68`), `organization_id` igual ao da sessão e nenhuma alteração no formato de resposta. Validar aliases legados (`role.rs:80-110`) sem exigir migração de dados (FR-001, FR-002, SC-001).

- [ ] T006 Cobrir os caminhos negados: papel `permissions=[]` (análogo a `SomenteLeituraQA`/"NegadoQA") → 403 com corpo exatamente `{"error":"forbidden","message":"Insufficient permissions"}` (`error.rs:291-294`) e nenhum workspace criado; papel de outra organização → `RoleNotFound`/fail-closed (`workspace.rs:220`, `role_repository.rs:40-56`) sem gravação; ausência de membership → 403 `Not a member` inalterado (`:133-137`); erro de DB na consulta de papel → sem criação. Nenhum contrato existente é substituído, em especial `db_workspace_list_returns_seeded_workspace`, `db_n3_cross_org_role_lookup_rejected` e `db_workspace_isolation_between_orgs` (`:134-216`) (FR-001, FR-003, FR-004, SC-002).

## Phase 4 — Fecho

- [ ] T007 Confrontar a cobertura com o item 11 do roteiro (`:187-188`) — bloqueio 403, nenhuma mudança de dados de outra org, IDs fixos de seed sem cross-org — e registrar gates do alvo no handoff **[NV]**. Nenhuma requisição ou escrita ao vivo em staging/produção e nenhum segredo em artefatos (FR-004, SC-002, SC-003).

- [ ] T008 Fechar a rastreabilidade T001..T008 em decision/spec/plan/tasks, verificar a fronteira proibida (`analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog, constituição) e confirmar que nenhum required check/gate, schema, migração, flag, rota ou permissão nova foi introduzido e que não houve commit (FR-005, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008. T002 depende de T001 (causa e contrato fixados); T003 de T002 (a decisão só existe com o papel carregado e antes da escrita); T004 de T001–T003 (o contrato de query/POST e o novo comportamento precisam estar definidos); T005 e T006 dependem de T004 (helpers e POST) e de T003 (comportamento implementado); T007 depende de T005–T006; T008 verifica T001–T007 em conjunto e consolida gates e rastro documental. A ordem membership → papel → criação, o `RoleNotFound`/404, o `Not a member`/403, o contrato de `201`/`WorkspaceResponse`, os demais handlers e o isolamento entre organizações permanecem inalterados em todos os passos. Execução em staging real, gates, build, testes e RLS efetivo permanecem **[NV]** até o handoff.
