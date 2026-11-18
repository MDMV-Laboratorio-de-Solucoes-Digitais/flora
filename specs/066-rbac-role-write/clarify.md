# Clarify: 066 — RBAC RoleWrite

**Data:** 2026-11-18T14:41Z

## Resoluções

1. **RoleWrite já existe?** Sim, em `models/role.rs:173` (`role:write`);
   Admin/Owner do seed já a possuem — não precisa migration.
2. **OrgAdmin continua?** Sim: `RoleWrite || OrgAdmin` mantém admin legado.
3. **Revoke =?** remover a membership na org (assunção registrada).
4. **Identidade 066?** NNN por inventário/`--dry-run`.

## Pendências

Nenhum `NEEDS CLARIFICATION`.
