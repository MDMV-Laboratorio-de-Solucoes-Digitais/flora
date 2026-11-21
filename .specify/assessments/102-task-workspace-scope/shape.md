# Shape: 102 — escopo por workspace na listagem de tarefas

**Data:** 2026-11-21 · **Branch**: `feature/102-task-workspace-scope@ae8632e10f9f8018ab61f8708ce9f8a6857e4e7d`

## Forma escolhida

Ligar duas pontas já existentes, sem criar superfície nova. No backend, `GET /api/v1/tasks` passa a aceitar `workspace_id` como query param opcional; quando presente, valida o tenant com `ensure_workspace_in_org` (`flora-api/src/tenant.rs:26-46`) e lista por `find_by_workspace_id` (`flora-core/src/traits/repository.rs:218-225`, já usado por `TaskService::list_tasks` em `flora-core/src/services/task_service.rs:94-104`). Quando ausente, mantém `find_by_assignee_id` e o filtro por organização atuais (`flora-api/src/routes/tasks.rs:150-165`). No frontend, `loadTasks()` envia o workspace selecionado e `onWorkspaceChange` dispara a recarga (`apps/frontend/src/routes/(app)/tasks/+page.svelte:52-54,115-119`). O padrão de rota escopada é o de canais (`flora-api/src/routes/channels.rs:173-212`) — nenhuma convenção paralela.

1. **Causa e rastro (T001 / intake, research)**: fixar a cadeia do roteiro 4.6/7.6 (`docs/governance/2026-11-21-roteiro-staging-flora.md:123,151,155`) e o código: seletor muda estado local, `loadTasks()` não envia workspace, `list_tasks` busca por responsável em todo o org (`tasks.rs:131-168`).
2. **Parâmetro opcional (T002 / FR-001)**: `Query<...>` em `list_tasks` com `workspace_id: Uuid` opcional; ausência preserva `find_by_assignee_id` (`:150-158`) e o filtro `organization_id` (`:163`); UUID malformado → 4xx, não é ignorado. Nenhuma rota nova.
3. **Guard e consulta (T003 / FR-002, FR-003)**: com parâmetro, manter membership (`:142-146`), chamar `ensure_workspace_in_org` (`tenant.rs:26-46`), usar `find_by_workspace_id(workspace_id, Pagination { limit: 50, offset: 0 }, None, None)` e reaplicar `!is_deleted && organization_id == org_id` (`tasks.rs:163`), pois a SQL do repositório filtra só por workspace (`task_repository.rs:53-58`). `TaskResponse` inalterado.
4. **Frontend (T004–T005 / FR-004)**: `loadTasks()` monta a URI com `?workspace_id=<selecionado>` quando houver seleção; `onWorkspaceChange` chama a recarga após atualizar `selectedWorkspaceId`/`authState`, com `isLoading`/`errorMessage` como em `refresh()` (`+page.svelte:56-67`). Sem logout e sem reload manual.
5. **Contratos (T006–T007 / FR-005)**: estender `flora-tests/src/contract/tasks.rs` com os cinco cenários (escopo, ausência de parâmetro, cross-org, UUID inválido, nenhuma tarefa de outra org) e cobrir no frontend a URL e o refetch; preservar `flora-tests/src/contract/auth.rs:799-802` e o guard de criação (`contract/tasks.rs:226-289`).
6. **Fecho (T008)**: rastreabilidade T001..T008 em assessment/spec/plan/tasks, gates do alvo registrados no handoff **[NV]** e verificação da fronteira proibida.

- **Quebrar o contrato sem parâmetro**: mitigado tornando `workspace_id` opcional e deixando o caminho antigo intocado (`tasks.rs:150-165`); `flora-tests/src/contract/auth.rs:799-802` continua como contrato de regressão.
- **BOLA via query param**: um `workspace_id` de outra organização poderia vazar tarefas se fosse aceito como mero filtro; mitigado por `ensure_workspace_in_org` (`tenant.rs:26-46`) antes da consulta e pela manutenção do filtro `organization_id == org_id` (`tasks.rs:163`), já que a SQL de `find_by_workspace_id` não filtra organização.
- **Escopo incompleto no frontend**: corrigir só o backend deixaria a UI sem refetch; corrigir só a UI não teria parâmetro para enviar. Mitigado por T002–T005 como um único caminho ponta a ponta.
- **Divergência de parâmetros**: `workspace_id` já é o nome usado em `channels.rs:173-177` e no body de criação de tarefa (`tasks.rs:33,77`); reutilizar o mesmo nome evita dois contratos.
- **Estado de carregamento/erro na troca**: uma recarga sem `isLoading`/`errorMessage` poderia deixar lista antiga em silêncio; mitigado reaproveitando o tratamento de `refresh()` (`+page.svelte:56-67`).
- **Mudança de comportamento dos filtros Kanban**: fora de escopo; `status`/`assignee_id` ficam `None` na consulta escopada e a lógica existente da UI permanece.
- **Resultado em staging real e gates**: **[NV]** neste artefato — a correção não foi executada contra staging/produção e nenhum gate/build/teste foi reexecutado aqui.

## Entrega

Diff restrito a `flora-api/src/routes/tasks.rs` (query param opcional, guard de tenant e consulta escopada), `apps/frontend/src/routes/(app)/tasks/+page.svelte` (envio do parâmetro e refetch), `flora-tests/src/contract/tasks.rs` e testes de frontend, além dos artefatos do ciclo 102. Arquivos lidos, não modificados: `flora-api/src/tenant.rs`, `flora-api/src/routes/channels.rs`, `flora-core/src/services/task_service.rs`, `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/task_repository.rs`. Verificação documental cobre T001–T008; gates, build, testes e o roteiro em staging real ficam **[NV]** até o handoff.
