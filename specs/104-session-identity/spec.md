# Feature Specification: 104 — Identidade da sessão na navegação

**Feature Branch**: `feature/104-session-identity`

**Created**: 2026-11-21

**Status**: Specified — `go` para implementação (decisão em `.specify/assessments/104-session-identity/decision.md`)

**Input**: inspeção determinística do baseline `23ff4bc809` — literais em `apps/frontend/src/routes/(app)/+layout.svelte:182-185` e `apps/frontend/src/lib/i18n/pt-BR.ts:53-54`, claim `email` em `flora-core/src/utils/jwt.rs:25,51` — e decisão `go` do ciclo 104

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Ver o e-mail real da sessão no menu (Priority: P1)

Como usuário autenticado, preciso que o menu da navegação mostre o e-mail da minha sessão, em vez de `usuario@flora.app`, para confirmar rapidamente qual conta está ativa.

**Why this priority**: o dropdown interpola `t['nav.userName']` e `t['nav.userEmail']` (`+layout.svelte:182-185`), que são constantes em `pt-BR.ts:53-54`; a claim `email` já é emitida pelo backend (`jwt.rs:25,51`) e `authState.token` já está no cliente (`AuthState.svelte.ts:36`). É uma correção de fidelidade da interface, sem nenhuma mudança de backend.

**Independent Test**: montar um token de sessão com payload `{"email":"qa-admin@flora.test"}` e verificar que `emailFromToken` devolve `qa-admin@flora.test` e que o dropdown mostra `qa-admin` e `qa-admin@flora.test`; trocar o token e confirmar que os textos mudam sem recarregar a página.

**Acceptance Scenarios**:

1. **Given** uma sessão com JWT cuja claim `email` é `qa-admin@flora.test`, **When** o dropdown do layout é renderizado, **Then** o segundo `<span>` mostra `qa-admin@flora.test` e o primeiro mostra `qa-admin` (parte local do e-mail).
2. **Given** `authState.token` é trocado por outro token com e-mail diferente, **When** a reatividade do Svelte roda, **Then** o menu passa a mostrar o novo e-mail e o novo nome sem recarregar a página.
3. **Given** um token com payload base64url sem padding e caracteres UTF-8, **When** o helper decodifica, **Then** o e-mail é devolvido exatamente, sem perda de acentuação.

---

### User Story 2 — Falhar fechado e manter o fallback do dicionário (Priority: P1)

Como mantenedor, preciso que qualquer token ausente ou inválido resulte no texto padrão do dicionário, sem exceção, log ou dado parcial na interface.

**Why this priority**: o cliente não valida assinatura (`jwt.rs:96-104` mostra que isso é feito no servidor); um decoder ingênuo poderia lançar, exibir `undefined` ou aceitar payload forjado como se fosse identidade confiável. O fallback atual (`Nome do usuário` / `usuario@flora.app`) é o comportamento seguro conhecido.

**Independent Test**: chamar o helper com `null`, `undefined`, `''`, token de 2 ou 4 segmentos, base64 inválido, base64 válido com JSON inválido e payloads sem `email` utilizável; confirmar `null` em todos os casos e que o layout continua exibindo as chaves `nav.userName` e `nav.userEmail`.

**Acceptance Scenarios**:

1. **Given** `authState.token` é `null` ou `''`, **When** o dropdown é renderizado, **Then** mostra `Nome do usuário` e `usuario@flora.app` (`pt-BR.ts:53-54`), sem erro.
2. **Given** um token com segmentos insuficientes, base64 corrompido ou JSON inválido, **When** `emailFromToken` é chamado, **Then** retorna `null` sem lançar e sem registrar log.
3. **Given** um payload válido, mas sem `email`, com `email` vazio/só espaços/não-string, ou com o próprio payload sendo array ou `null`, **When** o helper é chamado, **Then** retorna `null`.

---

### User Story 3 — Provar o comportamento com Vitest (Priority: P2)

Como mantenedor, preciso de testes unitários determinísticos para o helper puro, sem navegador, rede ou fixture de sessão.

