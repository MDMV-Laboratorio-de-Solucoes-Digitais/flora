# Release: 012 — Session Authority Revalidation & Contract Confidence (80/20)

**Date:** 2026-11-16  
**Target:** `dev` (PR from `feat/012-session-authority-revalidation`)  
**SemVer proposal:** dev-line increment; sem mudança de schema, sem breaking HTTP (rotas protegidas passam a exigir sessão ativa — mudança de segurança intencional).

## Deliveries

- **Session authority (P0):** `authenticate_bearer` valida o JWT e a sessão `jti` no PostgreSQL em `AuthExtractor`/`UserContext`/`OrgContext` (ativa, não expirada, user/org coerentes, nil-org suportado). Revogação/expiração passa a valer no próximo request protegido.
- **Erros seguros:** 401 genérico para token/sessão inválidos; 500 genérico para indisponibilidade do banco; detalhes apenas em `tracing`.
- **Frontend 401:** corpo estruturado preservado no `ApiError` (`code`/`message`/`field`/`body`) antes do `lockSession()`; fallback `unauthorized`.
- **Cobertura determinística:** 7 testes DB-backed em Postgres 18 (válida/revogada/expirada/ausente/user mismatch/org mismatch/nil-org); `db-tests` CI executa `contract::auth::db_`.
- **Documentação:** research internacional + audit frontend↔API; `OBJETIVO.md`/`ROADMAP.md` atualizados.

## Evidence

- `.specify/validations/012-session-authority-revalidation/validate.md`
- Local: fmt/check/clippy/testes Rust verdes; DB tests **12 passed** em PG18.7 descartável (porta alternativa; NV 5432); frontend check/lint/test:unit(server, 17)/build verdes.

## Traceability

- Atende `ROADMAP.md` P0 012: session authority, contrato de erro frontend, cobertura DB, sem ampliar produto.
- Follow-ups documentados: WebSocket upgrade, cache de sessão, assignee membership em `POST /tasks`, contratos chat/arquivos/telemetry.

## Rollback

- Reverter o merge do PR em `dev`. Sem migrações/dados irreversíveis; o comportamento anterior (confiança só no JWT) retorna.

## Risks / NV

- CI remoto/merge pendentes no PR; smoke integrado não re-executado neste ciclo (fluxo inalterado).
- Revogação no WebSocket só após follow-up; HTTP protegido já coberto.
