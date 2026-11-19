# Decision: 070 — contrato RBAC real no lugar do stub

**Verdict:** go (scoped)

## Decisão

Substituir o contrato vácuo por três contratos DB-gated, restritos a `flora-tests/src/contract/rbac.rs`, conforme `shape.md`. Nenhuma autorização adicional: é teste, com DB descartável criado por esta sessão e sem mudança de comportamento.

## Delimitadores

- Zero alteração em `flora-api/`, `flora-core/`, migrations, workflows, required checks, ADRs ou constituição.
- Nenhum piso de `db-tests` alterado; nenhum job renomeado.
- Merge apenas com CI verde observada ao vivo; sem auto-merge/force-push.

## NV

CI futura, Forgejo, produção e aprovações: **[NV]**.
