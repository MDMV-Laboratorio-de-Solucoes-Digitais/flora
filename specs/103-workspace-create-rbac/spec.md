# Feature Specification: 103 — RBAC na criação de workspace

**Feature Branch**: `feature/103-workspace-create-rbac`

**Created**: 2026-11-21

**Status**: Specified — `go` para implementação (decisão em `.specify/assessments/103-workspace-create-rbac/decision.md`)

**Input**: reprodução em staging local zerado pelo roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` (item 11) e decisão `go` do ciclo 103

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Negar criação de workspace a papel sem permissão (Priority: P1)

Como administrador/mantenedor, preciso que um usuário com papel somente leitura (`permissions=[]`, "NegadoQA") não consiga criar workspaces, porque hoje `POST /api/v1/workspaces` devolve 201 e grava um workspace mesmo para `SomenteLeituraQA`.

**Why this priority**: é o comportamento exigido pelo item 11 do roteiro (`docs/governance/2026-11-21-roteiro-staging-flora.md:187`): "ações administrativas bloqueadas (403) ou escondidas; nenhuma mudança de dados de outra organização". Criar workspace é escrita administrativa; update e delete já são protegidos (`flora-api/src/routes/workspace.rs:208-225,256-271`), e a exceção na criação é o bug.

**Independent Test**: autenticar um usuário cujo papel tenha `permissions=[]`, chamar `POST /api/v1/workspaces` com `{"name":"..."}` e confirmar 403 com corpo `{"error":"forbidden","message":"Insufficient permissions"}` e nenhum workspace criado; repetir com papel que tenha `WorkspaceWrite` ou `WorkspaceAdmin` e confirmar 201.

**Acceptance Scenarios**:

1. **Given** um membro autenticado do org com papel `permissions=[]` (`SomenteLeituraQA`), **When** `POST /api/v1/workspaces` é chamado, **Then** a resposta é 403 com o corpo genérico de `Error::Forbidden` (`flora-core/src/error.rs:291-294`) e nenhum workspace é gravado.
2. **Given** um membro com papel que contém `WorkspaceWrite` (canônico `workspace:write`, legado `WorkspaceWrite` ou `*`), **When** a criação é chamada, **Then** a resposta é 201 com `WorkspaceResponse` idêntico ao atual (`flora-api/src/routes/workspace.rs:40-68,150-153`).
3. **Given** um membro com papel que contém apenas `WorkspaceAdmin`, **When** a criação é chamada, **Then** a resposta é 201 — a criação exige o par de permissões do update, não a exigência exclusiva do delete (`:221-225`, `:269-271`).

---

### User Story 2 — Manter o contrato de sucesso e o fail-closed (Priority: P1)

Como mantenedor, preciso que a nova checagem não altere o contrato de criação nem enfraqueça as barreiras já existentes, e que um papel de outra organização não autorize a criação.

**Why this priority**: `org_id` da sessão, membership, `RoleNotFound`, `CreateWorkspaceInput`, `WorkspaceResponse`, `201` e o log de criação (`workspace.rs:126-153`) são contratos consumidos pela UI (`apps/frontend/src/lib/components/workspace/WorkspaceCreate.svelte:12-26`) e por futuros clientes. O item 11 também exige que IDs fixos/roles de outra org não permitam cross-org (`:188`).

**Independent Test**: chamar a criação com papéis válidos e comparar a resposta 201 com a atual; chamar com `role_id` de papel de outra organização e com usuário sem membership, confirmando falha fail-closed e nenhuma gravação.

**Acceptance Scenarios**:

1. **Given** um membro com permissão suficiente, **When** `POST /api/v1/workspaces` é chamado, **Then** o `org_id` do workspace criado é o da sessão, `WorkspaceResponse` mantém `id`, `organization_id`, `name`, `description`, `created_at` e `updated_at` (`workspace.rs:41-67`), e o status é 201.
2. **Given** um usuário sem membership no org da sessão, **When** a criação é chamada, **Then** permanece o `Error::Forbidden("Not a member")` atual (`:133-137`), sem consulta de papel e sem gravação.
3. **Given** um `membership.role_id` que não pertence ao org da sessão, **When** a criação é chamada, **Then** `find_by_id_and_organization` (`flora-core/src/repositories/role_repository.rs:40-56`) não resolve o papel e o handler devolve `Error::RoleNotFound` (`:220`, 404), sem criar workspace — o mesmo guard N3 já testado em `flora-tests/src/contract/workspace.rs:158-184`.

---

### User Story 3 — Erro genérico, sem vazamento (Priority: P2)

Como mantenedor, preciso que a negação seja indistinguível das demais negações genéricas da API, para não revelar papel, usuário, organização ou existência de recurso.

**Why this priority**: o item 11 do roteiro combina bloqueio com isolamento (`:187`); uma mensagem específica sobre o papel ou o workspace criaria um canal de informação. Update e delete já usam o mesmo erro (`workspace.rs:224,270`), e `Forbidden(_)` já serializa um corpo constante (`error.rs:291-294`).

**Independent Test**: comparar os corpos de 403 de criação, update e delete para um papel sem permissão e verificar que são idênticos e não contêm nomes ou IDs.

**Acceptance Scenarios**:

1. **Given** negação por falta de `WorkspaceWrite`/`WorkspaceAdmin` na criação, **When** a resposta é inspecionada, **Then** é `403` com `{"error":"forbidden","message":"Insufficient permissions"}`, idêntica à de update/delete, sem nomes, IDs ou lista de permissões.
2. **Given** a negação, **When** os logs são inspecionados, **Then** nenhuma linha nova é emitida pelo caminho de erro — o único `tracing::info!` do handler continua sendo o de sucesso (`:149`).

---

### Edge Cases

- Papel com `permissions=[]`: `has_permission` retorna `false` para ambas (`flora-core/src/models/role.rs:66-75`) → 403, antes de qualquer escrita.
- Papel com apenas `WorkspaceRead`: insuficiente; a criação exige escrita, não leitura.
- Permissões em `*` ou no formato legado `PascalCase`: aceitas por `has_permission` (`role.rs:62-110`); nenhuma migração de dados é necessária.
- `role_id` de outra organização: `find_by_id_and_organization` retorna `None` → `RoleNotFound` (404), sem criação e sem revelar dados do papel alheio.
- Usuário sem membership: `Forbidden("Not a member")` permanece como primeira barreira (`:133-137`).
- Falha de DB na consulta de papel: propagada pelo `?` existente (`:219`), sem criação — nenhum fallback que conceda acesso.
- `CreateWorkspaceInput` inválido: a validação atual do serviço (`flora-core/src/services/workspace_service.rs:29-39`) permanece; a checagem de permissão ocorre antes da chamada ao serviço.
- Contratos atuais de listagem, get, switch, update e delete: inalterados, incluindo `flora-tests/src/contract/workspace.rs:134-216`.
- Execução do roteiro em staging real, gates, build, testes e RLS efetivo: **[NV]** neste artefato, registrados no handoff.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001 — checagem de permissão na criação**: `POST /api/v1/workspaces` (`flora-api/src/routes/workspace.rs:120-155`) DEVE, após a membership (`:132-137`), carregar o papel via `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` (`:216-220`) e exigir `role.has_permission(flora_core::models::Permission::WorkspaceWrite) || role.has_permission(...::WorkspaceAdmin)` (`:221-223`), replicando `update_workspace`. Sem ambas, DEVE retornar `Error::Forbidden("Insufficient permissions")` (`:224`) antes de montar o input/chamar o serviço (`:139-147`). Nenhum required check/gate, rota, flag, schema, migração ou permissão nova.

- **FR-002 — contrato e fail-closed**: `org_id` da sessão e `Forbidden("Not a member")` (`:126-137`) permanecem; papel ausente no org DEVE gerar `Error::RoleNotFound(membership.role_id.to_string())` (`:220`, `error.rs:315-319`) sem gravação. Sucesso permanece idêntico: `CreateWorkspaceInput` (`:139-142`), `workspace_service.create_workspace` (`:144-147`), `tracing::info!` (`:149`), `201` e `WorkspaceResponse` (`:150-153`). `has_permission` aceita `*` e aliases legados (`role.rs:66-110`), preservando os papéis do seed (`dev-seed.sql:78-121`, `org.rs:136-165`).

- **FR-003 — erro sem vazamento**: a negação DEVE usar `Error::Forbidden("Insufficient permissions")`, que mapeia para 403 com corpo constante `{"error":"forbidden","message":"Insufficient permissions"}` (`error.rs:291-294`). Nenhum nome de papel/usuário/org/workspace, lista de permissões, ID ou indicação de existência na resposta; nenhum log novo no caminho negado.

- **FR-004 — contratos e regressão**: `flora-tests/src/contract/workspace.rs` DEVE acrescentar helper de seed com `&[Permission]` (padrão `flora-tests/src/contract/rbac.rs:55-81`) e POST autenticado, cobrindo `WorkspaceWrite` → 201, `WorkspaceAdmin` → 201, `permissions=[]` ("NegadoQA") → 403 e nenhuma gravação, papel de outra org → fail-closed sem gravação, sem membership → 403 inalterado, e formato de `WorkspaceResponse` preservado. Os contratos atuais (`:134-216`) DEVEM permanecer PASS; gates são registrados no handoff **[NV]** e o diff NÃO DEVE tocar em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*` ou `AGENTS.md`.

