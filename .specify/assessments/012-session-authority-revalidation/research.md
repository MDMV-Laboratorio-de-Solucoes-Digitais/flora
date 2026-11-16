# Research: 012 — Session Authority Revalidation & Contract Confidence

**Date:** 2026-11-16  
**Base:** `dev@598f08152` (cycle 011 merged)  
**Inputs:** code audit, `docs/research/2026-11-16-012-session-authority-international.md`, `docs/research/2026-11-15-012-frontend-api-contract-audit.md`, skills (`wshobson/agents@auth-implementation-patterns`, `mindrally/skills@jwt-security`, `lvtd-llc/skills@rust-sqlx-postgres-service`).

## Achados de implementação

- `flora-api/src/extractors/auth.rs` tinha três implementos duplicados de decodificação JWT (`UserContext`, `OrgContext`, `AuthExtractor`), sem consulta ao `sessions`. Todas as rotas protegidas usam `AuthExtractor` (org/workspace/channel/task/file/notification/search/auth select-org).
- `SessionService::validate_session(jti)` já retorna a linha; a validação de `is_active`, `expires_at`, `user_id` e `organization_id` era o elo que faltava.
- `AppState` já expõe `session_service`; `sessions.jti` possui índice; `find_by_jti` usa `fetch_optional` parametrizado.
- Testes unitários do extrator usavam `connect_lazy` e um JWT válido sem DB; precisavam ser adaptados. `flora-tests` já tem harness DB (`make_test_state`, `run_migrations`, `#[ignore]`, env `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`) e 5 testes select-org.
- `POST /auth/select-organization` e `/auth/refresh` já realizavam checagens parciais; o novo extrator as precede sem quebrá-las.
- WebSocket (`routes/websocket.rs`) ainda decodifica o JWT manualmente sem autoridade de sessão — fora do escopo HTTP deste ciclo (follow-up).

## Pesquisa externa

- OWASP/RFC: revogação de JWT exige estado (denylist/allowlist/status list); `jti` é a chave natural.
- Padrão Axum 0.8: `FromRequestParts` assíncrono com lookup de sessão no Postgres é aceitável para aplicação única; sem cache no início.
- Trade-offs: allowlist/session table = 1 leitura por request, revogação imediata; cache curto atrasa revogação e adiciona invalidação/availability — adiar.
- Erros: mensagens genéricas, log detalhado server-side.
- Contrato frontend: parse do corpo antes do 401; preservar status/código/campo; `lockSession()` centralizado.

## Contrato frontend↔API (audit dedicado)

`docs/research/2026-11-15-012-frontend-api-contract-audit.md` confirma:
1. 401 descartava corpo no `ApiClient` (corrigido neste ciclo);
2. logout nunca é chamado pelo frontend (follow-up de UX; o endpoint existe e a autoridade agora propaga revogação);
3. chat/arquivos/telemetry têm divergências de contrato (não-regressões; fora do slice 012, mas registradas para o ciclo 013+);
4. testes de contrato tasks/workspace/files/messaging são stubs `Ok(())`; sinal fraco.

## Decisão de escopo

Implementar autoridade de sessão no helper compartilhado + correção do 401 + testes DB determinísticos + CI `db-tests`; documentar follow-ups (WebSocket, cache, assignee membership, chat/files contracts).
