# Shape: 103 — RBAC na criação de workspace

**Data:** 2026-11-21 · **Branch**: `feature/103-workspace-create-rbac@14cb336173e01d63d6edb161a7b1ad09f520683b`

## Forma escolhida

Inserir no único handler carente o mesmo bloco de autorização já usado pelo irmão de escrita, sem criar superfície alguma. Em `create_workspace` (`flora-api/src/routes/workspace.rs:120-155`), depois da membership (`:132-137`), o papel é carregado com `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` (`:216-220`) e `has_permission` é chamado para `WorkspaceWrite` **ou** `WorkspaceAdmin` (`:221-223`); sem ambas, retorna `Error::Forbidden("Insufficient permissions")` (`:224`) antes da criação (`:139-147`). O resto do handler — `CreateWorkspaceInput`, serviço, `tracing::info!`, `201` e `WorkspaceResponse` — fica intacto (`:139-153`). Nenhuma importação, dependência, rota, flag, schema ou migração nova: `PgRoleRepository` e `RoleRepository` já estão em `:17-18`.

1. **Causa e rastro (T001 / intake, research)**: fixar item 11 do roteiro (`:184-188`) e a cadeia `create_workspace` sem consulta de papel (`workspace.rs:120-155`) versus update/delete com RBAC (`:208-225`, `:256-271`).
2. **Consulta de papel (T002 / FR-001, FR-002)**: reaproveitar a chamada e o erro de `update_workspace` — `find_by_id_and_organization` (`role_repository.rs:40-56`) com `Error::RoleNotFound` (`:220`) — após a membership, mantendo `Not a member` como primeira barreira (`:133-137`).
3. **Decisão de permissão e erro (T003 / FR-001, FR-003)**: `WorkspaceWrite || WorkspaceAdmin` (o par do update, não o do delete), `has_permission` com `*` e aliases legados (`role.rs:66-110`); negação com o mesmo `Error::Forbidden("Insufficient permissions")` → 403 de corpo constante (`error.rs:291-294`), sem log ou dado novo e sem chegar ao serviço de criação.
4. **Contratos positivos (T004–T005 / FR-004, SC-001)**: adaptar o helper de seed para aceitar `&[Permission]` no padrão de `rbac.rs:55-81`, adicionar POST autenticado e cobrir 201 para `WorkspaceWrite`, `WorkspaceAdmin` e papéis com `*` (seed Admin/Owner), com `WorkspaceResponse` inalterado.
5. **Contratos negativos e regressão (T006–T007 / FR-004, SC-002)**: `permissions=[]` ("NegadoQA") → 403 genérico e nenhum workspace gravado; papel de outra org → fail-closed sem criação; sem membership → 403 existente; preservar `db_workspace_list_returns_seeded_workspace`, `db_n3_cross_org_role_lookup_rejected` e `db_workspace_isolation_between_orgs` (`:134-216`).
6. **Fecho (T008)**: rastreabilidade T001..T008 em assessment/spec/plan/tasks, gates do alvo registrados no handoff **[NV]** e verificação da fronteira proibida e da ausência de commit.

- **Permissão errada (escopo excessivo)**: exigir `WorkspaceAdmin` like delete would block `Member`/`WorkspaceWrite` roles that hoje devem poder criar; mitigado adotando o par do update (`WorkspaceWrite || WorkspaceAdmin`), coerente com "escrever workspace".
- **Vazamento por mensagem**: texto customizado poderia revelar o papel ou a existência do workspace; mitigado reaproveitando `Error::Forbidden("Insufficient permissions")`, cujo corpo é constante (`error.rs:291-294`), e não acrescentando log no caminho de negação.
- **Cross-org via role_id**: consultar o papel apenas por id permitiria que um papel de outra org autorizasse a criação; mitigado por `find_by_id_and_organization` (`role_repository.rs:40-56`), o mesmo guard N3 já testado em `db_n3_cross_org_role_lookup_rejected` (`workspace.rs:158-184`).
- **Meia-correção por camada**: colocar a checagem no serviço mudaria update/delete e o contrato de domínio; mitigado mantendo a autorização na rota, como os irmãos (`workspace.rs:208-225,256-271`).
- **Compatibilidade com o seed**: papéis legados usam `WorkspaceWrite`/`WorkspaceAdmin` em `PascalCase`; `has_permission` já cobre esses aliases e `*` (`role.rs:66-110`), então Admin/Member/Owner seguem criando sem migração de dados.
- **Efeito em contratos existentes**: o helper `seed_member` atual (`contract/workspace.rs:54-103`) tem `permissions=[]` e só faz GET; nenhum contrato atual faz POST, logo a nova checagem não os quebra. O helper novo deve ser aditivo, não substitutivo.
- **Resultado em staging real e gates**: **[NV]** neste artefato — a correção não foi executada contra staging/produção e nenhum gate/build/teste foi reexecutado aqui.

## Entrega

Diff restrito a `flora-api/src/routes/workspace.rs` (consulta de papel + decisão de permissão antes da criação) e `flora-tests/src/contract/workspace.rs` (helper com permissões, POST e cenários positivos/negativos), além dos artefatos do ciclo 103. Arquivos lidos, não modificados: `flora-api/src/routes/mod.rs`, `flora-core/src/models/role.rs`, `flora-core/src/repositories/role_repository.rs`, `flora-core/src/services/workspace_service.rs`, `flora-core/src/error.rs`, `flora-tests/src/contract/rbac.rs`, `apps/frontend/src/lib/components/workspace/WorkspaceCreate.svelte` e o roteiro. Verificação documental cobre T001–T008; gates, build, testes e o roteiro em staging real ficam **[NV]** até o handoff.
