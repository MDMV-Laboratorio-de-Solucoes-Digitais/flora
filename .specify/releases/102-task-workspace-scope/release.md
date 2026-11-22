# Release: 102 — escopo por workspace na listagem de tarefas

**Ciclo**: 102 · **Data**: 2026-11-21 · **Branch**: `feature/102-task-workspace-scope` @ `ae8632e10`
**Baseline**: `ae8632e10` (`github/dev` após PR #342)

## Entregue

- `GET /api/v1/tasks` aceita `workspace_id` opcional (`ListTasksQuery`). Sem parâmetro, o comportamento anterior é preservado integralmente. Com parâmetro válido, a listagem usa `TaskService::list_tasks(..., Some(user_id))` e reaplica `!is_deleted && organization_id == org_id`.
- Guard BOLA: `ensure_workspace_in_org` valida o parâmetro contra a organização da sessão; workspace inexistente ou de outra org retorna 422 com `field=workspace_id`, sem revelar qual condição falhou.
- Frontend: `tasksEndpoint()` monta a URL com `workspace_id` codificado; trocar o workspace no seletor recarrega a lista (`isLoading`/`errorMessage` tratados), sem logout.
- Teste unitário puro `tasksEndpoint` (3 casos) e medição ao vivo em staging local zerado (4 casos de API + 2 de UI).

## SemVer

**Proposta:** PATCH `0.7.3` → `0.7.4`. Correção de comportamento de uma capacidade existente (listagem de tarefas), com parâmetro opcional e contrato anterior inalterado. Não é MINOR: nenhuma rota, coluna, migration, variável de ambiente, campo de resposta ou mudança de interface nova — apenas o filtro passa a respeitar o contexto já selecionado pelo usuário. Não é MAJOR: compatibilidade retroativa integral.

## Changelog (FR / T / bug)

- `FR-001` query param opcional em `GET /api/v1/tasks`; ausência mantém a lista por responsável em toda a org (T002).
- `FR-002` `ensure_workspace_in_org` antes da consulta, com 422 uniforme (T003).
- `FR-003` consulta escopada por workspace, mantendo `assignee_id = Some(user_id)` e os filtros de exclusão/organização (T003).
- `FR-004` frontend envia o parâmetro e recarrega ao trocar o seletor (T004, T005).
- `FR-005` rastreabilidade em assessoria/spec/plan/tasks/analyze/converge/R4/governança; gates e fronteira preservados (T006, T007, T008).
- **Bug**: nenhum ID de `.specify/bugs/` acionado. A falha é o item 4.6/7.6 do roteiro de staging (workspace selecionado não escopava tarefas), registrada em `docs/governance/2026-11-21-102-task-workspace-scope.md`.

## Evidência

`.specify/validations/102-task-workspace-scope/validate.md` (**R4 APROVADO**) · `specs/102-task-workspace-scope/analyze.md` (5/5 FR, 3/3 SC, 8/8 tarefas, 0 críticos, 1 divergência documentada) · `specs/102-task-workspace-scope/converge.md` (**Converged**) · `.specify/assessments/102-task-workspace-scope/{intake,research,define,shape,decision}.md` · `docs/governance/2026-11-21-102-task-workspace-scope.md`. Gate do alvo **PASS** reexecutado após o patch (`exit=0`, 10 legs / 77 pisos).

## Rollback

Reverter os quatro arquivos de código/teste do ciclo (`flora-api/src/routes/tasks.rs`, `apps/frontend/src/lib/components/tasks/tasks-query.ts`, `apps/frontend/tests/unit/tasks-query.test.ts`, `apps/frontend/src/routes/(app)/tasks/+page.svelte`) em **novo commit de reversão**, sem force-push e sem reescrita. Os documentos do ciclo permanecem como histórico. Não há migration, variável de ambiente nem dado persistente a desfazer; reapontar API/frontend para o build anterior se já publicado. Não reverter gate, required checks, constituição, ADR, RBAC, schema, PRs ou merges — nenhum foi alterado.

## [NV]

- Staging real da MDMV-Autonoma, CI remota, branch protection, PR/merge e deploy: **[NV]** até verificação ao vivo.
- Contrato DB-gated do T006 (substituído por teste unitário + API ao vivo): **[NV]**.
