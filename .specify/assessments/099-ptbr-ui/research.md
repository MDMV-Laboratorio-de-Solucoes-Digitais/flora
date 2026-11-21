# Research: 099 — interface 100% pt-BR

**Data:** 2026-11-21 · **Evidência local determinística** · branch `feature/099-ptbr-ui@7183a33fc`

## Primária (local)

1. `apps/frontend/src/app.html:2` — `<html lang="pt-BR">`. O idioma do documento
   **já está correto** no baseline; não há `lang="en"` a corrigir. A meta
   `text-scale` e os links de fontes não contêm texto de interface traduzível.
2. Levantamento I18nScout (handoff): ~140 literais em inglês em ~15 arquivos,
   nenhum módulo/archivo de tradução. `grep` por `i18n`/`locale` em
   `apps/frontend/src` não encontra dicionário nem helper — apenas
   `localeCompare` e os rótulos pt-BR do ciclo 096.
3. Shell e navegação: `apps/frontend/src/routes/(app)/+layout.svelte:129,137,145`
   expõe `Channels`, `DMs`, `Tasks`; `:155,158` repete `Channels` e
   `Open Channels to browse workspace channels.`; `:189,195,206` traz
   `My Account`, `Settings`, `Log out`; `:255,270` mostra `AI Insight Panel`,
   `How can I help you today?` e `Ask me to summarize...`; `:244` usa
   `Close AI Panel` / `Open AI Panel` como `title`.
4. Páginas autenticadas: `(app)/tasks/+page.svelte:136,140,170,184,191,196,198`
   (`Tasks`, descrição em inglês, `Refresh`, `No workspaces available`,
   `Task title`, `Create task`, `Loading...`); `(app)/channels/+page.svelte:114,117,129,151,170,172,196`
   (`Channels`, `New channel`, `No channels yet.`, `No channel selected`);
   `(app)/select-org/+page.svelte` e `(app)/settings/+page.svelte` também têm
   títulos e estados em inglês; `routes/demo/**` contém texto de demonstração.
5. Autenticação: `(auth)/+page.svelte` já está em pt-BR desde o ciclo 096 —
   `Bem-vindo ao Flora`, `Entrar`, `Entrar com SSO`, `Primeira configuração`,
   `Criar workspace` e mensagens de erro. O ponto aberto é
   `(auth)/callback/+page.svelte:69` (`Authenticating...`) e os `logger.error`
   em inglês (`:15,31,65`), que são mensagens de diagnóstico, não texto visível.
6. Componentes compartilhados: `chat/ChatLayout.svelte:104,195,208,225`
   (`Select a channel to view messages.`, `No messages yet.`,
   `Write a message…` / `Select a channel first`, `Send`),
   `chat/thread/ThreadReply.svelte:47` (`Reply to thread...`),
   `search/SearchBox.svelte:55-56,70-79` (`aria-label`, placeholder,
   `Searching…`, `No results for...`, `result/results`, `Search results`),
   `files/FileUpload.svelte:20,24,50`, `files/FilePreview.svelte`,
   `organization/OrganizationManager.svelte:21,33,34,42,46`,
   `workspace/WorkspaceCreate.svelte:31,32,36,41`, e fallbacks de erro
   (`Failed to load messages`, `Failed to create channel`, etc.).
7. Dados derivados: `apps/frontend/src/lib/components/tasks/kanban.svelte.ts:13-16`
   (`To Do`, `In Progress`, `Done` como títulos de coluna) e
   `apps/frontend/src/lib/utils/search.ts:1-6` (`Message`, `Task`, `File` em
   `RESULT_LABELS`). São strings visíveis apesar de viverem em módulos `.ts`.
8. Erros do backend: `ApiClient` devolve corpos com `error` e `field`; a tela de
   login (096) já fazia `switch` por `status` (401/409/422/429) e pelo código
   `oidc_error`, com textos em pt-BR espalhados no componente. Não havia um
   único ponto de mapeamento código/status → mensagem.
9. Testes: `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts:48+`
   já usa seletores e regexes pt-BR (`voce@exemplo.com`, `Senha`,
   `Mínimo de 15 caracteres`, `Entrar`, `Fazer primeira configuração`,
   `Criar workspace`, `/e-mail ou senha incorretos/i`, `/muitas tentativas/i`).
   Qualquer migração para o dicionário precisa preservar esses textos para não
   quebrar os selectors.
10. `apps/frontend/package.json` — `npm run check` (svelte-check), `npm run lint`
    (prettier + eslint) e `npm run test` (vitest + playwright) são os comandos
    locais que lastreiam SC-003; o gate do alvo está registrado como PASS no
    handoff e não foi reexecutado neste artefato.

## Diagnóstico

A causa é ausência de uma camada de tradução, não um atributo `lang` errado:
as strings vivem inline nos componentes, em módulos de domínio
(`kanban.svelte.ts`, `utils/search.ts`) e nos `switch` de erro. Isso produz
~140 literais em inglês em ~15 arquivos e impede uma auditoria única. A correção
é centralizar o texto em um dicionário pt-BR tipado e substituir cada literal
por uma chave, preservando os rótulos já traduzidos no ciclo 096 e os selectors
dos testes.

## Termos técnicos permitidos (exceções explícitas)

OIDC, OAuth, API, JWT, REST, AI, SSO e slug — além de nomes próprios de produto
(Flora), identificadores de código, rotas HTTP, valores canônicos de status da
API (`Todo`, `InProgress`, `Done`, `Archived`) e nomes de ferramentas
(Playwright, Vitest). `workspace` é mantido como termo técnico consagrado no
produto, conforme o dicionário existente.

## [NV]

- IdP externo, download de fontes, deploy remoto e observações em produção:
  **[NV]**. Nenhuma chamada externa foi feita.
- Execução de gates, build, testes e formatters: registrada como PASS no
  handoff consolidado; este artefato apenas inspecionou o código e não
  reexecutou verificações.
- Textos inseridos dinamicamente por serviços externos ou pelo backend em
  produção (fora dos códigos mapeados em FR-007): **[NV]**.