- **FR-005 — fronteira**: o diff DEVE ficar em `flora-api/src/routes/workspace.rs`, `flora-tests/src/contract/workspace.rs` e artefatos do ciclo 103; sem toques em gates, workflows, ADRs, constituição ou changelog, sem escrita em staging/produção e sem commit.

### Key Entities

- **POST /api/v1/workspaces**: rota de criação (`flora-api/src/routes/workspace.rs:84-92,120-155`); única mudança de comportamento é a checagem de permissão antes da escrita.
- **membership.role_id**: vínculo do usuário ao papel no org, obtido em `workspace.rs:210-213`; usado na consulta escopada de papel.
- **PgRoleRepository::find_by_id_and_organization**: `flora-core/src/repositories/role_repository.rs:40-56`; guard de tenant do papel (BOLA/cross-org).
- **Permission::WorkspaceWrite / WorkspaceAdmin**: `flora-core/src/models/role.rs:126-131,189-190`; exigência alternativa (`OU`) alinhada a `update_workspace`.
- **Role::has_permission**: `flora-core/src/models/role.rs:66-75`; aceita `workspace:write`, `workspace:admin`, os aliases legados e `*` — sem migração de dados.
- **Error::Forbidden / Error::RoleNotFound**: `flora-core/src/error.rs:291-294,315-319`; 403 genérico e 404 de papel inexistente no org.
- **WorkspaceResponse / StatusCode::CREATED**: contrato de resposta inalterado (`workspace.rs:41-68,150-153`).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001 — "NegadoQA" bloqueado**: com `SomenteLeituraQA` (`permissions=[]`), `POST /api/v1/workspaces` retorna 403 com o corpo genérico (não 201) e nenhum workspace é criado; com `WorkspaceWrite` ou `WorkspaceAdmin`, retorna 201 com resposta idêntica à atual (roteiro `:187`).
- **SC-002 — coerência RBAC e isolamento**: create exige o mesmo par de permissões de update; os três handlers de escrita negam com o mesmo 403; papel de outra org não autoriza criação, e os contratos de listagem/isolamento (`contract/workspace.rs:134-216`) permanecem PASS.
- **SC-003 — erro opaco e fronteira**: nenhum dado sensível no corpo/log de erro; nenhum gate/required check, schema, migração, flag, rota ou permissão novo; diff restrito ao ciclo 103 e sem commit. Execução em staging real e gates são **[NV]** neste artefato e ficam registrados no handoff.

## Assumptions

- A reprodução relatada em staging local zerado é a entrada; o item 11 do roteiro (`docs/governance/2026-11-21-roteiro-staging-flora.md:184-188`) define o comportamento esperado.
- A autorização de workspace continua sendo responsabilidade da rota — como em update/delete — e `WorkspaceService` não muda.
- `permissions=[]` é o estado do papel `SomenteLeituraQA`; `has_permission` já resolve formas canônicas, legadas e `*` (`role.rs:66-110`), então nenhuma migração de permissões é necessária.
- O helper `seed_member` atual (`contract/workspace.rs:54-103`) não faz POST; a nova checagem não quebra os contratos existentes. Um helper aditivo com permissões é suficiente.
- O roteiro aceita "403 **ou** escondidas": esconder a ação na UI não é necessário para fechar este ciclo.
- Gates, build, testes, formatters, RLS efetivo e execução do roteiro em staging real são **[NV]** neste artefato e pertencem ao handoff; esta spec não os reexecutou.
