# Decision: 099 — interface 100% pt-BR

- **Slug**: 099-ptbr-ui
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/099-ptbr-ui` @ `7183a33fc`
- **Âncora**: `.specify/feature.json` → `specs/099-ptbr-ui`
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema

O usuário exige 100% pt-BR na interface; outras línguas apenas em termos
técnicos (OIDC, OAuth, API, JWT, REST, AI, SSO, slug). O levantamento do
I18nScout encontrou ~140 literais em inglês em ~15 arquivos de `apps/frontend`
e nenhum mecanismo de tradução. `apps/frontend/src/app.html:2` **já** declara
`lang="pt-BR"`: o defeito não é o atributo, e sim o texto inline.

A cadeia verificável no código é:

1. `routes/(app)/+layout.svelte:129,137,145,155,158,189,195,206,255,270` —
   `Channels`, `DMs`, `Tasks`, `My Account`, `Settings`, `Log out`,
   `AI Insight Panel` e o texto do painel de IA em inglês.
2. `routes/(app)/tasks/+page.svelte:136,170,191,196,198` —
   `Tasks`, `No workspaces available`, `Create task`, `Loading...`;
   `routes/(app)/channels/+page.svelte:114,117,129,151,170,172,196` —
   `Channels`, `New channel`, `No channels yet.`, `No channel selected`.
3. `components/chat/ChatLayout.svelte:104,195,208,225`,
   `components/search/SearchBox.svelte:55-56,70-79`,
   `components/files/FileUpload.svelte:20,24,50`,
   `components/workspace/WorkspaceCreate.svelte:31,36`,
   `components/organization/OrganizationManager.svelte:33,42,46` —
   textos de interface, placeholders, `aria-label` e fallbacks de erro em inglês.
4. `components/tasks/kanban.svelte.ts:13-16` (`To Do`, `In Progress`, `Done`) e
   `lib/utils/search.ts:1-6` (`Message`, `Task`, `File`) — texto visível gerado
   em módulos `.ts`.
5. `routes/(auth)/callback/+page.svelte:69` — `Authenticating...`; a tela de
   login `(auth)/+page.svelte` e seu spec já estavam em pt-BR desde o ciclo 096.
6. `grep` por `i18n`/`locale` em `apps/frontend/src` — nenhum dicionário ou
   helper de tradução no baseline; ~140 literais, sem ponto único de auditoria.

## Decisão

Registrar o ciclo 099 como **tradução integral da interface para pt-BR com
dicionário centralizado**, com oito compromissos rastreáveis (FR-001..FR-008),
três critérios (SC-001..SC-003) e dez tarefas (T001..T010, todas concluídas):

1. **Dicionário (FR-001 / T001)**: `lib/i18n/pt-BR.ts` com `t` (`as const`,
   chaves achatadas) e `TranslationKey`; `lib/i18n/index.ts` reexporta e expõe
   `format()`.
2. **Idioma do documento (FR-002 / T002)**: manter `lang="pt-BR"` em
   `app.html:2`.
3. **Autenticação (FR-003 / T003)**: login/callback em pt-BR via dicionário,
   preservando os textos e selectors do ciclo 096.
4. **Navegação (FR-004 / T004)**: shell, menu, conta e painel de IA em pt-BR.
5. **Páginas (FR-005 / T005)**: `tasks`, `channels`, `dms`, `select-org`,
   `settings` e `demo/**`.
6. **Componentes (FR-006 / T006–T007)**: chat/thread, arquivos, organização,
   busca, tarefas, workspace, `kanban.svelte.ts` e `utils/search.ts`.
7. **Erros do backend (FR-007 / T008)**: `backendErrorMessage` mapeia códigos
   (`invalid_credentials`, `rate_limited`, `oidc_error`, `conflict`,
   `validation`, com o caso `admin_password`) e statuses 401/409/422/429 —
   contrato da API inalterado.
8. **Testes (FR-008 / T009) e fecho (T010)**: selectors pt-BR preservados,
   varredura de zero literais e gates/lint/testes verdes.

## Evidência de entrada

- Baseline `7183a33fc`, branch `feature/099-ptbr-ui`, `.specify/feature.json` →
  `specs/099-ptbr-ui`.
- Handoff I18nScout: ~140 strings em ~15 arquivos, sem i18n; `app.html` já
  `lang="pt-BR"`; gate do alvo PASS.
- `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts:48+` — selectors
  pt-BR que devem permanecer estáveis.
- `apps/frontend/package.json` — `check`, `lint`, `test` e `test:e2e` como
  verificações locais de SC-003.

## Racional

**go.** O defeito é independente de infraestrutura: todo o texto visível está no
repositório e pode ser substituído por chaves de um dicionário tipado, com
verificação por inspeção e pelos testes existentes. A mudança não altera
contrato de API, rotas, autenticação, gates, workflows, ADRs ou constituição; o
diff é reversível e restrito ao frontend e aos artefatos do ciclo. Manter os
textos do ciclo 096 protege os selectors e reduz o risco de regressão.

## [NV]

- IdP externo (Zitadel), rede, deploy remoto e comportamento em produção:
  **[NV]**. Nenhuma chamada externa foi feita.
- Gates, lint, type-check e testes: registrados como PASS no handoff
  consolidado; este artefato não reexecutou nenhuma verificação.
- Textos dinâmicos vindos de serviços externos fora dos códigos mapeados:
  **[NV]**.

## Fora de âmbito

Mudanças em `.specify/validations/`, `.specify/releases/`,
`.specify/memory/STATE.md`, `docs/PROGRESSO*`, `specs/*/analyze.md`,
`specs/*/converge.md`; alteração de required checks, workflows, ADRs ou
constituição; tradução de logs, comentários, nomes de código, chaves de erro da
API ou valores canônicos de status; qualquer idioma além de pt-BR na interface.

## Handoff

`/speckit.specify` — especificar o dicionário pt-BR e a substituição integral
dos literais como requisitos verificáveis (FR-001..FR-008 / SC-001..SC-003) e
dez tarefas T001..T010, preservando `lang="pt-BR"`, os selectors pt-BR, os
termos técnicos autorizados e o contrato de erros do backend.
