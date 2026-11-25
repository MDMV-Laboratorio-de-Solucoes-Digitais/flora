# Decision: 023 — Next vertical slice (busca PG-autoritativa e hardening de FTS)

- **Slug**: 023-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | constituição (Seed = Flora + PostgreSQL; Meilisearch = Grove) | R1 docs 023 (6, 8 agentes background) | auditorias de codebase e frontend/ops

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `search_service` configurado prefere Meilisearch, mas `index_task`/`index_file`/`index_message`/`delete_document` não têm call sites e `setup_index` nunca é chamado; o fail-open do 022 cobre só erros de consulta, não respostas vazias bem-sucedidas (`problem.md:7-11`; `research.md:8,15`). |
| Evidence strength | strong | Auditoria de código (zero write call sites) + docs de primary sources sobre SDK 0.33 (`wait_for_task` pode retornar `Ok(Task::Failed)`; `Client::new` não faz I/O) + constituição + divergência `enabled=true` vs `.env.example=false`, com fixture inalcançável determinística (`research.md:7-12,20`). |
| Value vs. inaction | strong | Elimina dependência da correção da busca de um índice nunca populado, tornando o comportamento previsível com e sem Meilisearch e recuperando nomes hifenizados (`problem.md:19-26`). |
| Feasibility / appetite | adequate | Remover o branch Meilisearch do request path e normalizar separadores é pequeno, mas exige migração aditiva que recrie colunas geradas/índices GIN sem regressão (`concept.md:6-10`; `problem.md:12`). |
| Strategic fit | strong | Constitution define Seed = Flora + PostgreSQL e Meilisearch como componente Grove; debloat/vendor-unlock é princípio central (`intake.md:5`; `research.md:7`). |
| Risk posture | adequate | Campo `source` é aditivo e sem consumidor no frontend canônico; respostas inalcançáveis são testadas sem serviço externo, enquanto relevância CJK/`pg_trgm` e reativação futura permanecem NV (`research.md:11-18`; `concept.md:10`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-25 preservado como `go`; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. S1 é o 80/20 do ciclo 023: PG FTS passa a ser o único backend autoritativo de `/api/v1/search`, Meilisearch fica dormente e explicitamente não-autoritativo (crate `flora-search` e profile compose `search` preservados para reativação futura com gate), a resposta ganha `source: "postgres"` e contratos DB-gated determinísticos (inclusive com `SearchService` configurado para endpoint inalcançável), separadores `-`/`.`/`_` são normalizados nas expressões geradas de `search_tsv`, e config/docs são alinhados ao perfil Seed. Baseline `dev@023bd2467` (022 merged, PR #225/#226, CI verde). R1 (8 agentes, 6 docs: `meilisearch-task-await-determinism`, `meilisearch-rebuild-backfill-pg`, `search-debloat-pg-only-vps`, `outbox-lite-indexing-pg`, `meilisearch-gated-ci-harness`, `macro-backfill-notification-reference`) converge com a constituição: o caminho Meilisearch é não-funcional, não apenas desatualizado, e teria bug latente de filtro plural/singular caso populado. Hooks, awaits, rebuild, CI Meilisearch, cross-engine parity, outbox/tx-coupling, `pg_trgm`/i18n/CJK, ACL e benchmarks seguem NV; a reativação é decisão isolada de 024+.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as quatro perguntas abaixo seguem como carried-forward, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Busca prefere um índice Meilisearch que nunca é escrito (zero hooks, sem `setup_index`, com bug latente de tipo), produzindo vazios silenciosos; nomes hifenizados não casam sob `'simple'` e config/docs divergem (`problem.md:7-16`).
- **Chosen approach**: S1 — `flora-api/src/routes/search.rs` sempre chama `search_pg` e adiciona `source: "postgres"`; Meilisearch dormente/não-autoritativo; migração aditiva normalizando separadores em `messages/tasks/files.search_tsv` (+ índices); `SearchConfig::default().enabled = false`, chaves `FLORA_SEARCH__*` no `.env.example` e nota de capacidade/degradação em `docs/dev-smoke.md` (`concept.md:5-10`; `decisão anterior:9-17`).
- **In scope / out of scope**:
  - **In**: remoção do branch Meilisearch do request path; campo `source` no contrato; migração das colunas geradas e dos índices parciais; alinhamento de config/docs; contratos DB-gated em PG18 (autoridade com `SearchService` inalcançável, sub-token de nome hifenizado, invariantes tenant/deleted/type/limit); SDD/validação/release, PR/CI verde, merge/sync e planejamento de 024. Sem mudança de frontend.
  - **Out/NV**: hooks de mutação, delete-on-soft-delete, wrapper de task-await, rebuild/backfill, blue-green `/swap-indexes`, CI Meilisearch, paridade entre engines, outbox/tx-coupling; `pg_trgm`/i18n/CJK, ACL de busca, validação 2vCPU/4GB, adoção no frontend/e2e; UUIDs determinísticos/coalescing de notificações, eventos de unassign, mentions/push (candidatos 024); non-goals: novos serviços obrigatórios, remoção de crates e mudanças no frontend.
- **Success metrics**: A busca retorna resultados do backend transacional mesmo com `SearchService` configurado (fixture inalcançável), com `source: "postgres"` na resposta; consultas por sub-token de nome hifenizado encontram o registro; config padrão e `.env.example` convergem; contrato de capacidade/degradação documentado (`problem.md:28-33`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: o campo `source` é compatível com consumidores externos não canônicos?]; [NEEDS CLARIFICATION: que nível de relevância para hifenização/CJK/`pg_trgm` é suficiente?]; [NEEDS CLARIFICATION: qual valor (`true` no código ou `false` no `.env.example`) representa a intenção oficial?]; [NEEDS CLARIFICATION: a reativação do Meilisearch precisa de gatilho/critérios já neste ciclo ou pode ser diferida?] (`problem.md:41-44`).