**Why this priority**: `apps/frontend/vite.config.ts:49` já inclui `tests/**/*.{test,spec}.{js,ts}` no projeto `server` (node) e `:28` exige asserções; `tests/unit/tasks-query.test.ts:1-21` mostra o padrão `describe/it/expect` com import relativo. A lógica de decodificação tem várias entradas inválidas e merece tabela de casos.

**Independent Test**: executar o projeto `server` do Vitest e confirmar que todos os casos de `tests/unit/jwt.test.ts` passam, com pelo menos uma asserção por teste.

**Acceptance Scenarios**:

1. **Given** `apps/frontend/tests/unit/jwt.test.ts`, **When** `npm run test:unit -- --run --project server` é executado, **Then** todos os testes passam e nenhum teste fica sem asserção.
2. **Given** a suíte, **When** seu conteúdo é revisado, **Then** cobre explicitamente e-mail válido, base64url sem padding/UTF-8 e todos os caminhos fail-closed listados na User Story 2.

---

### Edge Cases

- Token com menos ou mais de 3 segmentos, ou com segmento vazio: `null`.
- Payload em base64 padrão (`+`/`/`) em vez de base64url (`-`/`_`): normalização converte antes de `atob`.
- Payload sem padding: padding é completado até múltiplo de 4 antes da decodificação.
- Payload que decodifica bytes UTF-8 válidos mas JSON inválido: `null` via `catch`.
- JSON válido, mas array, `null` ou objeto sem `email`: `null`.
- `email` ausente, `''`, `'   '` ou número: `null`; apenas string não vazia após `trim()` é aceita.
- O helper retorna o e-mail original (com `trim()` apenas para validação), não um valor normalizado.
- `setOrganization` (`AuthState.svelte.ts:87-99`) troca o token sem tocar `user`: o valor derivado deve acompanhar o token, não `user`.
- E-mail igual ao literal de fallback (`usuario@flora.app`) não deve fazer o nome virar `usuario`: o nome derivado só é usado quando o e-mail é realmente da claim.
- Nenhuma decisão de autenticação, autorização ou rota passa a depender do e-mail decodificado; `enforceAuth` (`+layout.svelte:34-44`) e o `$effect` de websocket (`:63-84`) permanecem intocados.
- `Usuário <id>` no chat (`ChatLayout.svelte:124,130`, `pt-BR.ts:80`) é dívida fora de escopo: exige lookup de usuários; **[NV]** neste ciclo.
- Execução de testes/gates e validação em navegador real: **[NV]** neste artefato, registrada no handoff.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001 — helper puro e fail-closed**: `apps/frontend/src/lib/utils/jwt.ts` DEVE exportar `emailFromToken(token: string | null | undefined): string | null`. A implementação DEVE exigir exatamente 3 segmentos não vazios, normalizar o payload de base64url (`-`→`+`, `_`→`/`, padding), decodificar com `atob` + `Uint8Array` + `TextDecoder` (UTF-8) e `JSON.parse` dentro de `try/catch`, aceitando somente objeto não-array com `email` string não vazia após `trim()`. Qualquer anomalia DEVE retornar `null`, sem lançar, sem log, sem efeito colateral e sem usar o resultado para autorização (`flora-core/src/utils/jwt.rs:96-104`).

- **FR-002 — e-mail real via `$derived`**: `apps/frontend/src/routes/(app)/+layout.svelte` DEVE derivar `userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail'])` e exibir esse valor no segundo `<span>` do dropdown (`:183-185`), substituindo a interpolação fixa. O valor DEVE ser reativo a `authState.token`, inclusive após `setOrganization` (`AuthState.svelte.ts:87-99`), e NÃO DEVE ser usado em `enforceAuth` (`:34-44`), no `$effect` de websocket (`:63-84`), em rotas ou em qualquer verificação de acesso.

- **FR-003 — nome e fallback**: o primeiro `<span>` (`:182`) DEVE mostrar a parte local do e-mail quando houver e-mail real (por exemplo `qa-admin@flora.test` → `qa-admin`) e `t['nav.userName']` quando o helper retornar `null`. `apps/frontend/src/lib/i18n/pt-BR.ts:53-54` NÃO DEVE ser alterado; nenhuma chave é adicionada, renomeada ou removida, e nenhuma outra entrada (em especial `chat.author` em `:80`) muda.

