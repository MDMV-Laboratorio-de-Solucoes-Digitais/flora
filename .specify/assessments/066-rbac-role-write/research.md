# Research: 066 — RBAC RoleWrite

**Data:** 2026-11-18T14:39Z · **Evidência local**; remoto **[NV]**.

## Primária (em mapeamento pelo verificador isolado)

- `flora-api/src/routes/rbac.rs:131-164` — placeholders com
  `let _org_id = require_org_admin(...)` e retorno `Json("created"/id)`.
- `rbac.rs:77-95` — `require_org_admin` usa `can_manage_roles(&role)`.
- `rbac.rs:102-128` — `list_roles` implementado com `can_list_roles`.
- Modelo `Permission` em `flora-core/src/models` — presença de `RoleWrite`
  a confirmar no mapeamento; se ausente, adicionar variante + serialization.
- `RoleRepository::{create,update,delete}` e `MembershipRepository` — assinaturas
  a confirmar para assign/revoke.

## Decisão

Implementar os quatro handlers com: (1) checagem da permissão `RoleWrite`
da sessão (tenant da org), (2) `Role::new(org_id, name)` para criação,
(3) imutabilidade de `is_builtin`, (4) assign/revoke via repositório de
membership, (5) DTOs serde para corpos de requisição.

## Alternativas descartadas

- Manter `OrgAdmin` como única autoridade: SE escolheu permissão dedicada.
- Permitir edição de builtin: fora da decisão SE.

## Cobertura idiomática

PT-BR (artefatos) + EN (código). CI/remote: **[NV]**.
