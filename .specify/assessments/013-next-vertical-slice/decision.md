# Decision: 013 — Next vertical slice (workspace→channel→message, REST-first)

- **Slug**: 013-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | 012 decision/intake/research | docs/research/2026-11-15-012-frontend-api-contract-audit.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O happy path de comunicação do Seed está quebrado: não há listagem de canais, `thread_id` é descartado, path-tenancy (C19) não é validado e o FE usa IDs hardcoded não-UUID, com divergência snake_case↔camelCase (C4,C5,C12,C13) (`research.md:10-13`; `problem.md:4`). |
| Evidence strength | adequate | Auditoria frontend↔API (C4–C6, C11–C13, C18–C20, C22, C25) + leitura de código (`channels.rs`, `models.ts`, `ChatLayout.svelte`), com plano de verificação determinística (DTO/serialização, DB-gated PG18, Vitest); e2e real/NV (`research.md:3-7,19`; decisão anterior:34-41). |
| Value vs. inaction | strong | É a maior fatia P0 ainda faltante após 012: entrega comunicação persistida e legível em workspace selecionado, sem nova infra; B (binários) exige storage plumbing (C7–C8) e C (WS) não destrava o fluxo sem contratos/listing (`intake.md:11-18`; decisão anterior:10-11). |
| Feasibility / appetite | adequate | Backend já persiste canais/mensagens (`flora-api/src/routes/channels.rs`), o escopo é majoritariamente contrato/wiring/tenancy + mapper isolado; incerteza de schema do `thread_id` é risco NV (`research.md:16-18`; decisão anterior:44-47). |
| Strategic fit | strong | `OBJETIVO.md`/`ROADMAP.md` P0 exigem canais/mensagens persistidas e fatia vertical e2e verificável; Postgres-first, VPS-cheap, sem novos serviços obrigatórios (`intake.md:8,20-22`; `research.md:4`). |
| Risk posture | adequate | Não-goals duros (fanout/registry WS, binários, DMs UX, OIDC e2e, `apps/web`, deploy/merge), mapper isolado para limitar blast radius no FE e checagens de pertencimento da workspace à org (`decisão anterior:43-52`; `concept.md:19-20`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O scorecard confirma a escolha da Opção A — `workspace→channel→message` persistence+read (REST-first) com wiring mínimo — sobre B (upload binário, C7–C8) e C (WS authority/protocolo), conforme `intake.md:10-18`. Base: `dev` após PR #215 (cycle 012 session authority). Os fatos que sustentam o veredito permanecem: backend já implementa persistência em `flora-api/src/routes/channels.rs` mas bloqueia por ausência de `GET /`, `thread_id` dropado, gap de path-tenancy (C19) e DTO/ID drift no FE (C4,C5,C12); o realtime (WS path/auth/protocolo, C6,C20,C25) está separadamente quebrado, tornando REST o caminho confiável. A incerteza de schema do `thread_id`, o risco de escopo (paginação C22, permissões, anexos) e o blast radius no FE são mitigados pelos non-goals e pelo mapper isolado, não invertendo a decisão.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; questões de schema `thread_id` e paginação seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Usuários SMB não conseguem completar o happy path de comunicação do Seed no `apps/frontend`: canais não são descobertos/criados em contexto, mensagens não podem ser listadas/postadas em UUIDs reais e os campos divergem (C4,C5,C12,C13) (`problem.md:4`).
- **Chosen approach**: Opção A — fatia REST-first `workspace→channel→message`, tenant-safe e persistida: adicionar `GET /channels?workspace_id=` com auth/org/membership, persistir `thread_id`, validar `channel_id` do path em edit/delete (C19), mapper explícito snake_case→camelCase no FE e fluxo mínimo workspace→canal→mensagem; realtime, anexos e binários ficam de fora (`concept.md:4-28`; decisão anterior:5-6,13-32).
- **In scope / out of scope**:
  - **In (backend)**: `flora-api/src/routes/channels.rs` (`list_channels`, persistência de `thread_id`, checagem `channel_id` path vs body em edit/delete, testes de serialização); traits/repositórios `flora-core` (`list_by_workspace`/`find_by_workspace_id`, threading conforme schema); `apps/frontend/src/lib/api/mappers.ts` (`toChannel`/`toMessage`, `fileAttachments: []`), `models.ts`, `ChatLayout.svelte`/`ThreadReply.svelte` (IDs reais, payload `thread_id`), `channels/+page.svelte`, `MessageStore.svelte`, `(app)/+layout.svelte`; `flora-tests/src/contract/messaging.rs` (substituir stubs `Ok(())`), testes unitários dos mappers (decisão anterior:15-31).
  - **Out / NV**: WS broadcast/subscription registry, Valkey fanout, `/messages/sync`, WS session DB authority, reconnect/backoff (C6,C20,C25 parcial); upload/download binário e anexos (C7,C8,C13); UX de DMs, UI de membership/roles, search/notifications, semântica de listagem de tasks (C11), hardening cross-tenant de `workspace_id` em tasks/files (C18) salvo incidental, OIDC e2e, `apps/web`, deploy/merge; schema do `thread_id` se exigir migração — manter coluna existente ou dividir (NV) (decisão anterior:44-52).
- **Success metrics**: Membro em org/workspace selecionado consegue listar/criar canal, postar e ler mensagens persistidas (incluindo reply/thread id) via REST com contratos consistentes e tenancy preservada; testes determinísticos fixam DTOs, persistência/leitura e C19 sem depender de OIDC/WS (`problem.md:17-19`; decisão anterior:29-41).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: o schema suporta `thread_id` sem migração?]; [NEEDS CLARIFICATION: honrar `before`/paginação (C22) agora ou diferir?]; [NEEDS CLARIFICATION: como tratar DMs (`dms/+page.svelte`) sem UUIDs falsos?]; [NEEDS CLARIFICATION: quando retomar WS authority/fanout (C6,C20,C25) e anexos (C7,C8,C13)?]. NV: smoke completo com API+DB em portas padrão, OIDC/JWKS real, fanout/delivery WS, e2e browser, `apps/web` (decisão anterior:41).
