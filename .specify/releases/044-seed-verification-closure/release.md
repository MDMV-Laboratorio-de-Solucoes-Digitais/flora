# Release: 044 — Seed verification closure

**Ciclo:** 044 · **Data:** 2026-11-17 · **Baseline:** `dev@4ae58ccd2`.

## Entregue

- ADRs 0004–0006 (roles/RLS/rollout) com desenho validado em DB descartável;
  RLS enforcement permanece EVOLUTIVO (não ligado) — sem transação por request não é seguro.
- 5 contratos DB reais (workspace ×2, tasks ×3) substituindo stubs `Ok(())`.
- 2 bugs corrigidos: permissões do Member em `org.rs`, soft-delete em `get_task`.
- Decisão 80/20 + research do Seed (13 agentes) + validação.

## Evidência

`.specify/validations/044-seed-verification-closure/validate.md` ·
`docs/research/2026-11-17-044-seed-verification-closure.md`.

## Não feito (autonomia/Autorização)

Nenhum required check modificado, deploy/tag/segredos, RLS enforcement ligado,
FASE 7 Documents ou PGMQ/RustFS/Meilisearch instalados. Nenhuma alteração de outros
agentes revertida.

## Follow-ups

1. Autorizar promoção dos 5 contratos a legs `db-tests` (governança de required checks).
2. RBAC assign/revoke + último Owner (decisão do dono).
3. Wrapper de transação com `set_config` por request → then promover RLS (ADR-0005/0006).
4. Load/OOM 2vCPU/4GB, backup off-host, deploy Coolify — autorização do SE.
