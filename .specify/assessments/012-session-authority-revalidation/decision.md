# Decision: 012 — Session Authority Revalidation & Contract Confidence (80/20)

- **Slug**: 012-session-authority-revalidation
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior, 2026-11-16) | specs/012-session-authority-revalidation/{spec,plan,tasks}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Sessão revogada/expirada/vinculada a outro usuário/org era aceita em rotas protegidas até o `exp` do JWT (`AuthExtractor`/`UserContext`/`OrgContext` só decodificavam; `research.md:9-10`), e o `ApiClient` descartava `code/message/body` no 401 (`intake.md:11`; `problem.md:3-4`). |
| Evidence strength | strong | Evidência determinística: 7 novos testes DB-backed (válida/revogada/expirada/ausente/user mismatch/org mismatch/nil-org) além dos 5 select-org, em Postgres 18 (`research.md:12`; decisão anterior:10), com síntese R1 e citações `arquivo:linha` (`research.md:5,24-30`). |
| Value vs. inaction | strong | Fecha a lacuna P0 mais crítica do Seed: sem a correção, logout/troca de org/`revoke_all_for_user` não propagam antes do `exp` e o contrato do ciclo 011 permanece incompleto (`intake.md:9-11`; `ROADMAP.md` P0). |
| Feasibility / appetite | strong | Helper compartilhado + `SessionService::validate_session(jti)` já existente, `sessions.jti` indexado e `find_by_jti` parametrizado; sem migração e sem novo infra, uma leitura indexada por request aceita para PME/VPS (`research.md:10-11,20`; `intake.md:22-23`). |
| Strategic fit | strong | Alinha `OBJETIVO.md`/`ROADMAP.md` P0 (segurança/tenancy, evidência determinística, VPS barato); base `dev@598f08152`, cycle 011 merged (PR #214, CI 6/6 green) (`intake.md:3-4,19`). |
| Risk posture | adequate | Fail-closed com erros genéricos 401/500 e log detalhado server-side; cache distribuído/rate limiting adiados; CI remoto/merge, smoke integrado e latência sob carga não medidos — NV (`intake.md:26`; decisão anterior:26-28). |

## Verdict & Rationale

Veredito histórico **`go` preservado** (decisão de 2026-11-16, base `dev@598f08152`, cycle 011 merged via PR #214 com CI 6/6 green). O scorecard converge com a decisão anterior: a lacuna é P0 e de segurança crítica, a correção é pontual (helper compartilhado em `flora-api/src/extractors/auth.rs` + consultas já indexadas em `sessions`), sem migração e sem novo infra, com evidência DB-backed determinística em Postgres 18 (7 testes novos + 5 select-org). Nada nos artefatos inverte a escolha. A ressalva de custo por request (uma leitura indexada, aceitável para PME/VPS) e os NVs — CI remoto/merge verificados apenas no PR, smoke integrado não re-executado (porta `5432` externa ocupada; PG18 descartável em porta alternativa para DB tests), latência sob carga não medida (sem cache) — reforçam o recorte 80/20 em vez de contestá-lo. A correção do 401 no `ApiClient` completa o contrato do ciclo 011 com risco mínimo.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas de performance/configuração seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Autoridade de sessão parcial — extratores validam apenas assinatura/expiração do JWT, sem consultar a linha `jti` no PostgreSQL, e o `ApiClient` lança `ApiError` antes de ler o corpo no 401 (`problem.md:3-4`).
- **Chosen approach**: Autoridade única de autenticação — helper `authenticate_bearer` em `flora-api/src/extractors/auth.rs` (JWT + sessão por `jti`: ativa, não expirada, `user_id`/`organization_id` coerentes, nil-org), usado por `AuthExtractor`, `UserContext` e `OrgContext`; erros genéricos 401/500 com log server-side; sem cache neste ciclo (`concept.md:4-10`; decisão anterior:13-15).
- **In scope / out of scope**:
  - **In (012)**: (1) helper `authenticate_bearer` com validação JWT + sessão (`jti`, ativa, não expirada, user/org coerentes); (2) erros genéricos 401/500, log detalhado server-side, `require_org()` preservado; (3) frontend com parse do corpo antes do 401, `ApiError` preservando `code/message/body` e `lockSession()`; (4) testes DB-backed em `flora-tests` + job CI `db-tests` ampliado (`contract::auth::db_`); (5) documentação/evidência (`docs/research/...`), validação/release, atualização de `OBJETIVO.md`/`ROADMAP.md` (decisão anterior:12-17).
  - **Out / follow-up**: WebSocket upgrade session authority (o extrator HTTP não cobre `/ws`); cache de sessão (Moka/Valkey), invalidação distribuída, rate limiting; remoção ampla de headers `X-User-ID`/`X-Organization-ID` e RBAC fino; contratos chat/arquivos/telemetry (audit 012 documenta), assignee membership validation em `POST /tasks`; OIDC/JWKS e2e real, `apps/web`, migrações aplicadas (decisão anterior:19-24).
- **Success metrics**: Sessão revogada/expirada/mismatch não autoriza mais rotas protegidas; prova determinística em PG18 com 7 novos testes DB-backed + 5 select-org no job `db-tests`; 401 preserva corpo estruturado sem vazar detalhes de revogação vs assinatura (`research.md:11-12,21-22`; `problem.md:6`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: revalidação no WebSocket upgrade — como estender a autoridade ao `/ws`?]; [NEEDS CLARIFICATION: quando medir e introduzir cache de sessão sem atrasar revogação?]; [NEEDS CLARIFICATION: remoção ampla dos headers de tenant e RBAC fino — qual sequência?]; [NEEDS CLARIFICATION: como fechar os contratos chat/arquivos/telemetry e a validação de assignee em `POST /tasks`?]. NV: CI remoto/merge, smoke integrado e latência sob carga (`research.md:14,29-30`; decisão anterior:26-28).
