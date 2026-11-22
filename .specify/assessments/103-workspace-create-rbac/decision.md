# Decision: 103 — RBAC na criação de workspace

- **Slug**: 103-workspace-create-rbac
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/103-workspace-create-rbac` @ `14cb336173e01d63d6edb161a7b1ad09f520683b`
- **Âncora**: `.specify/feature.json` → `specs/103-workspace-create-rbac`
- **Evidência**: reprodução em staging local zerado (roteiro item 11) + inspeção determinística do código
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

O roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:184-188` fixa, no item 11, que ações administrativas com o segundo usuário devem ser bloqueadas com 403 e não podem alterar dados de outra organização (`:187`). Em staging local zerado, `SomenteLeituraQA` — papel com `permissions=[]` ("NegadoQA") — consegue `POST /api/v1/workspaces` e recebe 201. A cadeia explica o fato sem falha intermitente:

1. `flora-api/src/routes/mod.rs:39` e `workspace.rs:84-92` — a rota `POST /api/v1/workspaces` aponta para `create_workspace`, sem middleware de permissão específico.
2. `flora-api/src/routes/workspace.rs:126-137` — o handler obtém `org_id` da sessão e verifica apenas a existência de membership (`Forbidden("Not a member")`).
3. `flora-api/src/routes/workspace.rs:139-153` — sem consultar o papel, monta `CreateWorkspaceInput`, chama `workspace_service.create_workspace` e devolve `201` com `WorkspaceResponse`. É o único ponto que grava e responde; a lacuna está entre `:137` e `:139`.
4. `flora-core/src/services/workspace_service.rs:29-39` — o serviço valida o input e grava; não há RBAC na camada de domínio, por desenho.
5. `flora-api/src/routes/workspace.rs:208-225` e `:256-271` — update e delete já carregam o papel com `PgRoleRepository::find_by_id_and_organization` (`flora-core/src/repositories/role_repository.rs:40-56`) e nega com `Error::Forbidden("Insufficient permissions")` (`:224`, `:270`).
6. `flora-core/src/models/role.rs:66-75,80-110,126-131,189-190` — `has_permission` reconhece `workspace:write`, `workspace:admin`, os aliases legados `WorkspaceWrite`/`WorkspaceAdmin` e `*`; com `permissions=[]`, ambas as checagens são `false`.
7. `flora-core/src/error.rs:291-294` — `Forbidden(_)` já produz 403 com corpo constante, sem vazar informação.

O isolamento entre organizações e a checagem de membership continuam vigentes; o que falta é exigir a permissão de escrita de workspace na criação, como já se exige na atualização.

## Decisão

Registrar o ciclo 103 como **RBAC na criação de workspace**, com cinco compromissos rastreáveis (FR-001..FR-005), três critérios (SC-001..SC-003) e oito tarefas (T001..T008, todas na fila de implementação neste artefato):

