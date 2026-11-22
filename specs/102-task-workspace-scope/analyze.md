# Analyze: 102 — escopo por workspace na listagem de tarefas (READ-ONLY)

**Data**: 2026-11-21 · **Modo**: READ-ONLY · **Branch**: `feature/102-task-workspace-scope` @ `ae8632e10`
**Baseline**: `ae8632e10` (`github/dev` após PR #342)
**Artefatos revisados**: `.specify/feature.json`, `.specify/assessments/102-task-workspace-scope/{intake,research,define,shape,decision}.md` (veredito `go`), `specs/102-task-workspace-scope/{spec,plan,tasks}.md`, `docs/governance/2026-11-21-roteiro-staging-flora.md`, `flora-api/src/routes/tasks.rs`, `flora-api/src/tenant.rs`, `flora-core/src/services/task_service.rs`, `flora-core/src/repositories/task_repository.rs`, `apps/frontend/src/routes/(app)/tasks/+page.svelte`.

## Diagnóstico determinístico

- **Reprodução**: em staging local zerado, `QA-001` (workspace `Main Workspace`) permaneceu visível após selecionar `QA Workspace 2` no seletor de `/tasks` (roteiro itens 4.6 e 7.6).
- **Causa**: `list_tasks` (`flora-api/src/routes/tasks.rs:132-168`) buscava apenas `find_by_assignee_id(user_id, Pagination)` e pós-filtrava `organization_id == org_id`; `onWorkspaceChange` (`+page.svelte:115-119`) atualizava só `selectedWorkspaceId`/`authState`, sem recarregar a lista.
- **Capacidade ociosa**: `TaskService::list_tasks(workspace_id, pagination, status, assignee_id)` (`flora-core/src/services/task_service.rs:94-104`) e `TaskRepository::find_by_workspace_id` (`flora-core/src/repositories/task_repository.rs:41-115`) já existiam; nenhuma migration, trait ou novo serviço era necessário.
- **Guard BOLA reutilizável**: `ensure_workspace_in_org(&state, org_id, workspace_id)` (`flora-api/src/tenant.rs:26-46`) devolve o mesmo `Error::Validation { field: "workspace_id", message: "workspace not found" }` para workspace inexistente ou de outra org, sem oracle de enumeração.

## Checagem determinística

```
artifacts=8  critical=0  fr=5/5  sc=3/3  tasks=8/8  go=True  divergences=1
FR match= True ['FR-001','FR-002','FR-003','FR-004','FR-005']
SC match= True ['SC-001','SC-002','SC-003']
tasks unique= True ['T001'..'T008']
```

**Divergência registrada (não crítica)**: T006 previa contrato DB-gated em `flora-tests/src/contract/tasks.rs`. Adicionar um teste `#[ignore]` alteraria o total de 77 e exigiria atualizar `FLOORS` e `.github/workflows/rust.yml` — fora da fronteira do ciclo (gate/required checks). A cobertura foi substituída por (a) teste unitário frontend puro (`tasksEndpoint`) e (b) verificação ao vivo da API em staging local zerado, com quatro casos medidos. Nenhuma divergência de contrato entre assessoria, spec, plan, tasks e o código implementado.

## Rastreabilidade

| # | Requisito | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — `workspace_id` opcional em `GET /api/v1/tasks`; ausência preserva contrato atual | T002 | `ListTasksQuery` em `tasks.rs`; medição `sem_param status=200` com `QA-001` |
| R2 | FR-002 — workspace válido é validado na org da sessão | T003 | `ensure_workspace_in_org` em `tasks.rs:162-163`; medição `workspace_inexistente status=422` |
| R3 | FR-003 — lista restrita ao workspace e ao responsável, com `!is_deleted`/`organization_id` reaplicados | T003 | `TaskService::list_tasks(..., Some(user_id))` e filtro existente; `workspace_qa2 → []`, `workspace_main → QA-001` |
| R4 | FR-004 — frontend envia o parâmetro e recarrega ao trocar seletor | T004, T005 | `tasksEndpoint(selectedWorkspaceId)` e `onWorkspaceChange` assíncrono; UI medida com estado vazio e retorno |
| R5 | FR-005 — rastreabilidade, gates e fronteira preservados | T006, T007, T008 | Vitest 45/45, fmt/clippy/check/lint exit 0, gate do alvo reexecutado no handoff |

SC-001 (troca sem logout) coberto por R4; SC-002 (cross-org/inexistente fail-closed, contrato sem parâmetro intacto) por R1/R2/R3; SC-003 (nenhum segredo/gate/required check tocado) por R5.

## [NV]

- Staging real da MDMV-Autonoma (URLs/credenciais não entregues): toda medição foi feita em ambiente local zerado equivalente — **[NV]** para o FQDN de staging.
- PR/CI/branch protection/merge/deploy: **[NV]** até verificação ao vivo.
- T006 como contrato DB-gated: substituído conforme divergência acima; nova execução com Postgres real do ciclo permanece **[NV]**.

## Resultado

**PASS — 5/5 FR, 3/3 SC, 8/8 tarefas, 0 críticos, 1 divergência documentada.** Nenhuma edição nesta fase; gates/build/testes não reexecutados por este analyzer.
