# Implementation Plan: 102 — escopo por workspace na listagem de tarefas

**Branch**: `feature/102-task-workspace-scope` @ `ae8632e10f9f8018ab61f8708ce9f8a6857e4e7d` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/102-task-workspace-scope/decision.md` (go) e reprodução em staging local zerado pelo roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` (itens 4.6 e 7.6)

## Summary

Ligar as duas pontas da listagem de tarefas ao workspace selecionado. `GET /api/v1/tasks` passa a aceitar `workspace_id` UUID **opcional**: com o parâmetro, valida o tenant (`ensure_workspace_in_org`, `flora-api/src/tenant.rs:26-46`) e usa `TaskRepository::find_by_workspace_id` (`flora-core/src/traits/repository.rs:218-225`, já delegado por `TaskService::list_tasks` em `flora-core/src/services/task_service.rs:94-104`); sem ele, mantém `find_by_assignee_id` e o filtro por organização atuais (`flora-api/src/routes/tasks.rs:150-165`). No frontend, `loadTasks()` envia o parâmetro e `onWorkspaceChange` recarrega a lista sem logout. Contrato sem query string, formato de resposta, paginação e filtros Kanban permanecem idênticos. Nenhuma rota, schema, flag ou regra RBAC nova. Gates, build, testes e execução do roteiro em staging real são **[NV]** neste artefato — registrados no handoff.

## Technical Context

**Root cause**: `apps/frontend/src/routes/(app)/tasks/+page.svelte:115-119` troca `selectedWorkspaceId` sem recarregar; `loadTasks()` (`:52-54`) chama `GET /api/v1/tasks` sem parâmetro; `list_tasks` (`flora-api/src/routes/tasks.rs:131-168`) busca por responsável em todo o org (`:150-158`) e filtra apenas `!is_deleted && organization_id == org_id` (`:163`). Assim `QA-001`, do `Main Workspace`, aparece sob `QA Workspace 2` (roteiro `:123,151`).

**Target Path**: `flora-api/src/routes/tasks.rs` (query param opcional, guard de tenant, consulta escopada), `apps/frontend/src/routes/(app)/tasks/+page.svelte` (envio do parâmetro e refetch), `flora-tests/src/contract/tasks.rs` e testes de frontend, além dos artefatos do ciclo 102. Somente leitura: `flora-api/src/tenant.rs`, `flora-api/src/routes/channels.rs`, `flora-core/src/services/task_service.rs`, `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/task_repository.rs`.

**Testing**: com `workspace_id` → só tarefas daquele workspace (`QA-001` ausente no outro); sem parâmetro → conjunto atual preservado (`flora-tests/src/contract/auth.rs:799-802`); workspace cross-org, inexistente e UUID inválido → 4xx sem tarefas; nenhuma tarefa de outra org em qualquer caminho (`roteiro:155`); frontend → URL gerada por `loadTasks()` e refetch na troca de seletor. Gates do alvo no handoff **[NV]**.

**Constraints**: `workspace_id` UUID opcional com o mesmo nome de `channels.rs:173-177` e do body de criação (`tasks.rs:33,77`); malformado → 4xx, nunca silenciado; membership (`:142-146`) e `ensure_workspace_in_org` (`tenant.rs:26-46`) obrigatórios; manter filtros `!is_deleted`/`organization_id` (`:163`) porque a SQL do repositório filtra só workspace (`task_repository.rs:53-58`); `status`/`assignee_id` da consulta ficam `None`; `TaskResponse` e paginação (`limit: 50, offset: 0`) inalterados; sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição; sem commits.

## Steps

