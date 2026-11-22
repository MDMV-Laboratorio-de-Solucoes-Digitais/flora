# Feature Specification: 102 — escopo por workspace na listagem de tarefas

**Feature Branch**: `feature/102-task-workspace-scope`

**Created**: 2026-11-21

**Status**: Specified — `go` para implementação (decisão em `.specify/assessments/102-task-workspace-scope/decision.md`)

**Input**: reprodução em staging local zerado pelo roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` (itens 4.6 e 7.6) e decisão `go` do ciclo 102

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Trocar de workspace e ver somente as tarefas dele (Priority: P1)

Como usuário de QA em `/tasks`, preciso que a lista acompanhe o workspace selecionado no seletor, porque hoje `QA-001`, criada no `Main Workspace`, continua visível quando escolho `QA Workspace 2`.

**Why this priority**: é o comportamento exigido pelo item 4.6 do roteiro (`docs/governance/2026-11-21-roteiro-staging-flora.md:123`) e o bug reproduzido com `QA-001` (item 7.2, `:151`). Sem isso, a tela apresenta tarefas de outro contexto e a validação de staging falha.

**Independent Test**: criar `QA-001` no `Main Workspace`, criar `QA Workspace 2`, trocar pelo seletor e confirmar que `QA-001` desaparece sem logout; voltar ao `Main Workspace` e confirmar que reaparece.

**Acceptance Scenarios**:

1. **Given** `QA-001` existe no `Main Workspace` e `QA Workspace 2` está selecionado em `/tasks`, **When** a lista é carregada, **Then** `QA-001` NÃO aparece e nenhuma tarefa de outro workspace aparece.
2. **Given** o cenário anterior, **When** o seletor volta ao `Main Workspace`, **Then** `QA-001` reaparece sem logout e sem reload manual da página.
3. **Given** dois workspaces do mesmo org com tarefas próprias, **When** cada um é selecionado por vez, **Then** cada lista contém somente as tarefas do workspace selecionado (`TaskResponse` inalterado, `flora-api/src/routes/tasks.rs:85-104`).

---

### User Story 2 — Manter o contrato atual e o isolamento entre organizações (Priority: P1)

Como mantenedor, preciso que a listagem continue funcionando sem parâmetro — como os testes atuais a chamam — e que um `workspace_id` de outra organização nunca seja usado para ler tarefas.

**Why this priority**: `flora-tests/src/contract/auth.rs:799-802` chama `GET /api/v1/tasks` sem query string, e o item 7.6 do roteiro (`:155`) exige que nenhuma tarefa de outra organização apareça. Uma correção que exigisse o parâmetro ou o aceitasse sem validar tenant quebraria contratos e abriria BOLA (OWASP A01:2021 / CWE-639).

**Independent Test**: chamar `GET /api/v1/tasks` sem parâmetro e conferir o conjunto atual; chamar com `workspace_id` de outra org e com UUID inválido, conferindo 4xx e nenhuma tarefa.

**Acceptance Scenarios**:

1. **Given** um usuário autenticado e membro do org, **When** `GET /api/v1/tasks` é chamado sem `workspace_id`, **Then** a resposta é idêntica à de hoje: `find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })` (`flora-api/src/routes/tasks.rs:150-158`), filtrada por `!is_deleted` e `organization_id == org_id` (`:163`).
2. **Given** um `workspace_id` pertencente a outra organização, **When** a listagem é chamada com esse valor, **Then** a resposta é 4xx fail-closed via `ensure_workspace_in_org` (`flora-api/src/tenant.rs:26-46`), sem retornar tarefas e sem revelar se o workspace existe.
3. **Given** um `workspace_id` malformado, **When** a listagem é chamada, **Then** a resposta é 4xx de validação — o valor NÃO é ignorado nem cai no caminho antigo.
4. **Given** qualquer caminho de listagem, **When** a resposta é inspecionada, **Then** nenhuma tarefa de outra organização aparece, inclusive porque o filtro defensivo `organization_id == org_id` (`tasks.rs:163`) é mantido no caminho escopado.

---

### User Story 3 — Recarregar com estado e erro tratados (Priority: P2)

Como usuário, preciso que a troca de workspace recarregue a lista com o mesmo feedback de carregamento e erro do carregamento inicial, para não ficar olhando dados antigos sem aviso.

**Why this priority**: `onWorkspaceChange` (`apps/frontend/src/routes/(app)/tasks/+page.svelte:115-119`) hoje só muda estado local; uma recarga sem `isLoading`/`errorMessage` poderia manter silenciosamente a lista antiga em caso de falha.

**Independent Test**: trocar o seletor e observar que a lista é atualizada; simular falha na requisição e confirmar que `errorMessage` é exibido sem deslogar.

**Acceptance Scenarios**:

1. **Given** a tela `/tasks` carregada, **When** o workspace é trocado no seletor, **Then** `loadTasks()` é chamado com o novo `workspace_id`, mantendo a sessão e o `authState` atualizados (`+page.svelte:43-48,115-119`).
2. **Given** uma falha na requisição disparada pela troca, **When** o erro é tratado, **Then** `errorMessage` usa `apiErrorMessage(error_, 'error.loadTasks')`, como em `refresh()` (`+page.svelte:56-67`), e `isLoading` é finalizado em `finally`.
3. **Given** o `createTask()` existente, **When** uma tarefa é criada, **Then** o fluxo atual (`:69-95`) continua válido e recarrega a lista do workspace selecionado.

---

### Edge Cases

- Sem `workspace_id` na query: comportamento atual preservado; nenhum filtro novo é aplicado (contrato em `flora-tests/src/contract/auth.rs:799-802`).
- `workspace_id` com UUID inválido, vazio ou repetido: tratado como erro 4xx de validação; nunca silenciado para o caminho antigo.
- `workspace_id` de workspace inexistente ou de outra organização: 4xx fail-closed via `flora-api/src/tenant.rs:26-46`; nenhuma distinção que revele existência.
- `find_by_workspace_id` não filtra `organization_id` na SQL (`flora-core/src/repositories/task_repository.rs:53-58`): o handler mantém `!is_deleted && organization_id == org_id` (`tasks.rs:163`) também no caminho escopado.
- Tarefa soft-deleted (`is_deleted`): permanece oculta nos dois caminhos.
- Mais de 50 tarefas no workspace: paginação atual (`limit: 50, offset: 0`) é mantida; paginação configurável está fora de escopo.
- `status` e `assignee_id` da consulta escopada ficam `None` neste ciclo; filtros existentes da UI não são alterados.
- Nenhum workspace selecionado (lista vazia): `loadTasks()` não envia parâmetro e o comportamento atual é preservado.
- Troca rápida de workspace no seletor: cada mudança dispara uma recarga com o valor corrente de `selectedWorkspaceId`; a resposta exibida corresponde ao workspace vigente ao final do fluxo.
- Ausência de membership no org continua produzindo `Forbidden` (`tasks.rs:142-146`), inalterado.
- Execução do roteiro em staging real, gates, build, testes e RLS efetivo: **[NV]** neste artefato, registrados no handoff.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001 — parâmetro opcional**: `GET /api/v1/tasks` (`flora-api/src/routes/tasks.rs:109,131-168`) DEVE aceitar `workspace_id` como query param UUID opcional, seguindo `Query<ListChannelsQuery>` em `flora-api/src/routes/channels.rs:173-177`. Na ausência, DEVE manter `find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })` (`:150-158`) e o filtro `organization_id` (`:163`). Valor malformado DEVE gerar 4xx, nunca ser ignorado. Nenhuma rota nova.

- **FR-002 — validação de tenant**: com `workspace_id` presente, o handler DEVE preservar a checagem de membership (`:142-146`) e DEVE chamar `ensure_workspace_in_org(&state, org_id, workspace_id)` (`flora-api/src/tenant.rs:26-46`) antes de consultar. Workspace inexistente ou de outra org DEVE retornar 4xx fail-closed, sem tarefas e sem revelar existência. Os filtros `!is_deleted` e `organization_id == org_id` (`:163`) DEVEM ser aplicados também no caminho escopado.

- **FR-003 — consulta escopada**: com parâmetro válido, DEVE listar via `TaskRepository::find_by_workspace_id(workspace_id, Pagination { limit: 50, offset: 0 }, None, None)` (`flora-core/src/traits/repository.rs:218-225`), diretamente ou por `TaskService::list_tasks` (`flora-core/src/services/task_service.rs:94-104`). `TaskResponse` (`tasks.rs:85-104,116-129`) permanece idêntico. A resposta NÃO DEVE conter tarefa de outro workspace nem de outra organização.

- **FR-004 — frontend**: `loadTasks()` (`apps/frontend/src/routes/(app)/tasks/+page.svelte:52-54`) DEVE enviar `workspace_id` com `selectedWorkspaceId` quando houver seleção, e `onWorkspaceChange` (`:115-119`) DEVE recarregar a lista após atualizar o estado e o `authState`, com `isLoading`/`errorMessage` conforme `refresh()` (`:56-67`), sem logout e sem reload manual. `loadWorkspaces()` (`:40-50`) e `createTask()` (`:69-95`) permanecem válidos.

- **FR-005 — contratos e regressão**: `flora-tests/src/contract/tasks.rs` DEVE cobrir escopo por workspace, ausência de parâmetro, workspace cross-org, UUID inválido e ausência de tarefas de outra org; testes de frontend DEVEM cobrir a URL de `loadTasks()` e a recarga na troca. Contratos existentes (`flora-tests/src/contract/auth.rs:799-802`, `flora-tests/src/contract/tasks.rs:226-289`) DEVEM permanecer PASS; gates são registrados no handoff **[NV]** e o diff NÃO DEVE tocar em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*` ou `AGENTS.md`.

