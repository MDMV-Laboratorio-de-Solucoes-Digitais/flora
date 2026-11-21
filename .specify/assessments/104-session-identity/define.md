# Define: 104 — Identidade da sessão na navegação

**Data:** 2026-11-21 · **Branch**: `feature/104-session-identity@23ff4bc809caccff2cbc9018521d8762f308ddeb`

## Problema

O dropdown do usuário em `apps/frontend/src/routes/(app)/+layout.svelte:182-185` mostra `Nome do usuário` e `usuario@flora.app` para qualquer sessão, porque interpola diretamente `t['nav.userName']` e `t['nav.userEmail']`, cujos valores são literais em `apps/frontend/src/lib/i18n/pt-BR.ts:53-54`. A sessão já contém a informação real: `FloraClaims.email` (`flora-core/src/utils/jwt.rs:25`, gravado em `:51`) e `authState.token` no cliente (`apps/frontend/src/lib/state/AuthState.svelte.ts:36`). Falta um helper puro de leitura do payload e um valor `$derived` no layout. Nenhuma mudança de backend é necessária e nenhuma autorização pode passar a depender do valor lido no navegador.

## Requisitos funcionais

- **FR-001 — helper puro e fail-closed**: `apps/frontend/src/lib/utils/jwt.ts` DEVE exportar `emailFromToken(token: string | null | undefined): string | null`. A função DEVE dividir o token em exatamente 3 segmentos não vazios, normalizar o payload de base64url (`-`→`+`, `_`→`/`, padding), decodificar via `atob` + `Uint8Array` + `TextDecoder` (UTF-8), fazer `JSON.parse` em `try/catch` e aceitar somente um objeto não-array com `email` do tipo `string` e não vazio após `trim()`. Qualquer anomalia DEVE retornar `null`, sem lançar, sem log, sem armazenar o payload e sem usar o valor para autorização (`flora-core/src/utils/jwt.rs:96-104` mostra que a validação de assinatura é responsabilidade do servidor).

- **FR-002 — e-mail real no layout via `$derived`**: `apps/frontend/src/routes/(app)/+layout.svelte` DEVE derivar, com `$derived`, `userEmail = emailFromToken(authState.token) ?? t['nav.userEmail']` e exibir esse valor no segundo `<span>` do dropdown (`:183-185`), substituindo a interpolação fixa. O valor DEVE ser reativo a `authState.token` (inclusive após `setOrganization`, `AuthState.svelte.ts:87-99`) e NÃO DEVE ser usado em `enforceAuth` (`:34-44`), no `$effect` de websocket (`:63-84`), em rotas ou em qualquer decisão de acesso.

- **FR-003 — nome e fallback do dicionário**: o primeiro `<span>` (`:182`) DEVE continuar mostrando um nome legível; quando houver e-mail real, o nome DEVE ser derivado da parte local do e-mail (texto antes de `@`), e quando não houver, DEVE usar `t['nav.userName']`. `pt-BR.ts:53-54` permanece intacto e é o único fallback: nenhuma chave é renomeada, removida ou adicionada, e nenhuma outra entrada do dicionário (em especial `chat.author` em `:80`) é alterada.

- **FR-004 — testes Vitest**: `apps/frontend/tests/unit/jwt.test.ts` DEVE cobrir, no projeto `server` (`vite.config.ts:49`, node), com `describe/it/expect` no padrão de `tests/unit/tasks-query.test.ts:1-21`: (a) payload com `email` válido → e-mail; (b) base64url sem padding e UTF-8 → e-mail; (c) `null`, `undefined`, `''`, 2 segmentos, 4 segmentos, base64 inválido e base64 válido mas não-JSON → `null`; (d) payload sem `email`, com `email` vazio/só espaços/não-string, array ou `null` → `null`. Cada caso DEVE conter asserção (`vite.config.ts:28` exige `requireAssertions`). Nenhum teste de componente, rede ou navegador é necessário.

- **FR-005 — dívida e fronteira**: a dívida do chat (`ChatLayout.svelte:124,130` mostra `Usuário <id>`; `pt-BR.ts:80`) DEVE ser registrada como fora de escopo e **[NV]** nos artefatos; resolver exige lookup de usuários. O diff NÃO DEVE tocar backend, gates, required checks, `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição; factory somente leitura; nenhum commit.

## Critérios de sucesso

- **SC-001 — identidade correta**: com uma sessão cujo JWT contém `qa-admin@flora.test`, o menu mostra `qa-admin` e `qa-admin@flora.test`; o valor muda quando `authState.token` muda, sem recarregar a página.
- **SC-002 — fail-closed e fallback**: com token ausente ou corrompido, o helper retorna `null` e o menu volta a `Nome do usuário` / `usuario@flora.app`; nenhuma exceção, log ou valor parcial vaza para a interface.
- **SC-003 — escopo e segurança preservados**: nenhuma rota, autorização, claim, assinatura, gate, required check, schema, flag, chave de i18n adicional ou arquivo de outro ciclo é alterado; testes Vitest cobrem os caminhos positivos e negativos; execução de gates/build/testes e validação em navegador ficam **[NV]** neste artefato e são registradas no handoff.

## Fora de escopo

- Mudanças em backend Rust (`flora-core/src/utils/jwt.rs`), `AuthState`, o dicionário `pt-BR.ts` (inclusive `chat.author`), `ChatLayout.svelte`, emissão de JWT, verificação de assinatura no cliente, uso do e-mail para autorização, autenticação, rotas, flags, schema, migração, required checks/gates, workflows, ADRs, constituição ou changelog; escrever `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*` ou `AGENTS.md`; executar deploy/escrita em staging/produção ou fazer commit.
