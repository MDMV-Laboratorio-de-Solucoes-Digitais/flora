# Intake: 103 — RBAC na criação de workspace

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/103-workspace-create-rbac@14cb336173e01d63d6edb161a7b1ad09f520683b` · `.specify/feature.json` → `specs/103-workspace-create-rbac`

## Sinal

Bug reproduzido em staging local zerado seguindo o roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` item 11 (`:184-188`): o usuário `SomenteLeituraQA`, cujo papel `SomenteLeituraQA` tem `permissions=[]` ("NegadoQA"), consegue `POST /api/v1/workspaces` e recebe **201**, criando um workspace. O item 11 exige que "ações administrativas bloqueadas (403) ou escondidas; nenhuma mudança de dados de outra organização" (`:187`). A criação de workspace é uma ação de escrita administrativa e, hoje, escapa ao controle que já incide sobre update e delete.

A causa é determinística e localiza-se em um único handler. `create_workspace` (`flora-api/src/routes/workspace.rs:120-155`) verifica **apenas** a existência de membership no org (`:132-137`) e chama diretamente `workspace_service.create_workspace` (`:139-147`), devolvendo `201 Created` (`:150-153`). Não há consulta ao papel nem chamada a `has_permission`. Em contraste, `update_workspace` (`:188-234`) e `delete_workspace` (`:236-280`) carregam o papel com `PgRoleRepository::find_by_id_and_organization` (`:216-220`, `:264-268`) e exigem `WorkspaceWrite || WorkspaceAdmin` (`:221-225`) ou `WorkspaceAdmin` (`:269-271`), respectivamente, retornando `Error::Forbidden("Insufficient permissions")` (`:224`, `:270`). A capacidade de RBAC já existe e não é usada na criação; o reparo é alinhar esse handler ao padrão dos irmãos, sem superfície nova.

## Evidência

```
ROTEIRO (reprodução em staging local zerado):
docs/governance/2026-11-21-roteiro-staging-flora.md:184-188  item 11 — RBAC / isolamento;
                                                    :186     tentativa de ação administrativa com 2º usuário;
                                                    :187     esperado: 403 ou ação escondida, nenhuma mudança cross-org;
                                                    :188     IDs fixos de seed não permitem cross-org
Observação relatada: SomenteLeituraQA (permissions=[]) -> POST /api/v1/workspaces -> 201 ("NegadoQA")

ROTA E HANDLER SEM RBAC:
flora-api/src/routes/mod.rs:39                    nest("/api/v1/workspaces", workspace::create_workspace_router())
flora-api/src/routes/workspace.rs:84-92            create_workspace_router(): POST / -> create_workspace
flora-api/src/routes/workspace.rs:120-155          handler create_workspace
flora-api/src/routes/workspace.rs:132-137          só checagem de membership -> Forbidden("Not a member")
flora-api/src/routes/workspace.rs:139-147          CreateWorkspaceInput + workspace_service.create_workspace
flora-api/src/routes/workspace.rs:150-153          StatusCode::CREATED + WorkspaceResponse

PADRÃO JÁ VIGENTE NOS IRMÃOS:
flora-api/src/routes/workspace.rs:208-225          update: membership -> PgRoleRepository -> has_permission(WorkspaceWrite|WorkspaceAdmin) -> Forbidden
flora-api/src/routes/workspace.rs:256-271          delete: membership -> PgRoleRepository -> has_permission(WorkspaceAdmin) -> Forbidden
flora-api/src/routes/workspace.rs:17-18            imports de PgRoleRepository e RoleRepository já presentes no módulo

CAPACIDADE DE DOMÍNIO (sem alteração):
flora-core/src/repositories/role_repository.rs:40-56   find_by_id_and_organization(id, org_id) — escopo por organização
flora-core/src/models/role.rs:66-75                     has_permission: "*" | canônico | legado PascalCase
flora-core/src/models/role.rs:126-131,189-190            WorkspaceWrite / WorkspaceAdmin -> "workspace:write" / "workspace:admin"
flora-core/src/models/role.rs:80-110                     aliases legados "WorkspaceWrite" / "WorkspaceAdmin"
flora-core/src/services/workspace_service.rs:29-39      create_workspace valida input e grava — sem checagem de permissão

ERRO E CONTRATO:
flora-core/src/error.rs:291-294                      Forbidden(_) -> HTTP 403, corpo constante {"error":"forbidden","message":"Insufficient permissions"}
flora-core/src/error.rs:315-319                      RoleNotFound -> 404 not_found
flora-tests/src/contract/workspace.rs:54-103          seed_member com Role::new (permissions=[]) e sessão JWT
flora-tests/src/contract/workspace.rs:134-216          contratos DB de listagem e isolamento (a preservar)
flora-tests/src/contract/rbac.rs:55-81                 helper seed_member(..., permissions: &[Permission]) — padrão de papel com permissões
```

Nenhuma execução de gates, build, testes ou deploy foi feita neste artefato; o comportamento observado vem do roteiro e a cadeia de código foi inspecionada localmente. O resultado em staging real permanece **[NV]** até o handoff. Os nomes `SomenteLeituraQA` e "NegadoQA" vêm do cenário relatado; nenhum segredo, token ou dado de produção foi lido ou registrado aqui.

## Autorização

Modo autônomo. Decisão `go` registrada em `.specify/assessments/103-workspace-create-rbac/decision.md` antes da implementação. Escopo: (1) em `create_workspace`, após a checagem de membership existente, carregar o papel com `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` e exigir `role.has_permission(WorkspaceWrite) || role.has_permission(WorkspaceAdmin)`, reaproveitando o padrão de `update_workspace`; (2) negar com `Error::Forbidden("Insufficient permissions")` → 403, corpo genérico, sem vazar informação; (3) manter membership, corpo da requisição, `WorkspaceResponse`, `201` e logging atuais; (4) cobrir os cenários com contratos DB em `flora-tests/src/contract/workspace.rs`. Nenhum novo required check/gate, migration, schema, flag, rota ou required permission.

## Fronteira

Backend Rust (`flora-api/src/routes/workspace.rs`) e contratos (`flora-tests/src/contract/workspace.rs`), além dos artefatos do ciclo 103. Somente leitura: `flora-api/src/routes/mod.rs`, `flora-core/src/models/role.rs`, `flora-core/src/repositories/role_repository.rs`, `flora-core/src/services/workspace_service.rs`, `flora-core/src/error.rs`, `flora-tests/src/contract/rbac.rs` e o roteiro. Sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição. A factory `mdmv-dark-factory` é somente leitura. Nenhum commit é feito neste ciclo.
