# Converge: 102 — escopo por workspace na listagem de tarefas

**Data**: 2026-11-21 · **Branch**: `feature/102-task-workspace-scope` @ `ae8632e10`
**Modo**: artefato append-only — cada entrada registra o estado observado, sem reescrever entradas anteriores.

## Entrada 1 — 2026-11-21 (implementação e convergência)

**Status: Converged — sem novas tarefas.**

### Mudanças entregues

- `flora-api/src/routes/tasks.rs`: `ListTasksQuery { workspace_id: Option<Uuid> }`; `list_tasks` aceita o query param, mantém a checagem de membership, chama `ensure_workspace_in_org` quando o parâmetro está presente, usa `TaskService::list_tasks(workspace_id, pagination, None, Some(user_id))` e preserva o caminho anterior (`find_by_assignee_id`) e o pós-filtro `!is_deleted && organization_id == org_id`.
- `apps/frontend/src/lib/components/tasks/tasks-query.ts` (novo): `tasksEndpoint(workspaceId)` — helper puro, sem workspace mantém `/api/v1/tasks`, com workspace codifica `workspace_id`.
- `apps/frontend/tests/unit/tasks-query.test.ts` (novo): 3 testes (sem workspace, com workspace, codificação).
- `apps/frontend/src/routes/(app)/tasks/+page.svelte`: `loadTasks()` usa `tasksEndpoint(selectedWorkspaceId)`; `onWorkspaceChange` passa a ser assíncrono, marca `isLoading`, limpa `errorMessage`, recarrega e trata falha com `apiErrorMessage`, finalizando `isLoading` no `finally`.

2. Nenhum `#[ignore]` novo em `flora-tests`: o gate deriva pisos de `cargo --ignored --list`; um teste extra exigiria alterar `FLOORS` e `rust.yml`, fora da fronteira. Substituído por teste unitário + medição ao vivo (ver `analyze.md`).

1. `assignee_id: Some(user_id)` na consulta escopada — preserva a regra "minhas tarefas"; passar `None` ampliaria a visibilidade dentro da org (decisão de produto fora deste ciclo).
2. Nenhum `#[ignore]` novo em `flora-tests`: o gate deriva pisos de `cargo --ignored --list`; um teste extra exigiria alterar `FLOORS` e `rust.yml`,之外 da fronteira. Substituído por teste unitário + medição ao vivo (ver `analyze.md`).
3. O helper `ensure_workspace_in_org` foi reutilizado em vez de uma checagem ad hoc: erro uniforme para workspace inexistente/cross-org (422), sem enumeração.

### Verificação observada

- `cargo fmt --all -- --check` → exit 0.
- `CARGO_TARGET_DIR=… cargo check -p flora-api` → 0 erros; `cargo clippy -p flora-api --all-targets -- -D warnings` → exit 0.
- `bun run test:unit -- --run --project server` → 13 arquivos, **45 testes** (antes: 42).
- `bun run check` → 0 erros/0 warnings; `bun run lint` → exit 0.
- API em staging local zerado: sem parâmetro → `QA-001`; `workspace_id=QA Workspace 2` → `[]`; `workspace_id=Main Workspace` → `QA-001`; UUID inexistente → `422 {"error":"validation_error","field":"workspace_id"}`.
- UI: trocar para `QA Workspace 2` exibe "Nenhuma tarefa encontrada para o responsável/organização atual."; voltar a `Main Workspace` restaura `QA-001`, sem logout.

### Fronteira preservada

Nenhum toque em gate, required checks, constituição, ADR, schema, migration, RBAC, flags, `STATE.md` histórico, `PROGRESSO*` histórico, `AGENTS.md`, validate/release de outros ciclos. Nenhuma credencial real em artefato.
