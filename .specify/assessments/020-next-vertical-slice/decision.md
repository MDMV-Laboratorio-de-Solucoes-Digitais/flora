# Decision: 020 — Next vertical slice (FTS multi-entidade + notificação de atribuição)

- **Slug**: 020-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | R1 docs 020 (5, via intake/research) | auditorias `ses_e1d674de4ffevX4wonv9WQ0JAS`, `ses_e1d670690ffeDzKAP9DWM7qZlR`

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Instalações Seed sem Meilisearch não localizam tarefas nem arquivos (o fallback 019 cobria só mensagens) e responsáveis não recebem sinal in-app de atribuição (`intake.md:6,19-20`; `research.md:6-9`). |
| Evidence strength | adequate | R1 (`2026-11-16-020-{pg-fts-tasks-files-vps,task-assignment-notification-vps,macro-tasks-files-assignment-flows,db-gated-tests-search-notifications-assignment,pg18-multi-entity-fts-vps-ops}.md`) e 2 auditorias convergem para `search_tsv` aditivo e hook de transição; relevância/i18n e paridade Meilisearch permanecem NV (`research.md:6-15`). |
| Value vs. inaction | strong | Fecha o ciclo "encontrar e agir sobre o trabalho" e o loop de atribuição com modelos/repos/APIs existentes, sem novo serviço obrigatório em VPS de 2vCPU/4GB (`decisão anterior:14`; `concept.md:14`). |
| Feasibility / appetite | adequate | Estende o fallback com `search_tsv` gerado (`simple`) + GIN parcial em `tasks(title,description)`/`files(name,file_type)` e hook best-effort em `tasks.rs`; restrito pelo contrato `PATCH` (`or(existing)`) que não consegue limpar `assignee_id` (`research.md:13`). |
| Strategic fit | strong | `OBJETIVO.md`/`ROADMAP.md` pedem extensão Postgres-first de 019, verificável e compatível com o caminho Meilisearch, sem tocar no frontend (`intake.md:6,9`). |
| Risk posture | adequate | Contrato e Meilisearch preservados, merge tipado com `LIMIT` global e guardas self/no-op; NVs de unassign, dedup/idempotência, tx-coupling e relevância registrados (`concept.md:19`, `research.md:13-15`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. S1 é o 80/20 do ciclo 020: estender o fallback PG para `tasks(title,description)` e `files(name,file_type)` com `search_tsv` gerado/armazenado `'simple'` + GIN parcial, parsing seguro `COALESCE(websearch_to_tsquery('simple',$q), plainto_tsquery('simple',$q))`, resultados tipados `message|task|file` com `LIMIT` global, e emitir `NotificationType::Assignment` (`target_id=task.id`, `target_url=None`) apenas na transição efetiva `new != old && new != actor`, best-effort pós-persist. A convergência dos cinco docs R1 e das auditorias (`ses_e1d674de4f…`, `ses_e1d670690f…`) sustenta o recorte; baseline `dev@dde602b3b` (019 merged, PR #222, CI verde). A limitação de contrato — `assignee_id.or(existing.assignee_id)` não representa retirada — é registrada como gap/NV, não inventada como feature. Relevância/CJK, `pg_trgm`, ACL, paridade de indexação no Meilisearch, outbox/workers/push e benchmarks permanecem NV.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as incógnitas abaixo são carried-forward, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Tarefas e arquivos continuam impossíveis de encontrar em instâncias Seed sem Meilisearch e atribuições não geram aviso in-app, apesar de schemas/APIs existirem (`problem.md:5-9`).
- **Chosen approach**: S1 — extensão Postgres-first de 019 com `search_tsv`+GIN aditivos para tarefas/arquivos, resultados tipados no mesmo contrato `GET /api/v1/search`, e hook de `Assignment` em `flora-api/src/routes/tasks.rs` (create/update) com guardas de self/no-op e best-effort sem falhar a mutação (`decisão anterior:9-19`).
- **In scope / out of scope**:
  - **In**: migração aditiva e índices parciais (`WHERE is_deleted = false`); extensão do fallback PG preservando Meilisearch, tenant/membership, soft-delete, type-filter e limites; hook de atribuição; contratos DB-gated em PG18 para busca multi-entidade e notificação (create/self/no-op/transição/visibilidade); SDD/validação/release, PR/CI verde, merge/sync, planejamento para 021.
  - **Out/NV**: relevância/tokenização/i18n/CJK e `pg_trgm`, ACL por canal/workspace, paridade de indexação/consulta do Meilisearch para tarefas/arquivos, retirada explícita de atribuição, dedup/idempotência/acoplamento transacional/race, outbox/workers/push/realtime, mentions/completion, mudança no formato da API de notificações (`target_url`), benchmarks 2vCPU/4GB, browser e2e, frontend, remoção do Meilisearch, crates/infra multi-nó e novos serviços obrigatórios.
- **Success metrics**: `/api/v1/search` no fallback retorna hits tipados `message|task|file` com filtros de tenant e soft-delete preservados; toda transição efetiva para responsável diferente do ator gera `Assignment` visível na API, e self-assign/no-op/ausência de transição não geram; contratos DB-gated em PostgreSQL 18 cobrem busca multi-entidade e notificação com guardas determinísticas (`problem.md:24-27`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: a impossibilidade de retirar a atribuição deve permanecer como limitação conhecida ou existe decisão de produto para evoluir o contrato?]; [NEEDS CLARIFICATION: qual critério de ordenação entre hits de mensagens, tarefas e arquivos no merge global?]; [NEEDS CLARIFICATION: existe evidência de uso real de busca/atribuição em instâncias Seed para dimensionar prioridade?] (`problem.md:34-36`).
