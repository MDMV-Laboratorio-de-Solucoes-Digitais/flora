# Shape: 104 — Identidade da sessão na navegação

**Data:** 2026-11-21 · **Branch**: `feature/104-session-identity@23ff4bc809caccff2cbc9018521d8762f308ddeb`

## Forma escolhida

Um utilitário puro, um valor `$derived` e testes de tabela — nenhuma nova superfície de dados. `emailFromToken` lê apenas o payload base64url do JWT para exibição e devolve `string | null`; o layout deriva o e-mail de `authState.token` e usa as chaves existentes do dicionário somente quando a leitura falha. O backend continua sendo a única autoridade sobre a claim e sobre a assinatura (`flora-core/src/utils/jwt.rs:25,51,96-104`).

1. **Causa e rastro (T001 / intake, research)**: fixar `+layout.svelte:182-185` e `pt-BR.ts:53-54` como origem dos literais, e `jwt.rs:25,51` + `AuthState.svelte.ts:36` como fonte já disponível. Registrar o chat como dívida/[NV]. Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).
2. **Helper (T002 / FR-001)**: criar `apps/frontend/src/lib/utils/jwt.ts` com `emailFromToken(token: string | null | undefined): string | null`: 3 segmentos não vazios, normalização base64url, `atob`/`TextDecoder`/`JSON.parse` protegidos, objeto não-array, `email` string não vazia; qualquer desvio → `null`. Sem dependências, sem efeitos colaterais, sem verificação de assinatura no cliente.
3. **Layout (T003 / FR-002, FR-003)**: importar o helper em `+layout.svelte`, derivar `userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail'])` e trocar as duas interpolações (`:182-185`): nome = parte local do e-mail quando real, senão `t['nav.userName']`; e-mail = `userEmail`. `enforceAuth` (`:34-44`) e o `$effect` de websocket (`:63-84`) permanecem intactos.
4. **Testes positivos (T004 / FR-004, SC-001)**: `tests/unit/jwt.test.ts` com token de payload válido (base64url sem padding, UTF-8) → e-mail exato; cobrir o formato de 3 segmentos gerado com `base64url`.
5. **Testes negativos (T005 / FR-002, SC-002)**: `null`/`undefined`/`''`, 2 ou 4 segmentos, segmento vazio, base64 inválido, base64 válido com JSON inválido, payload sem `email`, `email` vazio/espaços/número, array e `null` → `null`. Todas as asserções explícitas, conforme `vite.config.ts:28`.
6. **Dívida e fecho documental (T006–T008)**: registrar `Usuário <id>` do chat (`ChatLayout.svelte:124,130`, `pt-BR.ts:80`) como fora de escopo/[NV], confrontar FR/SC com o baseline, conferir que nenhuma chave de i18n além das duas de fallback mudou e que nenhum artefato proibido ou commit apareceu.

- **E-mail como autorização (escopo excessivo)**: usar a claim lida no cliente para liberar ação ou rota permitiria fraude por payload forjado; mitigado limitando o helper à exibição e mantendo `decode_token` (`jwt.rs:96-104`) como única verificação, no servidor.
- **Fallback vazando como dado real**: se o literal `usuario@flora.app` fosse tratado como e-mail válido, o nome derivado viraria `usuario`; mitigado comparando o valor derivado com o fallback antes de derivar o nome e exibindo `t['nav.userName']` nesse caso.
- **Decodificação ingênua**: `atob` direto falha em base64url sem padding e em UTF-8; mitigado por normalização `-`/`_`, padding, `Uint8Array` e `TextDecoder`, tudo dentro de `try/catch`.
- **Token trocado sem reatividade**: capturar o e-mail uma única vez deixaria o menu desatualizado após `setOrganization` (`AuthState.svelte.ts:87-99`); mitigado por `$derived` sobre `authState.token`.
- **Extrapolação para o chat**: aproveitar o helper para resolver autores de mensagem exigiria lookup de usuários e mudaria `ChatLayout.svelte`/`chat.author`; mitigado mantendo a dívida fora do ciclo e marcada **[NV]**.
- **Resultado de testes/gates e navegador**: **[NV]** neste artefato — não executamos Vitest, `check`, `lint`, build nem validação visual aqui.

## Entrega

Diff de implementação restrito a `apps/frontend/src/lib/utils/jwt.ts` (novo, helper puro), `apps/frontend/tests/unit/jwt.test.ts` (novo, Vitest no projeto `server`) e `apps/frontend/src/routes/(app)/+layout.svelte` (import + `$derived` + duas interpolações), além dos artefatos do ciclo 104. Arquivos lidos, não modificados: `flora-core/src/utils/jwt.rs`, `apps/frontend/src/lib/state/AuthState.svelte.ts`, `apps/frontend/src/lib/i18n/pt-BR.ts`, `apps/frontend/src/lib/types/models.ts`, `apps/frontend/package.json`, `apps/frontend/vite.config.ts`, `apps/frontend/tests/unit/tasks-query.test.ts` e `ChatLayout.svelte`. Verificação documental cobre T001–T008; gates, build, testes e validação em navegador ficam **[NV]** até o handoff.
