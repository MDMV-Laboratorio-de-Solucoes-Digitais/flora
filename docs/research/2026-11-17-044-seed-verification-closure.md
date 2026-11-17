# 044 — Seed verification closure: decisão 80/20 e evidência

**Data:** 2026-11-17 · **Metodologia:** hub `mdmv-dark-factory` aplicado ao Flora;
gates determinísticos como autoridade (ADR-0001/0003), auditoria fresca, ADRs 0004–0006.

## 1. Decisão 80/20

O objetivo é **alcançar o estágio Seed v0.1**. 13 agentes avaliaram as capabilities:
o núcleo está entregue com o mecanismo **normativo do ADR-0001** (Postgres FTS,
filesystem local, tabela de notificações — NÃO Meilisearch/RustFS/PGMQ, que são
aspiracionais e foram movidos para Sprout/Grove). O gargalo não é feature, é
**verificação parcial**: contratos DB de `org/workspace` (6+4) e `tasks` (5+2) eram stubs
`Ok(())`, e dois bugs reais foram encontrados pela conversão.

| Hipótese | Impacto | Confiança | Esforço | Veredito |
|---|---|---|---|---|
| H1 — ADRs 0004–0006 (roles/RLS/rollout) | Alto (governança) | Alta | Baixo | **FEITO** |
| H2 — Converter stubs org/tasks em contratos DB | Alto (gate Seed) | Alta | Médio | **FEITO (5 contratos)** |
| H3 — Corrigir bugs revelados (Member perms, soft-delete) | Alto (correção) | Alta | Baixo | **FEITO (2 bugs)** |
| H4 — FASE 7 Documents (~30 arquivos) | Médio | Média | Alto | Não (não é pré-requisito Seed) |
| H5 — Ligar RLS enforcement no runtime | Alto | Baixa (sem transação por request) | Alto | Não — ADR-0005/0006, EVOLUTIVO |
| H6 — Instalar PGMQ/RustFS/Meilisearch | Baixo | — | Alto | Não — ADR-0001, debloat/VPS |

## 2. Evidência dos 13 agentes (seed-01..13)

- **Reconciliação (seed-01):** ADR-0001:28-32 normatiza os substitutos; constituição e
  código/CI são verdade operacional (ROADMAP:5-6); a lista do usuário é prosa histórica
  pré-ciclo-025. "Alcançar Seed" = provar o núcleo Flora+PostgreSQL.
- **FASEs (seed-02):** 0–6 e 12 ✅/🔄 com mecanismo autoritativo; 7 Documents adiado
  (não pré-requisito); bug `cargo test`→0 testes **já corrigido** no ciclo 036 — não reabrir.
- **Capabilities:** Org/Workspace PARCIAL (bug Member); Users/Roles quase pronto (RBAC
  enforcement só em workspace.rs); Messaging pronto single-node (sem consumidor Valkey
  SUBSCRIBE); Tasks pronto (stubs); Notifications PRONTO (tabela Postgres); Files PRONTO
  (local FS); Search PRONTO (PG FTS); Infra PRONTO com NVs (Coolify/load/off-host);
  AGPL/FASE 0 verde.
- **RLS (rls-01..10, seed-13):** prova executada em DB descartável corrigiu o desenho
  (owner via superuser, users sem org_id, GUC não persiste em autocommit). ADRs escritos
  com enforcement EVOLUTIVO — ligar FORCE hoje zeraria queries.

## 3. Bugs reais corrigidos neste ciclo

1. `flora-api/src/routes/org.rs:155` — role Member criada com `["read","write"]`
   (strings não reconhecidas por `has_permission`); trocado por strings canônicas.
2. `flora-core/src/repositories/task_repository.rs:32` — `find_by_id` não filtrava
   `is_deleted`, então `GET /tasks/{id}` retornava 200 após soft-delete; corrigido.

## 4. Verificação executada

- 5 contratos DB reais passando em PG18 descartável (porta 55433):
  `contract::workspace::db::db_workspace_*` (2) e `contract::tasks::db::db_task_*` (3).
- Gates: fmt, clippy `--all-targets -D warnings`, deny, `cargo test --workspace`,
  `check_gate_integrity.py` — todos verdes.
- **NV / follow-up:** promover os 5 contratos a legs do `db-tests` exige autorização
  (muda conteúdo de required check, mapa de autorização). Load/OOM, backup off-host,
  deploy Coolify, real-OP OIDC, RBAC assign/revoke, RLS enforcement — NV/decisão do SE.

## 5. Veredito Seed

**Núcleo Seed v0.1: entregue e verificado** (ADR-0001 gate: fmt/clippy/tests + 6 suítes
DB + smoke + CI frontend, sem self-certificação). Declarar "Seed alcançado" requer
aceitar explicitamente os NVs listados — não instalar PGMQ/RustFS/Meilisearch.
