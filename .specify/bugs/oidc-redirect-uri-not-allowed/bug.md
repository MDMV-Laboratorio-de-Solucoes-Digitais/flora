# Bug Report: `redirect_uri is not allowed` — conta bootstrap sem fallback de senha

- **Slug**: oidc-redirect-uri-not-allowed
- **Criado**: 2026-11-21
- **Fonte**: relato do usuário no branch `feature/096-auth-login-ptbr` @ `1910d1c7d`
- **Veredito**: válido
- **Severidade**: alta — bloqueia o ingresso em produção
- **Adjudicação**: ciclo 096 (`.specify/assessments/096-auth-login-ptbr/decision.md`, verdict `go`)

## Relato (verbatim / resumido)

> A conta `luis.granja@mdmv.com.br`, criada via bootstrap, não consegue entrar.
> O login falha com `redirect_uri is not allowed`; no ambiente não há provedor
> de identidade configurado e também não existe login local por senha.

## Sintoma

**Observado:** a tela de login só inicia o fluxo OIDC. O frontend envia
`redirect_uri=${location.origin}/callback`, que não é o callback configurado no
servidor e não está na allowlist; a API responde
`Error::OidcProvider("redirect_uri is not allowed")`. Ainda que o redirect fosse
aceito, o emissor padrão aponta para `http://localhost:8080`, onde não há IdP
disponível. A conta criada por bootstrap não possui credencial de senha e não
existe rota local que a aceite.

**Esperado:** a conta administradora criada no bootstrap consiga entrar no
ambiente real com uma credencial local (e-mail + senha), com sessão escopada à
organização, sem depender de IdP externo.

## Reprodução

1. Subir API e frontend sem as variáveis `ZITADEL_*` (configuração padrão).
2. Executar `POST /api/v1/bootstrap` com `admin_email=luis.granja@mdmv.com.br`
   (fluxo que cria a conta).
3. Abrir a tela de login e acionar "Sign In".
4. `GET /api/v1/auth/login?redirect_uri=<origin>/callback` retorna
   `redirect_uri is not allowed`; não há fallback de senha nem IdP reachable.

## Causa raiz — arquivo:linha

1. **`flora-core/src/config/oidc.rs:27-31`** — sem `ZITADEL_ISSUER_URL` /
   `ZITADEL_CLIENT_ID` / `ZITADEL_CLIENT_SECRET`, o default é
   `http://localhost:8080`, `flora@localhost` e secret vazio. O ambiente de
   produção não sobrescreve esses valores (`.env.example:66-68` mantém as três
   linhas comentadas), então não há IdP a quem redirecionar.
2. **`flora-core/src/config/oidc.rs:38`** — `allowed_redirect_uris: Vec::new()`:
   a allowlist extra de redirects nasce vazia.
3. **`flora-api/src/routes/auth.rs:101-116`** — o callback aceito por default é
   `{base_url}/api/v1/auth/callback` (linha 101) e qualquer `redirect_uri`
   diferente dele é rejeitado com `redirect_uri is not allowed` (linhas 106-116)
   porque a allowlist da linha 110 está vazia. O issuer usado na URL de
   autorização vem da config default (linha 99, montagem em 137).
4. **`apps/frontend/src/routes/(auth)/+page.svelte:43-45`** — o frontend envia
   sempre `${location.origin}/callback`, valor que não coincide com o callback
   do servidor e não está na allowlist: a rejeição é determinística, não
   intermitente. A mesma tela só trata erro de login OIDC
   (`+page.svelte:38-72`).
5. **`flora-api/src/routes/bootstrap.rs:32-37`** — `BootstrapRequest` contém
   apenas `organization_name` e `admin_email`; não aceita senha.
6. **`flora-api/src/routes/bootstrap.rs:147-158`** — o `INSERT INTO users` não
   grava nenhuma coluna de senha; a conta bootstrap nasce sem credencial local.
7. **`flora-core/src/models/user.rs:13-33`** — o modelo `User` não tem campo de
   hash de senha; **`flora-core/src/traits/auth_provider.rs:9-12`** cita o
   fallback local apenas como possibilidade futura, sem implementação.
8. Não existe rota `POST /api/v1/auth/password/login` nem serviço de hash de
   senha: `grep` por `argon2` / `password_hash` em `flora-core` e `flora-api`
   retorna apenas utilitários de validação (`flora-core/src/utils/validation.rs:59-105`).

## Correção adjudicada (ciclo 096)

Fallback local Argon2id (m=19456, t=2, p=1, PHC) com verificação de dummy hash e
401 genérico; senha exigida no bootstrap e gravada na mesma transação;
recuperação idempotente por `FLORA_BOOTSTRAP__ADMIN_EMAIL` /
`FLORA_BOOTSTRAP__ADMIN_PASSWORD` no startup, apenas quando `password_hash` é
NULL; rota `POST /api/v1/auth/password/login` com token/sessão e rate limit;
tela "Sign In" local em pt-BR e `lang="pt-BR"` em `app.html`.
Ver `.specify/assessments/096-auth-login-ptbr/decision.md` e
`specs/096-auth-login-ptbr/spec.md` (FR-001..FR-008 / SC-001..SC-003).

## [NV]

- A existência, configuração e disponibilidade de um IdP externo (Zitadel ou
  outro), o valor real de `FLORA_APP__BASE_URL` em produção e o comportamento
  remoto do FQDN **não foram observados neste snapshot**: **[NV]**. O diagnóstico
  vale para o código comitado e para os defaults; a correção não depende de IdP.
- O erro foi registrado pelo relato do usuário e confirmado por inspeção do
  código; nenhuma requisição HTTP ao ambiente ao vivo foi executada aqui.
