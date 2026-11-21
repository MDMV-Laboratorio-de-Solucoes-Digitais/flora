# Implementation Plan: 101 — login com `*` quando a opt-in de bootstrap está ativa

**Branch**: `feature/101-password-login-wildcard` @ `5890471be35ec21f7c8eba9545bca3544edbd8d9` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/101-password-login-wildcard/decision.md` (go) e diagnóstico determinístico do 401 no login após a recuperação de 100

## Summary

Reutilizar a opt-in `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` de 100 como alias de login:
`POST /api/v1/auth/password/login` aceita `email="*"` somente quando a opt-in
estiver ativa (`config.bootstrap.admin_email == "*"`), resolvendo o primeiro
usuário ativo por `created_at ASC, id ASC` — a mesma seleção do startup. A
verificação de senha, o 401 genérico, a sessão em formato OIDC, o JWT, o rate
limit `PasswordLogin` e o fluxo OIDC permanecem idênticos. Sem a opt-in, `*` é
um e-mail comum: busca exata, 401, nenhum caminho novo. Nenhum segredo em
log/resposta; variáveis temporárias removidas após o login. DB de produção,
login ao vivo e IdP externo **[NV]**.

## Technical Context

**Root cause**: em produção a opt-in `*` já gravou a senha no primeiro usuário
ativo (`applied = true`), mas `password_login()`
(`flora-api/src/routes/auth.rs:447-492`) normaliza `req.email` e faz
`find_by_email` exata (`:451-453`). Com `email="*"`, não há usuário,
`verify_or_dummy` devolve `false` e a rota retorna `Error::InvalidCredentials`
(`:455-460`). O e-mail real da conta é desconhecido de quem opera.

**Target Path**: `flora-api/src/routes/auth.rs` (alias `*` condicionado à opt-in e
resolução determinística), eventual helper compartilhado em `flora-core` para a
mesma seleção de `bootstrap.rs:129-143`, `flora-tests/src/contract/auth.rs`
(contratos de login), `.env.example` (documentação operacional) e artefatos do
ciclo 101. `flora-core/src/config/{mod.rs,bootstrap.rs}` e
`flora-api/src/state/app_state.rs` são lidos, não modificados.

**Testing**: opt-in `*` + senha → 200/sessão do primeiro ativo; sem opt-in →
401; variantes (`*@x.com`, `foo*`, whitespace) → 401; senha errada com `*` →
401 idêntico a desconhecido; nenhum usuário ativo → 401; conta inativa e sem
hash → 401 inalterados; formato de sessão preservado; nenhum segredo em
log/resposta. Gates do alvo (PASS no handoff). Ambiente remoto/IdP **[NV]**.

**Constraints**: nenhuma chave de config nova; `*` exato nos dois lados após
`trim()`; a seleção do login não pode divergir da do startup; senha redactada /
`skip_serializing`; nenhum segredo ou e-mail real em log/resposta/teste;
variáveis temporárias removidas após o uso; sem alterar OIDC, `POST /bootstrap`,
advisory lock, JWT ou rate limit; sem toques em `analyze/`, `converge/`,
`validate/`, `release/`, `STATE.md`, `PROGRESSO*` ou `AGENTS.md`.

## Steps

1. **Causa (T001)** — fixar em intake/research a cadeia determinística: opt-in
   `*` ativa, `applied = true`, login com `*` → `find_by_email` exata → `None`
   → `verify_or_dummy` → 401 genérico (`auth.rs:451-460`); e-mail real
   desconhecido, sem ler ou reproduzir credenciais.
2. **Opt-in (T002)** — aceitar `email == "*"` no login somente quando
   `state.config.bootstrap.admin_email` for exatamente `*` após `trim()`
   (reusar `is_wildcard_selector()`, `bootstrap.rs:56-60`); sem opt-in, busca
   exata e 401; variantes não ativam; nenhuma chave nova.
3. **Seleção (T003)** — com opt-in e `*`, resolver o primeiro usuário ativo
   (`is_active = TRUE`, `created_at ASC, id ASC LIMIT 1`, `bootstrap.rs:129-143`);
   sem usuário ativo → 401 genérico; sem escrita e sem fallback; fora do alias,
   busca exata inalterada.
4. **401 e sessão (T004)** — preservar `verify_or_dummy`, `Error::InvalidCredentials`
   (401 `invalid_credentials`), a resposta OIDC idêntica (`session_token`,
   `expires_in`, `user_id`, `next_step`), JWT, TTL, classe `PasswordLogin` e
   fluxo OIDC; o alias não é um novo método de autenticação.
5. **Sem segredo (T005)** — nenhuma senha, hash ou e-mail real em log, resposta,
   erro ou mensagem de teste; o alias não gera estado nem log novo além da
   sessão usual; redaction e `skip_serializing` de `BootstrapConfig` preservados.
6. **Contratos (T006)** — estender `flora-tests/src/contract/auth.rs`
   (`mod db_password_login`, `:1039-1183`): opt-in `*` → 200/sessão do primeiro
   ativo, sem opt-in → 401, variantes → 401, senha errada com `*` → 401 idêntico
   a desconhecido, nenhum usuário ativo → 401, formato de sessão e 401 genérico
   preservados.
7. **Env operacional (T007)** — `.env.example:36-45` documenta o login com
   `email="*"` enquanto a opt-in estiver ativa e a remoção das duas variáveis
   com restart após o login, sem valores reais; conferir ausência de segredo nos
   artefatos.
8. **Regressão e fecho (T008)** — gates do alvo PASS, rastreabilidade T001..T008
   e verificação da fronteira proibida.

## Constitution Check

Sem violações: nenhum required check, gate, contrato do `POST /bootstrap`, lock,
formato de 401, JWT, rate limit, emissão de sessão, workflow, ADR ou constituição
é alterado; nenhum dado de usuário ou credencial é exposto; o alias existe apenas
sob opt-in explícita e as variáveis temporárias são removidas após o uso; o diff
fica restrito ao login wildcard e aos artefatos do ciclo 101.

## Project Structure

```text
.specify/assessments/101-password-login-wildcard/{intake,research,define,shape,decision}.md
specs/101-password-login-wildcard/
├── spec.md      # FR-001..FR-004 / SC-001..SC-003
├── plan.md      # T001..T008
└── tasks.md     # todas [X]
flora-api/src/routes/auth.rs              # alias '*' sob opt-in + seleção determinística
flora-core/src/config/bootstrap.rs        # is_wildcard_selector()/seleção (reuso, sem mudança de contrato)
flora-core/src/config/mod.rs              # Figment FLORA_ + split("__") (inalterado)
flora-api/src/state/app_state.rs          # AppState.config: Arc<Config> (inalterado)
flora-api/src/middleware/rate_limit.rs    # classe PasswordLogin (inalterada)
flora-tests/src/contract/auth.rs          # contratos de login com alias
.env.example                              # '*' no login + remoção das variáveis
```

## Complexity Tracking

Nenhuma. São uma condição de opt-in reaproveitada, uma resolução determinística
já existente, a manutenção do 401/sessão atuais, contratos e documentação
operacional — todos verificáveis por teste e inspeção, sem nova dependência,
migração, chave de configuração, rota ou superfície pública.
