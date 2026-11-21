# Tasks: 099 — interface 100% pt-BR

**Input**: `specs/099-ptbr-ui/spec.md`, `plan.md`,
`.specify/assessments/099-ptbr-ui/decision.md`, handoff I18nScout

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T010 concluídas; gate do alvo PASS
(registrado no handoff consolidado).

## Phase 1 — Dicionário e idioma do documento

- [X] T001 Criar `apps/frontend/src/lib/i18n/pt-BR.ts` com `export const t =
  {...} as const` (chaves achatadas, incluindo app, sessão, autenticação,
  navegação, IA, canais, chat, busca, arquivos, organização, workspaces,
  configurações, tarefas, Kanban, erros e demonstração) e `export type
  TranslationKey = keyof typeof t`; criar `apps/frontend/src/lib/i18n/index.ts`
  reexportando `t`/`TranslationKey` e expondo `format(chave, valores)` que
  preenche marcadores `{id}`, `{query}`, `{name}` e `{title}` (FR-001,
  SC-001, SC-002).
- [X] T002 Confirmar que `apps/frontend/src/app.html:2` permanece com
  `lang="pt-BR"`, sem regressão para `en` e sem duplicação do atributo
  (FR-002).

## Phase 2 — Autenticação e shell

- [X] T003 Migrar `apps/frontend/src/routes/(auth)/+page.svelte` e
  `(auth)/callback/+page.svelte` para as chaves do dicionário: títulos,
  rótulos, placeholders, estados de envio e mensagens, preservando
  literalmente os textos pt-BR do ciclo 096 e trocando `Authenticating...` por
  `Autenticando...` (FR-001, FR-003, FR-008).
- [X] T004 Traduzir `apps/frontend/src/routes/(app)/+layout.svelte` e
  `routes/+layout.svelte`: menu (`Canais`, `Mensagens diretas`, `Tarefas`),
  dica de canais, conta do usuário, `Minha conta`, `Configurações`, `Sair`,
  `title` de abrir/fechar painel e todo o painel de insights de IA
  (FR-004, SC-001).

## Phase 3 — Páginas e componentes

- [X] T005 Traduzir as páginas `routes/(app)/tasks`, `channels`, `dms`,
  `select-org`, `settings` e `routes/demo/**`: cabeçalhos, descrições,
  formulários, opções vazias, estados de carregamento e mensagens de sucesso
  via `t` (FR-005, SC-001).
- [X] T006 Traduzir os componentes compartilhados em
  `lib/components/{chat,thread,files,organization,search,tasks,workspace}`:
  textos de chat e respostas, placeholders, erros de upload, criação de
  organização/workspace, busca (`aria-label`, `Searching…`, `No results`,
  `result/results`) e labels de filtro (FR-006, SC-001).
- [X] T007 Traduzir os textos visíveis gerados em módulos `.ts`:
  `lib/components/tasks/kanban.svelte.ts` (`A fazer`, `Em andamento`,
  `Concluído`) e `lib/utils/search.ts` (`Mensagem`, `Tarefa`, `Arquivo`),
  mantendo os valores canônicos de status da API inalterados (FR-006,
  SC-001, SC-002).

## Phase 4 — Erros do backend

- [X] T008 Implementar `backendErrorMessage(body, status, fallback)` em
  `lib/i18n/index.ts`: mapear `invalid_credentials`, `rate_limited`,
  `oidc_error`, `conflict`, `validation` (com caso especial
  `field === 'admin_password'`) e os statuses 401/409/422/429 para chaves do
  dicionário, com fallback específico da operação e sem alterar o contrato da
  API (FR-007, SC-002); substituir os `switch` espalhados nos componentes pelo
  mapeador (FR-003).

## Phase 5 — Testes e fecho

- [X] T009 Manter os selectors pt-BR em
  `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts`
  (`voce@exemplo.com`, `Senha`, `Mínimo de 15 caracteres`, `Entrar`,
  `Fazer primeira configuração`, `Criar workspace` e regexes de erro), com as
  mensagens lidas do dicionário e os mesmos textos, preservando a suíte
  existente (FR-008).
- [X] T010 Varrer a UI ativa (`routes/**`, `lib/components/**`,
  `kanban.svelte.ts`, `utils/search.ts`) confirmando zero literais em inglês
  (SC-001) e apenas os termos técnicos autorizados — OIDC, OAuth, API, JWT,
  REST, AI, SSO e slug — fora nomes próprios, rotas, identificadores e status
  canônicos (SC-002); verificar gates/lint/testes verdes (PASS no handoff) e a
  rastreabilidade T001..T010 em decision/spec/plan/tasks, confirmando que
  `validate/`, `release/`, `STATE.md` e `PROGRESSO*` não foram tocados
  (SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010. T002 é
apenas verificação e pode ocorrer em paralelo com T001; T003–T007 dependem de
T001; T008 depende de T001 e consolida os mapeadores de erro; T009 depende de
T003; T010 consolida a varredura, os gates e o rastro documental. IdP externo,
deploy remoto e produção permanecem **[NV]**.
