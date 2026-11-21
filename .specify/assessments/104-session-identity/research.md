# Research: 104 — Identidade da sessão na navegação

**Data:** 2026-11-21 · **Inspeção determinística do baseline `23ff4bc809`** · execução de gates/testes e validação em staging real **[NV]** · branch `feature/104-session-identity`

## Primária (local)

1. `apps/frontend/src/routes/(app)/+layout.svelte:182-185` — o dropdown do usuário interpola `t['nav.userName']` (`:182`) e `t['nav.userEmail']` (`:183-185`) sem consultar a sessão. O mesmo arquivo já importa `authState` (`:5`), `t` (`:24`) e lê `authState.token` em `enforceAuth` (`:36`) e nos `$effect` (`:58,66,70`); logo a fonte de dados está no escopo do componente, sem prop ou serviço novo.
2. `apps/frontend/src/lib/i18n/pt-BR.ts:53-54` — os valores são literalmente `'Nome do usuário'` e `'usuario@flora.app'`, na seção "Navegação / layout do app" (`:48`). O contrato do ciclo é mantê-los como **fallback**; nenhuma outra chave (por exemplo `chat.author` em `:80`) muda.
3. `flora-core/src/utils/jwt.rs:17-33` — `FloraClaims` contém `pub email: String` (`:25`) descrito como "User email for display", além de `sub`, `jti`, `organization_id`, `exp`, `iat`, `ver`. `FloraClaims::new` (`:38-55`) recebe `email: &str` e grava `email.to_string()` (`:51`); `encode_token` (`:82-89`) emite o token assinado. A claim já existe — não há trabalho de backend.
4. `flora-core/src/utils/jwt.rs:96-104` — `decode_token` valida assinatura e expiração no servidor com a chave secreta. O navegador não tem essa chave e **não deve** tentar verificar assinatura: a leitura do payload é apenas para exibição, nunca para autorização.
5. `apps/frontend/src/lib/state/AuthState.svelte.ts:36,69-81,87-99,170` — `token` é `$state<string | null>`; `login` e `setOrganization` atualizam o token e persistem em `flora.auth.v1`. O token muda ao trocar de organização, então o valor derivado no layout precisa ser reativo a `authState.token`, não capturado uma vez.
6. `apps/frontend/src/lib/utils/` — diretório de utilitários puros do frontend (`files.ts`, `logger.ts`, `search.ts`), com teste colocado ao lado (`logger.spec.ts`); é o lugar natural para `jwt.ts`. `apps/frontend/src/lib/types/models.ts:4-10` define `User.email`, mas `authState.user` é um `Record<string, unknown>` (`AuthState.svelte.ts:37`) e não é preenchido em todos os fluxos (por exemplo `setOrganization` em `:87-99` não toca `user`): o token é a fonte contínua e confiável para a claim.
7. Formato do JWT: três segmentos `header.payload.signature` separados por pontos. O payload é JSON codificado em base64url (alfabeto `-`/`_`, sem padding obrigatório). Decodificação segura exige: exatamente 3 segmentos não vazios, troca de `-`→`+` e `_`→`/`, padding até múltiplo de 4, `atob`, `Uint8Array`, `TextDecoder` (UTF-8) e `JSON.parse` dentro de `try/catch`.
8. Fail-closed: qualquer anomalia — token nulo/vazio, segmentos errados, base64 inválido, JSON inválido, payload não-objeto/array, `email` ausente, vazio, só espaços ou não-string — deve produzir `null`. O helper não lança, não faz log, não armazena o payload e não substitui dados por "chutes".
9. `apps/frontend/package.json:15,47` e `apps/frontend/vite.config.ts:27-53` — Vitest já está configurado com dois projetos; o projeto `server` (node) inclui `tests/**/*.{test,spec}.{js,ts}` (`:49`), que é onde `tests/unit/jwt.test.ts` é descoberto. `expect.requireAssertions: true` (`:28`) exige ao menos uma asserção por teste.
10. `apps/frontend/tests/unit/tasks-query.test.ts:1-21` — padrão a seguir: import direto de `vitest` e da função por caminho relativo, `describe` com casos pequenos e explícitos. Não é necessário componente, navegador ou mock de rede para testar o helper puro.
11. Svelte 5 com runes (`apps/frontend/vite.config.ts:15-18` força `runes: true`) — a reatividade deve usar `$derived` no script do layout. O valor pode ser derivado diretamente de `emailFromToken(authState.token)`; o `$effect` de autenticação (`:55-61`) continua responsável por redirecionamento e não precisa mudar.
12. `apps/frontend/src/lib/components/chat/ChatLayout.svelte:118-131` — cada mensagem mostra a inicial do `authorId` (`:124`) e `format('chat.author', { id: message.authorId.slice(0, 8) })` (`:130`), com `'chat.author': 'Usuário {id}'` (`pt-BR.ts:80`). Corrigir isso exige lookup de usuários/cache por `authorId`, fora do escopo deste ciclo; registrar como dívida e **[NV]**.

## Leitura da reprodução

- Os literais aparecem para todo mundo porque o layout nunca lê a claim; não dependem do token, do usuário ou da organização.
- A claim `email` já é emitida pelo backend (`jwt.rs:25,51`) e o token já está no cliente (`AuthState.svelte.ts:36`); a correção é exclusivamente de apresentação no frontend.
- Não existe helper de decodificação no baseline: `git grep emailFromToken HEAD -- apps/frontend` retorna vazio. Portanto o helper e seus testes são aditivos.
- O e-mail decodificado no cliente não é autoridade: nenhuma decisão de acesso, rota ou organização pode passar a depender dele.
- Nenhuma falha intermitente é necessária para explicar o sintoma: o componente simplesmente interpola constantes do dicionário.

## Diagnóstico

Cadeia fechada: sessão autenticada com token contendo `email` (`jwt.rs:25,51`) → `authState.token` disponível no layout (`AuthState.svelte.ts:36`, `+layout.svelte:5`) → dropdown interpola constantes `t['nav.userName']`/`t['nav.userEmail']` (`+layout.svelte:182-185`, `pt-BR.ts:53-54`) → identidade falsa. A correção é um helper puro e fail-closed mais um valor `$derived` no layout; o dicionário permanece apenas como fallback. O caso do chat é independente e fica como dívida.

## [NV]

- Execução de `npm run test:unit`, `check`, `lint`, build e gates em staging real: **[NV]** neste artefato — apenas inspeção documental; consolidação no handoff.
- Aparência final do menu em navegador e troca de conta/organização ao vivo: **[NV]** — a reatividade é verificada por construção (`$derived` sobre `authState.token`), não por execução aqui.
- Resolução de `Usuário <id>` no chat (avatars e nomes de autores) e eventual lookup/cache de usuários: **[NV]** / dívida registrada — fora de escopo do ciclo 104.
- Números de linha: referem-se ao baseline `23ff4bc809`; uma implementação paralela pode deslocá-los. Nenhum segredo, token real ou dado de produção foi lido ou registrado.
