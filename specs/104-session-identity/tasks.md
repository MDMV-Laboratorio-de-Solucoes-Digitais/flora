# Tasks: 104 — Identidade da sessão na navegação

**Input**: `specs/104-session-identity/spec.md`, `plan.md`,
`.specify/assessments/104-session-identity/decision.md`, inspeção do baseline `23ff4bc809`

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T008 em aberto (**fila a executar**); gates do alvo **[NV]** até o handoff. Os números de linha citados são do baseline `23ff4bc809`.
**Status de execução (2026-11-21):** T001–T008 executados e verificados; os `[ ]` abaixo são a fila original, não o estado corrente. Correções de lint registradas no `converge.md`.

## Phase 1 — Diagnóstico e artefatos de assessoria

- [ ] T001 Fixar em `.specify/assessments/104-session-identity/{intake,research}.md` a cadeia: `apps/frontend/src/routes/(app)/+layout.svelte:182-185` interpola `t['nav.userName']`/`t['nav.userEmail']` e `apps/frontend/src/lib/i18n/pt-BR.ts:53-54` define os literais; `flora-core/src/utils/jwt.rs:25,51` emite a claim `email`; `apps/frontend/src/lib/state/AuthState.svelte.ts:36,69-81,87-99` mantém `token` reativo; `apps/frontend/vite.config.ts:27-53` e `apps/frontend/tests/unit/tasks-query.test.ts:1-21` dão a convenção de testes. Registrar `Usuário <id>` de `ChatLayout.svelte:124,130`/`pt-BR.ts:80` como dívida fora de escopo e **[NV]**. Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).

## Phase 2 — Frontend: helper puro e reatividade

- [ ] T002 Criar `apps/frontend/src/lib/utils/jwt.ts` com `export function emailFromToken(token: string | null | undefined): string | null`: retornar `null` para token vazio; exigir `split('.')` com exatamente 3 segmentos não vazios; normalizar base64url (`-`→`+`, `_`→`/`) e completar padding; decodificar com `atob` → `Uint8Array` → `TextDecoder` → `JSON.parse` dentro de `try/catch`; aceitar somente objeto não-array com `email` do tipo `string` e não vazio após `trim()`; qualquer anomalia → `null`, sem throw, log, efeito colateral ou uso para autorização — a assinatura continua sendo validada por `decode_token` (`jwt.rs:96-104`) (FR-001, SC-002).

- [ ] T003 Em `apps/frontend/src/routes/(app)/+layout.svelte`, importar `emailFromToken` de `$lib/utils/jwt` e declarar `const userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail']);`. Trocar as duas interpolações de `:182-185`: primeiro `<span>` com a parte local do e-mail quando `userEmail !== t['nav.userEmail']` (por exemplo `qa-admin@flora.test` → `qa-admin`), senão `t['nav.userName']`; segundo `<span>` com `userEmail`. Manter `enforceAuth` (`:34-44`), o `$effect` de websocket (`:63-84`), imports existentes e o restante do markup idênticos; nenhuma chave de `pt-BR.ts` muda (FR-002, FR-003, SC-001, SC-002).

## Phase 3 — Testes Vitest

- [ ] T004 Em `apps/frontend/tests/unit/jwt.test.ts`, usar `describe/it/expect` e import relativo no padrão de `apps/frontend/tests/unit/tasks-query.test.ts:1-21`. Gerar tokens `header.<payload-base64url>.signature` e cobrir o caminho positivo: `email` simples e payload com caracteres UTF-8/sem padding devolvem exatamente o e-mail; cada teste com asserção explícita, pois `vite.config.ts:28` exige `requireAssertions` (FR-004, SC-001).

- [ ] T005 Cobrir os caminhos fail-closed: `null`, `undefined`, `''`, 2 segmentos, 4 segmentos, segmento vazio, base64 inválido, base64 válido com JSON inválido, payload sem `email`, `email` vazio/só espaços/número, payload array e `null` → `toBeNull()`, sem exceção e sem log. Confirmar que os testes são descobertos pelo projeto `server` (`vite.config.ts:49`, node), sem navegador, rede ou fixture (FR-001, FR-004, SC-002).

## Phase 4 — Fecho

- [ ] T006 Registrar a dívida do chat: `ChatLayout.svelte:124` (inicial de `authorId`) e `:130` (`format('chat.author', { id: message.authorId.slice(0, 8) })`) continuarão mostrando `Usuário <id>` (`pt-BR.ts:80`), pois exigem lookup/cache de usuários. Fora de escopo e **[NV]**; nenhum arquivo do chat é alterado neste ciclo.

- [ ] T007 Conferir que o e-mail decodificado não alimenta autenticação, autorização, rotas, flags ou $effects; que `pt-BR.ts:53-54` segue idêntico e que nenhuma outra chave de i18n foi tocada. Registrar `npm run test:unit`, `check`, `lint`, build, gates e validação visual em navegador no handoff **[NV]**; nenhuma requisição ou escrita em staging/produção e nenhum segredo em artefatos.

- [ ] T008 Fechar a rastreabilidade T001..T008 em decision/spec/plan/tasks, verificar a fronteira proibida (`analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog, constituição) e confirmar que nenhum required check/gate, schema, flag, rota, claim, assinatura ou chave de i18n nova foi introduzido e que não houve commit (FR-005, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008. T002 depende de T001 (contrato de fail-closed e fonte da claim fixados); T003 de T002 (o `$derived` consome o helper e precisa do fallback definido); T004 de T002 (testar a função existente); T005 de T004 e T002 (a tabela negativa depende do contrato positivo e do helper); T006 registra a dívida sem bloquear o código; T007 depende de T003–T005 (valida a exibição, os testes e a fronteira); T008 verifica T001–T007 em conjunto e consolida o rastro documental. Em todos os passos, `enforceAuth`, o `$effect` de websocket, `decode_token`, as chaves do dicionário e o comportamento do chat permanecem inalterados. Execução de testes/gates e validação em navegador permanecem **[NV]** até o handoff.
