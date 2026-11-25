# Decision: 102 — escopo por workspace na listagem de tarefas

- **Slug**: 102-task-workspace-scope
- **Decided**: 2026-11-25
- **Verdict**: go (decisão original 2026-11-21, preservada)
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (anterior) | roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:116-123,148-155` | spec/plan/tasks 102

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Roteiro 4.6/7.6 exige tarefas escopadas ao workspace sem logout; `QA-001` do `Main Workspace` continua visível sob `QA Workspace 2` (`roteiro:123,151,155`; `research.md:1-2,14-17`). |
| Evidence strength | adequate | Reprodução em staging local zerado mais cadeia de código determinística nos dois lados, com contratos existentes nomeados; gates, build e staging real seguem **[NV]** (`intake.md:11-35`; `research.md:3-13`). |
| Value vs. inaction | strong | Fecha um bug de escopo observável usando consulta, guard de tenant e precedente de rota já presentes, sem nova rota/schema/flag/RBAC e sem quebrar o contrato sem parâmetro (`decisão anterior:45`). |
| Feasibility / appetite | adequate | Um query param opcional validado, reuso de `find_by_workspace_id` e uma recarga no frontend; custo aceito restrito a `/tasks` (`shape.md`; `decisão anterior:27-31,45`). |
| Strategic fit | strong | Permanece no modelo tenant por `organization_id`, sem serviço obrigatório novo, com fronteira de diff e artefatos preservada (`define.md:FR-001..FR-005,SC-001..SC-003`). |
| Risk posture | adequate | BOLA coberto em duas camadas (`ensure_workspace_in_org` + filtro `organization_id` no handler) e meia-correção evitada ao tratar API e UI como um caminho; RLS/rede **[NV]** (`research.md:8,24-26`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. O registro original data de 2026-11-21, branch `feature/102-task-workspace-scope` @ `ae8632e10f9f8018ab61f8708ce9f8a6857e4e7d`, âncora `.specify/feature.json` → `specs/102-task-workspace-scope`, com evidência de reprodução em staging local zerado (roteiro itens 4.6 e 7.6) e inspeção determinística do código.

Causa raiz verificável e dupla: `onWorkspaceChange` atualiza `selectedWorkspaceId`/`authState` mas não recarrega (`apps/frontend/src/routes/(app)/tasks/+page.svelte:115-119`); `loadTasks()` chama `GET /api/v1/tasks` sem `workspace_id` (`:52-54`); `list_tasks` não extrai query e busca `find_by_assignee_id(user_id, Pagination { limit: 50, offset: 0 })` (`flora-api/src/routes/tasks.rs:106-114,131-168,150-158`), filtrando apenas `!is_deleted && organization_id == org_id` (`:160-165`); a capacidade já existe em `TaskService::list_tasks` (`flora-core/src/services/task_service.rs:94-104`) e `TaskRepository::find_by_workspace_id` (`flora-core/src/traits/repository.rs:218-225`; SQL em `flora-core/src/repositories/task_repository.rs:41-58`, `WHERE workspace_id = $1 AND is_deleted = false`); o precedente de rota escopada é `flora-api/src/routes/channels.rs:173-212` com guard `ensure_workspace_in_org` (`flora-api/src/tenant.rs:26-46`). O isolamento entre organizações já funciona; falta o escopo por workspace dentro da mesma organização, nas duas pontas.

| Compromisso rastreável | Conteúdo preservado |
|---|---|
| FR-001 / T002 — parâmetro opcional | `GET /api/v1/tasks` aceita `workspace_id` UUID opcional no padrão de `channels.rs:173-177`; sem parâmetro mantém `find_by_assignee_id` e filtro por org; UUID inválido → 4xx, nunca silenciado; nenhuma rota nova. |
| FR-002 / T003 — tenant/BOLA | Manter membership (`tasks.rs:142-146`) e chamar `ensure_workspace_in_org` (`tenant.rs:26-46`) antes de listar; workspace de outra org ou inexistente → 4xx fail-closed, sem vazar existência; reaplicar `!is_deleted && organization_id == org_id` (`tasks.rs:163`). |
| FR-003 / T003 — consulta escopada | `find_by_workspace_id(workspace_id, Pagination { limit: 50, offset: 0 }, None, None)` via `TaskService::list_tasks` ou diretamente; `TaskResponse` inalterado (`tasks.rs:85-104,116-129`); nenhuma tarefa de outro workspace/org. |
| FR-004 / T004–T005 — frontend | `loadTasks()` envia `workspace_id` com `selectedWorkspaceId`; `onWorkspaceChange` recarrega após atualizar estado, com `isLoading`/`errorMessage` como `refresh()` (`:56-67`), sem logout nem reload manual. |
| FR-005 / T006–T008 — contratos/fronteira | `flora-tests/src/contract/tasks.rs` cobre escopo, ausência de parâmetro, cross-org, UUID inválido e nenhuma tarefa de outra org; testes de frontend cobrem URL e refetch; `contract/auth.rs:799-802` e guard de criação (`contract/tasks.rs:226-289`) permanecem PASS. |

Critérios preservados: **SC-001** reprodução do roteiro fechada (trocar para `QA Workspace 2` remove `QA-001`; voltar ao `Main Workspace` a reaparece); **SC-002** retrocompatibilidade e isolamento (contrato sem query permanece; cross-org/malformado nunca lista); **SC-003** identificadores `workspace_id`, nenhuma rota/flag/schema/RBAC acrescentada e diff dentro da fronteira. Fila **T001–T008** na implementação. Racional histórico: a correção reaproveita consulta, guard e padrão já presentes; o parâmetro opcional preserva o contrato usado pelos testes; o risco de BOLA é coberto em duas camadas e o de meia-correção tratando backend e frontend como um único caminho; FR/SC são verificáveis por contratos e inspeção locais, diff reversível e fronteira preservada. Custo aceito: um query param adicional e uma recarga na troca de seletor, ambos restritos a `/tasks`.

**[NV]** preservados: execução do roteiro em staging real zerado após a correção e estado do DB (a reprodução é a entrada; nenhuma requisição/escrita ao vivo neste artefato); gates, build, testes e formatters (consolidados no handoff); RLS efetivo do PostgreSQL e comportamento de rede do `ApiClient` (guard de aplicação mantido independentemente).

Fora de âmbito preservado: mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`; alteração de required checks, workflows, ADRs, constituição ou changelog; migração, schema, RLS, RBAC ou nova rota/flag; mudança em `POST/PATCH/DELETE /api/v1/tasks`, canais, mensagens, Kanban, status patch, i18n ou seletor global; adição de filtros de status/responsável na listagem; quebra do contrato sem `workspace_id`; deploy, escrita em staging/produção e commits.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante registrada nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Em staging local zerado, trocar para `QA Workspace 2` não escopa a lista de `/tasks`: `QA-001` (`Main Workspace`) permanece visível, embora o roteiro exija contexto trocado sem logout e tarefas escopadas ao workspace (`roteiro:116-123,148-155`).
- **Chosen approach**: `workspace_id` opcional e validado em `GET /api/v1/tasks` (padrão `channels.rs`), consulta via `find_by_workspace_id`, contrato sem parâmetro preservado; frontend envia o parâmetro e recarrega na troca; API e UI tratadas como um único caminho.
- **In scope / out of scope**:
  - **In**: FR-001..FR-005 / SC-001..SC-003 e fila T001..T008; rota handler e guard de tenant em `tasks.rs`/`tenant.rs`, reuso de `task_service.rs`/`repository.rs`, `+page.svelte`, contratos `tasks.rs` e testes de frontend; fronteira de artefatos preservada.
  - **Out/NV**: migração, schema, RLS, RBAC, nova rota/flag, escrita de tarefa/canais/mensagens/Kanban/i18n/seletor global; `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog, constituição; deploy/escrita ao vivo/commits; execução em staging real, gates, RLS e comportamento de rede **[NV]**.
- **Success metrics**: `QA-001` desaparece ao selecionar `QA Workspace 2` e reaparece ao voltar ao `Main Workspace`, sem logout; `GET /api/v1/tasks` sem parâmetro devolve exatamente o conjunto atual; workspace de outra org/UUID inválido resulta em 4xx fail-closed e nenhuma tarefa cross-org em qualquer caminho; contratos `auth.rs:799-802` e `tasks.rs:226-289` permanecem PASS.
- **Carried-forward open questions**: `[NV]` execução do roteiro em staging real zerado e estado do DB; `[NV]` gates/build/testes/formatters; `[NV]` RLS efetivo e comportamento de rede do `ApiClient` — nenhuma bloqueia o `go`.
