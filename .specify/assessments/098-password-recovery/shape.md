# Shape: 098 — recuperação da senha do administrador bootstrap via env

**Data:** 2026-11-21 · **Branch**: `feature/098-password-recovery@76f936e245f70a7bd2735887e7ff8bc5e0c5c06f`

## Forma escolhida

Usar as próprias credenciais já configuradas como chave de recuperação
temporária: com `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
`FLORA_BOOTSTRAP__ADMIN_PASSWORD` presentes, o hash é sobrescrito a cada startup,
inclusive sobre uma senha existente. O resultado é logado sem segredo e a
documentação orienta a remover as variáveis após a recuperação. Nenhuma chave de
configuração nova é introduzida.

1. **Causa e rastro (T001)**: fixar em intake/research a cadeia do 401 — conta
   com hash gravado pelo bootstrap, recuperação anterior condicionada a
   `password_hash IS NULL`, retorno descartado e 401 genérico no login.
2. **Chave existente (T002)**: confirmar que a recuperação é acionada por
   `ADMIN_EMAIL` + `ADMIN_PASSWORD` (`bootstrap.rs:21-42,57-81`), com senha
   redactada/`skip_serializing` e validação 15..1024 — sem nova variável.
3. **Sobrescrita (T003)**: `apply_startup_password()` passou a usar
   `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1`
   (`bootstrap.rs:117-128`), incondicional, com hash via `PasswordService`;
   `set_password_hash_if_absent` não é a via de recuperação.
4. **Observabilidade (T004)**: `app_state.rs:96-99` registra `applied` com
   "Startup administrator password recovery"; `bootstrap.rs:111-115,130-132`
   registra e-mail ausente e sobrescrita, sem senha nem hash.
5. **Contratos de startup (T005)**: estender `flora-tests/src/contract/bootstrap.rs`
   com sem env, e-mail ausente, hash pré-existente sobrescrito, sem hash e
   ausência de segredo em log/resposta, ajustando o contrato antigo de
   idempotência à nova semântica.
6. **Contratos de login (T006)**: em `flora-tests/src/contract/auth.rs`
   (`db_password_login`), confirmar 200/sessão com a senha recuperada e 401
   genérico inalterado para senha errada, sem hash e conta inativa
   (`:1131,1156,1169`).
7. **Env operacional (T007)**: `.env.example:32-39` documenta as duas variáveis
   como temporárias, com instrução de remover ambas e reiniciar após a
   recuperação; conferir redaction em todo o fluxo.
8. **Regressão e fecho (T008)**: gates do alvo PASS no handoff, rastreabilidade
   T001..T008 em decision/spec/plan/tasks e verificação da fronteira proibida
   (`validate/`, `release/`, `STATE.md`, `PROGRESSO*`).

## Riscos e mitigação

- **Reaplicação da senha temporária**: mitigada pela documentação explícita em
  `.env.example:32-39` — remover ambas as variáveis e reiniciar após o login;
  enquanto estiverem definidas, o hash é sobrescrito em cada startup por design
  (FR-002/FR-003/SC-001).
- **Vazamento de credencial**: senha com `skip_serializing` e `Debug` redactado,
  validação e hash antes da escrita; nenhuma mensagem de log inclui senha ou
  hash (FR-001/FR-002/SC-002).
- **Silêncio operacional**: o retorno não é mais descartado — `applied` é logado
  no startup e os motivos de skip/sobrescrita são registrados sem segredo
  (FR-001/SC-002).
- **Divergência de contrato de login**: rota, `verify_or_dummy`, 401 genérico,
  JWT e rate limit não mudam — apenas o hash armazenado passa a corresponder à
  senha do env (FR-004/SC-001).
- **Regressão do bootstrap/OIDC**: `POST /bootstrap`, advisory lock, emissão de
  sessão e allowlist OIDC permanecem intocados (FR-005/SC-003).
- **Produção real**: nenhuma escrita ou login ao vivo é executado neste ciclo;
  o resultado depende do deploy e do DB reais **[NV]**.

## Entrega

Diff restrito a `flora-core/src/config/bootstrap.rs`,
`flora-api/src/state/app_state.rs`, contratos `flora-tests`, `.env.example` e
artefatos do ciclo 098. Verificação local cobre T001–T008 e os gates do alvo
(PASS conforme handoff consolidado). DB de produção, login ao vivo,
Deploy/Coolify, IdP externo e allowlist OIDC permanecem **[NV]**.
