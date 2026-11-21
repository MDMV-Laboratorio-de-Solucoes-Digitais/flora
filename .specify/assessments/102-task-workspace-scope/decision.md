# Decision: 102 — escopo por workspace na listagem de tarefas

- **Slug**: 102-task-workspace-scope
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/102-task-workspace-scope` @ `ae8632e10f9f8018ab61f8708ce9f8a6857e4e7d`
- **Âncora**: `.specify/feature.json` → `specs/102-task-workspace-scope`
- **Evidência**: reprodução em staging local zerado (roteiro itens 4.6 e 7.6) + inspeção determinística do código
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

O roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:116-123` cria `QA Workspace 2`, troca de workspace pelo seletor e espera que "tarefas/canais ficam escopados ao workspace selecionado" (`:123`). O item 7 cria `QA-001 — validar staging` (`:151`) e espera persistência sem tarefas de outra organização (`:155`). Em staging local zerado, `QA-001` (Main Workspace) continua visível sob `QA Workspace 2`. A cadeia explica o fato sem falha intermitente:

1. `apps/frontend/src/routes/(app)/tasks/+page.svelte:115-119` — `onWorkspaceChange` atualiza `selectedWorkspaceId` e `authState`, mas não recarrega a lista.
2. `apps/frontend/src/routes/(app)/tasks/+page.svelte:52-54` — mesmo ao recarregar, `loadTasks()` chamaria `GET /api/v1/tasks` sem `workspace_id`.
3. `flora-api/src/routes/tasks.rs:106-114,131-168` — `list_tasks` não extrai query; busca `find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })` (`:150-158`) e filtra só `!is_deleted && organization_id == org_id` (`:163`).
4. `flora-core/src/services/task_service.rs:94-104` e `flora-core/src/traits/repository.rs:218-225` — `list_tasks`/`find_by_workspace_id` já existem; a SQL filtra `workspace_id = $1 AND is_deleted = false` (`flora-core/src/repositories/task_repository.rs:53-58`), não organização.
5. `flora-api/src/routes/channels.rs:173-212` e `flora-api/src/tenant.rs:26-46` — há precedente de rota escopada com validação de tenant; não é preciso inventar convenção.

O isolamento entre organizações já funciona; o que falta é o escopo por workspace dentro da mesma organização, nas duas pontas (API e UI).

## Decisão

Registrar o ciclo 102 como **escopo por workspace na listagem de tarefas**, com cinco compromissos rastreáveis (FR-001..FR-005), três critérios (SC-001..SC-003) e oito tarefas (T001..T008, todas na fila de implementação neste artefato):

1. **Parâmetro opcional (FR-001 / T002)**: `GET /api/v1/tasks` aceita `workspace_id` UUID opcional, no padrão de `channels.rs:173-177`. Sem o parâmetro, `list_tasks` mantém `find_by_assignee_id` e o filtro por org (`tasks.rs:150-165`) — contrato existente preservado. UUID inválido → 4xx, nunca silenciado.
2. **Validação de tenant (FR-002 / T003)**: com parâmetro, manter membership (`:142-146`) e chamar `ensure_workspace_in_org` (`tenant.rs:26-46`) antes de listar; workspace de outra org ou inexistente → 4xx fail-closed, sem vazar existência; reaplicar `!is_deleted && organization_id == org_id` (`tasks.rs:163`) porque a consulta do repositório não filtra organização.
3. **Consulta escopada (FR-003 / T003)**: usar `TaskRepository::find_by_workspace_id(workspace_id, Pagination { limit: 50, offset: 0 }, None, None)` (`repository.rs:218-225`, via `TaskService::list_tasks` em `task_service.rs:94-104` ou diretamente), com `TaskResponse` inalterado (`tasks.rs:85-104,116-129`). Nenhuma tarefa de outro workspace ou organização na resposta.
4. **Frontend ponta a ponta (FR-004 / T004–T005)**: `loadTasks()` envia `workspace_id` com `selectedWorkspaceId` (`+page.svelte:52-54`) e `onWorkspaceChange` recarrega a lista após atualizar o estado (`:115-119`), com `isLoading`/`errorMessage` como em `refresh()` (`:56-67`), sem logout nem reload manual.
5. **Contratos e regressão (FR-005 / T006–T008)**: `flora-tests/src/contract/tasks.rs` cobre escopo, ausência de parâmetro, cross-org, UUID inválido e nenhuma tarefa de outra org; testes de frontend cobrem URL e refetch. `flora-tests/src/contract/auth.rs:799-802` e o guard de criação (`contract/tasks.rs:226-289`) permanecem PASS; fronteira de artefatos preservada.

