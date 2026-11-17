# Release: 045 — Fecho cross-tenant (workspace_id do body)

**Ciclo:** 045 · **Data:** 2026-11-17 · **Baseline:** `dev@aa25f3f`.

## Entregue

- Bug cross-tenant em `POST /tasks` fechado: `workspace_id` do body validado contra a org da sessão antes da criação.
- Módulo compartilhado `tenant.rs` (`ensure_workspace_in_org`), reusado por files e tasks.
- Contrato DB `db_task_cross_org_workspace_rejected` (swap test OWASP).
- Decisão 80/20 + 6 relatórios de pesquisa + validação.

## Evidência

`.specify/validations/045-cross-tenant-workspace-fix/validate.md` ·
`docs/research/2026-11-17-045-cross-tenant-workspace-fix.md`.

## Não feito (autonomia/Autorização)

Nenhum required check, deploy, tag, RLS enforcement, ADR ou trabalho de outros agentes alterado.

## Follow-ups

1. Autorizar promoção dos contratos a legs `db-tests` (workspace 2, tasks 3 → piso 48).
2. Resíduos C1-C30 (logout frontend, `revoke_by_jti`, DTOs mortos, /users/invite, /telemetry/logs).
3. Wrapper de transação (ciclo A ~200 LOC) → depois promover RLS (ADR-0005/0006).
4. RBAC assign/revoke + último Owner (dono).
