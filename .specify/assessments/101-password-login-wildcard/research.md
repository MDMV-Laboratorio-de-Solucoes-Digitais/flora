# Research: 101 — login com `*` quando a opt-in de bootstrap está ativa

**Data:** 2026-11-21 · **Evidência local determinística + relato de produção** · DB real/login ao vivo **[NV]** · branch `feature/101-password-login-wildcard@5890471be35ec21f7c8eba9545bca3544edbd8d9`

## Primária (local)

1. `flora-api/src/routes/mod.rs:30` — o handler de login está montado em `/api/v1/auth/password/login` via `auth::create_auth_router()` (`auth.rs:80-88`); esta é a única superfície de rota alterada no ciclo.
2. `flora-api/src/routes/auth.rs:447-453` — `password_login()` normaliza `req.email` com `trim().to_lowercase()` (`:451`) e chama `repo.find_by_email(&email)` (`:452-453`). Não há tratamento de token especial: `*` é apenas uma string que não corresponde a nenhum e-mail.
3. `flora-api/src/routes/auth.rs:455-460` — `stored_hash` vem do usuário encontrado; `verify_or_dummy()` (`flora-core/src/services/password_service.rs:75-86`) gasta o mesmo trabalho com um hash dummy quando o usuário não existe e devolve `false`. Em seguida, `!password_ok || !found.is_active` retorna `Error::InvalidCredentials` → 401 genérico (`:458-460`). É exatamente a resposta observada em produção com `email="*"`.
4. `flora-api/src/routes/auth.rs:463-491` — o sucesso cria sessão (`jti` UUIDv7, TTL de `app.session_ttl_secs`), emite JWT via `encode_token` e devolve `session_token`, `expires_in`, `user_id`, `next_step: "select_or_create_organization"`. Esse payload é o contrato OIDC que deve permanecer idêntico no caminho wildcard.
5. `flora-core/src/config/bootstrap.rs:56-60` — `is_wildcard_selector()` já define o token da opt-in: `admin_email.trim() == "*"`. `validate()` (`:69-95`) aceita esse token ou e-mail válido, nunca variantes como `*@x.com`; `Debug` redactado (`:35-46`) e `skip_serializing` (`:31`) permanecem. Reutilizar a mesma definição evita divergência entre startup e login.
6. `flora-core/src/config/bootstrap.rs:129-143` — com a opt-in, `apply_startup_password()` seleciona o primeiro usuário ativo com `WHERE is_active = TRUE ORDER BY created_at ASC, id ASC LIMIT 1`; sem usuário, aviso e `Ok(false)` (`:144-155`). O login sob `email="*"` deve resolver exatamente o mesmo `id` — caso contrário a senha aplicada no startup não corresponderia à conta autenticada.
7. `flora-core/src/config/mod.rs:46-52` — `Figment` com prefixo `FLORA_` e `split("__")` mapeia `FLORA_BOOTSTRAP__ADMIN_EMAIL` para `config.bootstrap.admin_email`; `Config::validate()` (`:59-67`) já chama `bootstrap.validate()`. `AppState.config: Arc<Config>` (`flora-api/src/state/app_state.rs:23-28`) está disponível no handler, então **nenhuma variável de configuração nova é necessária**: a opt-in é lida do próprio config.
8. `flora-api/src/middleware/rate_limit.rs:136,151,195,241-245` — a classe `PasswordLogin` limita por IP (5/60s). O alias `*` entra no mesmo handler e na mesma classe; nenhuma rota, classe ou chave de rate limit é adicionada.
9. `flora-tests/src/contract/auth.rs:1039-1183` — `mod db_password_login` já cobre 200 com sessão (`:1104-1127`), 401 idêntico para senha errada e usuário desconhecido (`:1131-1152`), usuário sem hash (`:1156-1166`) e conta inativa (`:1169-1182`). São os contratos a estender — não substituir — com os cenários do alias.
10. `flora-tests/src/contract/bootstrap.rs:410,568-598` — `db_startup_password_wildcard_targets_first_active_user` e `db_startup_password_wildcard_without_active_user_is_noop` fixam a seleção determinística e o no-op sem usuário ativo; o login deve espelhar esse critério.
11. `.env.example:36-45` — documenta a recuperação temporária por `*` e a remoção das variáveis após o login; passa a explicar que, enquanto a opt-in estiver ativa, o próprio `*` pode ser usado como `email` no login, sem conhecer o endereço real.

## Leitura do relato de produção

- `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e senha configurada; startup com `applied = true`: o hash da senha temporária foi gravado no primeiro usuário ativo (mecanismo de 100).
- `POST /api/v1/auth/password/login` com `email="*"` → 401 `invalid_credentials`: consequência direta da busca exata (`find_by_email`) e do `verify_or_dummy`, não de senha incorreta.
- O e-mail real da conta é desconhecido de quem opera: enviá-lo não é uma opção; o alias `*` é o único identificador disponível e já é, por si só, a opt-in explícita.
- Os fatos são mutuamente consistentes com o código: não é necessário invocar falha de hash, migração, deploy parcial ou valor de env incorreto.

## Diagnóstico

Cadeia fechada: opt-in `*` ativa no startup → hash sobrescrito no primeiro usuário ativo → login com `*` faz busca exata → nenhum usuário → dummy verify → 401 genérico. A senha está correta; o que falta é o alias de login. A extensão é deliberadamente estreita: o mesmo token `*` que já é opt-in explícita passa a ser aceito como `email` no login **apenas** quando `config.bootstrap.admin_email` também é `*`; a resolução usa a mesma seleção determinística de `bootstrap.rs:129-143`. Qualquer outro valor de `ADMIN_EMAIL` — inclusive typo — mantém o comportamento atual: `*` é e-mail inválido, busca exata falha, 401 genérico, nenhuma conta alterada. Assim o mecanismo não pode ser ativado por acidente nem criar um segundo caminho de autenticação.

## [NV]

- Estado real do DB de produção (e-mail real do administrador, `is_active`, `created_at`, valor efetivo das variáveis) e o login ao vivo: **[NV]**. Nenhuma credencial foi lida e nenhuma requisição de login/bootstrap foi executada contra produção nesta investigação.
- Filtro de logging real, Deploy/Coolify, IdP externo e allowlist OIDC: **[NV]** — independentes deste ciclo e não observados.
- Correção não executada contra produção neste artefato; gates, build e testes são evidência consolidada do handoff. **[NV]**
- Cobertura idiomática: pt-BR nos artefatos; inglês em nomes de código e nas mensagens de log existentes, seguindo a convenção do repositório.
