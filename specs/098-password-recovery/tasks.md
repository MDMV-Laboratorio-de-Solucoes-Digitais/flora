# Tasks: 098 — recuperação da senha do administrador bootstrap via env

**Input**: `specs/098-password-recovery/spec.md`, `plan.md`,
`.specify/assessments/098-password-recovery/decision.md`, diagnóstico determinístico do 401

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T008 concluídas; gate do alvo PASS
(registrado no handoff consolidado).

## Phase 1 — Diagnóstico

- [X] T001 Fixar em `.specify/assessments/098-password-recovery/{intake,research}.md`
  a cadeia determinística do 401: conta com hash gravado pelo bootstrap
  (`flora-api/src/routes/bootstrap.rs:294-295`), recuperação anterior limitada a
  `password_hash IS NULL` (`flora-core/src/repositories/user_repository.rs:93-110`),
  retorno descartado no startup e 401 genérico
  (`flora-api/src/routes/auth.rs:451-460`); registrar que o env correto é
  informação do relato, sem ler ou reproduzir credenciais (FR-001, SC-002).

## Phase 2 — Chave e escrita

- [X] T002 Confirmar que a recuperação é acionada exclusivamente por
  `FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` — já
  configuradas em produção — sem nenhuma chave adicional; manter
  `BootstrapConfig` (`flora-core/src/config/bootstrap.rs:21-42`) com
  `skip_serializing`, `Debug` redactado e `validate()` de presença/e-mail/senha
  15..1024 (`:57-81`), sem valores em mensagens de erro (FR-002, SC-002).
- [X] T003 Substituir a escrita condicional por um UPDATE incondicional em
  `apply_startup_password()` (`bootstrap.rs:117-128`):
  `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1`,
  validando e hasheando a senha via `PasswordService` (Argon2id) antes da
  escrita; `set_password_hash_if_absent` (`user_repository.rs:93-110`) permanece,
  mas fora da via de recuperação (FR-003, SC-001).

## Phase 3 — Observabilidade

- [X] T004 Manter o resultado observável em `flora-api/src/state/app_state.rs:96-99`:
  `let applied = ...` seguido de `tracing::info!(applied = applied, "Startup
  administrator password recovery")`; garantir aviso para e-mail inexistente
  (`bootstrap.rs:111-115`) e registro da sobrescrita (`:130-132`), sem senha,
  hash ou dados pessoais extras em nenhuma mensagem (FR-001, SC-002).

## Phase 4 — Contratos

- [X] T005 Ajustar `flora-tests/src/contract/bootstrap.rs` à nova semântica:
  sem env → `applied = false` e sem escrita; e-mail inexistente → aviso e
  `applied = false`; conta com hash pré-existente → hash substituído; conta sem
  hash → hash gravado; e nenhuma senha em log ou resposta. O contrato anterior
  de idempotência (`db_startup_password_applies_only_to_users_without_hash`) é
  atualizado para refletir a sobrescrita (FR-003, FR-004, SC-001, SC-002).
- [X] T006 Em `flora-tests/src/contract/auth.rs` (`db_password_login`,
  `:1039-1169`), confirmar que `POST /api/v1/auth/password/login` com a senha
  recuperada retorna 200 com sessão no formato OIDC e que o 401 genérico permanece
  idêntico para senha errada, usuário sem hash e conta inativa (`:1131,1156,1169`);
  classe de rate limit `PasswordLogin` inalterada (FR-004, SC-001, SC-003).

## Phase 5 — Fecho

- [X] T007 Documentar em `.env.example:32-39` `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
  `FLORA_BOOTSTRAP__ADMIN_PASSWORD` como credenciais temporárias, com instrução
  de remover ambas e reiniciar após a recuperação, sem valores reais; conferir
  que nenhuma credencial aparece em `Debug`, serialização, resposta de erro,
  mensagem de teste ou artefato do ciclo (FR-002, FR-005, SC-002).
- [X] T008 Consolidar os gates do alvo (PASS no handoff), fechar a rastreabilidade
  T001..T008 em decision/spec/plan/tasks e confirmar que nenhum arquivo de
  `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`
  foi tocado; OIDC, `POST /bootstrap`, advisory lock e rate limit permanecem
  inalterados, e DB de produção, login ao vivo e IdP externo permanecem **[NV]**
  (FR-005, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008. T002 depende de T001;
T003 de T002; T004 de T003; T005 depende de T002–T004; T006 depende de T003 e
T005; T007 depende de T002; T008 verifica T001–T007 em conjunto e consolida gates
e rastro documental. Em nenhum passo são alterados o contrato do
`POST /bootstrap`, o 401 genérico, o JWT, o rate limit ou o fluxo OIDC. As
variáveis `ADMIN_EMAIL`/`ADMIN_PASSWORD` são temporárias e devem ser removidas
após a recuperação. DB de produção, login ao vivo, Deploy/Coolify, IdP externo e
allowlist OIDC permanecem **[NV]**.