- **FR-004 — cobertura Vitest**: `apps/frontend/tests/unit/jwt.test.ts` DEVE usar `describe/it/expect` no padrão de `apps/frontend/tests/unit/tasks-query.test.ts:1-21`, ser descoberto pelo projeto `server` (`vite.config.ts:49`, ambiente node) e conter ao menos uma asserção por teste (`:28`). Casos obrigatórios: e-mail válido; base64url sem padding/UTF-8; `null`, `undefined`, `''`, 2 e 4 segmentos, segmento vazio, base64 inválido, JSON inválido; payload sem `email`, com `email` vazio/só espaços/não-string, array e `null`.

- **FR-005 — dívida e fronteira**: a dívida `Usuário <id>` (`ChatLayout.svelte:124,130` e `pt-BR.ts:80`) DEVE ser registrada como fora de escopo e **[NV]**; resolver exige lookup de usuários. O diff NÃO DEVE tocar backend, gates, required checks, `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição; a factory `mdmv-dark-factory` é somente leitura e nenhum commit é feito.

### Key Entities

- **`emailFromToken`**: helper puro em `apps/frontend/src/lib/utils/jwt.ts`; lê apenas a claim `email` do payload JWT para exibição, com fail-closed.
- **`FloraClaims.email`**: `flora-core/src/utils/jwt.rs:25`, preenchida em `:51` por `FloraClaims::new`; fonte da identidade, sem alteração de backend.
- **`authState.token`**: `apps/frontend/src/lib/state/AuthState.svelte.ts:36`; estado reativo que alimenta o `$derived` do layout.
- **`userEmail`**: valor `$derived` em `+layout.svelte`, igual ao e-mail da claim ou ao literal de fallback do dicionário.
- **`t['nav.userName']` / `t['nav.userEmail']`**: `pt-BR.ts:53-54`; literais mantidos exclusivamente como fallback.
- **`decode_token`**: `flora-core/src/utils/jwt.rs:96-104`; única validação de assinatura/expiração, no servidor.
- **`chat.author`**: `pt-BR.ts:80`, usado em `ChatLayout.svelte:130`; dívida registrada, fora de escopo.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001 — identidade correta**: com `email = qa-admin@flora.test` no JWT, o dropdown mostra `qa-admin` e `qa-admin@flora.test`; ao trocar `authState.token`, os textos mudam sem recarregar a página.
- **SC-002 — fail-closed**: para token nulo/vazio, malformado, com JSON inválido ou sem `email` utilizável, o helper retorna `null` e o menu mostra `Nome do usuário` / `usuario@flora.app`, sem exceção, log ou valor parcial.
- **SC-003 — escopo e segurança**: nenhuma rota, autorização, claim, assinatura, gate, required check, schema, flag ou chave de i18n adicional é alterado; os testes Vitest cobrem caminhos positivos e negativos; diff restrito ao ciclo 104 e sem commit. Execução de testes/gates e validação em navegador são **[NV]** neste artefato e ficam no handoff.

## Assumptions

- O baseline é `23ff4bc809`; os números de linha citados se referem a esse commit e podem se deslocar após a implementação.
- A claim `email` já é emitida para sessões Flora (`jwt.rs:25,51`); nenhuma mudança de backend, emissão ou schema é necessária.
- O cliente não verifica assinatura: a leitura do payload é exclusivamente para exibição e nunca substitui `decode_token` (`jwt.rs:96-104`).
- `authState.token` é a fonte contínua da sessão, inclusive após `setOrganization` (`AuthState.svelte.ts:87-99`); `authState.user` é um `Record<string, unknown>` e não é atualizado em todos os fluxos.
- Os literais `pt-BR.ts:53-54` devem permanecer como fallback, sem novas chaves ou traduções.
- Svelte 5 roda em modo runes (`vite.config.ts:15-18`), portanto a reatividade é expressa com `$derived`.
- Resolver o chat (`Usuário <id>`) exige lookup de usuários e pertence a ciclo próprio; nenhum dado de produção foi lido neste artefato.
