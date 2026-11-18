# Shape: 066 — RBAC RoleWrite

## Forma escolhida

Implementação em `flora-api/src/routes/rbac.rs`: DTOs serde; helper
`can_manage_roles` passa a aceitar `RoleWrite || OrgAdmin`; handlers reais
com repositórios; PATCH/DELETE para update/delete com guarda `is_builtin`.
`revoke_role` = remover a membership do usuário na organização (assunção
registrada). `assign_role` = delete+create da membership (sem método update).

## Alternativas descartadas

- `RoleWrite` sozinho sem OrgAdmin: quebraria admin legado.
- Editar builtin: SE decidiu imutável.
- Usar `RbacService::has_permission`: exige `&User`, não disponível na rota.

## Risco e reversão

Risco médio: endpoints antes no-op passam a mutar dados. Reversão: `git revert`.
Nenhum deploy separado; CI cobre compile/testes.

## Verificação

Testes unitários de autorização + gates + CI. DB real **[NV]** fora dos legs.
