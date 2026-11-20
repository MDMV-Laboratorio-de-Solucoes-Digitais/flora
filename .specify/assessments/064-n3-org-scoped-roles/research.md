# Research: 064 — N3 org-scoped roles

**Data:** 2026-11-18T11:14Z · **Evidência local + auditoria local**; produção **[NV]**.

## Primária (executada)

1. `psql -d flora`: 0 órfãos (`LEFT JOIN roles ... WHERE r.id IS NULL`),
   0 cross-org (`r.organization_id <> m.organization_id`), 3 memberships, 3 roles.
2. `role_repository.rs` — `find_by_id`, `update`, `delete` por `id` puro.
3. `rbac_service.rs:163-186` — `get_role(role_id)` sem org; chamado por
   `has_permission` (`:116`) e `is_admin` (`:158`).
4. `permission_service.rs:91-95,187-192,219-223` — buscas por id puro;
   `assign_role:136` já valida `role.organization_id == organization_id`.
5. `flora-api/src/routes/rbac.rs` — org vem da sessão (`require_org_admin`).
6. `20240818000001_init_schema.sql:40-49` — `roles.organization_id NOT NULL`;
   `:33` — `role_id` sem REFERENCES.

## Decisão

**Camada 1**: novo método `find_by_id_and_organization(id, org)` no repositório
e troca dos call sites que têm a org disponível; onde a assinatura não a tem,
passá-la a partir da sessão (escopo da autorização SE).

**Camada 2**: migration `UNIQUE(organization_id, id)` (aditiva) e migration
de FK composto com pré-voo `DO $$ ... RAISE EXCEPTION $$` — em produção suja,
o deploy falha fechado em vez de corromper/remover dados. `ON DELETE RESTRICT`
evita apagar memberships ao excluir um papel em uso.

## Cobertura idiomática

PT-BR (artefatos) + EN (código/SQL). Produção/remote: **[NV]**.
