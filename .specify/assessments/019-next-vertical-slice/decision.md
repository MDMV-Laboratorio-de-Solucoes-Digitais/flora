# Decision: 019 — Next vertical slice (PG FTS de mensagens + notificação de resposta)

- **Slug**: 019-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | R1 docs 019/018 (via intake/research) | auditorias `ses_e1d793fa1ffeg18iLwVe1BQrV1`, `ses_e1d77e0a0ffe9WZE984FSoaAv7`, `ses_e1d78fb75ffezHWpPzuDQ60xJS`

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `/api/v1/search` depende de Meilisearch e devolve 503 sem ele, e nenhuma mutação emite notificação embora tabela/modelo/repo/rotas e o tipo `Reply` existam (`problem.md:7-14`). |
| Evidence strength | adequate | R1 (`2026-11-16-019-{postgres-fts-search-vps,inapp-notification-wiring-vps,macro-mutation-search-notification-flows}.md`) e 3 auditorias convergem para mensagens-only FTS + reply-in-thread, com contratos DB-gated em PG18; relevância zh-CN/ja e carga 2vCPU/4GB seguem NV (`research.md:5-14`). |
| Value vs. inaction | strong | Restaura busca em deploy Seed enxuto e fecha um loop de feedback colaborativo reaproveitando schemas/APIs existentes, sem Kafka/SQS/OpenSearch/outbox/workers/push (`decisão anterior:12`; `intake.md:16-18`). |
| Feasibility / appetite | adequate | Migração aditiva de `search_tsv` + GIN parcial e um hook best-effort pós-persist em `post_message`, com parsing seguro e guardas de tenant/self/parent inválido (`concept.md:9-22`). |
| Strategic fit | strong | `OBJETIVO.md`/`ROADMAP.md` definem 019 como wiring essencial de busca/notificações, Postgres-first e verificável, sem novo serviço obrigatório; frontend não exige mudança (`intake.md:6,9`; `research.md:11`). |
| Risk posture | adequate | Contrato `q`/`type`/`limit` preservado e fallback explícito testado; tokenização/perf e duplicação mitigados por `'simple'`, limites e guardas, com NVs registrados (`concept.md:24-30`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. S1 é o 80/20 do ciclo: tornar `/api/v1/search` utilizável sem Meilisearch via FTS aditivo de mensagens (degradação para PG em vez de 503, contrato estável e tenant-safe) e emitir uma notificação `Reply` ao autor da mensagem pai em resposta em thread, validada em PostgreSQL 18 descartável. A convergência entre `OBJETIVO.md`/`ROADMAP.md`, os três docs R1 e as auditorias (`ses_e1d793fa…`, `ses_e1d77e0a…`, `ses_e1d78fb…`) sustenta o recorte; o baseline foi `dev@3c8eddbfd` (018 + hardening de arquivos, PR #221, CI verde). O hook é best-effort/não fatal, sem Valkey/`flora-notifications` crate e sem outbox; Meilisearch permanece opcional. Relevância/i18n (zh-CN/ja), ACL por canal, tuning de 2vCPU/4GB, outbox/workers/push e e2e de browser permanecem NV — reforçam o escopo determinístico em vez de contestá-lo.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante registrada nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Busca global indisponível em deployments Seed sem Meilisearch (503) e ausência de sinal in-app para respostas em thread, apesar de storage/repos/API de notificações já existirem (`problem.md:7-14`).
- **Chosen approach**: S1 — `messages.search_tsv` (`tsvector` gerado/armazenado, `'simple'`) + GIN parcial (`WHERE is_deleted = false`), caminho PG em `/api/v1/search` com `websearch_to_tsquery`/fallback `plainto_tsquery` e degradação para PG quando Meilisearch ausente; hook pós-persist em `channels.rs::post_message` criando `NotificationType::Reply` para o autor do pai com guardas de mesma org/canal, não-excluído e `parent.sender_id != sender` (`concept.md:9-22`).
- **In scope / out of scope**:
  - **In**: migração aditiva; wiring state/config/route preservando contrato e checagens de tenant; notificação best-effort não fatal; contratos DB-gated em PG18 (match, isolamento, excluídos, `type`/query vazia, disponibilidade sem Meilisearch; criação de `Reply`, sem self-notify, sem notify em parent inválido/cross-scope, visibilidade na API); SDD/validação/release, PR/CI verde, merge/sync e atualizações de planejamento.
  - **Out/NV**: busca de tasks/files, `pg_trgm`, relevância/i18n e qualidade zh-CN/ja, ACL por canal, backfill/reindex, outbox/workers/push/realtime, fanout de mention/assignment/completion, dedup uniqueness (salvo se trivial), benchmarks 2vCPU/4GB, browser e2e/multi-client; non-goals: remover Meilisearch, UI frontend de busca/notificações, convergência de crates, infra multi-nó e novos serviços obrigatórios.
- **Success metrics**: `/api/v1/search` serve mensagens via PG FTS sem Meilisearch com `q`/`type`/`limit` e isolamento por `organization_id` preservados; uma resposta em thread gera `Reply` visível na API ao autor do pai, sem auto-notificação nem notificação em parent inválido; contratos determinísticos em PostgreSQL 18 cobrem todos os casos acima (`problem.md:17-19`; `concept.md:23-27`).
- **Carried-forward open questions**: `[NV]` qualidade de FTS para zh-CN/ja e tuning de relevância; `[NV]` ordenação/perf e carga em 2vCPU/4GB; `[NV]` filtragem ACL por canal na busca; `[NV]` backfill/reindex e entrega outbox/workers/push/realtime; `[NV]` fanout de mention/assignment/completion e dedup uniqueness — nenhuma bloqueia o `go`.
