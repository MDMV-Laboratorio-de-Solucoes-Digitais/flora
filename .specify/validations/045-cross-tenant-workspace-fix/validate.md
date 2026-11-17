# Validation: 045 — Fecho cross-tenant (workspace_id do body)

**Data:** 2026-11-17 · **Baseline:** `dev@aa25f3f` · **Tipo:** correção de segurança + contrato DB.

## Gates

| Gate | Resultado |
|---|---|
| `cargo fmt --all -- --check` | limpo |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | ok |
| `cargo test --workspace` | verde (13 suítes) |
| `python3 scripts/ci/check_gate_integrity.py` | PASS (piso 43 intacto) |

## Contratos DB (PG18 descartável :55433 / flora_045)

- `db_task_create_and_get_roundtrip` — PASS
- `db_task_patch_status_and_assignee_tristate` — PASS
- `db_task_soft_delete_hides_from_get` — PASS
- `db_task_cross_org_workspace_rejected` — **PASS** (próprio → 201; workspace de outra org → 4xx; assignee de outra org → 4xx)
- 6 contratos `contract::files::db_` — PASS (helper movido não regrediu)

## Bug fechado

`POST /tasks` com `workspace_id` de outra org retornava 201; agora retorna 4xx.
A auditoria author-blind (GO) apontou um resíduo BOLA: `assignee_id` do body também
não era validado. Corrigido em acompanhamento: `ensure_user_in_org` em `tenant.rs`,
aplicado em `create_task` e no tri-state do `PATCH` (só no `Some(uuid)`, nunca no
`null` de unassign); contrato estendido para cobrir ambos os vetores.
Classificação: A01:2021 / CWE-639 / BOLA. Fix centralizado em `tenant.rs`.

## NV / follow-ups

Promover contratos a `db-tests` (autorização SE); logout/401/invite/resíduos C1-C30;
wrapper de transação para RLS (2 ciclos, ADR-0006); RBAC assign/revoke (dono).
