# Implementation Plan: 096 — login local pt-BR para conta bootstrap

**Branch**: `feature/096-auth-login-ptbr` @ `1910d1c7d` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/096-auth-login-ptbr/decision.md` (go) e
`.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`

## Summary

Acrescentar um fallback de autenticação local: senha Argon2id no bootstrap e em
uma rota de login com rate limit, recuperação idempotente por env no startup e
tela "Sign In" em pt-BR. O fluxo OIDC existente permanece intacto; IdP externo
**[NV]**.

## Technical Context

**Root cause**: defaults OIDC em `http://localhost:8080`
(`flora-core/src/config/oidc.rs:27-31`), `allowed_redirect_uris` vazio (`:38`),
redirect do frontend fora do callback aceito
(`flora-api/src/routes/auth.rs:101-116` + `+page.svelte:43-45`) e conta bootstrap
sem senha (`bootstrap.rs:32-37,147-158`).

**Target Path**: `flora-core/src/migrations/sql/`, `models/user.rs`,
`repositories/user_repository.rs`, `PasswordService` em `flora-core`,
`routes/bootstrap.rs`, `routes/auth/password/login`, middleware de rate limit,
config de startup e `apps/frontend` (tela + `app.html`).

**Testing**: testes de hash/verificação, bootstrap transacional, endpoint e
recuperação por env; testes e `npm run check` do frontend; gates do alvo (PASS
no handoff). Ambiente remoto/IdP **[NV]**.

**Constraints**: 401 genérico, dummy hash, sem credencial em log/resposta,
colunas nullable, sem alterar OIDC/gates/workflows/ADRs; sem toques em
`analyze/`, `converge/`, `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Steps

1. **Deps e dados** — declarar crates de password hashing (T001); migration com
   `password_hash`/`password_updated_at` (T002); modelo e repositório (T003).
2. **PasswordService** — Argon2id `m=19456,t=2,p=1`, PHC, dummy hash, limpeza de
   segredos (T004) e testes (T005).
3. **Bootstrap** — exigir `admin_password` e gravar o hash na transação (T006)
   com testes de rejeição e atomicidade (T007).
4. **Login local** — rota `POST /api/v1/auth/password/login` com token/sessão e
   rate limit (T008), testes de sucesso/falha genérica (T009).
5. **Recuperação** — env no startup só com hash NULL, idempotente (T010) e
   testes (T011).
6. **Frontend** — formulário pt-BR (T012), `lang="pt-BR"` e testes (T013).
7. **Fecho** — lockfile/supply-chain/gates e rastreabilidade T001..T014 (T014).

## Constitution Check

Sem violações: nenhum required check, gate, contrato OIDC, migração destrutiva,
workflow, ADR ou constituição é alterado; credenciais nunca são expostas; o diff
fica restrito ao fallback e aos artefatos do ciclo 096.

## Project Structure

```text
.specify/bugs/oidc-redirect-uri-not-allowed/bug.md
.specify/assessments/096-auth-login-ptbr/{intake,research,define,shape,decision}.md
specs/096-auth-login-ptbr/
├── spec.md      # FR-001..FR-008 / SC-001..SC-003
├── plan.md      # T001..T014
└── tasks.md     # todas [X]
flora-core/src/migrations/sql/          # nova migration (password_hash)
flora-core/src/models/user.rs           # campo opcional
flora-core/src/repositories/user_repository.rs
flora-core/src/.../password_service.rs  # Argon2id + PHC + dummy hash
flora-api/src/routes/bootstrap.rs       # admin_password na transação
flora-api/src/routes/auth.rs            # rota de login local
flora-api/src/middleware/rate_limit.rs  # classe de login local
flora-core/src/config/                  # recuperação por env no startup
apps/frontend/src/routes/(auth)/+page.svelte
apps/frontend/src/app.html              # lang="pt-BR"
Cargo.lock                              # novas crates
```

## Complexity Tracking

Nenhuma. São duas colunas nullable, um serviço de hash isolado, uma rota com
rate limit, uma recuperação condicional e uma tela local — todas verificáveis
por teste e inspeção.
