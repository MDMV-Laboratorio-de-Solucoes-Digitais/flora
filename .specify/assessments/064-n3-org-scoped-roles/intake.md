# Intake: 064 — N3 org-scoped roles

**Data:** 2026-11-18T11:14Z · **Âncora:** `github/dev@448090327` · branch `feature/064-n3-org-scoped-roles`

## Sinal

`memberships.role_id` não tem FK (`20240818000001_init_schema.sql:33`) e a
resolução de papéis usa `find_by_id(id)` sem escopo por organização. Isso
permite, em tese, uma membership da org A referenciar um papel da org B e
conferir permissões de B dentro de A — escalada cross-tenant latente.

## Autorização SE (registrada em STATE 11:13Z)

1. camada 1 em código (org-scoping);
2. `UNIQUE(organization_id, id)` em `roles`;
3. auditoria read-only de órfãos/mismatch;
4. FK composto com `ON DELETE RESTRICT` se a auditoria estiver limpa;
5. se houver sujeira, retornar dados para escolha de limpeza.

## Evidência (verificada)

- Auditoria em `flora` (local, 11:12Z): `orfaos=0`, `cross_org=0`,
  `memberships=3`, `roles=3`. **Produção não auditada — [NV]**.
- `rbac_service::get_role`, `permission_service::{check_permission,
  list_permissions, update_role_permissions}` e `role_repository::{find_by_id,
  update, delete}` resolvem por id puro; `assign_role` já possui guarda de org (`permission_service.rs:136`).
- **NV**: `docs/adr/0006:27-31` exige SE para deploy; ADR/C5/C6 autorizados em ciclos separados.

## 80/20

`(I×C)/(E×R)` = (4×5)/(5×3) = **1.33**, candidato autorizado de maior prioridade.

## Fronteiras

Escopo: repositório/serviços/rotas, 2 migrations, testes, artefatos do ciclo.
Fora: deploy em produção, limpeza de dados, ADRs, C5/C6. Factory `ce3d4213…`.
