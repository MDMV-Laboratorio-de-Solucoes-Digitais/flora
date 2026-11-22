# Decision: 104 — Identidade da sessão na navegação

- **Slug**: 104-session-identity
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/104-session-identity` @ `23ff4bc809caccff2cbc9018521d8762f308ddeb`
- **Âncora**: `.specify/feature.json` → `specs/104-session-identity`
- **Evidência**: inspeção determinística do baseline `23ff4bc809` (código do frontend, claim do JWT e configuração do Vitest)
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

O menu do usuário em `apps/frontend/src/routes/(app)/+layout.svelte:182-185` interpola `t['nav.userName']` e `t['nav.userEmail']`, que são literais fixos em `apps/frontend/src/lib/i18n/pt-BR.ts:53-54` (`Nome do usuário`, `usuario@flora.app`). A identidade real da sessão já existe e está acessível sem requisição nova:

1. `flora-core/src/utils/jwt.rs:17-33` — `FloraClaims` contém `pub email: String` (`:25`); `FloraClaims::new` (`:38-55`) preenche a claim com `email.to_string()` (`:51`).
2. `flora-core/src/utils/jwt.rs:96-104` — `decode_token` valida assinatura e expiração no servidor; o navegador não tem a chave e não deve verificar assinatura.
3. `apps/frontend/src/lib/state/AuthState.svelte.ts:36,69-81,87-99` — `token` é `$state<string | null>` e é atualizado em `login` e `setOrganization`.
4. `apps/frontend/src/routes/(app)/+layout.svelte:5,36,58,66,70` — o layout já importa e lê `authState.token`; apenas o trecho do dropdown ignora a sessão.
5. `apps/frontend/vite.config.ts:27-53` — Vitest já tem o projeto `server` (node), que descobre `tests/**/*.{test,spec}.{js,ts}` (`:49`), com `requireAssertions` (`:28`).
6. `apps/frontend/src/lib/components/chat/ChatLayout.svelte:124,130` e `pt-BR.ts:80` — `Usuário <id>` por autor é um problema vizinho que exige lookup de usuários; fora de escopo.

Não há falha intermitente: o componente exibe constantes do dicionário e nunca consulta a claim.

## Decisão

Registrar o ciclo 104 como **Identidade da sessão na navegação**, com cinco compromissos rastreáveis (FR-001..FR-005), três critérios (SC-001..SC-003) e oito tarefas (T001..T008, fila de implementação neste artefato):

1. **Helper puro (FR-001 / T002)**: `emailFromToken(token: string | null | undefined): string | null` em `apps/frontend/src/lib/utils/jwt.ts`. Decodifica apenas o payload base64url (3 segmentos, `-`/`_`, padding, `atob`, `TextDecoder`, `JSON.parse` em `try/catch`), exige objeto não-array com `email` string não vazia; qualquer anomalia → `null`. Sem log, sem efeito colateral e sem uso para autorização.
2. **Reatividade no layout (FR-002 / T003)**: `const userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail']);` em `+layout.svelte`; segundo `<span>` mostra `userEmail`. `enforceAuth` (`:34-44`), o `$effect` de websocket (`:63-84`) e rotas permanecem intactos.
3. **Nome e fallback (FR-003 / T003)**: primeiro `<span>` mostra a parte local do e-mail quando ele é real (por exemplo `qa-admin`) e `t['nav.userName']` quando o helper retorna `null`. `pt-BR.ts:53-54` fica intacto e é o único fallback; nenhuma outra chave de i18n muda.
4. **Testes (FR-004 / T004–T005)**: `apps/frontend/tests/unit/jwt.test.ts` cobre payload válido, base64url sem padding/UTF-8 e os caminhos fail-closed (token nulo/vazio, segmentos errados, base64/JSON inválidos, `email` ausente/vazio/não-string, payload array ou `null`), com asserções explícitas no projeto `server` do Vitest.
5. **Fronteira e dívida (FR-005 / T006–T008)**: diff restrito a `jwt.ts`, `jwt.test.ts`, `+layout.svelte` e artefatos do ciclo 104; dívida do chat registrada como fora de escopo/**[NV]**; sem backend, gates, required checks, `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição; factory somente leitura; sem commit.

## Evidência de entrada

- Baseline `23ff4bc809`, branch `feature/104-session-identity` criada a partir desse HEAD; `.specify/feature.json` aponta para `specs/104-session-identity`.
- `apps/frontend/src/routes/(app)/+layout.svelte:182-185` e `apps/frontend/src/lib/i18n/pt-BR.ts:53-54` — literais na navegação.
- `flora-core/src/utils/jwt.rs:17-33,38-55,96-104` — claim `email`, emissão e validação servidora da assinatura.
- `apps/frontend/src/lib/state/AuthState.svelte.ts:36,69-81,87-99,170` — token reativo e sua atualização.
- `apps/frontend/vite.config.ts:27-53`, `apps/frontend/package.json:15,47` e `apps/frontend/tests/unit/tasks-query.test.ts:1-21` — infraestrutura e convenção de testes.
- `apps/frontend/src/lib/components/chat/ChatLayout.svelte:118-131` e `pt-BR.ts:80` — dívida do chat, fora de escopo.

## Racional

**go.** A lacuna é de apresentação e tem correção pequena, verificável e sem efeito transversal: a claim já é emitida (`jwt.rs:25,51`), o token já chega ao cliente (`AuthState.svelte.ts:36`) e o layout já o lê em outras partes (`+layout.svelte:36,58`). Um helper puro com fail-closed evita exceções e valores parciais na interface, e o `$derived` mantém o menu correto quando o token muda. Manter os literais do dicionário como fallback preserva o contrato de i18n e a experiência antes do login, sem inventar chaves nem traduções. A leitura do payload é explicitamente não-autoritativa: nenhuma rota, flag ou decisão de acesso passa a depender dela, e a validação de assinatura continua apenas no servidor (`jwt.rs:96-104`). Os testes Vitest são determinísticos e não exigem navegador, rede ou fixture. A dívida do chat fica documentada para um ciclo próprio, com lookup de usuários, sem ampliar este escopo.

## [NV]

- Execução de `npm run test:unit`, `check`, `lint`, build e gates: **[NV]** neste artefato — consolidada no handoff; apenas inspeção documental aqui.
- Validação visual do menu e da troca de conta/organização em navegador real: **[NV]** — a reatividade é garantida por `$derived`, não medida neste artefato.
- Resolução de `Usuário <id>` no chat e eventual cache/lookup de usuários: **[NV]** / dívida — fora de escopo do ciclo 104.
- Números de linha referem-se a `23ff4bc809`; nenhum segredo, token real ou dado de produção foi lido ou registrado neste ciclo.

## Fora de âmbito

Mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`; alteração de required checks, gates, workflows, ADRs, constituição ou changelog; mudança em `flora-core/src/utils/jwt.rs`, `AuthState`, `ChatLayout.svelte`, no dicionário `pt-BR.ts` (inclusive `chat.author`), em rotas, flags, schema ou migrações; uso do e-mail decodificado para autorização ou verificação de assinatura no cliente; deploy, escrita em staging/produção e commits.

## Handoff

`/speckit.specify` — especificar a identidade da sessão na navegação como requisitos verificáveis (FR-001..FR-005 / SC-001..SC-003) e fila T001..T008, com helper puro fail-closed, `$derived` no layout, literais do dicionário apenas como fallback, cobertura Vitest e dívida do chat registrada. Testes, gates e validação em navegador ficam no handoff **[NV]**.
