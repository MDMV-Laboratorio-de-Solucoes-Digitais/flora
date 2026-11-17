# Decision: 042 — Real RBAC role listing

**Verdict:** go (scoped)

## Chosen slice
Approve DTO `RoleResponse` (id, name, description, permissions, is_builtin) and the
authorization model restricted to `GET /roles`: tenancy exclusively from the verified
session via `require_org()`, membership + `OrgRead | OrgAdmin` (or `*`) via a pure
`can_list_roles()` helper. Organizations with no roles return `[]` (contract preserved).

## Deferred (owner decisions)
- `create/get/{id}/assign/revoke` role — semantics, DTO, `revoke_role` unimplemented,
  last-Owner protection (c041-02).
- RLS / least-privilege role (c041-03) — `[NV]` until SE authorization.

## Non-goals / non-claims
No full RBAC enforcement across routes, no PermissionService in AppState, no
RLS/role change, no ADR/constitution/required-check change, no deploy/tag/release.
Do NOT claim more than this one endpoint is real.

## NV
DB-gated isolation test not added (module-private helper; no piso requires it);
remote CI [NV]. ADR-0003 makes merge a machine decision under 4 green required checks.
