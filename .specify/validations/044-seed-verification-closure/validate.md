# Validation: 044 — Seed verification closure

**Data:** 2026-11-17 · **Baseline:** `dev@4ae58ccd2` · **Tipo:** contratos DB reais + 2 bugs + 3 ADRs.

## Gates

| Gate | Resultado |
|---|---|
| `cargo fmt --all -- --check` | limpo |
| `cargo clippy --workspace --all-targets -- -D warnings` | "No issues found" |
| `cargo deny check bans licenses sources` | ok |
| `cargo test --workspace` | verde (35 passed / 48 ignored no crate principal) |
| `python3 scripts/ci/check_gate_integrity.py` | PASS (4 required checks + piso 43 intacto) |

## Contratos DB executados (PG18 descartável :55433)

- `contract::workspace::db::db_workspace_list_returns_seeded_workspace` — PASS
- `contract::workspace::db::db_workspace_isolation_between_orgs` — PASS (sem cross-org leak)
- `contract::tasks::db::db_task_create_and_get_roundtrip` — PASS
- `contract::tasks::db::db_task_patch_status_and_assignee_tristate` — PASS (status + tri-state)
- `contract::tasks::db::db_task_soft_delete_hides_from_get` — PASS (após correção)

## Bugs corrigidos (encontrados pelos contratos)

1. `org.rs:155` — Member permissions `["read","write"]` → strings canônicas.
2. `task_repository.rs:32` — `find_by_id` filtra `AND NOT is_deleted`.

## ADRs

0004 (role separation), 0005 (RLS fail-closed — enforcement EVOLUTIVO, não ligado),
0006 (rollout sem condicionar required checks) — todos com desenho corrigido pela prova
em DB descartável (`/tmp/opencode/dark-factory-80-20/rls-10-executed-proof.md`).

## NV / não autorizado

Promover os 5 contratos a legs `db-tests` (muda required check), deploy Coolify,
tag/release, segredos, RBAC assign/revoke, RLS enforcement, load/OOM, backup off-host,
real-OP OIDC. Nada de outros agentes foi revertido; working tree limpo antes das edições.
