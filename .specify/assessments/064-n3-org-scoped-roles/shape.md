# Shape: 064 — N3 org-scoped roles

## Forma escolhida

Duas camadas, na ordem autorizada pelo SE:

1. **Código**: novo `find_by_id_and_organization(id, org)` no trait/impl;
   `RbacService::get_role` passa a receber a org e valida o cache; troca dos
   call sites em `permission_service`, `rbac.rs`, `workspace.rs`, `org.rs`.
   Assinaturas públicas de `list_permissions`/`update_role_permissions`
   ganham `organization_id` (sem chamadores externos — verificado).
2. **Migrations**: `UNIQUE(organization_id, id)` em `roles` (aditiva) e FK
   composto com pré-voo `RAISE EXCEPTION` se houver órfãos/mismatch;
   `ON DELETE RESTRICT`.

## Alternativas descartadas

- Só FK sem camada 1: banco protegeria, mas app ainda resolveria papel errado
  se o banco estiver sujo.
- Limpeza automática de dados: não autorizada; sujeira volta ao SE.
- Habilitar RLS: C6, separado e sem produção.

## Risco e reversão

Risco: assinaturas públicas de serviço mudam (sem chamadores externos no repo);
`RoleNotFound` passa a ocorrer para papéis legados sem org — comportamento
correto e fail-closed. Reversão: `git revert` dos commits; migrations são
aditivas e reversíveis via `DOWN` equivalente se necessário.

## Verificação

Teste de cross-org rejeitado, migrations em DB limpo (FK rejeita insert
inválido), gates e auditoria registrada. Produção **[NV]**.
