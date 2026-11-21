# Intake: 096 — login local pt-BR para conta bootstrap

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/096-auth-login-ptbr@1910d1c7d` · `.specify/feature.json` → `specs/096-auth-login-ptbr`

## Sinal

Bug reportado: a conta `luis.granja@mdmv.com.br`, criada por
`POST /api/v1/bootstrap`, não consegue entrar. O único caminho de login é o OIDC,
cujos defaults apontam para `http://localhost:8080` com secret vazio
(`flora-core/src/config/oidc.rs:27-31`) e cuja allowlist extra de redirect nasce
vazia (`:38`). O frontend envia `${location.origin}/callback`
(`apps/frontend/src/routes/(auth)/+page.svelte:43-45`), que a checagem exata em
`flora-api/src/routes/auth.rs:101-116` rejeita com
`redirect_uri is not allowed`. Não há senha, hash ou rota de login local: o
bootstrap só pede `admin_email` (`bootstrap.rs:32-37`) e insere o usuário sem
credencial (`:147-158`).

## Evidência

```
flora-core/src/config/oidc.rs:27-31      issuer default http://localhost:8080
flora-core/src/config/oidc.rs:38         allowed_redirect_uris = Vec::new()
flora-api/src/routes/auth.rs:101-116     callback exato + 401/erro "redirect_uri is not allowed"
apps/frontend/src/routes/(auth)/+page.svelte:43-45  redirect_uri = origin/callback
flora-api/src/routes/bootstrap.rs:32-37  payload sem senha
flora-api/src/routes/bootstrap.rs:147-158 INSERT users sem coluna de senha
flora-core/src/models/user.rs:13-33      User sem password_hash
grep argon2/password_hash (core+api)     -> 0 serviço de senha (só validate_password)
.env.example:66-68                       ZITADEL_* comentados
apps/frontend/src/app.html:2             lang="en"
```

Registro completo: `.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`.

## Autorização

Modo autônomo. Decisão `go` registrada em
`.specify/assessments/096-auth-login-ptbr/decision.md` antes da implementação.
Escopo: fallback local com Argon2id + recuperação por env no startup + tela de
login pt-BR. IdP externo permanece **[NV]**.

## Fronteira

Rust (core/api), migrations SQL, config/startup, frontend (SvelteKit), lockfile e
artefatos do ciclo. Sem toques em `analyze/`, `converge/`, `validate/`,
`release/`, `.specify/memory/STATE.md` ou `PROGRESSO*`. Nenhuma mudança de gate,
workflow, ADR ou constituição.
