# Tasks: 102 — escopo por workspace na listagem de tarefas

**Input**: `specs/102-task-workspace-scope/spec.md`, `plan.md`,
`.specify/assessments/102-task-workspace-scope/decision.md`, reprodução do roteiro de staging (itens 4.6 e 7.6)

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T008 em aberto (**fila a executar**); gates do alvo **[NV]** até o handoff.
**Status de execução (2026-11-21):** T001–T005 e T007–T008 executados e verificados; T006 substituído por teste unitário puro + medição ao vivo da API (justificativa em `analyze.md`/`converge.md`). Os `[ ]` abaixo permanecem como registro da fila original, não como estado corrente.

## Phase 1 — Diagnóstico e artefatos de assessoria

- [ ] T001 Fixar em `.specify/assessments/102-task-workspace-scope/{intake,research}.md` a cadeia do bug: roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:123,151,155` (4.6 troca de workspace; 7.2 cria `QA-001`; 7.6 isolamento), `apps/frontend/src/routes/(app)/tasks/+page.svelte:115-119` (seletor sem recarga) e `:52-54` (`GET /api/v1/tasks` sem parâmetro), `flora-api/src/routes/tasks.rs:131-168` (`find_by_assignee_id` em todo o org; só filtro de `organization_id` em `:163`). Registrar a capacidade já existente (`flora-core/src/services/task_service.rs:94-104`, `flora-core/src/traits/repository.rs:218-225`) e o precedente (`flora-api/src/routes/channels.rs:173-212`). Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).

## Phase 2 — Backend: parâmetro, guard e consulta

- [ ] T002 Adicionar `workspace_id: Option<Uuid>` como query param de `GET /api/v1/tasks` em `flora-api/src/routes/tasks.rs:106-114,132-135`, seguindo o padrão `Query<ListChannelsQuery>` de `channels.rs:173-177`. Na ausência, manter exatamente `find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })` (`:150-158`) e o filtro por `organization_id` (`:163`) — contrato usado por `flora-tests/src/contract/auth.rs:799-802`. UUID malformado DEVE gerar 4xx de validação, nunca ser ignorado. Nenhuma rota nova (FR-001, SC-002).

- [ ] T003 Com `workspace_id` válido: manter a checagem de membership (`tasks.rs:142-146`), chamar `ensure_workspace_in_org(&state, org_id, workspace_id)` (`flora-api/src/tenant.rs:26-46`) antes de consultar e listar via `find_by_workspace_id(workspace_id, Pagination { limit: 50, offset: 0 }, None, None)` (`flora-core/src/traits/repository.rs:218-225`), diretamente ou por `TaskService::list_tasks` (`flora-core/src/services/task_service.rs:94-104`). Reaplicar `!is_deleted && organization_id == org_id` no mapeamento (`:160-165`), pois a SQL do repositório filtra só workspace (`flora-core/src/repositories/task_repository.rs:53-58`). `TaskResponse` (`:85-104,116-129`) e paginação inalterados; workspace cross-org/inexistente → 4xx fail-closed sem vazar existência (FR-002, FR-003, SC-002, SC-003).

## Phase 3 — Frontend: envio e recarga

- [ ] T004 Parametrizar `loadTasks()` em `apps/frontend/src/routes/(app)/tasks/+page.svelte:52-54`: incluir `?workspace_id=<selectedWorkspaceId>` (codificado) quando houver seleção, lendo o valor corrente de `selectedWorkspaceId` (`:26`). Sem seleção, manter `GET /api/v1/tasks`. `refresh()` já chama `loadWorkspaces()` antes de `loadTasks()` (`:60-61`), preservando o carregamento inicial (FR-004, SC-001).

- [ ] T005 Tornar `onWorkspaceChange` (`+page.svelte:115-119`) assíncrono: atualizar `selectedWorkspaceId` e `authState.setWorkspace(value || null)`, então recarregar com `isLoading = true`, `errorMessage = ''`, `catch` via `apiErrorMessage(error_, 'error.loadTasks')` e `finally { isLoading = false; }`, espelhando `refresh()` (`:56-67`). Sem logout, sem reload manual e sem alterar `loadWorkspaces()` (`:40-50`) nem `createTask()` (`:69-95`) (FR-004, SC-001).

## Phase 4 — Contratos

- [ ] T006 Estender `flora-tests/src/contract/tasks.rs` (helper `call()`, `:112-134`) com: `workspace_id` do próprio org → somente tarefas desse workspace e `QA-001` ausente no outro; sem parâmetro → conjunto atual preservado; workspace de outra organização → 4xx e nenhuma tarefa; UUID inválido → 4xx; nenhuma tarefa de outra org em nenhum caminho. Não substituir os contratos existentes, em especial `db_task_cross_org_workspace_rejected` (`:226-289`) e `flora-tests/src/contract/auth.rs:799-802` (FR-001, FR-002, FR-003, FR-005, SC-002).

- [ ] T007 Cobrir no frontend, em `apps/frontend/tests/`, que `loadTasks()` inclui o `workspace_id` selecionado na URL e que a troca de seletor dispara a recarga com `isLoading`/`errorMessage` tratados; validar manualmente os itens 4.6 e 7.6 do roteiro em staging zerado no handoff **[NV]** (FR-004, FR-005, SC-001).

## Phase 5 — Fecho

- [ ] T008 Fechar a rastreabilidade T001..T008 em decision/spec/plan/tasks, registrar gates do alvo no handoff **[NV]** e confirmar que nenhum arquivo de `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*` ou `AGENTS.md` foi tocado, que não houve commit e que rota/schema/RBAC/flags e o contrato sem parâmetro permanecem inalterados (FR-005, SC-002, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008. T002 depende de T001; T003 de T002 (o guard e a consulta só fazem sentido com o parâmetro); T004 de T002 (contrato de query definido); T005 de T004; T006 depende de T002–T003; T007 depende de T004–T005; T008 verifica T001–T007 em conjunto e consolida gates e rastro documental. O caminho sem `workspace_id`, o contrato de escrita de tarefas, a paginação, o formato de resposta e o isolamento entre organizações permanecem inalterados em todos os passos. Execução em staging real, gates, build, testes e RLS efetivo permanecem **[NV]** até o handoff.
