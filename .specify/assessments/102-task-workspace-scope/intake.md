# Intake: 102 — escopo por workspace na listagem de tarefas

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/102-task-workspace-scope@ae8632e10f9f8018ab61f8708ce9f8a6857e4e7d` · `.specify/feature.json` → `specs/102-task-workspace-scope`

## Sinal

Bug reproduzido em staging local zerado seguindo o roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` (itens 4.6 e 7.6): com `QA Workspace 2` selecionado no seletor da tela `/tasks`, a tarefa `QA-001 — validar staging` (criada no `Main Workspace`, item 7.2) continua visível na lista. O item 4.6 do roteiro exige explicitamente que "contexto muda sem logout; tarefas/canais ficam escopados ao workspace selecionado" (`:123`), e o item 7.6 exige persistência e nenhuma tarefa de outra organização (`:155`). A falha é de escopo **entre workspaces da mesma organização**: o isolamento entre organizações existe e não está em discussão.

A causa é determinística e dupla. No backend, `GET /api/v1/tasks` não aceita seletor de workspace: `list_tasks` (`flora-api/src/routes/tasks.rs:131-168`) busca por responsável com `find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })` (`:150-158`) e filtra apenas `!is_deleted && organization_id == org_id` (`:163`). A capacidade já existe e não é usada: `TaskService::list_tasks(workspace_id, pagination, status, assignee_id)` (`flora-core/src/services/task_service.rs:94-104`) delega a `TaskRepository::find_by_workspace_id` (`flora-core/src/traits/repository.rs:218-225`), implementada com `WHERE workspace_id = $1 AND is_deleted = false` (`flora-core/src/repositories/task_repository.rs:41-58`). No frontend, `loadTasks()` chama `GET /api/v1/tasks` sem parâmetro (`apps/frontend/src/routes/(app)/tasks/+page.svelte:52-54`) e `onWorkspaceChange` (`:115-119`) apenas atualiza `selectedWorkspaceId` e o `authState`, sem recarregar a lista. Corrigir só uma das duas pontas não resolve o cenário observado.

## Evidência

```
ROTEIRO (reprodução em staging local zerado):
docs/governance/2026-11-21-roteiro-staging-flora.md:116-123  item 4 — cria 'QA Workspace 2', troca pelo seletor;
                                                    :123     item 4.6 — esperado: contexto muda sem logout; tarefas/canais escopados ao workspace selecionado
                                                    :148-155 item 7 — Tarefas; :151 cria 'QA-001 — validar staging';
                                                    :155     item 7.6 — esperado: persistência após reload; nenhuma tarefa de outra organização aparece

BACKEND (escopo ausente):
flora-api/src/routes/tasks.rs:106-114       router: GET / -> list_tasks (sem query param)
flora-api/src/routes/tasks.rs:131-168       handler list_tasks
flora-api/src/routes/tasks.rs:142-146       checagem de membership (org_id da sessão)
flora-api/src/routes/tasks.rs:150-158       find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })
flora-api/src/routes/tasks.rs:160-165       filtro apenas !is_deleted && organization_id == org_id

CAPACIDADE JÁ EXISTENTE (não ligada):
flora-core/src/services/task_service.rs:94-104        list_tasks(workspace_id, pagination, status, assignee_id)
flora-core/src/traits/repository.rs:218-225           TaskRepository::find_by_workspace_id(...)
flora-core/src/repositories/task_repository.rs:41-58  SQL WHERE workspace_id = $1 AND is_deleted = false

PRECEDENTE DE ROTA ESCOPADA:
flora-api/src/routes/channels.rs:173-212   GET /api/v1/channels?workspace_id=... valida org e usa find_by_workspace_id
flora-api/src/tenant.rs:26-46              ensure_workspace_in_org(state, org_id, workspace_id) — fail closed (BOLA)

FRONTEND (sem parâmetro e sem recarga):
apps/frontend/src/routes/(app)/tasks/+page.svelte:26          selectedWorkspaceId ($state)
apps/frontend/src/routes/(app)/tasks/+page.svelte:40-50       loadWorkspaces() escolhe/propaga workspace
apps/frontend/src/routes/(app)/tasks/+page.svelte:52-54       loadTasks() -> GET /api/v1/tasks (sem workspace_id)
apps/frontend/src/routes/(app)/tasks/+page.svelte:56-67       refresh() -> loadWorkspaces() e loadTasks()
apps/frontend/src/routes/(app)/tasks/+page.svelte:115-119     onWorkspaceChange() só seta estado local + authState

TESTES / CONTRATOS EXISTENTES:
flora-tests/src/contract/tasks.rs:112-134    helper call() da suíte db (GET / sem param)
flora-tests/src/contract/tasks.rs:226-289    db_task_cross_org_workspace_rejected — guard BOLA de create
flora-tests/src/contract/auth.rs:799-802     GET /api/v1/tasks sem query param (contrato a preservar)
```

Nenhuma execução de gates, build, testes ou deploy foi feita neste artefato; o comportamento observado vem do roteiro e a cadeia de código foi inspecionada localmente. O resultado final em staging real permanece **[NV]** até o handoff.

## Autorização

Modo autônomo. Decisão `go` registrada em `.specify/assessments/102-task-workspace-scope/decision.md` antes da implementação. Escopo: (1) aceitar `workspace_id` como query param **opcional** em `GET /api/v1/tasks`, validando que o workspace pertence à organização da sessão (`ensure_workspace_in_org`, `flora-api/src/tenant.rs:26-46`) e usando `TaskRepository::find_by_workspace_id`; (2) manter integralmente o contrato sem o parâmetro, usado pelos testes atuais; (3) fazer o frontend enviar o workspace selecionado em `loadTasks()` e recarregar a lista em `onWorkspaceChange`, sem logout; (4) cobrir os cenários com contratos backend/frontend. Nenhuma migração, schema, RBAC, rota nova ou mudança de contrato de criação/atualização de tarefa. Nenhuma escrita em staging/produção neste ciclo.

## Fronteira

Backend Rust (`flora-api/src/routes/tasks.rs`, com reuso de `flora-api/src/tenant.rs` e do repositório de tarefas em `flora-core`), frontend Svelte (`apps/frontend/src/routes/(app)/tasks/+page.svelte`) e testes (`flora-tests/src/contract/tasks.rs`, testes de frontend), além dos artefatos do ciclo. Sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição. A factory `mdmv-dark-factory` é somente leitura. Nenhum commit é feito neste ciclo.
