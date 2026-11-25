# Decision: 104 — Identidade da sessão na navegação

- **Slug**: 104-session-identity
- **Decided**: 2026-11-25
- **Verdict**: go (decisão original 2026-11-21, preservada)
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (anterior) | baseline `23ff4bc809` (frontend, claim JWT, Vitest) | spec/plan/tasks 104

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O dropdown interpola literais fixos (`+layout.svelte:182-185`, `pt-BR.ts:53-54`) para toda sessão, exibindo identidade falsa embora `FloraClaims.email` já exista (`jwt.rs:25,51`; `intake.md:7-20`). |
| Evidence strength | adequate | Inspeção determinística do baseline `23ff4bc809` em código, claim e infraestrutura Vitest; execução de testes/gates e validação em navegador **[NV]** (`research.md:1-12`). |
| Value vs. inaction | strong | Correção pequena de fidelidade da interface sem backend, rota, flag ou autorização nova; reaproveita token já presente no cliente (`decisão anterior:45`). |
| Feasibility / appetite | adequate | Helper puro fail-closed, um `$derived` no layout e testes Vitest determinísticos no projeto `server` já configurado (`shape.md`; `define.md:FR-001..FR-004`). |
| Strategic fit | strong | Preserva contrato de i18n (literais como único fallback), `enforceAuth`, websocket e autoridade do servidor sobre assinatura, sem serviço novo (`define.md:FR-003,FR-005`). |
| Risk posture | adequate | Payload lido apenas para exibição, nunca para autorização; anomalias retornam `null` sem log/exceção; dívida do chat e navegador **[NV]** (`decisão anterior:28-32,47-52`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. Registro original de 2026-11-21, branch `feature/104-session-identity` @ `23ff4bc809caccff2cbc9018521d8762f308ddeb`, âncora `.specify/feature.json` → `specs/104-session-identity`, evidência de inspeção determinística do baseline `23ff4bc809` (código do frontend, claim do JWT e configuração do Vitest).

Causa determinística: `t['nav.userName']`/`t['nav.userEmail']` (`apps/frontend/src/routes/(app)/+layout.svelte:182-185`) são literais `Nome do usuário`/`usuario@flora.app` em `apps/frontend/src/lib/i18n/pt-BR.ts:53-54`. A identidade real já está acessível: `FloraClaims` contém `pub email: String` (`flora-core/src/utils/jwt.rs:17-33,25`), preenchida por `FloraClaims::new` (`:38-55,51`); `decode_token` valida assinatura e expiração no servidor (`:96-104`); `authState.token` é `$state` atualizado em `login`/`setOrganization` (`apps/frontend/src/lib/state/AuthState.svelte.ts:36,69-81,87-99,170`); o layout já lê o token (`:5,36,58,66,70`); Vitest tem o projeto `server` em `apps/frontend/vite.config.ts:27-53` (`:49`, `requireAssertions` em `:28`). O caso vizinho `Usuário <id>` no chat (`ChatLayout.svelte:118-131,124,130`; `pt-BR.ts:80`) exige lookup de usuários e fica fora de escopo. Não há falha intermitente: o componente nunca consulta a claim.

| Compromisso rastreável | Conteúdo preservado |
|---|---|
| FR-001 / T002 — helper puro | `emailFromToken(token: string \| null \| undefined): string \| null` em `apps/frontend/src/lib/utils/jwt.ts`; exatamente 3 segmentos não vazios, `-`→`+`, `_`→`/`, padding, `atob`, `TextDecoder`, `JSON.parse` em `try/catch`; objeto não-array com `email` string não vazia; qualquer anomalia → `null`, sem log, efeito colateral ou autorização. |
| FR-002 / T003 — reatividade | `const userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail']);` no layout, segundo `<span>` mostra `userEmail`; `enforceAuth` (`:34-44`), `$effect` de websocket (`:63-84`) e rotas intactos. |
| FR-003 / T003 — nome e fallback | Primeiro `<span>` mostra a parte local do e-mail quando real (ex.: `qa-admin`) e `t['nav.userName']` quando o helper retorna `null`; `pt-BR.ts:53-54` intacto e único fallback; nenhuma outra chave de i18n muda. |
| FR-004 / T004–T005 — testes | `apps/frontend/tests/unit/jwt.test.ts` cobre payload válido, base64url sem padding/UTF-8 e fail-closed (token nulo/vazio, 2/4 segmentos, base64/JSON inválidos, `email` ausente/vazio/não-string, payload array/`null`), com asserções explícitas no projeto `server`. |
| FR-005 / T006–T008 — fronteira/dívida | Diff restrito a `jwt.ts`, `jwt.test.ts`, `+layout.svelte` e artefatos do ciclo 104; dívida do chat fora de escopo/**[NV]**; sem backend, gates, `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição; factory somente leitura; sem commit. |

Critérios preservados: **SC-001** sessão com `qa-admin@flora.test` mostra `qa-admin` e o e-mail, sem recarregar quando o token muda; **SC-002** token ausente/corrompido → `null` e fallback `Nome do usuário`/`usuario@flora.app`, sem exceção, log ou valor parcial; **SC-003** nenhuma rota, autorização, claim, assinatura, gate, flag, schema ou chave de i18n adicional alterada. Fila **T001–T008**. Racional histórico: a lacuna é de apresentação e a correção não tem efeito transversal; helper puro com fail-closed evita exceções; `$derived` mantém o menu correto após troca de token; os literais preservam i18n e a experiência pré-login; a leitura do payload é explicitamente não-autoritativa e a validação de assinatura permanece só no servidor (`jwt.rs:96-104`); testes determinísticos não exigem navegador, rede ou fixture; a dívida do chat fica para ciclo próprio.

**[NV]** preservados: execução de `npm run test:unit`, `check`, `lint`, build e gates (apenas inspeção documental; consolidação no handoff); validação visual do menu e da troca de conta/organização em navegador real (reatividade por `$derived`, não medida); resolução de `Usuário <id>` no chat e eventual cache/lookup de usuários (dívida, fora do ciclo); números de linha referem-se a `23ff4bc809`; nenhum segredo, token real ou dado de produção lido ou registrado.

Fora de âmbito preservado: mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, required checks, workflows, ADRs, constituição ou changelog; mudança em `jwt.rs`, `AuthState`, `ChatLayout.svelte`, no dicionário `pt-BR.ts` (inclusive `chat.author`), rotas, flags, schema ou migrações; uso do e-mail decodificado para autorização ou verificação de assinatura no cliente; deploy, escrita em staging/produção e commits.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante registrada nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: O menu do usuário exibe sempre `Nome do usuário`/`usuario@flora.app`, embora a claim `email` da sessão já esteja no token e o layout já o leia (`+layout.svelte:182-185`, `pt-BR.ts:53-54`, `jwt.rs:25,51`, `AuthState.svelte.ts:36`).
- **Chosen approach**: `emailFromToken` puro e fail-closed em `lib/utils/jwt.ts` + `$derived` no layout, com os literais do dicionário apenas como fallback e leitura explicitamente não-autoritativa.
- **In scope / out of scope**:
  - **In**: FR-001..FR-005 / SC-001..SC-003, fila T001..T008; `jwt.ts`, `jwt.test.ts`, `+layout.svelte`, casos positivos e negativos no projeto `server` do Vitest, dívida do chat registrada.
  - **Out/NV**: backend/`jwt.rs`, `AuthState`, `ChatLayout.svelte`, `chat.author`, autorização a partir do payload, verificação de assinatura no cliente, rotas/flags/schema/migrações; `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog, constituição; testes/gates/navegador e resolução do chat **[NV]**.
- **Success metrics**: com `qa-admin@flora.test`, o menu mostra `qa-admin` e `qa-admin@flora.test` e acompanha mudanças de `authState.token` sem reload; token ausente/corrompido → `null` e fallback, sem exceção ou log; a assinatura continua validada apenas no servidor.
- **Carried-forward open questions**: `[NV]` `npm run test:unit`, `check`, `lint`, build e gates; `[NV]` validação visual em navegador e troca de conta/organização; `[NV]` dívida do chat (`Usuário <id>`) com lookup de usuários — nenhuma bloqueia o `go`.
