# Validation: 064 — N3 org-scoped roles

**Data:** 2026-11-18T11:29Z · **Baseline:** `github/dev@448090327`
**Tipo:** correção de isolamento multi-tenant + migrations; sem deploy.

## Gates e verificações (2026-11-18T11:20–11:28Z)

| Gate | Resultado |
|---|---|
| Auditoria read-only `flora` | órfãos 0, cross-org 0, memberships 3, roles 3 |
| `psql` em DB limpo: constraints | `uq_roles_org_id` (u), `fk_memberships_role_org` (f) |
| Insert cross-org | **rejeitado** pela FK |
| Insert órfão | **rejeitado** pela FK |
| Delete papel em uso | **rejeitado** (`ON DELETE RESTRICT`) |
| Migration em DB sujo | **abortou**: `N3: cross-tenant membership/role pair` |
| `db_n3_cross_org_role_lookup_rejected` | PASS (cross None, próprio Some) |
| 8 legs `db-tests` com DB real | 51 testes, 0 falhas (workspace piso 3) |
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 8 pisos / 51 total |
| `cargo fmt/clippy/deny/build/test` | exit 0 (após correção de cache/fmt) |
| `bun run check/lint/test/build` | 0/0, lint ok, 42/42, build ok |
| Verificador isolado | PASS 7/7 |

## Evidência

- Diff: 7 fontes core/API, 2 migrations, 1 teste, `rust.yml`, guardião —
  nenhum ADR/RLS/config de produção alterado.
- Nenhum `find_by_id(membership.role_id)`/`find_by_id(role_id)` restante em
  serviços/rotas; cache de papéis valida a organização.
- Produção **[NV]**: sem acesso para auditoria/deploy. A migration falha
  fechado se houver dados inválidos no deploy — comportamento desejado.

## SDD

Autorização SE em `STATE.md` antes do BLOCO 6; `analyze` PASS; R1 `go (scoped)`;
FR 4/4, SC 4/4, T001–T005 concluídas; `converge.md` com entrada 1.
Scripts SDD da factory (HEAD `ce3d4213`, em `/tmp`): completeness/DAG/task-id
RED por dívida legada — informativo; converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR ao vivo [NV]).** R4 aprovado
antes de R7. CI do PR observada ao vivo no Bloco 7 antes do merge (ADR-0003).

## NV

Dados de produção, deploy da migration e aprovação de limpeza: **[NV]**.
Sem merge/auto-merge/tag/deploy nesta fase.
