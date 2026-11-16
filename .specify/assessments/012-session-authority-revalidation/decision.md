# Decision: 012 — Session Authority Revalidation & Contract Confidence (80/20)

**Date:** 2026-11-16  
**Verdict:** **go**

## Rationale
- Fecha a lacuna P0 mais crítica do Seed: sessão revogada/expirada/vinculada a outro usuário/org era aceita em rotas protegidas até o `exp` do JWT. A correção é pontual (helper compartilhado + consultas já indexadas), sem migração e sem novo infra.
- O custo por request (uma leitura indexada em `sessions`) é aceitável para PME/VPS e reverte a premissa de “JWT stateless” para os casos onde revogação importa.
- Correção do 401 no `ApiClient` completa o contrato do ciclo 011 com risco mínimo.
- Evidência determinística forte: 7 novos testes DB-backed (válida/revogada/expirada/ausente/user mismatch/org mismatch/nil-org) além dos 5 select-org, em Postgres 18.

## In-scope (012)
1. Helper `authenticate_bearer` em `flora-api/src/extractors/auth.rs` validando JWT + sessão (`jti`, ativa, não expirada, user/org coerentes), usado por `AuthExtractor`, `UserContext` e `OrgContext`.
2. Erros genéricos 401/500, log detalhado server-side; `require_org()` preservado.
3. Frontend: parse do corpo antes do 401, `ApiError` preservando `code/message/body` e `lockSession()`.
4. Testes DB-backed em `flora-tests` + job CI `db-tests` ampliado (`contract::auth::db_`).
5. Documentação/evidência: `docs/research/...`, validação/release, atualização de `OBJETIVO.md`/`ROADMAP.md`.

## Out of scope / follow-up
- WebSocket upgrade session authority (HTTP extractor não cobre `/ws`).
- Cache de sessão (Moka/Valkey), invalidação distribuída, rate limiting.
- Remoção ampla de headers `X-User-ID`/`X-Organization-ID` e RBAC fino.
- Contratos chat/arquivos/telemetry (audit 012 documenta), assignee membership validation em `POST /tasks`.
- OIDC/JWKS e2e real; `apps/web`; migrações aplicadas.

## NV
- CI remoto/merge verificados apenas no PR; smoke integrado não re-executado neste ciclo (porta `5432` externa ocupada; PG18 descartável em porta alternativa para DB tests).
- Tempo de latência sob carga não medido (sem cache; leitura indexada é o caminho padrão).
