# Tasks: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

**Input**: `specs/100-admin-recovery/spec.md`, `plan.md`,
`.specify/assessments/100-admin-recovery/decision.md`, diagnóstico determinístico do `applied = false`

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T008 concluídas; gate do alvo PASS
(registrado no handoff consolidado).

## Phase 1 — Diagnóstico

- [X] T001 Fixar em `.specify/assessments/100-admin-recovery/{intake,research}.md`
  a cadeia determinística do `applied = false`: organizações pré-existentes
  (`GET /api/v1/bootstrap/status` com `available = false`,
  `flora-tests/src/contract/bootstrap.rs:441-483`), env presente com e-mail
  inexistente, `find_by_email` → `None` → aviso e `Ok(false)` sem escrita
  (`flora-core/src/config/bootstrap.rs:110-116`), e `applied` observável em
  `flora-api/src/state/app_state.rs:93-99`; registrar que o env correto é
  informação do relato, sem ler ou reproduzir credenciais (FR-002, SC-002).

## Phase 2 — Opt-in e seleção

- [X] T002 Aceitar o token literal `*` em `BootstrapConfig::validate()`
  (`flora-core/src/config/bootstrap.rs:57-81`) somente quando
  `FLORA_BOOTSTRAP__ADMIN_PASSWORD` estiver presente, comparando de forma exata
  após `trim()` (`:67`); manter a validação de e-mail para todo o resto, senha
  15..1024 (`:78`), `is_configured()` (`:44-49`), `Debug` redactado (`:31-42`) e
  `skip_serializing` (`:27`), sem nova chave de configuração (FR-001, SC-002).
- [X] T003 Selecionar o primeiro usuário ativo em `apply_startup_password()`
  (`bootstrap.rs:96-132`) quando `ADMIN_EMAIL=*`, via `SELECT id FROM users WHERE
  is_active = true ORDER BY created_at ASC, id ASC LIMIT 1`, usando o `id`
  retornado na sobrescrita; sem usuário ativo, aviso e `Ok(false)`; fora do
  opt-in, manter a busca exata `repo.find_by_email()` (`:111`) — typo ou e-mail
  inexistente → aviso e `applied = false`, sem **nenhum** fallback para a
  seleção (FR-001, FR-002, SC-002).

## Phase 3 — Escrita e observabilidade

- [X] T004 Manter a sobrescrita idempotente em `apply_startup_password()`:
  `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1`
  (`bootstrap.rs:120-128`), incondicional quanto a `password_hash`, com senha
  validada e hasheada via `PasswordService` (Argon2id) antes do UPDATE
  (`:78,119`); `rows_affected() > 0` → `applied = true`; reaplicações com a
  mesma senha não criam usuários, sessões nem efeitos colaterais;
  `set_password_hash_if_absent` (`user_repository.rs:93-110`) permanece fora da
  via de recuperação (FR-003, SC-001).
- [X] T005 Manter o resultado observável sem segredo: `app_state.rs:93-99` com
  `let applied = ...` seguido de `tracing::info!(applied = applied, "Startup
  administrator password recovery")`; garantir aviso para e-mail inexistente e
  nenhum usuário ativo (`bootstrap.rs:112-116`) e registro da sobrescrita via
  `user_id` (`:130`), sem senha, hash ou e-mail real em nenhuma mensagem
  (FR-004, SC-003).

## Phase 4 — Contratos

- [X] T006 Estender os testes: em `flora-tests/src/contract/bootstrap.rs`
  (`db_startup_password_overwrites_existing_hash`, `:340-407`), cobrir opt-in
  `*` com seleção e sobrescrita, e-mail exato inexistente com `applied = false`,
  typo sem fallback, nenhum usuário ativo e senha inválida; nos testes unitários
  de `BootstrapConfig` (`bootstrap.rs:135-169`), cobrir aceitação de `*` e
  rejeição de valores parecidos, sem segredo em `Debug`; em
  `flora-tests/src/contract/auth.rs` (`db_password_login`, `:1104-1173`),
  confirmar 200/sessão com a senha recuperada e 401 genérico inalterado para
  senha errada, usuário sem hash e conta inativa; nenhum segredo em log ou
  resposta (FR-005, FR-004, SC-001, SC-002, SC-003).

## Phase 5 — Fecho

- [X] T007 Documentar em `.env.example:32-39` `FLORA_BOOTSTRAP__ADMIN_EMAIL=*`
  como opt-in explícito, com o critério "primeiro usuário ativo", a instrução
  de remover ambas as variáveis e reiniciar após a recuperação, e o alerta de
  que um typo **não** aciona a seleção, sem valores reais; conferir que nenhuma
  credencial aparece em `Debug`, serialização, resposta de erro, mensagem de
  teste ou artefato do ciclo (FR-005, SC-003).
- [X] T008 Consolidar os gates do alvo (PASS no handoff), fechar a rastreabilidade
  T001..T008 em decision/spec/plan/tasks e confirmar que nenhum arquivo de
  `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`
  foi tocado; OIDC, `POST /bootstrap`, advisory lock e rate limit permanecem
  inalterados, e DB de produção, login ao vivo e IdP externo permanecem **[NV]**
  (FR-005, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008. T002 depende de T001;
T003 de T002; T004 de T003; T005 de T004; T006 depende de T002–T005; T007
depende de T002; T008 verifica T001–T007 em conjunto e consolida gates e rastro
documental. Em nenhum passo são alterados o contrato do `POST /bootstrap`, o
401 genérico, o JWT, o rate limit ou o fluxo OIDC. O token `*` e a senha são
temporários e devem ser removidos após a recuperação. DB de produção, login ao
vivo, Deploy/Coolify, IdP externo e allowlist OIDC permanecem **[NV]**.
