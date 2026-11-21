# Intake: 104 — Identidade da sessão na navegação

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/104-session-identity@23ff4bc809caccff2cbc9018521d8762f308ddeb` · `.specify/feature.json` → `specs/104-session-identity`

## Sinal

O menu do usuário na navegação do app mostra sempre os mesmos textos, qualquer que seja a sessão autenticada: `Nome do usuário` e `usuario@flora.app`. No baseline `23ff4bc809`, `apps/frontend/src/routes/(app)/+layout.svelte:182-185` interpola diretamente `t['nav.userName']` e `t['nav.userEmail']`, e `apps/frontend/src/lib/i18n/pt-BR.ts:53-54` define esses valores como literais fixos. A identidade real da sessão já está disponível sem requisição nova: o JWT de sessão carrega a claim `email` (`flora-core/src/utils/jwt.rs:25`, preenchida por `FloraClaims::new` em `:38-55`, notadamente `:51`), e `authState.token` é lido no próprio layout (`+layout.svelte:5,36,58,66,70`).

O efeito é de fidelidade da interface, não de sigilo: não há vazamento de dado de produção porque o que aparece é um literal, mas a navegação apresenta uma identidade falsa e impede que o usuário confirme rapidamente qual conta está ativa. O caso vizinho do chat é mais grave e distinto: `apps/frontend/src/lib/components/chat/ChatLayout.svelte:124,130` mostra a inicial de `authorId` e `format('chat.author', { id: message.authorId.slice(0, 8) })`, com `chat.author = 'Usuário {id}'` (`pt-BR.ts:80`); resolver isso exige lookup/cache de usuários e está **fora de escopo**, registrado abaixo como dívida e **[NV]**.

## Evidência

```
NAVEGAÇÃO COM LITERAIS (baseline 23ff4bc809):
apps/frontend/src/routes/(app)/+layout.svelte:5       import { authState } from '$lib/state/AuthState.svelte';
apps/frontend/src/routes/(app)/+layout.svelte:24      import { t } from '$lib/i18n';
apps/frontend/src/routes/(app)/+layout.svelte:36      enforceAuth() já lê authState.token;
apps/frontend/src/routes/(app)/+layout.svelte:58,66,70 $effect rastreia authState.token e compõe wsKey;
apps/frontend/src/routes/(app)/+layout.svelte:180-186  dropdown com as duas interpolações fixas;
apps/frontend/src/routes/(app)/+layout.svelte:182     {t['nav.userName']} -> "Nome do usuário";
apps/frontend/src/routes/(app)/+layout.svelte:184     {t['nav.userEmail']} -> "usuario@flora.app".
apps/frontend/src/lib/i18n/pt-BR.ts:53-54             'nav.userName' / 'nav.userEmail' com os literais;
apps/frontend/src/lib/i18n/pt-BR.ts:48                seção "Navegação / layout do app".

FONTE DA IDENTIDADE (sem mudança de backend):
flora-core/src/utils/jwt.rs:17-33                     FloraClaims: sub, jti, organization_id, email, exp, iat, ver;
flora-core/src/utils/jwt.rs:25                        pub email: String — "User email for display";
flora-core/src/utils/jwt.rs:38-55                     FloraClaims::new(user_id, jti, organization_id, email, ttl_secs);
flora-core/src/utils/jwt.rs:51                        email: email.to_string();
flora-core/src/utils/jwt.rs:82-89                     encode_token(claims, secret) -> String;
flora-core/src/utils/jwt.rs:96-104                    decode_token valida assinatura/expiração no servidor.
apps/frontend/src/lib/state/AuthState.svelte.ts:36    token = $state<string | null>(null);
apps/frontend/src/lib/state/AuthState.svelte.ts:69-81 login(token, user, organizationId) atualiza token e persiste;
apps/frontend/src/lib/state/AuthState.svelte.ts:87-99 setOrganization(token, organizationId) também atualiza token;
apps/frontend/src/lib/state/AuthState.svelte.ts:170   export const authState = new AuthState().

FERRAMENTA DE TESTE JÁ PRESENTE:
apps/frontend/package.json:15                        "test:unit": "vitest";
apps/frontend/package.json:47                        vitest ^4.1.8;
apps/frontend/vite.config.ts:27-53                   projetos 'client' (browser/.svelte.spec.ts) e 'server' (node/tests/**);
apps/frontend/vite.config.ts:28                      expect.requireAssertions: true;
apps/frontend/tests/unit/tasks-query.test.ts:1-21   convenção describe/it/expect com import relativo.

DÍVIDA VIZINHA (FORA DE SCOPE):
apps/frontend/src/lib/components/chat/ChatLayout.svelte:118-131  lista de mensagens por autor;
apps/frontend/src/lib/components/chat/ChatLayout.svelte:124       inicial de message.authorId;
apps/frontend/src/lib/components/chat/ChatLayout.svelte:130       format('chat.author', { id: message.authorId.slice(0, 8) });
apps/frontend/src/lib/i18n/pt-BR.ts:80                           'chat.author': 'Usuário {id}'.
```

Nenhum gate, build, teste, formatter ou deploy foi executado neste artefato; a inspeção é documental sobre o baseline `23ff4bc809`, e os números de linha acima se referem a esse commit. Nenhum segredo, token real ou dado de produção foi lido ou registrado aqui.

## Autorização

Modo autônomo. Decisão `go` em `.specify/assessments/104-session-identity/decision.md` antes da implementação. Escopo: (1) helper puro `emailFromToken(token)` em `apps/frontend/src/lib/utils/jwt.ts`, que decodifica somente o payload base64url da claim `email` e retorna `string | null` com fail-closed; (2) cobertura Vitest em `apps/frontend/tests/unit/jwt.test.ts` no projeto `server`; (3) `+layout.svelte` passa a derivar o e-mail com `$derived` a partir de `authState.token`, exibindo o e-mail real e usando `t['nav.userName']`/`t['nav.userEmail']` apenas como fallback; (4) dívida do chat (`Usuário <id>`) registrada como fora de escopo/[NV]. Nenhuma mudança de backend, assinatura/verificação de JWT, gate, required check, chave de i18n diferente das duas de fallback, rota, schema, flag, changelog ou commit.

## Fronteira

Artefatos novos do ciclo 104 (`.specify/assessments/104-session-identity/` e `specs/104-session-identity/`) e, na implementação, `apps/frontend/src/lib/utils/jwt.ts`, `apps/frontend/tests/unit/jwt.test.ts` e `apps/frontend/src/routes/(app)/+layout.svelte`. Somente leitura: `flora-core/src/utils/jwt.rs`, `apps/frontend/src/lib/state/AuthState.svelte.ts`, `apps/frontend/src/lib/i18n/pt-BR.ts`, `apps/frontend/src/lib/types/models.ts`, `apps/frontend/package.json`, `apps/frontend/vite.config.ts`, `apps/frontend/tests/unit/tasks-query.test.ts` e `ChatLayout.svelte`. Sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição. A factory `mdmv-dark-factory` é somente leitura. Nenhum commit é feito neste ciclo.
