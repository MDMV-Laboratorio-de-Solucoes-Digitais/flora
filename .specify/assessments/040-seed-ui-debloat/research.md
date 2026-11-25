# Idea Research: 040 — Seed UI debloat + board real

- **Slug**: 040-seed-ui-debloat
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high

**Cobertura idiomática:** pt-BR_en (prosa em pt-BR; nomes de código, rotas e status mantidos em inglês).

## Users & Demand

- O demandante registrado é o maintainer/SE da MDMV; o objetivo declarado é "alternativa self-hosted em VPS barata", com o cliente Macro original (`apps/web`, 5.844 arquivos / 43 MB, zero refs funcionais) como peso de checkout e o `KanbanBoard` órfão como funcionalidade ausente. [source: .specify/assessments/040-seed-ui-debloat/intake.md:10,18-19; decision.md:6-11 | system] (confidence: high)
- O usuário final implícito é quem cria e acompanha tarefas no Seed: a rota `(app)/tasks` já carrega tarefas da API e o board substitui o `<ul>` inline sem novo endpoint; a persistência usa o `PATCH /api/v1/tasks/{id}` existente. [source: specs/040-seed-ui-debloat/spec.md:18-22; apps/frontend/src/routes/(app)/tasks/+page.svelte:98-114,210-215 | system] (confidence: high)
- Pesquisa anterior do próprio ciclo registrou, como mínimo de PME para o board, criar tarefa com assignee+status, ver board, filtrar e ver "minhas" — demanda funcional documentada, não observação de usuário ao vivo. [source: docs/research/2026-11-17-040-seed-ui-debloat.md:42-44 | system] (confidence: medium)
- Não há entrevista, telemetria ou métrica de uso de VPS anexada a estes artefatos; a demanda é inferida do objetivo registrado, não medida. [source: inferência sem fonte primária de usuário | ASSUMPTION] (confidence: medium)

## Prior Art

