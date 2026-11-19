# Research: E2E dedicado de criação de tarefa

- **Coletado**: 2026-11-19T10:13Z · fontes: código local e config do Playwright.

## Evidência

- `playwright.config.ts:3-9` — `baseURL` 4173, `testMatch: '**/*.e2e.ts'`, `retries: 0`, `workers: 1`.
- `playwright.config.ts:19-23` — projeto `setup` e `chromium` com `storageState: playwright/.auth/alice.json`, `dependencies: ['setup']`.
- `playwright.config.ts:26-41` — `webServer` API (`cargo run -q -p flora-api`) e web (`npm run build && npm run preview`), `reuseExistingServer` fora de CI.
- `seed.setup.ts:17-32` — `psql ... -d flora -f dev-seed.sql -f scripts/dev-smoke-session.sql`; `:33-49` minter de storageState; `:61-70` exige org Acme na API.
- `seed-flow.e2e.ts:10-21` — seleção de org com escopo na linha da Acme (evita strict-mode); `:41-46` cria tarefa e confirma mensagem + card.
- `tasks/+page.svelte:161-191` — `Label for="workspace"`/`<select id="workspace">`, `Label for="title"`, `id="desc"`, checkbox “Assign to me”, botão `Create task` desabilitado sem workspace/título.
- `tasks/+page.svelte:68-88` — POST `/api/v1/tasks`, `Task created: ${created.title}`, `loadTasks()`.
- `KanbanBoard.svelte:57-58,78` — `role="region"`, `aria-label="{col.title} column"`, `CardTitle` com o título.

## Riscos

Sessão de um uso só (select-org rotaciona jti); IDs fixos do seed; título pode colidir com banner (usar `exact: true`); assignee depende de `authState.user?.id`; setup depende de Postgres/API/web — execução local exige ambiente isolado.
