# Define: 099 — interface 100% pt-BR

**Data:** 2026-11-21 · **Branch**: `feature/099-ptbr-ui@7183a33fc`

## Problema

A interface do Flora mistura português e inglês. O atributo `lang="pt-BR"` já
existe, mas ~140 literais em inglês seguem espalhados em ~15 arquivos: menu
(`Channels`, `DMs`, `Tasks`), páginas (`Create task`, `No channel selected`),
componentes (`Write a message…`, `File too large.`, `Search results`), títulos de
coluna do Kanban (`To Do`, `In Progress`, `Done`), labels de busca
(`Message`, `Task`, `File`) e a tela de callback (`Authenticating...`). Sem um
dicionário central, não há como auditar nem garantir que a interface inteira
esteja em pt-BR. A correção precisa preservar os textos já traduzidos no ciclo
096 e os selectors dos testes, e manter apenas os termos técnicos permitidos.

## Requisitos funcionais

- **FR-001**: o frontend DEVE ter um dicionário pt-BR centralizado e tipado em
  `apps/frontend/src/lib/i18n/pt-BR.ts` (`export const t = {...} as const` com
  chaves achatadas e `export type TranslationKey`), reexportado por
  `apps/frontend/src/lib/i18n/index.ts` junto de `format(chave, valores)` para
  marcadores `{nome}`. Nenhuma string visível da UI ativa pode ficar literal no
  componente.
- **FR-002**: `apps/frontend/src/app.html` DEVE permanecer com
  `lang="pt-BR"`; o atributo já está correto no baseline e NÃO DEVE ser
  regressido para `en` nem duplicado.
- **FR-003**: as telas de autenticação
  `apps/frontend/src/routes/(auth)/+page.svelte` e
  `(auth)/callback/+page.svelte` DEVEM usar o dicionário para títulos, rótulos,
  placeholders, estados de envio e mensagens de erro, preservando os textos
  pt-BR já existentes e o status "Authenticating..." traduzido.
- **FR-004**: o layout e a navegação
  (`apps/frontend/src/routes/(app)/+layout.svelte` e
  `apps/frontend/src/routes/+layout.svelte`) DEVEM exibir menu, dica de canais,
  conta do usuário, ações de configuração/saída e o painel de IA em pt-BR,
  incluindo atributos `title` e `aria-label`.
- **FR-005**: as páginas sob `apps/frontend/src/routes/(app)/**`
  (`tasks`, `channels`, `dms`, `select-org`, `settings`) e as páginas de
  demonstração `routes/demo/**` DEVEM ter títulos, descrições, formulários,
  estados de carregamento/vazio e mensagens de sucesso em pt-BR via FR-001.
- **FR-006**: os componentes compartilhados em
  `apps/frontend/src/lib/components/**` (chat, thread, arquivos, organização,
  busca, tarefas, workspace) DEVEM usar o dicionário, assim como os módulos
  derivados `lib/components/tasks/kanban.svelte.ts` (títulos de coluna) e
  `lib/utils/search.ts` (labels de tipo de resultado).
- **FR-007**: `apps/frontend/src/lib/i18n/index.ts` DEVE expor
  `backendErrorMessage(body, status, fallback)` mapeando os códigos
  `invalid_credentials`, `rate_limited`, `oidc_error`, `conflict`, `validation`
  (com caso especial `field === 'admin_password'`) e os statuses HTTP
  401/409/422/429 para chaves do dicionário, com fallback específico por
  operação. A API DEVE continuar usando os mesmos códigos/status — nenhuma
  mudança de contrato backend.
- **FR-008**: os testes e selectors DEVEM continuar em pt-BR.
  `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts` preserva os
  seletores `voce@exemplo.com`, `Senha`, `Mínimo de 15 caracteres`, os botões
  `Entrar`/`Fazer primeira configuração`/`Criar workspace` e as regexes de erro;
  as mensagens passam a ser lidas do dicionário com os mesmos textos.

## Critérios de sucesso

- **SC-001**: varredura da UI ativa (`apps/frontend/src/routes/**` e
  `apps/frontend/src/lib/components/**`, mais `kanban.svelte.ts` e
  `utils/search.ts`) encontra **zero literais em inglês** em texto visível —
  títulos, rótulos, placeholders, `aria-label`, `title`, estados e mensagens.
- **SC-002**: os únicos termos em inglês que permanecem são os técnicos
  autorizados: OIDC, OAuth, API, JWT, REST, AI, SSO e slug (além de nomes
  próprios, rotas, identificadores e valores canônicos de status da API).
- **SC-003**: gates do alvo, lint (`prettier`/`eslint`), type-check
  (`svelte-check`) e testes (`vitest`/`playwright`) estão verdes (PASS registrado
  no handoff); o diff fica restrito a `apps/frontend` e aos artefatos do ciclo,
  sem tocar em `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Fora de escopo

Mudanças no backend, no contrato da API, nos códigos de erro, em rotas ou no
fluxo OIDC; alteração de `lang` para outra língua; tradução de mensagens de log
(`logger.error`), comentários de código, nomes de variáveis, chaves de erro da
API ou valores canônicos de status; mudanças em gates, workflows, ADRs,
constituição ou documentação de governança. IdP externo e deploy remoto permanecem
**[NV]**.
