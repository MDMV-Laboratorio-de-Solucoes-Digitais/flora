# Feature Specification: 099 — interface 100% pt-BR

**Feature Branch**: `feature/099-ptbr-ui`

**Created**: 2026-11-21

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/099-ptbr-ui/decision.md` e handoff I18nScout (~140 literais em inglês em ~15 arquivos, sem i18n)

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Navegar pelo Flora em português (Priority: P1)

Como usuário brasileiro, preciso ver menus, páginas e mensagens em pt-BR, para
entender a interface sem traduzir termos comuns na cabeça.

**Why this priority**: é o requisito direto do usuário. O shell do app mostra
`Channels`, `DMs`, `Tasks`, `My Account`, `Settings` e `Log out`
(`routes/(app)/+layout.svelte:129-206`), e as páginas autenticadas exibem
`Create task`, `No channels yet.` e `Loading...`. São os textos que o usuário vê
primeiro e com mais frequência.

**Independent Test**: abrir o shell autenticado e conferir que os rótulos da
barra lateral, da conta e do painel de IA vêm do dicionário e estão em pt-BR;
repetir em `tasks`, `channels`, `dms`, `select-org` e `settings`.

**Acceptance Scenarios**:

1. **Given** uma sessão autenticada, **When** o usuário abre o layout do app,
   **Then** vê `Canais`, `Mensagens diretas`, `Tarefas`, `Minha conta`,
   `Configurações` e `Sair`, sem literais em inglês.
2. **Given** a página de tarefas, **When** a lista carrega, **Then** os títulos,
   o formulário, os estados de carregamento/vazio e a mensagem de criação estão
   em pt-BR via `t`.
3. **Given** o painel lateral de IA, **When** o usuário o abre, **Then** o título,
   a dica e o placeholder aparecem em pt-BR (`Painel de insights de IA`,
   `Como posso ajudar você hoje?`, `Pergunte à IA...`), preservando `AI` apenas
   onde é termo técnico.

---

### User Story 2 — Entender erros e estados sem sair do português (Priority: P1)

Como usuário, preciso que erros da API, estados de envio e mensagens de sucesso
sejam exibidos em pt-BR, para saber o que aconteceu sem ler códigos técnicos.

**Why this priority**: os fallbacks de erro estavam espalhados nos componentes
(`Failed to load messages`, `Upload failed.`, `Search failed.`) e a tela de login
fazia um `switch` próprio por status/código. Sem mapeamento único, mensagens
novas podem voltar a aparecer em inglês.

**Independent Test**: disparar `invalid_credentials`, `rate_limited`,
`oidc_error`, `conflict`, `validation` e `field === 'admin_password'`, além dos
statuses 401/409/422/429, e conferir que `backendErrorMessage()` devolve o texto
pt-BR correto, com o fallback da operação quando não há código nem status
mapeado.

**Acceptance Scenarios**:

1. **Given** um corpo `{ error: 'invalid_credentials' }` com status 401,
   **When** o frontend trata a resposta, **Then** exibe `E-mail ou senha
   incorretos.`.
2. **Given** `{ error: 'validation', field: 'admin_password' }`, **When** o erro
   é mapeado, **Then** exibe a mensagem específica de complexidade (15 a 1024
   caracteres).
3. **Given** um erro sem código e sem status mapeado, **When** o erro é exibido,
   **Then** usa o fallback pt-BR da operação — nunca o texto bruto do backend.

---

### User Story 3 — Manter um único ponto de tradução e os testes estáveis (Priority: P2)

Como mantenedor, preciso de um dicionário tipado e de testes com selectors
estáveis, para auditar a interface e traduzir novas strings sem caçar literais
um a um.

**Why this priority**: sem `TranslationKey`, uma chave errada passaria
silenciosamente; sem preservar os rótulos já usados nos testes do ciclo 096, a
tradução quebraria `login-error.svelte.spec.ts`.

**Independent Test**: conferir que `t` é `as const`, que `TranslationKey` é
`keyof typeof t`, que `format()` preenche `{id}`/`{query}`/`{name}`/`{title}` e
que o spec de login continua passando com os mesmos seletores.

**Acceptance Scenarios**:

1. **Given** o módulo `lib/i18n/index.ts`, **When** um componente importa `t`,
   **Then** o tipo da chave é validado pelo compilador.
2. **Given** o spec `(auth)/login-error.svelte.spec.ts`, **When** os testes
   rodam, **Then** os seletores `voce@exemplo.com`, `Senha`, `Entrar`,
   `Fazer primeira configuração` e `Criar workspace` continuam válidos.
3. **Given** uma varredura da UI ativa, **When** o texto visível é auditado,
   **Then** não há literais em inglês fora da lista de termos técnicos.

---

### Edge Cases

- `apps/frontend/src/app.html:2` já está `lang="pt-BR"` — manter; não regressir
  para `en` nem duplicar o atributo.
- Textos gerados em módulos `.ts` (`kanban.svelte.ts`, `utils/search.ts`) também
  são interface e precisam sair do dicionário.
- `title`, `aria-label`, placeholders e mensagens de estado (`Searching…`,
  `Loading...`, `Authenticating...`) contam como texto visível.
- Termos técnicos autorizados: OIDC, OAuth, API, JWT, REST, AI, SSO e slug —
  além de nomes próprios (Flora), rotas HTTP, identificadores de código e valores
  canônicos de status da API (`Todo`, `InProgress`, `Done`, `Archived`).
  `workspace` permanece como termo técnico do produto. Quando `AI` é texto
  corrente, usa-se a forma pt-BR `IA`.
- `logger.error`, comentários, nomes de variáveis e chaves de erro da API não
  são interface: permanecem como estão.
- Os textos do ciclo 096 na tela de login devem ser mantidos literalmente para
  não quebrar os selectors existentes.
- IdP externo, deploy remoto e textos dinâmicos de serviços externos fora dos
  códigos mapeados permanecem **[NV]**.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O frontend DEVE ter um dicionário pt-BR centralizado e tipado em
  `apps/frontend/src/lib/i18n/pt-BR.ts` (`export const t = {...} as const`,
  chaves achatadas, `export type TranslationKey`), reexportado por
  `apps/frontend/src/lib/i18n/index.ts` com `format(chave, valores)` para
  marcadores `{nome}`; nenhuma string visível da UI ativa pode ficar literal no
  componente.
- **FR-002**: `apps/frontend/src/app.html` DEVE permanecer com `lang="pt-BR"`;
  o atributo já está correto no baseline e NÃO DEVE ser regressido para `en`.
- **FR-003**: as telas `routes/(auth)/+page.svelte` e
  `(auth)/callback/+page.svelte` DEVEM usar o dicionário para títulos, rótulos,
  placeholders, estados de envio e mensagens de erro, preservando os textos
  pt-BR do ciclo 096 e traduzindo `Authenticating...` para `Autenticando...`.
- **FR-004**: o layout e a navegação (`routes/(app)/+layout.svelte` e
  `routes/+layout.svelte`) DEVEM exibir menu, dica de canais, conta do usuário,
  ações de configuração/saída e painel de IA em pt-BR, incluindo `title` e
  `aria-label`.
- **FR-005**: as páginas `routes/(app)/tasks`, `channels`, `dms`, `select-org`,
  `settings` e `routes/demo/**` DEVEM ter títulos, descrições, formulários,
  estados de carregamento/vazio e mensagens de sucesso em pt-BR via FR-001.
- **FR-006**: os componentes em `lib/components/**` (chat, thread, arquivos,
  organização, busca, tarefas, workspace) DEVEM usar o dicionário, assim como
  `lib/components/tasks/kanban.svelte.ts` (títulos de coluna) e
  `lib/utils/search.ts` (labels de tipo de resultado).
- **FR-007**: `lib/i18n/index.ts` DEVE expor
  `backendErrorMessage(body, status, fallback)` mapeando os códigos
  `invalid_credentials`, `rate_limited`, `oidc_error`, `conflict`, `validation`
  (com caso especial `field === 'admin_password'`) e os statuses HTTP
  401/409/422/429 para chaves do dicionário, com fallback por operação; o
  contrato da API NÃO é alterado.
- **FR-008**: os testes e selectors DEVEM continuar em pt-BR.
  `(auth)/login-error.svelte.spec.ts` preserva `voce@exemplo.com`, `Senha`,
  `Mínimo de 15 caracteres`, os botões `Entrar`/`Fazer primeira
  configuração`/`Criar workspace` e as regexes de erro, com as mensagens agora
  lidas do dicionário.

### Key Entities

- **`t` / `TranslationKey`**: dicionário achatado `as const` e união de chaves;
  único lugar onde o texto visível da UI mora.
- **`format`**: preenche marcadores `{id}`, `{query}`, `{name}`, `{title}` sem
  alterar marcadores ausentes.
- **`backendErrorMessage`**: mapeador código/status → mensagem pt-BR, com
  prioridade de código sobre status e fallback da operação.
- **Páginas e componentes**: consumidores do dicionário; nenhum literal de
  interface permanece inline.
- **Selectors pt-BR**: rótulos estáveis usados por `login-error.svelte.spec.ts`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: varredura da UI ativa (`apps/frontend/src/routes/**`,
  `apps/frontend/src/lib/components/**`, `kanban.svelte.ts` e
  `utils/search.ts`) encontra **zero literais em inglês** em texto visível —
  títulos, rótulos, placeholders, `aria-label`, `title`, estados e mensagens.
- **SC-002**: os únicos termos em inglês da interface são os técnicos
  autorizados: OIDC, OAuth, API, JWT, REST, AI, SSO e slug (mais nomes próprios,
  rotas, identificadores e valores canônicos de status da API).
- **SC-003**: gates do alvo, lint (`prettier`/`eslint`), type-check
  (`svelte-check`) e testes (`vitest`/`playwright`) estão verdes (PASS registrado
  no handoff); o diff fica restrito a `apps/frontend` e aos artefatos do ciclo,
  sem tocar em `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Assumptions

- O diagnóstico é o do handoff I18nScout e de `research.md`: ~140 literais em
  ~15 arquivos, ausência de camada de tradução e `lang="pt-BR"` já correto.
- O idioma do produto é pt-BR; não há seletor de idioma nem conteúdo vindos de
  backend em outro idioma além dos códigos mapeados em FR-007 **[NV]**.
- Os rótulos já traduzidos no ciclo 096 são contrato de testes e devem ser
  preservados textualmente.
- Termos técnicos listados em SC-002 e nos Edge Cases são as únicas exceções;
  `workspace` é mantido como termo técnico do produto.
- Verificações e estado de gate são os registrados no handoff consolidado; este
  artefato não reexecutou gates, build, testes ou formatters.