### Key Entities

- **GET /api/v1/tasks**: rota de listagem (`flora-api/src/routes/tasks.rs:106-114,131-168`); com `workspace_id` usa consulta escopada, sem ele mantém a busca por responsável de sempre.
- **workspace_id**: identificador do workspace (UUID), mesmo nome usado em `channels.rs:173-177` e no body de criação de tarefa (`tasks.rs:33,77`); na listagem é opcional e validado contra a organização da sessão.
- **TaskRepository::find_by_workspace_id**: `flora-core/src/traits/repository.rs:218-225`, implementado em `flora-core/src/repositories/task_repository.rs:41-58`; filtra `workspace_id` e `is_deleted`, não `organization_id`.
- **ensure_workspace_in_org**: guard de tenant em `flora-api/src/tenant.rs:26-46`; impede que um workspace de outra organização seja usado como filtro (BOLA).
- **selectedWorkspaceId**: estado do seletor em `apps/frontend/src/routes/(app)/tasks/+page.svelte:26`, propagado ao `authState` e agora também à requisição de listagem.
- **TaskResponse**: contrato de resposta inalterado (`tasks.rs:85-104,116-129`).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001 — roteiro fechado**: com `QA-001` no `Main Workspace`, selecionar `QA Workspace 2` remove `QA-001` da lista sem logout, e voltar ao `Main Workspace` a faz reaparecer (roteiro `:123,151`).
- **SC-002 — isolamento e retrocompatibilidade**: `GET /api/v1/tasks` sem parâmetro continua devolvendo o conjunto atual (contratos existentes PASS); `workspace_id` de outra org, inexistente ou malformado nunca lista tarefas, e nenhuma tarefa cross-org aparece em qualquer caminho (roteiro `:155`).
- **SC-003 — contrato estável e fronteira preservada**: nome `workspace_id`, formato de `TaskResponse` e paginação atuais permanecem; nenhuma rota, flag, schema, migração ou regra RBAC nova é introduzida, e o diff permanece dentro do escopo do ciclo 102. Execução em staging real e resultado dos gates são **[NV]** neste artefato e ficam registrados no handoff.

## Assumptions

- A reprodução relatada em staging local zerado é a entrada; os itens 4.6 e 7.6 do roteiro (`docs/governance/2026-11-21-roteiro-staging-flora.md:123,155`) definem o comportamento esperado.
- O isolamento entre organizações já funciona (`tasks.rs:163`); o bug é de escopo entre workspaces da mesma organização.
- `find_by_workspace_id` e `TaskService::list_tasks` estão disponíveis e não precisam ser alterados; a SQL do repositório não filtra organização, então o guard de tenant e o filtro defensivo no handler são obrigatórios.
- `workspace_id` é opcional para preservar os contratos atuais; nenhum cliente existente é obrigado a mudar.
- Filtros de status/responsável da UI não são movidos para a query de listagem neste ciclo.
- Gates, build, testes, formatters, RLS efetivo e execução do roteiro em staging real são **[NV]** neste artefato e pertencem ao handoff; esta spec não os reexecutou.