- O `KanbanBoard` anterior era 100% mock (tarefas t1–t4, tags, avatar) com drag apenas em `$state` local; o componente atual é controlado: `tasks`/`onStatusChange` via `$props()` e colunas derivadas com `$derived(groupByStatus(tasks))`, sem estado duplicado. [source: docs/research/2026-11-17-040-seed-ui-debloat.md:35-37; apps/frontend/src/lib/components/tasks/KanbanBoard.svelte:12-24 | system] (confidence: high)
- O backend já oferecia CRUD de tarefas e aceitava `status` em `Todo|InProgress|Done|Archived`; mover card não exigiu novo endpoint nem migration. [source: flora-api/src/routes/tasks.rs:120,267-302; flora-core/src/models/task.rs:92-127 | system] (confidence: high)
- A lógica de agrupamento e validação foi isolada em módulo puro testável no projeto `server` do Vitest (gate required), com 6 testes iniciais e helpers puros para serialização do PATCH. [source: apps/frontend/src/lib/components/tasks/kanban.svelte.ts:4-36; apps/frontend/src/lib/components/tasks/status-patch.svelte.ts:1-17; apps/frontend/tests/unit/kanban.test.ts:17-74; apps/frontend/vite.config.ts:47-49 | system] (confidence: high)
- `$derived` recomputeia quando dependências mudam e não deve ter efeitos colaterais — o padrão usado no componente segue essa semântica documentada do Svelte 5.57. [source: https://github.com/sveltejs/svelte/blob/svelte%405.57.0/documentation/docs/02-runes/03-%24derived.md | web] (confidence: high)
- Os dois arquivos removidos (`tasks.spec.ts`, `files.spec.ts`) tinham sufixo `.spec.ts`, enquanto o Playwright executa apenas `testMatch: '**/*.e2e.ts'`; a documentação upstream confirma que só arquivos que casam com `testMatch` são executados. [source: apps/frontend/playwright.config.ts:17-18; .specify/validations/040-seed-ui-debloat/validate.md:19-23; https://github.com/microsoft/playwright/blob/main/docs/src/test-api/class-testconfig.md | system+web] (confidence: high)
- `apps/web` havia sido importado do Macro original em `82ee437f2` e não era referenciado por workspace bun, workflows, configs ou `@coparse`; a remoção foi registrada como diff somente `D`. [source: docs/research/2026-11-17-040-seed-ui-debloat.md:29-32; .specify/validations/040-seed-ui-debloat/validate.md:17-25 | system] (confidence: high)

## Market & Context

- O norte estratégico do repositório é uma alternativa self-hosted viável ao Macro para PMEs em VPS barata; o ciclo 040 é registrado como entregue no ROADMAP. [source: ROADMAP.md:4,58 | system] (confidence: high)
- O ciclo 039 (restore drill, RTO 97 s, caps 1 vCPU/1 GB) precede o 040 e contextualiza a preocupação com footprint em VPS; não há medição de custo de hospedagem anexada ao 040. [source: ROADMAP.md:57; docs/research/2026-11-17-040-seed-ui-debloat.md:49-51 | system] (confidence: medium)
- O contrato de verificação foi congelado antes da implementação (c040-10): check/lint/test:unit `--project server`/build no frontend, fmt e integridade de gate no Rust, diff apenas `D`, zero refs a `apps/web`/`@coparse` e auditoria cega em contexto fresco. [source: specs/040-seed-ui-debloat/spec.md:33-36; specs/040-seed-ui-debloat/tasks.md:3-11 | system] (confidence: high)
- Os required checks autoritativos são `frontend-build`, `rust-build`, `db-tests`, `dependency-review`; ADR-0003 rege o merge como decisão de máquina sob quatro checks verdes. [source: scripts/ci/check_gate_integrity.py:28-32; .specify/assessments/040-seed-ui-debloat/decision.md:28-29 | system] (confidence: high)
- Afirmar redução de custo financeiro em VPS seria inferência: o artefato prova apenas I/O de checkout e contagem de arquivos, sem benchmark de binário, build ou runtime. [source: inferência sem medição de custo | ASSUMPTION] (confidence: medium)

## Data & Constraints

- Fora de escopo por decisão registrada: backend/migration, RBAC/RLS/tenancy/authz, Tauri (ADR-0002), Documentos FASE 7, paridade Macro, ADR/constituição, deploy/tag/release/secrets e proteção de required checks. [source: .specify/assessments/040-seed-ui-debloat/decision.md:19-23; specs/040-seed-ui-debloat/spec.md:25-31 | system] (confidence: high)
- Status canônicos do board são `Todo|InProgress|Done`; `Archived` é omitido. O backend aceita quatro valores e, para status inválido, faz fallback silencioso para o status atual em vez de 400; o frontend bloqueia não-canônicos antes de enviar. [source: apps/frontend/src/lib/components/tasks/kanban.svelte.ts:5,19-35; flora-core/src/models/task.rs:92-125; flora-api/src/routes/tasks.rs:298-302; apps/frontend/src/lib/components/tasks/status-patch.svelte.ts:9-16 | system] (confidence: high)
- `GET /tasks` é assignee-scoped quando não há `workspace_id`, com limite de 50 itens e filtro por `organization_id` da sessão; não há ordem persistida pelo board. [source: flora-api/src/routes/tasks.rs:140-178; specs/040-seed-ui-debloat/spec.md:30-31 | system] (confidence: high)
- Gates registrados no validate: check 0/0, lint limpo, 10 arquivos/30 testes → 11 arquivos/36 testes, build ok, fmt e `check_gate_integrity.py` PASS; a auditoria encontrou e corrigiu o caminho `changeStatus`→PATCH sem teste, elevando para 39 testes. [source: .specify/validations/040-seed-ui-debloat/validate.md:6-15,44-52 | system] (confidence: high)
- No checkout desta worktree (`e88dc8e4d`), `apps/` contém apenas `frontend/`, os dois `.spec.ts` mortos não existem e os módulos do board estão presentes — coerente com o diff de remoção registrado. [source: apps/; apps/frontend/src/lib/components/tasks/; worktree HEAD `e88dc8e4d` | system] (confidence: high)
- Forgejo/proteções e resultado de CI remoto não foram observados nesta sessão: **[NV]**. [source: .specify/assessments/040-seed-ui-debloat/decision.md:25-29; specs/040-seed-ui-debloat/spec.md:41-43 | system] (confidence: high)

## Evidence Against the Idea

- A auditoria author-blind em contexto fresco emitiu **NO-GO para merge "dark" pleno** embora nenhum gate determinístico tenha falhado; a conversão para GO dependia de CI remoto verde e de decisão de produto sobre `Archived`. [source: .specify/validations/040-seed-ui-debloat/validate.md:36-56 | system] (confidence: high)
- Defeito MEDIUM registrado: o caminho real `changeStatus`→PATCH não era coberto (apenas helpers puros); a correção veio depois do contrato congelado, mostrando que o congelamento não cobria a serialização. [source: .specify/validations/040-seed-ui-debloat/validate.md:44-49 | system] (confidence: high)
- O board não mostra tarefas `Archived`, não persiste ordem de colunas e não é visão de time (GET assignee-scoped); foram explicitamente declarados non-goals, portanto não devem ser atribuídos à ideia. [source: specs/040-seed-ui-debloat/spec.md:30-31; .specify/validations/040-seed-ui-debloat/validate.md:49-50; flora-api/src/routes/tasks.rs:163-171 | system] (confidence: high)
- DnD sem `onkeydown` e `taskStore` órfão foram achados de baixa severidade mantidos; lanes `client`/browser e2e não são gates required, então a interação do board não está protegida por gate. [source: .specify/validations/040-seed-ui-debloat/validate.md:50-52,63; specs/040-seed-ui-debloat/spec.md:40-41 | system] (confidence: high)
- O valor histórico de UX de `apps/web` não foi auditado ([NV]) e a remoção é irreversível no checkout; não há evidência primária local de que nenhum usuário dependesse daquele cliente. [source: .specify/assessments/040-seed-ui-debloat/decision.md:25-27; docs/research/2026-11-17-040-seed-ui-debloat.md:42-43 | system] (confidence: medium)
- Observações sobre o Macro remoto (`docs.macro.com` 403, `macro.com` JSON-LD) foram registradas em ciclo anterior, mas não foram reconsultadas nesta sessão: hosts não allowlisted, **[UNVERIFIED — fetch skipped: host not allowlisted]**. [source: docs/research/2026-11-17-040-seed-ui-debloat.md:42-44 | system] (confidence: medium)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Qual inventário exato de referências autoriza a remoção integral de `apps/web` em todos os checkouts?] [source: .specify/assessments/040-seed-ui-debloat/intake.md:23 | system] (confidence: high)
- [NEEDS CLARIFICATION: Os dois arquivos `.spec.ts` citados são os únicos e2e mortos, ou existem outros fora do `testMatch`?] [source: .specify/assessments/040-seed-ui-debloat/intake.md:24 | system] (confidence: high)
- [NEEDS CLARIFICATION: Qual comportamento esperado quando uma transição de status inválida chega ao backend, dado o fallback silencioso?] [source: .specify/assessments/040-seed-ui-debloat/intake.md:25; flora-api/src/routes/tasks.rs:298-302 | system] (confidence: high)
- [NEEDS CLARIFICATION: Como serão preservados estados de empty/loading e a ordem das colunas ao trocar o `<ul>` pelo board?] [source: .specify/assessments/040-seed-ui-debloat/intake.md:26 | system] (confidence: high)
- [NEEDS CLARIFICATION: O valor histórico de UX de `apps/web` foi auditado antes da remoção ou permanece [NV]?] [source: .specify/assessments/040-seed-ui-debloat/intake.md:27 | system] (confidence: high)
- [NEEDS CLARIFICATION: Quem valida a auditoria cega em contexto fresco e qual critério define o board como "real", considerando o NO-GO registrado?] [source: .specify/assessments/040-seed-ui-debloat/intake.md:28; .specify/validations/040-seed-ui-debloat/validate.md:40 | system] (confidence: high)

## Sources

- `system` — `.specify/assessments/040-seed-ui-debloat/intake.md:10,18-28` — lido diretamente 2026-11-25; policy: source local primária.
- `system` — `.specify/assessments/040-seed-ui-debloat/decision.md:6-29` — lido diretamente 2026-11-25; policy: source local primária.
- `system` — `specs/040-seed-ui-debloat/spec.md:6-43` e `tasks.md:3-11` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `.specify/validations/040-seed-ui-debloat/validate.md:6-64` — lido diretamente 2026-11-25; policy: source local primária.
- `system` — `docs/research/2026-11-17-040-seed-ui-debloat.md:27-51` e `docs/research/index.md:158` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `apps/frontend/src/lib/components/tasks/{kanban.svelte.ts,status-patch.svelte.ts,KanbanBoard.svelte,types.ts}`, `apps/frontend/src/routes/(app)/tasks/+page.svelte`, `apps/frontend/tests/unit/kanban.test.ts`, `apps/frontend/playwright.config.ts`, `apps/frontend/vite.config.ts` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `flora-api/src/routes/tasks.rs:120,140-178,267-302` e `flora-core/src/models/task.rs:92-127` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `ROADMAP.md:4,57-58`, `scripts/ci/check_gate_integrity.py:28-32`, `docs/PROGRESSO.md:555` — lidos diretamente 2026-11-25; policy: source local primária.
- `web` — `https://github.com/sveltejs/svelte/blob/svelte%405.57.0/documentation/docs/02-runes/03-%24derived.md` — host `github.com`, policy `allowlisted`, acesso 2026-11-25 (o leitor serviu o espelho `raw.githubusercontent.com` do mesmo arquivo; nenhum link da página foi seguido).
- `web` — `https://github.com/microsoft/playwright/blob/main/docs/src/test-api/class-testconfig.md` — host `github.com`, policy `allowlisted`, acesso 2026-11-25 (mesma observação de espelho raw).
- `auto-refused` — `https://docs.macro.com/tasks` e `https://macro.com/tasks` — hosts não allowlisted; **[UNVERIFIED — fetch skipped: host not allowlisted]**; apenas registro histórico local, sem reconsulta.
- `auto-refused` — `https://svelte.dev/docs` e `https://playwright.dev/docs` — hosts não allowlisted; **[UNVERIFIED — fetch skipped: host not allowlisted]**; documentação equivalente obtida via GitHub allowlisted.
