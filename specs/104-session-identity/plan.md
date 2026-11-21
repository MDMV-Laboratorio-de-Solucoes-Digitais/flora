# Implementation Plan: 104 — Identidade da sessão na navegação

**Branch**: `feature/104-session-identity` @ `23ff4bc809caccff2cbc9018521d8762f308ddeb` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/104-session-identity/decision.md` (go) e inspeção determinística do baseline `23ff4bc809` (literais no layout, claim `email` no JWT, AuthState e Vitest já presentes)

## Summary

Substituir os literais fixos do dropdown por um valor derivado da sessão: `emailFromToken` lê a claim `email` do payload JWT de forma pura e fail-closed, e `+layout.svelte` usa `$derived` para mostrar o e-mail real e, como nome, sua parte local. Os literais `nav.userName`/`nav.userEmail` permanecem intactos no dicionário e são usados apenas quando a leitura falha. Nenhuma mudança de backend, autorização, gate, required check, rota, flag, schema, chave de i18n adicional ou commit. A dívida do chat (`Usuário <id>`) fica registrada como fora de escopo e **[NV]**.

## Technical Context

**Root cause**: `apps/frontend/src/routes/(app)/+layout.svelte:182-185` interpola `t['nav.userName']` e `t['nav.userEmail']`, definidos como constantes em `apps/frontend/src/lib/i18n/pt-BR.ts:53-54`. O token já contém `email` (`flora-core/src/utils/jwt.rs:25,51`) e já é lido no layout (`:5,36,58,66,70`), mas nunca é usado no dropdown. Não existe helper de decodificação no baseline (`git grep emailFromToken HEAD -- apps/frontend` é vazio).

**Target Path**: `apps/frontend/src/lib/utils/jwt.ts` (novo, helper puro) e `apps/frontend/tests/unit/jwt.test.ts` (novo, Vitest no projeto `server`), `apps/frontend/src/routes/(app)/+layout.svelte` (import + `$derived` + duas interpolações), além dos artefatos do ciclo 104. Somente leitura: `flora-core/src/utils/jwt.rs`, `apps/frontend/src/lib/state/AuthState.svelte.ts`, `apps/frontend/src/lib/i18n/pt-BR.ts`, `apps/frontend/src/lib/types/models.ts`, `apps/frontend/package.json`, `apps/frontend/vite.config.ts`, `apps/frontend/tests/unit/tasks-query.test.ts` e `apps/frontend/src/lib/components/chat/ChatLayout.svelte`.

**Testing**: no projeto `server` do Vitest (`vite.config.ts:49`, node), `tests/unit/jwt.test.ts` cobre: payload com `email` válido → e-mail; base64url sem padding e UTF-8 → e-mail; `null`/`undefined`/`''`, 2 e 4 segmentos, segmento vazio, base64 inválido, JSON inválido, payload sem `email`, `email` vazio/só espaços/não-string, array e `null` → `null`. Cada teste com asserção explícita (`:28`). Nenhum teste de navegador, rede ou fixture de sessão. Gates do alvo no handoff **[NV]**.

**Constraints**: o helper é puro, não verifica assinatura (ela é feita no servidor por `decode_token`, `jwt.rs:96-104`) e nunca é usado para autorização; `pt-BR.ts:53-54` é o único fallback e nenhuma outra chave muda; `$derived` deve reagir a `authState.token`, inclusive após `setOrganization` (`AuthState.svelte.ts:87-99`); `enforceAuth` (`+layout.svelte:34-44`) e o `$effect` de websocket (`:63-84`) não mudam; sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição; sem commits.

## Steps

1. **Causa (T001)** — fixar em intake/research a cadeia do baseline `23ff4bc809`: literais em `+layout.svelte:182-185` e `pt-BR.ts:53-54`, claim `email` em `jwt.rs:25,51`, token reativo em `AuthState.svelte.ts:36,87-99` e convenção Vitest em `vite.config.ts:27-53`/`tasks-query.test.ts:1-21`. Registrar `Usuário <id>` do chat como dívida/**[NV]**. Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).
2. **Helper (T002)** — criar `apps/frontend/src/lib/utils/jwt.ts` com `emailFromToken(token: string | null | undefined): string | null`: early return para token vazio; `split('.')` com exatamente 3 segmentos não vazios; `replaceAll('-', '+')`/`replaceAll('_', '/')`; `padEnd` até múltiplo de 4; `atob` → `Uint8Array` → `TextDecoder` → `JSON.parse` em `try/catch`; guard de objeto não-array, presença de `email`, typeof `string` e `trim()` não vazio; `null` em qualquer falha (FR-001).
3. **Layout (T003)** — em `+layout.svelte`, importar `emailFromToken` de `$lib/utils/jwt`, declarar `const userEmail = $derived(emailFromToken(authState.token) ?? t['nav.userEmail']);` e substituir as interpolações de `:182-185`: primeiro `<span>` com a parte local do e-mail quando `userEmail !== t['nav.userEmail']`, senão `t['nav.userName']`; segundo `<span>` com `userEmail`. Nada mais no script muda (FR-002, FR-003, SC-001, SC-002).
4. **Testes positivos (T004)** — em `tests/unit/jwt.test.ts`, montar tokens com payload JSON em `base64url` e verificar o retorno exato para `email` simples e com caracteres UTF-8/sem padding; manter o padrão `describe/it/expect` de `tasks-query.test.ts:1-21` (FR-004, SC-001).
5. **Testes negativos (T005)** — cobrir `null`, `undefined`, `''`, `'only.two'`, `'a.b.c.d'`, segmento vazio/base64 inválido, base64 válido com JSON inválido, payload sem `email`, `email` vazio/espaços/número, array e `null`, sempre com `toBeNull()`; verificar que nada é lançado (FR-001, FR-004, SC-002).
6. **Dívida do chat (T006)** — registrar em assessment/spec/plan/tasks que `ChatLayout.svelte:124,130` e `chat.author` (`pt-BR.ts:80`) permanecem como `Usuário <id>`: dependem de lookup de usuários, estão fora de escopo e são **[NV]**; nenhum arquivo do chat é alterado.
7. **Validação documental (T007)** — conferir que o e-mail decodificado não é usado fora da exibição, que as duas chaves de i18n seguem idênticas e que nenhuma outra chave mudou; registrar gates, `check`, `lint`, testes e validação em navegador no handoff **[NV]**, sem escrita em staging/produção.
8. **Fecho (T008)** — rastreabilidade T001..T008 em decision/spec/plan/tasks, verificação da fronteira proibida e confirmação de que nenhum required check/gate, schema, flag, rota, claim ou chave de i18n nova foi introduzido e que não houve commit.

## Constitution Check

Sem violações: a mudança é local ao frontend, não cria superfície de autorização nem altera contrato de sessão. O helper é puro e fail-closed; a validação de assinatura continua exclusivamente no servidor (`jwt.rs:96-104`). A claim `email` e o emissor de token não mudam; `pt-BR.ts:53-54` permanece como fallback e nenhuma outra chave é tocada. `enforceAuth`, websocket, rotas, gates, required checks, workflows, ADRs e constituição seguem idênticos; o diff fica restrito a dois arquivos novos, ao layout e aos artefatos do ciclo 104.

## Project Structure

```text
.specify/assessments/104-session-identity/{intake,research,define,shape,decision}.md
specs/104-session-identity/
├── spec.md      # FR-001..FR-005 / SC-001..SC-003
├── plan.md      # T001..T008
└── tasks.md     # fila T001..T008 (a executar)
apps/frontend/src/lib/utils/jwt.ts          # novo: emailFromToken puro e fail-closed
apps/frontend/tests/unit/jwt.test.ts        # novo: Vitest (projeto server) com casos positivos e negativos
apps/frontend/src/routes/(app)/+layout.svelte  # import + $derived + duas interpolações
flora-core/src/utils/jwt.rs                 # FloraClaims.email (reuso, inalterado)
apps/frontend/src/lib/state/AuthState.svelte.ts # token reativo (reuso, inalterado)
apps/frontend/src/lib/i18n/pt-BR.ts         # nav.userName / nav.userEmail como fallback (inalterado)
apps/frontend/vite.config.ts                # projetos Vitest e requireAssertions (reuso, inalterado)
apps/frontend/src/lib/components/chat/ChatLayout.svelte # dívida Usuário <id> (fora de escopo)
```

## Complexity Tracking

Nenhuma. Um helper puro de ~15 linhas, um `$derived` e uma tabela de casos Vitest bastam; não há dependência nova, chamada de rede, estado adicional, componente novo, mudança de backend ou abstração compartilhada. A decodificação manual do payload é intencionalmente local: evitar dependência de biblioteca JWT no cliente reforça que o valor é apenas para exibição, nunca para autorização.
