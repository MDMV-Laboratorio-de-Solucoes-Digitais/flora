# Shape: 101 — login com `*` quando a opt-in de bootstrap está ativa

**Data:** 2026-11-21 · **Branch**: `feature/101-password-login-wildcard@5890471be35ec21f7c8eba9545bca3544edbd8d9`

## Forma escolhida

Reutilizar o token que já é a opt-in explícita de 100. `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` hoje habilita a sobrescrita da senha do primeiro usuário ativo; o mesmo literal passa a ser aceito como `email` no `POST /api/v1/auth/password/login` **apenas enquanto essa opt-in estiver ativa**. A rota resolve o mesmo registro determinístico (`is_active = TRUE`, `created_at ASC, id ASC LIMIT 1`) e segue pelo fluxo de verificação e sessão já existente. Nenhuma chave de configuração nova, nenhuma rota nova, nenhum método de autenticação novo: o `*` é um alias para a conta cuja senha foi recuperada.

1. **Causa e rastro (T001)**: fixar em intake/research a cadeia — opt-in `*` ativa, `applied = true`, mas o login com `*` faz `find_by_email` exata → `None` → `verify_or_dummy` → 401 genérico (`auth.rs:451-460`); o e-mail real é desconhecido.
2. **Opt-in (T002 / FR-001)**: aceitar `email == "*"` somente quando `config.bootstrap.admin_email`, lido de `FLORA_BOOTSTRAP__ADMIN_EMAIL`, for exatamente `*` após `trim()` (reusar `is_wildcard_selector()`, `bootstrap.rs:56-60`). Sem opt-in, busca exata e 401; variantes nunca ativam; nenhuma chave nova.
3. **Seleção (T003 / FR-002)**: com opt-in e `*`, resolver o primeiro usuário ativo pela mesma consulta de `bootstrap.rs:129-143`; sem usuário ativo → 401 genérico, sem escrita e sem fallback. A seleção do login não pode divergir da do startup.
4. **401 e sessão (T004 / FR-003)**: preservar `verify_or_dummy`, `Error::InvalidCredentials` (401 `invalid_credentials`) e a resposta OIDC idêntica (`session_token`, `expires_in`, `user_id`, `next_step`), com o mesmo JWT, TTL, classe `PasswordLogin` e fluxo OIDC; senha errada, sem opt-in e usuário desconhecido continuam indistinguíveis.
5. **Sem segredo (T005 / FR-003, SC-003)**: nenhuma senha, hash ou e-mail real em log, resposta, erro ou mensagem de teste; o alias não gera estado nem log novo além da sessão usual, e não se torna um segundo método de autenticação.
6. **Contratos (T006 / FR-004)**: estender `flora-tests/src/contract/auth.rs` (`mod db_password_login`, `:1039-1183`): opt-in `*` → 200/sessão do primeiro ativo; sem opt-in → 401; variantes → 401; senha errada com `*` → 401 idêntico a desconhecido; nenhum usuário ativo → 401; formato de sessão e 401 genérico preservados; sem segredo em log/resposta.
7. **Env operacional (T007 / FR-004, SC-003)**: `.env.example:36-45` documenta o login com `email="*"` enquanto a opt-in estiver ativa e a remoção das duas variáveis com restart após o login, sem valores reais; conferir ausência de segredo nos artefatos.
8. **Regressão e fecho (T008)**: gates do alvo PASS no handoff, rastreabilidade T001..T008 em decision/spec/plan/tasks e verificação da fronteira proibida (`validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`).

## Riscos e mitigação

- **Segundo caminho de autenticação**: mitigado porque o alias só existe quando a opt-in de bootstrap já está definida, usa a mesma seleção do startup e passa pelo mesmo `verify_or_dummy`; sem a opt-in, `*` é apenas um e-mail inválido (FR-001/FR-003/SC-002).
- **Conta errada autenticada**: mitigado pela ordem determinística (usuário ativo mais antigo) — a mesma de `bootstrap.rs:129-143` — que garante que a senha aplicada por 100 corresponda à conta resolvida pelo login (FR-002/SC-001). Se houver vários usuários ativos, o critério é documentado em `.env.example`.
- **Fallback acidental em typo**: um typo em `ADMIN_EMAIL` ou no `email` enviado é string diferente de `*`; segue a busca exata, falha e devolve o 401 genérico — sem seleção e sem escrita (FR-001/FR-002/SC-002).
- **Vazamento de credencial**: a senha permanece `skip_serializing` e com `Debug` redactado em `BootstrapConfig` (`bootstrap.rs:31,35-46`); nenhuma mensagem inclui senha, hash ou e-mail real, e o 401 não distingue os motivos (FR-003/SC-003).
- **Divergência de contrato de login**: rota, `verify_or_dummy`, 401, JWT, TTL, rate limit `PasswordLogin` e fluxo OIDC não mudam — apenas a resolução do identificador quando a opt-in está ativa (FR-003/SC-001).
- **Regressão do bootstrap/OIDC**: `POST /bootstrap`, advisory lock, emissão de sessão e allowlist OIDC permanecem intocados (FR-004/SC-003).
- **Produção real**: nenhuma escrita ou login ao vivo é executado neste ciclo; o resultado depende do deploy e do DB reais **[NV]**.

## Entrega

Diff restrito a `flora-api/src/routes/auth.rs` (alias `*` sob opt-in), eventual helper compartilhado em `flora-core` para a seleção determinística, `flora-tests/src/contract/auth.rs` (contratos), `.env.example` e artefatos do ciclo 101. Verificação local cobre T001–T008 e os gates do alvo (PASS conforme handoff consolidado). DB de produção, login ao vivo, Deploy/Coolify, IdP externo e allowlist OIDC permanecem **[NV]**.
