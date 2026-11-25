# Decision: 021 — Next vertical slice (unassign, dedup e precedência de busca)

- **Slug**: 021-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | R1 docs 021 (5, via intake/research) | auditorias `ses_e1d515825ffeRc6tFzKjrFXEzD`, `ses_e1d515824ffewqOPRTcuB2Liu6`

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O `PATCH` colapsa `assignee_id` ausente e `null` (`or(existing)`), tornando `Some -> None` impossível, e não há guarda contra duplicatas óbvias nem precedência de busca explicitada (`problem.md:7-15`). |
| Evidence strength | adequate | R1 (`2026-11-16-021-{meilisearch-task-file-search-parity-vps,task-unassign-contract-notification-semantics,macro-search-parity-unassign-notifications,db-gated-tests-unassign-search-parity,optional-meilisearch-pg-coherence-vps}.md`) e 2 auditorias sustentam tri-state, transição efetiva e dedup barato; paridade Meilisearch e janela de corrida são NV (`research.md:6-14`). |
| Value vs. inaction | strong | Semântica explícita de "não alterar/retirar/definir" e menos ruído de notificação fecham um gap real de correção do contrato de tarefas (`problem.md:26-30`). |
| Feasibility / appetite | adequate | `Option<Option<Uuid>>` (serde double-option) e dedup query-before-insert são baratos, mas reinterpretar `null` é evolução de contrato e exige testes de regressão (`research.md:6,13`). |
| Strategic fit | strong | Consolida a direção Postgres-first do roadmap, fixando o fallback PG como base de correção e registrando explicitamente o diferimento do Meilisearch (`intake.md:6,9`; `concept.md:5-10`). |
| Risk posture | adequate | Transições determinísticas e U0 (sem notificação em unassign) preservam o comportamento atual; dedup é race-prone/unread-only e a paridade Meilisearch segue como NV documentado (`research.md:7-8,14`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. S1 é o 80/20 do ciclo 021: tri-state `assignee_id` no `PATCH` (omit = sem mudança, `null` = limpar/unassign, valor = definir), notificação `Assignment` somente em transição efetiva para destinatário não-ator (sem notify em unassign/self/no-op), dedup best-effort query-before-insert para `Assignment` não-lido duplicado por `(user_id,target_id)`, endurecimento dos contratos DB-gated do fallback PG e decisão explícita de precedência — Meilisearch preferido quando configurado, PG fallback autoritativo para paridade multi-entidade, com indexação de mutação diferida (sem hooks neste ciclo). Baseline `dev@0dcde353a` (020 merged, PR #223, CI verde); convergência dos cinco docs R1 e das auditorias (`ses_e1d515825f…`, `ses_e1d515824f…`). A mudança de significado de `null` é tratada como evolução de contrato, não como detalhe interno; unicidade/outbox/tx-coupling e paridade Meilisearch ficam NV.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as quatro perguntas abaixo seguem como carried-forward, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Contrato de atualização de tarefa não distingue "omitido" de `null` (unassign impossível), notificações de atribuição podem duplicar em retries e a precedência entre backends de busca não está registrada de forma inequívoca (`problem.md:7-15`).
- **Chosen approach**: S1 — `UpdateTaskRequest.assignee_id` como `Option<Option<Uuid>>`, resolução determinística `old -> new`, `Assignment` só em `Some(new)` com `new != old && new != actor`, dedup não-lido query-before-insert, hardening do fallback PG e decisão documentada de precedência/paridade (`concept.md:5-10`; `decisão anterior:9-19`).
- **In scope / out of scope**:
  - **In**: parsing tri-state e regras de merge no servidor; hook de transição em `flora-api/src/routes/tasks.rs` (sem notify em unassign/self/no-op, best-effort); dedup de unread `Assignment` por `(user_id,target_id)`; normalização de tokens de tipo, limites e invariantes tenant/soft-delete no fallback PG; contratos DB-gated em PG18; SDD/validação/release, PR/CI verde, merge/sync.
  - **Out/NV**: hooks de indexação/paridade Meilisearch para tarefas/arquivos/mensagens, testes determinísticos ancorados no Meilisearch, equivalência de ranking/snippets, `pg_trgm`/i18n/CJK e ACL por canal/workspace, constraint de unicidade/outbox transacional/idempotência forte, tipo de evento específico para unassign, mudança na política de associação, menções/completion/realtime/push, UI frontend, benchmarks 2vCPU/4GB, browser e2e, novos serviços obrigatórios, convergência de crates, remoção do Meilisearch e infra multi-nó.
- **Success metrics**: O contrato diferencia omitido/`null`/valor e permite `Some -> None`; assign/reassign/retirada/self/no-op/dedup têm comportamento observável distinto sem duplicata não-lida óbvia; contratos PG18 cobrem transições e bordas da busca (normalização, limites, tenant, soft-delete); precedência entre backends registrada nos artefatos (`problem.md:32-37`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: reinterpretação de `assignee_id: null` de "ignorado" para "retirar" é aceitável para clientes existentes?]; [NEEDS CLARIFICATION: retirada deve gerar evento próprio ou permanecer sem notificação?]; [NEEDS CLARIFICATION: a janela de corrida residual da guarda de duplicatas é aceitável neste ciclo?]; [NEEDS CLARIFICATION: paridade de tarefas/arquivos no Meilisearch é diferida ou requisito do contrato de busca?] (`problem.md:42-45`).
