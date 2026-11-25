# Decision: 022 — Next vertical slice (dedup no banco, degradação fail-open e CI DB-gated)

- **Slug**: 022-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | R1 docs 022 (6, via intake/research) | auditorias `ses_e1c0bc4e5`, `ses_e1c0bc4c8`

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | A dedup do 021 é query-before-insert não-lida e race-prone, sem unicidade no banco; falhas de consulta não degradam explicitamente para PG e nem todas as suítes DB-gated rodam na CI (`problem.md:7-14`). |
| Evidence strength | adequate | R1 (`2026-11-16-022-{meilisearch-indexing-hooks-reindex-parity-vps,meilisearch-pg-coherence-degradation-contract-vps,notification-idempotency-unassign-semantics,db-meilisearch-gated-test-harness,macro-search-notification-reference-flows,meilisearch-vps-resource-ops}.md`) e 2 auditorias convergem; correção do predicado do índice parcial e staleness do Meilisearch exigem verificação PG18 (`research.md:6-13`). |
| Value vs. inaction | strong | Elimina duplicatas não-lidas sob concorrência/retries com migração pequena, melhora resiliência da busca sem novo serviço e amplia proteção contra regressões na CI (`decisão anterior:17`; `research.md:9`). |
| Feasibility / appetite | adequate | Índice parcial + `ON CONFLICT DO NOTHING` e alteração do job `db-tests` são baratos, mas exigem que `create` retorne `Option` e que callers tratem supressão como sucesso (`research.md:11-12`). |
| Strategic fit | strong | Mantém PG FTS como base transacional de correção e contrato operacional explícito de degradação, alinhado ao perfil Seed/VPS (`intake.md:6-9`; `research.md:8`). |
| Risk posture | adequate | Arbiter do índice deve casar com o `ON CONFLICT`; supressão geral de unread é hardening intencional e o frescor do Meilisearch saudável continua NV (`research.md:11-13`; `concept.md:9`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. S1 é o 80/20 do ciclo 022: índice único parcial `UNIQUE (user_id, event_type, target_id) WHERE is_read = false` com `INSERT ... ON CONFLICT ... DO NOTHING RETURNING` (`NotificationRepository::create` passa a devolver `Option`; supressão não é erro), eliminando a janela TOCTOU do guard do 021; manutenção da semântica U0 (unassign não emite `Assignment`) e entrega best-effort pós-persist; fail-open da busca para PG FTS em erro de consulta do Meilisearch, preservando precedência quando saudável; e extensão do job `db-tests` com `contract::search::db_`, `contract::notifications::db_` e `contract::files::db_` em PG18 descartável. Baseline `dev@b5be7d16f` (021 merged, PR #224, CI verde); convergência dos seis docs R1 e das auditorias (`ses_e1c0bc4e5`, `ses_e1c0bc4c8`). Hooks de indexação, task-await, rebuild, Meilisearch-gated CI e outbox/tx-coupling permanecem NV — o contrato de degradação é explícito sobre isso.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as quatro perguntas abaixo seguem como carried-forward, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Dedup de notificações depende de consulta best-effort sem garantia do banco, busca não degrada explicitamente em erro do backend preferencial e contratos DB-gated de busca/notificações/arquivos não são executados na CI (`problem.md:7-14`).
- **Chosen approach**: S1 — unread dedup imposto por índice parcial + `ON CONFLICT DO NOTHING`, search fail-open para `search_pg` em erro do Meilisearch com precedência inalterada quando saudável, e enforcement dos contratos DB-gated no `db-tests` (`concept.md:5-10`; `decisão anterior:9-20`).
- **In scope / out of scope**:
  - **In**: migração aditiva do índice parcial; `create` conflict-safe com `Option<Notification>` e callers atualizados; transições 021 e best-effort preservadas; fallback em erro de consulta com log de warning; contratos DB-gated de dedup/supressão e invariantes existentes; `.github/workflows/rust.yml` (`db-tests`, PG18, `--ignored --test-threads=1`); SDD/validação/release, PR/CI verde, merge/sync, planejamento para 023. Sem mudança de frontend.
  - **Out/NV**: hooks de indexação Meilisearch (task/file/message), delete-on-soft-delete, espera por tarefas assíncronas, rebuild/backfill, CI/harness ancorado no Meilisearch, equivalência de ranking/snippets, outbox/tx-coupling, política de coalescência/estado de leitura, evento específico para unassign, menções/completion/realtime/push, exposição de `target_url`, `pg_trgm`/i18n/CJK, ACL por canal/workspace, benchmarks 2vCPU/4GB, e2e/multi-client, novos serviços obrigatórios, convergência de crates, remoção do Meilisearch e infra multi-nó.
- **Success metrics**: Inserções concorrentes/repetidas do mesmo `(user_id,event_type,target_id)` não-lido não produzem mais de uma notificação ativa; erro de consulta no backend preferencial resulta em resposta coerente com a base PG sem degradar tenant/soft-delete; suítes DB-gated de busca, notificações e arquivos executam na CI em PG18; notificações legítimas de tipos/alvos distintos continuam visíveis (`problem.md:29-33`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: a supressão de duplicatas se aplica a todos os tipos de notificação não-lida ou apenas a `Assignment`?]; [NEEDS CLARIFICATION: após a leitura, uma nova notificação para o mesmo alvo deve ser permitida?]; [NEEDS CLARIFICATION: a degradação para PG em erro de consulta deve ser silenciosa ao cliente ou explicitada na resposta?]; [NEEDS CLARIFICATION: o frescor do Meilisearch quando responde com sucesso, mas desatualizado, deve ser tratado neste ciclo?] (`problem.md:39-42`).
