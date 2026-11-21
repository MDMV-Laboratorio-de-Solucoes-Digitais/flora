# Implementation Plan: 098 — recuperação da senha do administrador bootstrap via env

**Branch**: `feature/098-password-recovery` @ `76f936e245f70a7bd2735887e7ff8bc5e0c5c06f` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/098-password-recovery/decision.md` (go) e diagnóstico determinístico do 401 em produção (096)

## Summary

Usar as próprias variáveis `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
`FLORA_BOOTSTRAP__ADMIN_PASSWORD` — já configuradas em produção — como
credenciais temporárias de recuperação: com ambas presentes, o hash do
administrador é sobrescrito a cada startup, inclusive sobre uma senha existente,
e o booleano `applied` é registrado em log sem segredo. As variáveis devem ser
removidas após a recuperação. A idempotência antiga, o contrato do
`POST /bootstrap`, o 401 genérico, o JWT, o rate limit e o fluxo OIDC não mudam.
DB de produção, login ao vivo e IdP externo **[NV]**.

## Technical Context

**Root cause**: a conta de produção tem `password_hash` não nulo gravado pelo
bootstrap de 096 (`flora-api/src/routes/bootstrap.rs:294-295`). A recuperação
anterior só agia com hash nulo e o chamador descartava o retorno; o login
comparava com o hash antigo e devolvia 401 genérico
(`flora-api/src/routes/auth.rs:451-460`).

**Target Path**: `flora-core/src/config/bootstrap.rs` (config sem chave nova,
validação, UPDATE incondicional e logs), `flora-core/src/config/mod.rs` (Figment
já suporta as variáveis), `flora-core/src/repositories/user_repository.rs`
(helper `set_password_hash_if_absent` preservado, fora da via de recuperação),
`flora-api/src/state/app_state.rs` (log de `applied`),
`flora-tests/src/contract/{bootstrap,auth}.rs` (contratos), `.env.example`
(documentação operacional) e artefatos do ciclo 098.

**Testing**: contratos de startup (sem env / e-mail ausente / hash pré-existente
sobrescrito / sem hash / nenhum segredo em log e resposta) e contratos de login
(200 com a senha recuperada; 401 inalterado para senha errada, sem hash e conta
inativa); gates do alvo (PASS no handoff). Ambiente remoto/IdP **[NV]**.

**Constraints**: nenhuma chave de config nova; senha redactada/`skip_serializing`;
nenhum segredo em log/resposta/teste; variáveis temporárias removidas após o
uso; sem alterar OIDC, `POST /bootstrap`, advisory lock, JWT ou rate limit; sem
toques em `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md` ou
`PROGRESSO*`.

## Steps

1. **Causa (T001)** — fixar em intake/research a cadeia determinística do 401 e
   a ausência de sinal nos logs.
2. **Chave (T002)** — confirmar `ADMIN_EMAIL` + `ADMIN_PASSWORD` como credenciais
   de recuperação, com redaction, validação 15..1024 e nenhuma chave adicional.
3. **Sobrescrita (T003)** — `apply_startup_password` com UPDATE incondicional
   (`bootstrap.rs:120-128`), senha validada e hasheada antes da escrita.
4. **Observabilidade (T004)** — `app_state.rs:96-99` loga `applied`;
   `bootstrap.rs:111-115,130-132` registram e-mail ausente e sobrescrita sem
   segredo.
5. **Contratos de startup (T005)** — estados de env/e-mail/hash e ausência de
   segredo, ajustando o contrato antigo de idempotência à nova semântica.
6. **Contratos de login (T006)** — 200 com a senha recuperada; 401 genérico
   preservado para senha errada, sem hash e conta inativa.
7. **Env operacional (T007)** — `.env.example:32-39` documenta as duas variáveis
   como temporárias, com instrução de remoção; checar redaction.
8. **Regressão e fecho (T008)** — gates do alvo PASS, rastreabilidade T001..T008
   e verificação da fronteira proibida.

## Constitution Check

Sem violações: nenhum required check, gate, contrato do `POST /bootstrap`, lock,
formato de 401, JWT, rate limit, workflow, ADR ou constituição é alterado; nenhum
dado de usuário ou credencial é exposto; as variáveis temporárias são removidas
após o uso e o diff fica restrito à recuperação e aos artefatos do ciclo 098.

## Project Structure

```text
.specify/assessments/098-password-recovery/{intake,research,define,shape,decision}.md
specs/098-password-recovery/
├── spec.md      # FR-001..FR-005 / SC-001..SC-003
├── plan.md      # T001..T008
└── tasks.md     # todas [X]
flora-core/src/config/bootstrap.rs        # ADMIN_EMAIL/ADMIN_PASSWORD + UPDATE incondicional + logs
flora-core/src/config/mod.rs              # Figment FLORA_ + split("__") (inalterado)
flora-core/src/repositories/user_repository.rs  # set_password_hash_if_absent (fora da recuperação)
flora-api/src/state/app_state.rs          # log de applied (sem _applied)
flora-tests/src/contract/bootstrap.rs     # contratos de startup
flora-tests/src/contract/auth.rs          # contratos de login
.env.example                              # variáveis temporárias + instrução de remoção
```

## Complexity Tracking

Nenhuma. São um UPDATE incondicional com hash via `PasswordService`, um log de
resultado, contratos e documentação operacional — todos verificáveis por teste e
inspeção, sem nova dependência, migração, chave de config ou superfície pública.
