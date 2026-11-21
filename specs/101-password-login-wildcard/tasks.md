# Tasks: 101 — login com `*` quando a opt-in de bootstrap está ativa

**Input**: `specs/101-password-login-wildcard/spec.md`, `plan.md`,
`.specify/assessments/101-password-login-wildcard/decision.md`, diagnóstico determinístico do 401 no login

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T008 concluídas; gate do alvo PASS
(registrado no handoff consolidado).

## Phase 1 — Diagnóstico

- [X] T001 Fixar em `.specify/assessments/101-password-login-wildcard/{intake,research}.md`
  a cadeia determinística do 401: opt-in `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` ativa e
  hash aplicado ao primeiro usuário ativo (`applied = true`,
  `flora-api/src/state/app_state.rs:93-99`), mas `password_login` normaliza
  `req.email` e faz `find_by_email` exata
  (`flora-api/src/routes/auth.rs:451-453`); `email="*"` → `None` →
  `verify_or_dummy` → `Error::InvalidCredentials` (`:455-460`). O e-mail real é
  desconhecido de quem opera; nenhuma credencial é lida ou reproduzida
  (FR-002, SC-002).

## Phase 2 — Opt-in e seleção

- [X] T002 Aceitar `email == "*"` em `POST /api/v1/auth/password/login` somente
  quando `state.config.bootstrap.admin_email` (carregado de
  `FLORA_BOOTSTRAP__ADMIN_EMAIL` via `flora-core/src/config/mod.rs:46-52`) for
  exatamente `*` após `trim()`, reusando a definição de
  `is_wildcard_selector()` (`flora-core/src/config/bootstrap.rs:56-60`). Sem a
  opt-in, manter a busca exata e o 401 genérico; variantes (`*@x.com`, `foo*`,
  whitespace) não ativam; nenhuma chave de configuração nova (FR-001, SC-002).
- [X] T003 Com a opt-in e `email="*"`, resolver o primeiro usuário ativo via
  `SELECT ... FROM users WHERE is_active = TRUE ORDER BY created_at ASC, id ASC
  LIMIT 1` (`flora-core/src/config/bootstrap.rs:129-143`), a mesma consulta da
  recuperação de 100; usar esse registro no restante do fluxo. Sem usuário
  ativo → 401 genérico, sem escrita e sem fallback; fora do alias, manter
  `repo.find_by_email()` (`auth.rs:453`) inalterado. A seleção do login não pode
  divergir da seleção do startup (FR-001, FR-002, SC-001, SC-002).

## Phase 3 — 401, sessão e segredo

- [X] T004 Preservar o contrato de falha e sucesso: `verify_or_dummy`
  (`flora-core/src/services/password_service.rs:75-86`) e
  `Error::InvalidCredentials` (401 `invalid_credentials`, `auth.rs:455-460`)
  para senha errada, usuário sem hash, conta inativa, nenhum usuário ou alias
  sem opt-in — sem distinção de motivo; no sucesso, a resposta idêntica em
  formato OIDC (`session_token`, `expires_in`, `user_id`, `next_step`),
  JWT/TTL (`auth.rs:463-491`) e classe de rate limit `PasswordLogin`
  (`flora-api/src/middleware/rate_limit.rs:136,151,195`). OIDC, `POST /bootstrap`
  e emissão de sessão permanecem inalterados (FR-003, SC-001).
- [X] T005 Garantir que nenhuma senha, hash ou e-mail real aparece em log,
  resposta, erro ou mensagem de teste; que o alias não gera estado nem log novo
  além da sessão usual e não se torna um segundo método de autenticação; e que
  `Debug` redactado e `skip_serializing` de `BootstrapConfig`
  (`bootstrap.rs:31,35-46`) permanecem (FR-003, SC-003).

## Phase 4 — Contratos

- [X] T006 Estender `flora-tests/src/contract/auth.rs` (`mod db_password_login`,
  `:1039-1183`) com: opt-in `*` e senha configurada → 200 e sessão do primeiro
  usuário ativo; sem opt-in → 401 genérico; variantes (`*@x.com`, `foo*`,
  whitespace) → 401; senha errada com `*` → 401 idêntico ao de usuário
  desconhecido; nenhum usuário ativo → 401; conta inativa e usuário sem hash →
  401 inalterados; formato de sessão preservado; nenhum segredo em log ou
  resposta. Não substituir os contratos existentes — apenas acrescentar os
  cenários do alias (FR-004, FR-003, SC-001, SC-002, SC-003).

## Phase 5 — Fecho

- [X] T007 Documentar em `.env.example:36-45` que, enquanto
  `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` estiver ativa, `POST /api/v1/auth/password/login`
  pode usar `email="*"` com a senha temporária, e que ambas as variáveis devem
  ser removidas com restart após o login; alertar que um typo **não** ativa o
  alias e não incluir valores reais; conferir que nenhuma credencial aparece em
  `Debug`, serialização, resposta de erro, mensagem de teste ou artefato do
  ciclo (FR-004, SC-003).
- [X] T008 Consolidar os gates do alvo (PASS no handoff), fechar a rastreabilidade
  T001..T008 em decision/spec/plan/tasks e confirmar que nenhum arquivo de
  `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*` ou
  `AGENTS.md` foi tocado; OIDC, `POST /bootstrap`, advisory lock, JWT e rate
  limit permanecem inalterados, e DB de produção, login ao vivo e IdP externo
  permanecem **[NV]** (FR-004, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008. T002 depende de T001;
T003 de T002; T004 de T003; T005 de T004; T006 depende de T002–T005; T007
depende de T002; T008 verifica T001–T007 em conjunto e consolida gates e rastro
documental. Em nenhum passo são alterados o contrato do `POST /bootstrap`, o
401 genérico, o JWT, o rate limit, a emissão de sessão ou o fluxo OIDC. O alias
`*` e a senha são temporários e devem ser removidos após o login. DB de
produção, login ao vivo, Deploy/Coolify, IdP externo e allowlist OIDC permanecem
**[NV]**.
