# Decision: 096 — login local pt-BR para conta bootstrap

- **Slug**: 096-auth-login-ptbr
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/096-auth-login-ptbr` @ `1910d1c7d`
- **Âncora**: `.specify/feature.json` → `specs/096-auth-login-ptbr`
- **Bug**: `.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

A conta bootstrap `luis.granja@mdmv.com.br` não consegue entrar. A cadeia é
determinística e está toda no código comitado:

1. `flora-core/src/config/oidc.rs:27-31` — defaults de OIDC sem env:
   `http://localhost:8080`, `flora@localhost`, secret vazio;
   `.env.example:66-68` mantém as variáveis comentadas.
2. `flora-core/src/config/oidc.rs:38` — `allowed_redirect_uris` nasce vazio.
3. `flora-api/src/routes/auth.rs:101-116` — só o callback exato
   `{base_url}/api/v1/auth/callback` é aceito; outros retornam
   `redirect_uri is not allowed`.
4. `apps/frontend/src/routes/(auth)/+page.svelte:43-45` — o frontend envia
   `${location.origin}/callback`, necessariamente fora da allowlist.
5. `flora-api/src/routes/bootstrap.rs:32-37,147-158` e
   `flora-core/src/models/user.rs:13-33` — a conta é criada sem senha; não
   existe `password_hash`, rota de login local ou serviço de hash
   (`flora-core/src/traits/auth_provider.rs:9-12` cita o fallback só como ideia).

Não há indício de falha intermitente de rede ou DNS: o comportamento observado
é exatamente o que os defaults e a checagem de allowlist produzem.

## Decisão

Registrar o ciclo 096 como **fallback de autenticação local com interface pt-BR**,
com oito compromissos rastreáveis (FR-001..FR-008), três critérios
(SC-001..SC-003) e quatorze tarefas (T001..T014, todas concluídas):

1. **Dados (FR-001 / T002–T003)**: migration com `password_hash` e
   `password_updated_at` nullable; modelo e repositório atualizados.
2. **Criptografia (FR-002 / T004–T005)**: Argon2id `m=19456,t=2,p=1`, PHC,
   dummy hash e 401 genérico.
3. **Bootstrap (FR-003 / T006–T007)**: `admin_password` exigido e hash gravado
   na mesma transação.
4. **Login local (FR-004 / T008–T009)**: `POST /api/v1/auth/password/login`
   com token/sessão equivalentes ao fluxo OIDC.
5. **Recuperação (FR-005 / T010–T011)**: env no startup, apenas com hash NULL,
   idempotente, sem senha em log.
6. **Rate limit (FR-006 / T008)**: classe própria na rota de login local.
7. **Interface (FR-007–FR-008 / T012–T013)**: "Sign In" local em pt-BR e
   `lang="pt-BR"`.
8. **Fecho (T001, T014)**: dependências/lockfile, supply-chain e gates do alvo.

## Evidência de entrada

- Baseline `1910d1c7d`, branch `feature/096-auth-login-ptbr`.
- Relato do usuário e causa raiz com arquivo:linha em
  `.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`.
- `flora-core/src/config/mod.rs:43-64` — Figment com prefixo `FLORA_` e
  `split("__")`, habilitando `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `ADMIN_PASSWORD`.
- `flora-api/src/middleware/rate_limit.rs:138-193` — padrão de classes.
- `flora-api/src/routes/auth.rs:247-291` — contrato de sessão do callback OIDC.
- `flora-core/src/migrations/sql/` — migrations versionadas por
  `sqlx::migrate!`.
- `apps/frontend/src/app.html:2` — `lang="en"` a ser trocado.

## Racional

**go.** O defeito é consequência direta de cinco pontos verificáveis no código:
defaults OIDC sem IdP, allowlist vazia, redirect do frontend fora do callback,
conta bootstrap sem credencial e ausência de rota local. A correção é um
fallback isolado — nenhuma alteração de gate, contrato OIDC, schema além das
duas colunas nullable ou infraestrutura externa. Todos os FR/SC são
verificáveis por testes locais e inspeção; o diff é reversível.

## [NV]

- IdP externo (Zitadel), URIs registradas, segredo, disponibilidade de rede e
  o comportamento do ambiente remoto/produção **não foram executados nem
  observados neste snapshot**: **[NV]**. O veredito `go` autoriza o ciclo; não
  afirma que um IdP existe ou que o deploy foi observado.
- O gate do alvo está registrado como PASS no handoff consolidado; este
  artefato não reexecutou gates, build, testes ou formatters.

## Fora de âmbito

Mudanças em `analyze/`, `converge/`, `validate/`, `release/`,
`.specify/memory/STATE.md` ou `PROGRESSO*`; alteração de required checks,
workflows, ADRs ou constituição; configuração de IdP externo; novos métodos de
login além de senha local.

## Handoff

`/speckit.specify` — especificar o fallback como requisitos verificáveis
(FR-001..FR-008 / SC-001..SC-003) e tarefas T001..T014, com Argon2id e 401
genérico, senha no bootstrap, recuperação idempotente por env, rota de login com
rate limit, tela pt-BR e fronteira preservada.
