# Converge: 066 — RBAC RoleWrite

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T14:50Z — entrada 1

- Autorização SE registrada em `STATE.md` 14:38Z antes do BLOCO 6; analyze PASS.
- Implementação em `flora-api/src/routes/rbac.rs`: DTOs (`CreateRoleRequest`,
  `UpdateRoleRequest`, `UserRoleRequest`), validação de permissões via
  `Permission::from_str`, `can_manage_roles = RoleWrite || OrgAdmin`, guarda
  pura `can_modify_role` para builtins, CRUD real (POST/GET/PATCH/DELETE) e
  assign/revoke com repositório de membership.
- `revoke_role` = remover a membership do usuário na org (assunção registrada);
  `assign_role` = delete+create da membership (sem método update no repositório).
- Nenhum ADR/RLS/constituição/workflow/migration alterado; nenhum deploy.
- Correções de lint (`const fn`, `String::from`, import não usado) commitadas
  após `clippy` inicial RED; gates finais verdes: guardião PASS, fmt/clippy/
  deny/build/test exit 0, frontend 0/0/lint/42/42/build.
- Testes unitários: 5 passam (inclui `role:write`, `org:admin`, builtin).
- Factory fonte `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3`; nada transplantado.