1. **Causa (T001)** — fixar em intake/research a cadeia do roteiro 4.6/7.6 (`docs/governance/2026-11-21-roteiro-staging-flora.md:123,151,155`): seletor muda estado local, `loadTasks()` envia `GET /api/v1/tasks` sem workspace, `list_tasks` busca por responsável em todo o org (`tasks.rs:131-168`). Registrar capacidade já existente (`task_service.rs:94-104`, `repository.rs:218-225`) e precedente de rota escopada (`channels.rs:173-212`).
2. **Parâmetro (T002)** — adicionar extrator de query a `list_tasks` (`tasks.rs:132-135`) com `workspace_id: Option<Uuid>` (padrão `Query` de `channels.rs:177`). Sem parâmetro, preservar integralmente `:150-165`; com valor malformado, 4xx de validação — sem fallback silencioso. Nenhuma rota nova.
3. **Guard e consulta (T003)** — com `workspace_id`: manter membership (`:142-146`), chamar `ensure_workspace_in_org(&state, org_id, workspace_id)` (`tenant.rs:26-46`) e listar via `find_by_workspace_id(workspace_id, Pagination { limit: 50, offset: 0 }, None, None)` (`repository.rs:218-225`) — diretamente ou por `TaskService::list_tasks` (`task_service.rs:94-104`). Reaplicar `!is_deleted && organization_id == org_id` (`:163`) no mapeamento para `TaskResponse` (`:160-165`), preservando formato e paginação.
4. **Frontend: envio (T004)** — `loadTasks()` (`+page.svelte:52-54`) monta a URI com `?workspace_id=${encodeURIComponent(selectedWorkspaceId)}` quando houver seleção; usa o valor corrente de `selectedWorkspaceId` (`:26`), aproveitando que `refresh()` chama `loadWorkspaces()` antes de `loadTasks()` (`:60-61`). Sem workspace selecionado, mantém a chamada sem parâmetro.
5. **Frontend: refetch (T005)** — tornar `onWorkspaceChange` (`:115-119`) assíncrono: atualizar `selectedWorkspaceId` e `authState`, então recarregar com `isLoading = true`, `errorMessage = ''`, `catch` com `apiErrorMessage(error_, 'error.loadTasks')` e `finally { isLoading = false; }`, espelhando `refresh()` (`:56-67`). Sem logout, sem reload manual, sem alterar `createTask()` (`:69-95`).
6. **Contratos backend (T006)** — estender `flora-tests/src/contract/tasks.rs` (helper `call()` em `:112-134`) com: `workspace_id` do próprio org → só tarefas desse workspace e `QA-001` ausente no outro; sem parâmetro → conjunto atual; workspace cross-org → 4xx e nenhuma tarefa; UUID inválido → 4xx; nenhuma tarefa de outra org em nenhum caminho. Preservar `flora-tests/src/contract/auth.rs:799-802` e o guard de criação (`contract/tasks.rs:226-289`).
7. **Contratos frontend (T007)** — cobrir em `apps/frontend/tests/` que `loadTasks()` inclui o `workspace_id` selecionado na URL e que a troca de seletor dispara a recarga com tratamento de erro; validar manualmente o roteiro 4.6/7.6 em staging zerado no handoff **[NV]**.
8. **Fecho (T008)** — rastreabilidade T001..T008 em decision/spec/plan/tasks, gates do alvo registrados no handoff **[NV]**, verificação da fronteira proibida e confirmação de que nenhum commit foi feito.

## Constitution Check

Sem violações: nenhum required check, gate, contrato de escrita de tarefa, membership, regra RBAC, schema, workflow, ADR ou constituição é alterado; o novo parâmetro é validado contra a organização da sessão (`tenant.rs:26-46`) e o filtro defensivo de organização é mantido (`tasks.rs:163`), não expondo tarefas cross-org; o contrato sem parâmetro e o formato de resposta seguem idênticos; o diff fica restrito à listagem de tarefas, ao frontend de `/tasks`, aos testes e aos artefatos do ciclo 102.

## Project Structure

```text
.specify/assessments/102-task-workspace-scope/{intake,research,define,shape,decision}.md
specs/102-task-workspace-scope/
├── spec.md      # FR-001..FR-005 / SC-001..SC-003
├── plan.md      # T001..T008
└── tasks.md     # fila T001..T008 (a executar)
flora-api/src/routes/tasks.rs                  # GET / com workspace_id opcional + guard + find_by_workspace_id
flora-api/src/tenant.rs                        # ensure_workspace_in_org (reuso, inalterado)
flora-api/src/routes/channels.rs               # padrão Query<ListChannelsQuery> (reuso, inalterado)
flora-core/src/services/task_service.rs        # list_tasks existente (reuso, inalterado)
flora-core/src/traits/repository.rs            # TaskRepository::find_by_workspace_id (reuso, inalterado)
flora-core/src/repositories/task_repository.rs # SQL por workspace (reuso, inalterado)
apps/frontend/src/routes/(app)/tasks/+page.svelte  # loadTasks parametrizado + onWorkspaceChange com refetch
flora-tests/src/contract/tasks.rs              # contratos de escopo e validação de tenant
apps/frontend/tests/                           # URL de loadTasks e refetch na troca
docs/governance/2026-11-21-roteiro-staging-flora.md  # itens 4.6 e 7.6 (leitura)
```

## Complexity Tracking

Nenhuma. São um query param opcional, um guard de tenant existente, uma consulta de repositório já implementada, uma parametrização de URL no frontend e testes — todos verificáveis por contrato e inspeção, sem nova dependência, migração, rota, flag ou superfície pública. A única complexidade aceita é manter os dois caminhos de listagem (com e sem `workspace_id`) coexistindo até que o contrato antigo possa ser removido em ciclo futuro — fora de escopo aqui.
