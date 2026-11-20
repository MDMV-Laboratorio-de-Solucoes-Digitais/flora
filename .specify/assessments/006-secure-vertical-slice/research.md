# Research: 006 — Secure Vertical Slice (Auth/Tenancy + Core Flow)

**Date:** 2026-11-15  
**Status:** synthesized (background audits + best-practices research completed)

## Inputs
- `OBJETIVO.md`, `ROADMAP.md` (P0): auth/contexto inconsistente, falta de fatia e2e verificável, contrato frontend↔backend desalinhado.
- `docs/research/2026-11-15-macro-alternative-80-20-research.md`: alvo cheap-VPS (2 vCPU/4 GB), Postgres-first.
- Auditoria prévia + auditorias em background (cycle 006).

## Synthesized findings

### Backend auth/tenancy (evidence: route/extractor audits)
- **Antes:** rotas críticas (workspaces/channels/messages/tasks/files/search/notifications) confiavam em headers `X-User-ID`/`X-Organization-ID` (spoofáveis); org extraía user via decode manual de Bearer; WS aceitava headers primeiro e caía para JWT. `AuthExtractor` (JWT validado, `sub/org_id/email`, `x-workspace-id`) existia mas não era usado por handlers.
- **Risco central:** identidade/tenant derivados do cliente sem verificação criptográfica/membership consistente; `list_tasks` por assignee sem filtro de org; `create_channel` sem validar `workspace_id ∈ org`; update/delete de workspace sem checar tenancy do recurso.
- **Direção validada:** extrator central JWT → contexto verificado no servidor; membership + checagem de tenancy do recurso por request; não confiar em headers de tenant/identidade.

### Frontend↔backend contract (evidence: frontend audit)
- Fluxo OIDC/login não round-tripa `state`; callback emite JWT com `organization_id = nil` (`next_step=select_or_create_organization`) mas frontend ignora.
- Frontend envia apenas Bearer (quando presente), sem contexto org/workspace; APIs protegidas (antes header-based) e shapes divergem (org create sem `slug`, WS em `/ws` vs `/api/v1/ws`, auth WS por query/header, payloads/envelopes e campos distintos, sem listagem de canais, `/messages/sync` ausente, tasks UI estática e `GET /tasks` assignee-scoped).
- **Slice mais próximo (REST-first):** contexto org/workspace + tarefas; mensagens exigem também discovery/WS/contrato. E2E UI completo **NV** sem alinhamento maior de contrato/frontend.

### Best practices (high-level; Axum/SvelteKit/OWASP guidance)
- Derivar tenant de identidade verificada e validar autorização/membership em DB; usar extrator tipado para contexto; rejeitar credenciais inválidas explicitamente.
- Para browser, sessão/cookie HttpOnly+Secure+SameSite ou bearer em cookie; evitar localStorage para tokens sensíveis; SvelteKit `hooks.server` para resolver sessão.
- Realtime mínimo em VPS pequena: in-process/SSE quando possível, WS apenas se bidirecional; autenticar upgrade e validar membership por canal; backpressure/limites.
- Migração pragmática: convergir para contexto canônico, fallback dev apenas se explícito/configurado (não default exposto), depois remover.

## Applied in this cycle (implemented)
- `flora-api/src/extractors/auth.rs`: `AuthExtractor::require_org()` (rejeita org nil).
- Rotas refatoradas para **JWT Bearer via `AuthExtractor`** + membership/tenant checks: `routes/workspace.rs`, `routes/tasks.rs`, `routes/channels.rs` (inclui validação `workspace_id ∈ org` ao criar canal), `routes/org.rs`, `routes/files.rs`, `routes/search.rs`, `routes/notifications.rs`.
- `routes/websocket.rs`: extração de contexto **apenas por Bearer JWT verificado** (remove trust em headers `x-user-id/x-organization-id`), rejeita org nil.

## Remaining gaps / NV
- Token pós-seleção de org: callback ainda emite org `nil`; rotas que exigem org falham até existir mecanismo de escopo de token/sessão (fluxo de seleção não implementado neste slice).
- Contrato frontend↔backend e UI do fluxo núcleo continuam desalinhados (auth state, seleção org/workspace, endpoints/shapes/WS) — **E2E UI NV**.
- WS: sem validação de membership em subscribe/fanout completo (além de ownership de canal no send) — hardening adicional fora do escopo mínimo.
- Verificação runtime integrada (Postgres/Valkey/Meilisearch/OIDC) **NV** neste ambiente.