1. **Consulta de papel (FR-001, FR-002 / T002)**: em `create_workspace`, após a membership (`workspace.rs:132-137`), chamar `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` (`workspace.rs:216-220`, `role_repository.rs:40-56`), com `Error::RoleNotFound` quando o papel não pertence ao org — exatamente o padrão de `update_workspace`. `PgRoleRepository`/`RoleRepository` já importados em `:17-18`.
2. **Decisão e negação (FR-001, FR-003 / T003)**: exigir `role.has_permission(Permission::WorkspaceWrite) || role.has_permission(Permission::WorkspaceAdmin)` (`:221-223`), o par do update (não o `WorkspaceAdmin` exclusivo do delete); sem ambas, `Error::Forbidden("Insufficient permissions")` (`:224`) antes do serviço de criação (`:139-147`). Corpo 403 constante (`error.rs:291-294`), nenhum log ou detalhe novo. `*` e aliases legados continuam valendo (`role.rs:66-110`), preservando os papéis do seed.
3. **Contrato de sucesso inalterado (FR-002 / T002–T003)**: `org_id` da sessão, `Forbidden("Not a member")`, `RoleNotFound`, `CreateWorkspaceInput`, `WorkspaceResponse`, `tracing::info!` e `201` permanecem idênticos (`:126-153`); nenhuma rota, flag, schema, migração, required check/gate ou permissão nova.
4. **Contratos (FR-004 / T004–T007)**: `flora-tests/src/contract/workspace.rs` ganha helper de seed com `&[Permission]` (padrão `rbac.rs:55-81`) e POST autenticado; cobre 201 para `WorkspaceWrite`/`WorkspaceAdmin`/`*`, 403 genérico e nenhuma gravação para `permissions=[]` ("NegadoQA"), fail-closed para papel de outra org, 403 de membership ausente, e preserva os contratos atuais de listagem/isolamento (`:134-216`).
5. **Fronteira (FR-005 / T008)**: diff restrito a `flora-api/src/routes/workspace.rs`, `flora-tests/src/contract/workspace.rs` e artefatos do ciclo 103; sem `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição; factory somente leitura; sem commit.

## Evidência de entrada

- Baseline `14cb336173e01d63d6edb161a7b1ad09f520683b`, branch `feature/103-workspace-create-rbac` criada a partir desse HEAD.
- Roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:184-188` — item 11; reprodução relatada: `SomenteLeituraQA` (`permissions=[]`) → `POST /api/v1/workspaces` → 201 ("NegadoQA").
- `flora-api/src/routes/workspace.rs:120-155` — criação sem consulta de papel; `:208-225` e `:256-271` — padrão de RBAC em update/delete.
- `flora-core/src/repositories/role_repository.rs:40-56` — `find_by_id_and_organization`, guard de tenant do papel.
- `flora-core/src/models/role.rs:66-110,126-131,182-214` — `has_permission`, aliases legados e strings canônicas das duas permissões.
- `flora-core/src/services/workspace_service.rs:29-39` — gravação sem autorização na camada de serviço.
- `flora-core/src/error.rs:291-294,315-319` — 403 genérico e 404 de `RoleNotFound`.
- `flora-tests/src/contract/workspace.rs:54-103,134-216` e `flora-tests/src/contract/rbac.rs:55-81` — suíte a estender e padrão de seed com permissões.

## Racional

**go.** O bloqueio é consequência direta de uma lacuna verificável em um único handler: a criação não lê o papel, enquanto update e delete já o leem e negam com 403. A correção reaproveita repositório, enum de permissões, mensagem de erro e convenção de rota já presentes — sem nova superfície, migração, gate, flag ou regra de autorização. Escolher o par de permissões do update mantém coerência com a escrita de workspace e não endurece indevidamente o fluxo dos papéis `Member`/`WorkspaceWrite`. O erro é o mesmo de update/delete, com corpo HTTP constante, portanto não há vazamento de papel, usuário ou existência de recurso. O guard `find_by_id_and_organization` cobre o risco cross-org apontado pelo item 11 do roteiro. Todos os FR/SC são verificáveis por contratos DB e inspeção locais; o diff é reversível e a fronteira é preservada. Custo aceito: um bloco de autorização a mais no handler, idêntico ao já existente nos irmãos — repetição intencional, sem abstração nova.

## [NV]

- Execução do item 11 em staging real zerado após a correção e estado do DB/staging de produção: **[NV]** — a reprodução é a entrada; nenhuma requisição ou escrita ao vivo foi feita neste artefato.
- Gates, build, testes e formatters: **[NV]** neste artefato — consolidados no handoff; este documento não os reexecutou.
- Visibilidade do botão de criação na UI e RLS efetivo do PostgreSQL: **[NV]** — o roteiro aceita bloqueio por 403; a checagem de aplicação é mantida independentemente deles.
- Nomes `SomenteLeituraQA`/`NegadoQA` e demais detalhes do fixture vêm do cenário relatado; nenhum segredo, token ou dado de produção foi lido ou registrado neste ciclo.

## Fora de âmbito

Mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`; alteração de required checks, workflows, ADRs, constituição ou changelog; mudança em update/delete, `WorkspaceService`, repositórios, `has_permission`, modelo de erros, listagem/switch de workspaces ou tratamento de erro/UI do frontend; criação de migração, schema, RLS, role fixo, flag, rota ou permissão nova; deploy, escrita em staging/produção e commits.

## Handoff

`/speckit.specify` — especificar o RBAC de criação de workspace como requisitos verificáveis (FR-001..FR-005 / SC-001..SC-003) e fila T001..T008, com consulta de papel escopada ao org, exigência de `WorkspaceWrite || WorkspaceAdmin`, 403 genérico antes da gravação, contrato de sucesso inalterado, cobertura de contratos e fronteira preservada. Execução em staging e gates ficam no handoff **[NV]**.
