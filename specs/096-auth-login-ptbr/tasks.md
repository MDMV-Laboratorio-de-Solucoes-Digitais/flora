# Tasks: 096 — login local pt-BR para conta bootstrap

**Input**: `specs/096-auth-login-ptbr/spec.md`, `plan.md`,
`.specify/assessments/096-auth-login-ptbr/decision.md`,
`.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T014 concluídas; gate do alvo PASS
(registrado no handoff consolidado).

## Phase 1 — Dependências e dados

- [X] T001 Declarar as dependências de hash de senha (`argon2` / `password-hash`
  com PHC) nos manifests Rust, mantendo as versões compatíveis com o workspace
  e sem alterar crates de autenticação já existentes (FR-002).
- [X] T002 Criar migration versionada em `flora-core/src/migrations/sql/`
  adicionando `users.password_hash` (TEXT, nullable) e
  `users.password_updated_at` (TIMESTAMPTZ, nullable), segura em banco vazio e
  populado, sem modificar colunas de contas OIDC (FR-001, SC-003).
- [X] T003 Estender `flora-core/src/models/user.rs` e
  `flora-core/src/repositories/user_repository.rs` com os campos de senha e a
  consulta por e-mail normalizado necessária ao login local, preservando
  `oidc_subject` e o fluxo OIDC (FR-001, FR-004).

## Phase 2 — PasswordService

- [X] T004 Implementar `PasswordService` com Argon2id `m=19456`, `t=2`, `p=1` e
  serialização PHC; `verify` usa dummy hash quando usuário/hash não existe e
  limpa segredos em memória (FR-002, SC-002).
- [X] T005 Testar `PasswordService`: hash/verify com senha correta, senha errada,
  usuário ausente e hash nulo, confirmando resultado booleano uniforme e PHC
  com parâmetros esperados (FR-002, SC-002).

## Phase 3 — Bootstrap transacional

- [X] T006 Exigir `admin_password` em `BootstrapRequest`, validar complexidade
  com `flora-core/src/utils/validation.rs:59-105` e gravar o hash Argon2id na
  mesma transação que cria organização, usuário, papel, associação, workspace e
  sessão — sem escrita parcial (FR-003, SC-003).
- [X] T007 Testar o bootstrap: rejeição sem `admin_password`, rejeição de senha
  fraca sem INSERT, hash persistido na conta criada e sessão emitida com o
  escopo correto (FR-003, SC-001, SC-003).

## Phase 4 — Login local e rate limit

- [X] T008 Implementar `POST /api/v1/auth/password/login` com e-mail e senha,
  verificação via `PasswordService`, `401` genérico idêntico em toda falha e
  emissão de `session_token`/`expires_in` equivalentes ao callback OIDC;
  adicionar classe de rate limit própria sem alterar as existentes (FR-002,
  FR-004, FR-006, SC-001, SC-002).
- [X] T009 Testar o endpoint: login válido com sessão aceita em rota
  autenticada, e-mail inexistente / senha errada / conta sem hash retornando o
  mesmo `401`, além do comportamento do rate limit (FR-004, FR-006, SC-001,
  SC-002).

## Phase 5 — Recuperação por env no startup

- [X] T010 Implementar a recuperação no startup por
  `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `FLORA_BOOTSTRAP__ADMIN_PASSWORD` via Figment
  (`FLORA_` + `__`), aplicando hash apenas quando a conta existir com
  `password_hash IS NULL`, de forma idempotente e sem logar a senha (FR-005,
  SC-002, SC-003).
- [X] T011 Testar a recuperação: conta sem hash recebe credencial, reinício com
  as mesmas env não sobrescreve o hash, e-mail inexistente ou env parcial não
  cria conta nem erro com credencial vazada (FR-005, SC-003).

## Phase 6 — Frontend em pt-BR

- [X] T012 Implementar o formulário "Sign In" local em pt-BR na tela de login,
  chamando `POST /api/v1/auth/password/login`, com mensagem genérica de erro e
  opção SSO preservada (FR-007, SC-002).
- [X] T013 Trocar `lang="en"` por `lang="pt-BR"` em
  `apps/frontend/src/app.html` e traduzir para pt-BR as strings da tela de
  login, incluindo erros e estados de envio; atualizar os testes frontend
  relacionados (FR-008).

## Phase 7 — Fecho

- [X] T014 Validar `Cargo.lock`/supply-chain das novas crates, executar os gates
  do alvo (PASS no handoff) e fechar a rastreabilidade T001..T014 em
  decision/spec/plan/tasks, confirmando que nenhum arquivo de `analyze/`,
  `converge/`, `validate/`, `release/`, `STATE.md` ou `PROGRESSO*` foi tocado
  (SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 →
T012 → T013 → T014. T005 depende de T004; T007 de T006; T009 de T008; T011 de
T010; T012/T013 podem seguir em paralelo após T009; T014 consolida lockfile,
gates e rastro documental. IdP externo e deploy remoto permanecem **[NV]**.
