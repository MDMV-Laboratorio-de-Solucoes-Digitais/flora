# Implementation Plan: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

**Branch**: `feature/100-admin-recovery` @ `9cf6775a7bd46395648ca3491536535373e0de6b` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/100-admin-recovery/decision.md` (go) e diagnóstico determinístico do `applied = false` em produção (após 098)

## Summary

Estender a recuperação de 098 com um opt-in explícito: `ADMIN_EMAIL=*` — um
token inválido como e-mail — seleciona o primeiro usuário ativo
(`is_active = true`, `ORDER BY created_at ASC, id ASC LIMIT 1`) e sobrescreve o
hash com a senha do env de forma idempotente. Fora do opt-in, a busca por e-mail
permanece exata e um typo nunca aciona a seleção. O resultado continua observável
via `applied`, sem segredo. Variáveis temporárias devem ser removidas após o
login. O 401 genérico, o JWT, o `POST /bootstrap`, o rate limit e o OIDC não
mudam. DB de produção, login ao vivo e IdP externo **[NV]**.

## Technical Context

**Root cause**: em produção há organizações (`status.available = false`), o env
aponta para um e-mail inexistente e
`apply_startup_password()` (`flora-core/src/config/bootstrap.rs:96-132`) faz
busca exata (`repo.find_by_email`, `:111`); sem correspondência, registra aviso
e retorna `Ok(false)` (`:112-116`). O e-mail real do administrador é
desconhecido de quem opera.

**Target Path**: `flora-core/src/config/bootstrap.rs` (token `*` em `validate()`,
seleção do primeiro usuário ativo, UPDATE incondicional, logs sem segredo),
`flora-api/src/state/app_state.rs` (log de `applied` — inalterado),
`flora-tests/src/contract/{bootstrap,auth}.rs` e testes unitários de
`BootstrapConfig` (contratos), `.env.example` (documentação operacional) e
artefatos do ciclo 100.

**Testing**: opt-in `*` com seleção e login 200; e-mail inexistente e typo sem
fallback (`applied = false`); nenhum usuário ativo; senha inválida; ausência de
segredo em log/resposta; 401 genérico preservado. Gates do alvo (PASS no
handoff). Ambiente remoto/IdP **[NV]**.

**Constraints**: nenhuma chave de config nova; somente o literal `*` ativa a
seleção; senha redactada/`skip_serializing`; nenhum segredo ou e-mail real em
log/resposta/teste; variáveis temporárias removidas após o uso; sem alterar
OIDC, `POST /bootstrap`, advisory lock, JWT ou rate limit; sem toques em
`analyze/`, `converge/`, `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Steps

1. **Causa (T001)** — fixar em intake/research a cadeia determinística:
   organizações existem, e-mail do env inexistente, `find_by_email` → `None`,
   aviso e `applied = false`, sem escrita.
2. **Opt-in (T002)** — `BootstrapConfig::validate()` aceita o token literal `*`
   somente com senha presente; todo o resto continua exigindo e-mail válido e
   senha 15..1024; `is_configured()` e redaction preservados.
3. **Seleção (T003)** — com `*`, consulta determinística do primeiro usuário
   ativo (`is_active = true`, `created_at ASC, id ASC LIMIT 1`); sem usuário
   ativo → aviso e `Ok(false)`; fora do opt-in, busca exata inalterada, sem
   fallback.
4. **Escrita (T004)** — UPDATE incondicional de `password_hash` e
   `password_updated_at` (`bootstrap.rs:120-128`), com hash via `PasswordService`
   antes da escrita; `rows_affected() > 0` → `applied = true`, idempotente.
5. **Observabilidade (T005)** — `app_state.rs:93-99` mantém `applied` em log;
   `bootstrap.rs:112-116,130` registra skip e `user_id`, sem senha, hash ou
   e-mail real.
6. **Testes (T006)** — contratos de bootstrap/auth e testes unitários: opt-in,
   seleção, typo sem fallback, nenhum usuário ativo, senha inválida, login 200 e
   401 genérico preservado, nenhum segredo em log/resposta.
7. **Env operacional (T007)** — `.env.example:32-39` documenta `*` como opt-in
   explícito, o critério de seleção e a remoção das variáveis após o login;
   checar redaction em todo o fluxo.
8. **Regressão e fecho (T008)** — gates do alvo PASS, rastreabilidade T001..T008
   e verificação da fronteira proibida.

## Constitution Check

Sem violações: nenhum required check, gate, contrato do `POST /bootstrap`, lock,
formato de 401, JWT, rate limit, workflow, ADR ou constituição é alterado; nenhum
dado de usuário ou credencial é exposto; o opt-in é explícito e as variáveis
temporárias são removidas após o uso; o diff fica restrito à recuperação e aos
artefatos do ciclo 100.

## Project Structure

```text
.specify/assessments/100-admin-recovery/{intake,research,define,shape,decision}.md
specs/100-admin-recovery/
├── spec.md      # FR-001..FR-005 / SC-001..SC-003
├── plan.md      # T001..T008
└── tasks.md     # todas [X]
flora-core/src/config/bootstrap.rs        # token '*', seleção do 1º ativo, UPDATE idempotente, logs
flora-core/src/config/mod.rs              # Figment FLORA_ + split("__") (inalterado)
flora-core/src/repositories/user_repository.rs  # set_password_hash_if_absent (fora da recuperação)
flora-api/src/state/app_state.rs          # log de applied (inalterado)
flora-tests/src/contract/bootstrap.rs     # contratos de startup / opt-in
flora-tests/src/contract/auth.rs          # contratos de login
.env.example                              # '*' como opt-in + remoção das variáveis
```

## Complexity Tracking

Nenhuma. São um token de validação, uma consulta determinística de seleção, um
UPDATE idempotente, logs, contratos e documentação operacional — todos
verificáveis por teste e inspeção, sem nova dependência, migração, chave de
configuração ou superfície pública.
