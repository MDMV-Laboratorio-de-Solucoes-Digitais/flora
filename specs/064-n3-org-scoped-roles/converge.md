# Converge: 064 — N3 org-scoped roles

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T11:29Z — entrada 1

- Decisão SE registrada em `STATE.md` 11:13Z (antes do BLOCO 6); R1 `go (scoped)`;
  analyze PASS. Verificador de código mapeou 9 call sites.
- **Camada 1 (código)**: novo `find_by_id_and_organization(id, org)` no trait
  e impl; `RbacService::get_role(role_id, org)` com validação da org no cache;
  troca em `permission_service` (4), `rbac.rs` (2), `workspace.rs` (2),
  `org.rs` (1). `list_permissions`/`update_role_permissions` ganharam
  `organization_id` (sem chamadores externos no repo).
- **Camada 2 (migrations)**: `UNIQUE(organization_id, id)` em `roles` e FK
  composto `(organization_id, role_id) → roles(organization_id, id)` com
  `ON DELETE RESTRICT` e pré-voo `RAISE EXCEPTION` para órfãos/cross-org.
- **Auditoria local** (11:12Z, DB `flora`): órfãos 0, cross-org 0,
  3 memberships, 3 roles. **Produção não auditada — [NV]**.
- **Validação em DB temporário limpo**: FK rejeitou insert cross-org e órfão;
  `RESTRICT` impediu excluir papel em uso; deploy sujo abortou com
  `N3: cross-tenant membership/role pair`. Teste contratual
  `db_n3_cross_org_role_lookup_rejected` PASS; 8 legs = 51 testes verdes
  (workspace 2→3, com pisos espelhados em `rust.yml`/guardião).
- Verificador isolado PASS 7/7; `fmt`/`clippy` ficaram RED no 1º ciclo e
  foram corrigidos (cache com `.filter()` + formatação), revalidados exit 0.
- Nenhum deploy, limpeza de dados, ADR, RLS (C6) ou RoleWrite (C5) neste ciclo.
- Factory fonte `ce3d42132edb9466b75eab968be775ebdea38985`; nada transplantado.
