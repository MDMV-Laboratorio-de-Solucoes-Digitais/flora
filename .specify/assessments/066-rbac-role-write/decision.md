# Decision: 066 — RBAC RoleWrite

**Verdict:** go (scoped — autorização SE em STATE 14:38Z)

## Decisão

Substituir os placeholders por CRUD real, com permissão `RoleWrite` dedicada
e `OrgAdmin` mantida como autoridade padrão; builtins imutáveis.
Nenhuma decisão adicional de autorização é necessária.

## Delimitadores

- 1 arquivo de rota + artefatos; sem RLS/produção/ADR/constituição.

## NV

Execução real em DB/PR: **[NV]** — CI observada antes do merge.
