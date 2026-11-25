# Decision: 015 — Next vertical slice (realtime mínimo + catch-up determinístico)

- **Slug**: 015-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | 014 decision | docs/research/2026-11-16-014-realtime-delivery-vps-options.md | docs/research/2026-11-16-014-ws-codebase-audit.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | 013 persistiu mensagens REST e 014 escopou autoridade/protocolo WS, mas falta registry/fanout, REST não emite eventos e `list_messages` não é cursor-determinístico (`before` aceito mas offset/0 e filtro em-handler), sem catch-up no FE (`research.md` via decisão anterior:12; `intake.md:25-31`). |
| Evidence strength | adequate | Consenso multilíngue (EN/es/de/fr/zh-CN/ja; ko/ru parcial NV) por keyset/cursor estável (UUIDv7/`created_at,id`), fanout in-process viável single-node e NOTIFY/pubsub não duráveis; plano DB-gated PG18 + Vitest, sem e2e ao vivo (`decisão anterior:15,38-56`; `intake.md:38-40`). |
| Value vs. inaction | strong | Fecha a fatia de comunicação antes de B (binários) e C (operabilidade) e antes de adaptadores de escala; mensagens commitadas tornam-se visíveis em near-realtime e sempre recuperáveis no reconnect/reload (decisão anterior:12-14). |
| Feasibility / appetite | adequate | Postgres já é source of truth; cursor + hub in-process reusam WS/auth/DTO de 014 sem novo serviço obrigatório, cabendo em 2 vCPU/4 GB; dependência do merge 014 e sufiência de índices são NV (`decisão anterior:13,66-67,85-91`). |
| Strategic fit | strong | ROADMAP P0.2/4 (realtime mínimo confiável) e perfil Seed Postgres-first/VPS-barato; correção = Postgres + catch-up determinístico, push como best-effort (decisão anterior:8,78-83; `intake.md:6,38`). |
| Risk posture | adequate | Non-goals duros (multi-node, exactly-once, Valkey/NOTIFY, outbox), buffers limitados, dedupe por `id`, interface adapter-friendly e DB como caminho de recuperação; NV-1..NV-5 registrados (decisão anterior:57-76,85-91). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O scorecard confirma a Opção A — realtime delivery mínimo + catch-up determinístico para mensagens persistidas (single-node, Postgres-authoritative) — sobre B (upload binário) e C (operabilidade Seed/VPS), em ciclo pós-014 (PR #217 em andamento/merge esperado). Fatos preservados: `GET/POST /channels` e `/messages` já existem com checagens org/workspace/channel, mas `Pagination{limit,offset:0}` e filtro de deletados em-handler impedem catch-up por cursor; `GET /ws` tem helpers de auth DB-backed (target 014) e envelopes tagged, porém `Subscribe` é ack-only, sem registry/fanout/consumer bridge; `flora-messaging` é publish-only; o FE usava `/api/v1/ws` com query token e shapes divergentes. A dependência do estado final de 014 (NV-1), a ausência de medições de load/memória em VPS (NV-2), cobertura multilíngue parcial (NV-3), sufiência de schema/índice (NV-4) e `apps/web`/e2e fora de escopo (NV-5) limitam o escopo, não invertem a decisão de entregar recuperação determinística primeiro.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; merges 014, índices de cursor e limites de buffer seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Entrega incompleta após 013/014: sem subscription registry/fanout, mutations REST não emitem eventos, `list_messages` não é determinístico por cursor e o FE não tem sync/catch-up funcional (`decisão anterior:12`; `intake.md:29-31`).
- **Chosen approach**: Opção A — cursor keyset estável (`after`/`before` por `created_at,id`/UUIDv7) + hub de broadcast in-process por processo, com canonical `NewMessage` alinhado a `MessageResponse`, dedupe por `id`, backpressure limitado e DB/REST como autoridade; sem Valkey/NOTIFY obrigatórios (decisão anterior:5-8,21-36).
- **In scope / out of scope**:
  - **In (backend)**: traits/repositórios de mensagens em `flora-core` (consulta por cursor, ordenação determinística, índice se necessário), `flora-api/src/routes/channels.rs` (honrar `before`, adicionar `after`/metadado de cursor, publicar evento canônico pós-commit), `routes/mod.rs` (alias fino `/messages/sync` se preciso), `flora-api/src/state/app_state.rs` (hub process-local) e `routes/websocket.rs` (subscriptions por org/canal, register/cleanup, forward com buffers limitados, manter auth 014); **FE**: `lib/api/messages.ts`, `lib/api/websocket.ts`, `lib/api/mappers.ts`, `MessageStore.svelte.ts` (upsert idempotente por `id`), `ChatLayout.svelte` e layouts; **testes/docs**: unit tests de cursor/envelope/routing, contratos DB-gated PG18 (ordenação, tenant isolation, sem duplicata/gap no conjunto commitado), Vitest, `db-tests` no CI e `docs/dev-smoke.md` (decisão anterior:21-43).
  - **Out / NV**: upload/download binário e anexos (B), refactors de operabilidade/Seed (C: Meilisearch boot, compose profiles/limits, pool/env) salvo notas incidentais, multi-node/cross-process fanout, exactly-once durability, ponte Valkey consumer, listener NOTIFY/outbox/streams, presença/typing/receipts, reações, search/notifications, taxonomia completa edit/delete, novo `/messages/sync` além do alias fino, Playwright/e2e, OIDC/JWKS real, `apps/web`, deploy/merge/release/secrets. NV-1: 014 merge/runtime e detalhes finais de auth/protocolo não verificados; NV-2: sem medições de load/memória; NV-3: ko/ru/pt-BR parcial; NV-4: índices de `messages` a confirmar; NV-5: `apps/web`/browser e2e (decisão anterior:69-91).
- **Success metrics**: Mensagem commitada fica visível para outros clientes autorizados no mesmo nó em near-realtime e é sempre recuperável por cursor após reconnect/reload, sem duplicatas ou gaps silenciosos; contratos DB-gated fixam ordenação/tenant/mutação e o FE faz merge idempotente por `id` (`decisão anterior:38-43`; `intake.md:29-31`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: 014 já está mergeado/runtime estável para servir de base?]; [NEEDS CLARIFICATION: `created_at,id` ou UUIDv7 como chave de cursor e os índices existentes a suportam?]; [NEEDS CLARIFICATION: limites de buffer/subscription e política drop-oldest-with-catch-up para 4 GB?]; [NEEDS CLARIFICATION: como definir visibilidade de mensagens editadas/deletadas no catch-up e semântica de eco do remetente?]. NV-3: cobertura ko/ru/pt-BR parcial; NV-5: `apps/web`/e2e (`decisão anterior:60-64,85-91`).
