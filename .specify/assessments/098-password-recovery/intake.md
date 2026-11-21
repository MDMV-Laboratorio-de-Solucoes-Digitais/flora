# Intake: 098 — recuperação da senha do administrador bootstrap via env

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/098-password-recovery@76f936e245f70a7bd2735887e7ff8bc5e0c5c06f` · `.specify/feature.json` → `specs/098-password-recovery`

## Sinal

Relato de produção (096): o login local com `luis.granja@mdmv.com.br` em
`POST /api/v1/auth/password/login` retorna **401**, embora
`FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` estejam
presentes e corretos no ambiente. Os logs não mostravam nenhum sinal da
recuperação — o filtro suprime `flora_core` e o chamador descartava o resultado.
O diagnóstico é determinístico: a recuperação anterior só agia quando
`password_hash IS NULL`; a conta pré-existente, criada pelo próprio
`POST /bootstrap` de 096 (que já grava o hash), era ignorada — logo a senha do
env nunca era aplicada e o 401 era o resultado esperado do código, não uma falha
intermitente.

## Evidência

```
DIAGNÓSTICO (código que produzia o 401):
flora-api/src/routes/bootstrap.rs:294-295  POST /bootstrap valida e hasheia admin_password na transação
flora-core/src/config/bootstrap.rs (antes)  password_hash.is_some() -> Ok(false) sem log
flora-core/src/repositories/user_repository.rs:93-110
                                            set_password_hash_if_absent: ... WHERE password_hash IS NULL
flora-api/src/routes/auth.rs:451-460        verify_or_dummy; 401 InvalidCredentials idêntico para
                                            senha errada, sem hash e conta inativa
flora-api/src/state/app_state.rs (antes)    let _applied = ...;  resultado descartado

IMPLEMENTAÇÃO ATUAL (recuperação):
flora-core/src/config/bootstrap.rs:1-7      doc: ADMIN_EMAIL + ADMIN_PASSWORD são credenciais
                                            temporárias que sobrescrevem o hash a CADA startup
flora-core/src/config/bootstrap.rs:21-42    BootstrapConfig: admin_email/admin_password,
                                            skip_serializing + Debug [REDACTED]
flora-core/src/config/bootstrap.rs:57-81    validate(): presença, e-mail e senha 15..1024, sem segredo em erro
flora-core/src/config/bootstrap.rs:96-134   apply_startup_password(): com ambas as vars e e-mail
                                            existente, UPDATE incondicional do hash (:120-128),
                                            inclusive com hash pré-existente; info/warn sem senha
flora-api/src/state/app_state.rs:96-99      let applied = ...; tracing::info!(applied, "Startup administrator password recovery")
flora-api/src/middleware/rate_limit.rs:150,195
                                            classe PasswordLogin (5/60s) — rota alcançável
.env.example:32-39                          documenta ADMIN_EMAIL/ADMIN_PASSWORD como TEMPORÁRIAS,
                                            com instrução de remover após a recuperação
```

Nenhuma credencial real foi lida, registrada ou transmitida nesta investigação.
A correção do env é a informação do relato; os valores secretos não são
reproduzidos neste artefato.

## Autorização

Modo autônomo. Decisão `go` registrada em
`.specify/assessments/098-password-recovery/decision.md` antes da implementação.
Escopo: as próprias variáveis `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
`FLORA_BOOTSTRAP__ADMIN_PASSWORD` — já configuradas em produção — passam a
sobrescrever o hash a cada startup, inclusive sobre credencial existente, com o
resultado registrado em log e sem segredo. As variáveis são temporárias e devem
ser removidas após a recuperação. Rota de login, contrato do `POST /bootstrap`,
rate limit e fluxo OIDC permanecem inalterados. O DB real de produção e o login
ao vivo permanecem **[NV]**.

## Fronteira

Backend Rust (`flora-core/src/config/bootstrap.rs`, `flora-api/src/state/app_state.rs`,
repositórios/contratos conforme o ciclo), documentação de env (`.env.example`) e
artefatos do ciclo. Sem toques em `validate/`, `release/`,
`.specify/memory/STATE.md` ou `PROGRESSO*`. Nenhuma mudança de gate, workflow,
ADR ou constituição. Nenhuma senha real em log, resposta, teste ou artefato.
