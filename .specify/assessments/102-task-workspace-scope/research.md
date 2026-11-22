# Research: 102 — escopo por workspace na listagem de tarefas

**Data:** 2026-11-21 · **Evidência local determinística + reprodução em staging local zerado** · execução de gates/testes e staging real **[NV]** · branch `feature/102-task-workspace-scope@ae8632e10f9f8018ab61f8708ce9f8a6857e4e7d`

## Primária (local)

1. `docs/governance/2026-11-21-roteiro-staging-flora.md:116-123` — item 4 cria `QA Workspace 2` (`:121`), troca de workspace pelo seletor (`:122`) e fixa o comportamento esperado na íntegra: "contexto muda sem logout; tarefas/canais ficam escopados ao workspace selecionado" (`:123`). É o critério direto violado pelo bug.
2. `docs/governance/2026-11-21-roteiro-staging-flora.md:148-155` — item 7 cria `QA-001 — validar staging` (`:151`) e espera persistência após reload e nenhuma tarefa de **outra organização** (`:155`). O isolamento entre organizações já é garantido; o cenário reprovado é o de outra workspace **da mesma organização**, que o item 4.6 cobre.
3. `flora-api/src/routes/tasks.rs:106-114` — `create_tasks_router()` registra `GET /` → `list_tasks` sem extrator de query. Assinar o handler com `Query<...>` é a extensão natural e não exige nova rota.
4. `flora-api/src/routes/tasks.rs:131-168` — `list_tasks` obtém `org_id` da sessão (`:136-139`), mantém a checagem de membership (`:142-146`), e chama `find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })` (`:150-158`). O filtro final (`:160-165`) considera `!is_deleted` e `organization_id == org_id` — não há nenhuma condição de workspace. Essa é a linha que deixa `QA-001` passar.
5. `flora-core/src/services/task_service.rs:94-104` — `list_tasks(workspace_id, pagination, status, assignee_id)` já existe e delega a `self.task_repo.find_by_workspace_id(...)`; a capacidade de domínio está pronta e não precisa ser criada.
6. `flora-core/src/traits/repository.rs:218-225` e `flora-core/src/repositories/task_repository.rs:41-58` — `find_by_workspace_id` recebe workspace, paginação, status e responsável; a SQL base filtra `WHERE workspace_id = $1 AND is_deleted = false ORDER BY created_at DESC LIMIT $2` (`:53-58`), com variantes para status/assignee nas linhas seguintes. A consulta **não** filtra `organization_id`: o filtro defensivo do handler (`tasks.rs:163`) e a garantia de tenant (item 8) precisam ser preservados no caminho novo.
7. `flora-api/src/routes/channels.rs:173-212` — precedent direto: `GET /api/v1/channels?workspace_id=...` usa `Query<ListChannelsQuery>` (`:177`), valida que o workspace pertence à organização da sessão (`:191-203`) e só então chama `find_by_workspace_id` (`:205-208`). É o padrão a seguir em tasks, evitando uma segunda convenção.
8. `flora-api/src/tenant.rs:26-46` — `ensure_workspace_in_org(state, org_id, workspace_id)` carrega o workspace e compara `workspace.organization_id != org_id`, devolvendo erro de validação fail-closed sem revelar se o recurso existe (`:35-43`). É o guard BOLA (OWASP A01:2021 / CWE-639) a aplicar antes de listar.
9. `apps/frontend/src/routes/(app)/tasks/+page.svelte:52-54` — `loadTasks()` chama `ApiClient.request<ApiTask[]>('/api/v1/tasks')`, sem `workspace_id`. Mesmo que o backend passe a aceitar o parâmetro, o frontend não o envia.
10. `apps/frontend/src/routes/(app)/tasks/+page.svelte:115-119` — `onWorkspaceChange(event)` atualiza `selectedWorkspaceId` e `authState.setWorkspace(value || null)` e termina; não há chamada a `loadTasks()`. A lista permanece com o conteúdo do carregamento anterior até um reload manual.
11. `apps/frontend/src/routes/(app)/tasks/+page.svelte:40-50,56-67` — `loadWorkspaces()` escolhe o workspace preferido (ou o primeiro) e o propaga ao `authState`; `refresh()` executa `loadWorkspaces()` **antes** de `loadTasks()` (`:60-61`), portanto um `loadTasks()` que leia `selectedWorkspaceId` no momento da chamada já parte do workspace correto no carregamento inicial.
12. `apps/frontend/src/routes/(app)/tasks/+page.svelte:69-95,121-131` — `createTask()` já envia `workspace_id` no corpo (`:75-84`) e recarrega a lista (`:89`); `onMount()` (`:121-131`) só chama `refresh()`. Nenhum desses caminhos é substituído: a mudança se limita a parametrizar `loadTasks()` e a dispará-la na troca de seletor.
13. `flora-tests/src/contract/tasks.rs:112-134` — helper `call()` da suíte DB permite exercitar `GET /` com token e URI arbitrários; `db_task_cross_org_workspace_rejected` (`:226-289`) já fixa o guard de BOLA na criação. `flora-tests/src/contract/auth.rs:799-802` exercita `GET /api/v1/tasks` sem query string: esse contrato continua válido e deve permanecer PASS.

## Leitura da reprodução

- Item 4.6 (`roteiro:123`): a troca de workspace preserva a sessão, mas as tarefas não acompanham o novo contexto.
- Item 7.2 (`roteiro:151`): `QA-001` é criada no `Main Workspace` e atribuída ao usuário de QA — critério usado por `find_by_assignee_id`, que a devolve em qualquer workspace do org.
- Item 7.6 (`roteiro:155`): a expectativa escrita é de isolamento entre organizações, já atendida pelo filtro `organization_id == org_id` (`tasks.rs:163`); o bug está no nível workspace, dentro da mesma organização.
- Os fatos são mutuamente consistentes com o código: não é preciso invocar falha de RLS, cache, session stale ou permissão ausente.

## Diagnóstico

Cadeia fechada: seletor muda `selectedWorkspaceId` (`+page.svelte:115-119`) → `loadTasks()` não é chamado e, ainda que fosse, enviaria `GET /api/v1/tasks` sem workspace (`:52-54`) → `list_tasks` busca por responsável em todo o org (`tasks.rs:150-158`) → o único filtro é `organization_id` (`:163`) → `QA-001`, do `Main Workspace`, permanece visível sob `QA Workspace 2`. A correção é estreita e simétrica: parâmetro opcional no backend, com validação de tenant e consulta por workspace já existente; no frontend, envio do parâmetro e recarga na troca. O contrato sem parâmetro é mantido para não quebrar os testes atuais.

## [NV]

- Execução real do roteiro em staging zerado após a correção, com `QA-001` e `QA Workspace 2`, e o estado do DB/staging de produção: **[NV]** — a reprodução relatada é a entrada; nenhuma escrita ou requisição ao vivo foi executada neste artefato.
- Resultado de gates, build, testes e formatters: **[NV]** neste artefato — serão consolidados no handoff; este documento apenas inspecionou código.
- RLS efetivo do PostgreSQL e comportamento do `ApiClient` em cenários de erro de rede: **[NV]** — o guard de aplicação (`tenant.rs:26-46`) e o filtro defensivo (`tasks.rs:163`) são mantidos independentemente do RLS.
- Cobertura idiomática: pt-BR nos artefatos; inglês em nomes de código, rotas e mensagens de erro existentes, seguindo a convenção do repositório.
