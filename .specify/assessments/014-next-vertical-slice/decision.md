# Decision: 014 — Next vertical slice (WS session authority + protocol)

- **Slug**: 014-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | 013 artifacts | docs/research/2026-11-16-013-websocket-session-authority.md | docs/research/2026-11-16-seed-vps-profile-audit.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `/ws` faz upgrade com JWT decode apenas (`routes/websocket.rs:100-141,281`), sem `validate_session(jti)`/active/expiry/revogação, enquanto HTTP já é DB-backed desde 012; FE alvo `/api/v1/ws?token=` vs backend `/ws` com header, envelopes divergentes (`problem.md:4,12-14`; `research.md:11-13`). |
| Evidence strength | adequate | Auditoria de código + pesquisa multilíngue (EN/pt-BR/es/de/fr/zh-CN; ja/ko/ru triangulados NV) com consenso: autenticar antes do upgrade, authority em DB por `jti`, transporte browser-safe, janela de revogação explícita (`research.md:4,20-30`; decisão anterior:38-46). |
| Value vs. inaction | strong | Fecha trust boundary inconsistente na mesma data plane org-scoped e destrava uso confiável de WS no browser; é pré-requisito de fanout (B) e evita construir push sobre base não autenticada (`intake.md:15-16`; decisão anterior:10-12). |
| Feasibility / appetite | adequate | Reaproveita `session_service`/helper de 012, sem novo serviço obrigatório; complexidade moderada em transporte de token no browser, janela de revogação e contrato FE (`intake.md:12-17`; decisão anterior:48-56). |
| Strategic fit | strong | ROADMAP P0.1/P0.2 e perfil Seed VPS 2 vCPU/4 GB; handshake-only ou revalidação esporádica com jitter cabe no orçamento, ao contrário de fanout stateful prematuro (`intake.md:7-10,25`; decisão anterior:13-14,65-70). |
| Risk posture | adequate | Fail-closed com erros genéricos, nunca logar token, `1008`/`SESSION_INVALID` na invalidação, non-goals duros para fanout/sync e docs de janela de revogação; NVs explícitos (`research.md:22-25`; decisão anterior:48-75). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O scorecard sustenta a Opção A — WS session authority + alinhamento de protocolo — sobre B (fanout/delivery) e C (operabilidade Seed/VPS), conforme `intake.md:12-21`. Base: `dev @530d63fea` (cycle 013 merged, PR #216, CI green). Fatos preservados: HTTP extractors já validam `jti` no Postgres (012/PR #215) mas `flora-api/src/routes/websocket.rs` decodifica JWT sem checagem de sessão; `apps/frontend/src/lib/api/websocket.ts` usa `/api/v1/ws?token=` e eventos (`workspace.user.removed`, `/messages/sync`) que o backend não implementa; tagged envelopes `Subscribe/Unsubscribe/SendMessage/Ping` ↔ `Subscribed/.../NewMessage/MessageSent/Pong/Error`, com `SendMessage` checando dono da org e persistindo, mas sem registry/fanout. A ressalva de janela de revogação (handshake-only vs recheck jittered), o tradeoff de transporte no browser e os NVs (nenhum runtime measurement, browser e2e ou CI remoto neste assessment; ja/ko/ru limitados) reforçam o recorte, não o invertem. C e B seguem como slices futuros.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; transporte de token, janela de revogação e schema de revalidação seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: O caminho realtime não é seguro, autoritativo nem utilizável pelo cliente: `/ws` aceita upgrade com JWT-only (sem DB `jti`/revogação/expiração) e o frontend usa path/transporte/eventos incompatíveis, deixando sessões revogadas vivas e tenant/lifecycle semântica indefinida (`problem.md:4,18-22`).
- **Chosen approach**: Opção A — extrair helper compartilhado (`verify_bearer_token(token, state) -> VerifiedSession`) reaproveitando JWT decode + `SessionService::validate_session(jti)` (ativo/expiry/user/org) e usá-lo também no handshake `/ws` antes de `on_upgrade`; alinhar path/transporte (Bearer + fallback browser documentado, nunca logar token), envelopes e close (`SESSION_INVALID`/`1008`) com erros genéricos 401/500 (decisão anterior:19-36; `research.md:28-30`).
- **In scope / out of scope**:
  - **In (backend)**: `flora-api/src/extractors/auth.rs` (helper compartilhado) e `extractors/mod.rs`, `flora-api/src/routes/websocket.rs` (resolver token por transporte, verificar antes do upgrade, passar `user_id/org_id/jti` ao handler, checagens de org do canal, nil-org, recheque opcional jittered), `routes/mod.rs` (alinhar/aliar `/ws` e `/api/v1/ws`); **FE**: `apps/frontend/src/lib/api/websocket.ts` (URL/transporte, envelopes tagged, sem retry cego em falha de auth, backoff limitado + jitter), `(app)/+layout.svelte` (rebind em logout/refresh/troca de org); **testes/docs**: unit tests de transporte/envelope, contratos DB-gated PG18 (matriz aceito/rejeitado: válido, revogado/inativo, expirado, ausente, user/org mismatch, nil-org, cross-org), job `db-tests`, Vitest e `docs/dev-smoke.md` (decisão anterior:19-46).
  - **Out / NV**: fanout/registry/broadcast multi-client/cross-node, Valkey/NOTIFY push, `/api/v1/messages/sync`, replay/histórico, presença/typing/receipts, anexos/DMs/UX de roles, search/notifications, OIDC/JWKS e2e, Playwright, `apps/web`, deploy/merge/release/secrets, refator ampla Seed/VPS (candidato C). NV: medições live, e2e browser, CI remoto, sufiência do schema/índice para revalidação, ja/ko/ru (decisão anterior:58-75).
- **Success metrics**: Handshake WS aplica a mesma autoridade de sessão do HTTP (assinatura + `jti` ativo/não expirado/user/org consistente) com falhas genéricas pré-upgrade; path/transporte documentados e determinísticos; compatibilidade mínima FE↔BE para connect/auth/error/close sem retry infinito; tenancy preservada no handshake e nas ações de mensagem; matriz DB-gated cobrindo aceite/rejeição (problem.md:30-35; decisão anterior:33,44-46).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: qual transporte de token no browser adotar (cookie HttpOnly/SameSite, ticket one-shot via subprotocol, ou query fallback) e como evitar vazamento em logs?]; [NEEDS CLARIFICATION: janela de revogação — handshake-only, recheck jittered 60–300s ou push invalidation?]; [NEEDS CLARIFICATION: regra de rebind/disconnect em logout, refresh e troca de org?]; [NEEDS CLARIFICATION: como tratar nil-org no WS em paridade com `require_org()`?]. NV-1: nenhuma medição live/browser e2e/CI remoto neste assessment (`research.md:61-63`; decisão anterior:72-75).