## Evidência de entrada

- Baseline `ae8632e10f9f8018ab61f8708ce9f8a6857e4e7d`, branch `feature/102-task-workspace-scope`.
- Roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:123,151,155` — itens 4.6, 7.2 e 7.6; reprodução relatada em staging local zerado.
- `flora-api/src/routes/tasks.rs:106-114,131-168` — rota e handler sem parâmetro de workspace; busca por responsável e filtro por organização.
- `flora-core/src/services/task_service.rs:94-104`, `flora-core/src/traits/repository.rs:218-225`, `flora-core/src/repositories/task_repository.rs:41-58` — capacidade de listagem por workspace já implementada.
- `flora-api/src/routes/channels.rs:173-212` e `flora-api/src/tenant.rs:26-46` — precedente de query escopada e guard BOLA.
- `apps/frontend/src/routes/(app)/tasks/+page.svelte:40-50,52-54,56-67,115-119` — seleção de workspace, `loadTasks()` sem parâmetro e `onWorkspaceChange` sem recarga.
- `flora-tests/src/contract/tasks.rs:112-134,226-289` e `flora-tests/src/contract/auth.rs:799-802` — contratos a estender e a preservar.

## Racional

**go.** O bloqueio é consequência direta de duas lacunas verificáveis e complementares: a API não oferece filtro de workspace e a UI não envia nem recarrega. A correção reaproveita consulta, guard de tenant e padrão de rota já presentes, sem nova rota, schema, migração, flag, RBAC ou mudança de contrato de escrita. O parâmetro é opcional, portanto o contrato sem query string — usado por testes existentes — continua válido. O risco de BOLA pelo novo seletor é coberto em duas camadas (`ensure_workspace_in_org` e filtro de `organization_id` no handler), e o risco de meia-correção é coberto ao tratar backend e frontend como um único caminho. Todos os FR/SC são verificáveis por contratos e inspeção locais; o diff é reversível e a fronteira é preservada. Custo aceito: um query param adicional e uma recarga na troca de seletor — ambos restritos a `/tasks`.

## [NV]

- Execução do roteiro em staging real zerado após a correção e estado do DB: **[NV]** — a reprodução é a entrada; nenhuma requisição ou escrita ao vivo foi feita neste artefato.
- Gates, build, testes e formatters: **[NV]** neste artefato — consolidados no handoff; este documento não os reexecutou.
- RLS efetivo do PostgreSQL e comportamento de rede do `ApiClient`: **[NV]** — as verificações de aplicação são mantidas independentemente deles.

## Fora de âmbito

Mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`; alteração de required checks, workflows, ADRs, constituição ou changelog; migração, schema, RLS, RBAC ou nova rota/flag; mudança em `POST/PATCH/DELETE /api/v1/tasks`, canais, mensagens, Kanban, status patch, i18n ou seletor global; adição de filtros de status/responsável na listagem; quebra do contrato sem `workspace_id`; deploy, escrita em staging/produção e commits.

## Handoff

`/speckit.specify` — especificar o escopo por workspace como requisitos verificáveis (FR-001..FR-005 / SC-001..SC-003) e fila T001..T008, com `workspace_id` opcional e validado em `GET /api/v1/tasks`, consulta via `find_by_workspace_id`, contrato sem parâmetro preservado, frontend enviando o parâmetro e recarregando na troca, cobertura de contratos e fronteira preservada. Execução em staging e gates ficam no handoff **[NV]**.
