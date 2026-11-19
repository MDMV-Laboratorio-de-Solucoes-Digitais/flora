# Research: 070 — contrato RBAC real no lugar do stub

**Data:** 2026-11-19T02:58Z · **Evidência local + DB descartável executada nesta sessão** · remoto futuro **[NV]**.

## Primária (local)

1. `flora-tests/src/contract/rbac.rs:1-7` — único teste é `Ok(())`, sem router, sem DB, sem asserção.
2. `flora-api/src/routes/rbac.rs:86-107` — `can_list_roles`: `OrgRead || OrgAdmin`; `:112-117` — `can_manage_roles`: `RoleWrite || OrgAdmin`; `:120-123` — `can_modify_role`: `!is_builtin`.
3. `flora-api/src/routes/rbac.rs:128-145` — `require_org_admin` usa `auth.require_org()` (sessão), membership e role no mesmo `organization_id` — nenhum header de tenant.
4. `flora-api/src/routes/rbac.rs:168-320` — handlers reais; `RoleResponse` omite `organization_id`/timestamps (teste puro já existente em `rbac.rs:337-367`).
5. `flora-tests/src/contract/workspace.rs:5-7,38-52,134-215` — padrão canônico: `#[cfg(test)] mod db`, `db_state()` que `bail!` sem env, `#[tokio::test] #[ignore = "requires Postgres; ..."]`, `make_test_state` de `auth.rs:38`, seed de user/org/role/membership/session e `encode_token` com `config.app.jwt_secret`.
6. `flora_core::Permission` aceita `org:read`, `role:write`, `org:admin` (`flora-core/src/models/role.rs:208,245`); `Role::new` cria role customizada (`is_builtin=false`).
7. `pg_isready` → Postgres 18.6; banco descartável `flora_070_rbac_verify` criado nesta sessão (nenhum banco de outra sessão tocado).

## Desenho do teste

Três contratos DB-gated cobrindo o que o stub não cobre:
- listar roles do tenant da sessão e **não** vazar roles de outra organização;
- criar role exige `RoleWrite`/`OrgAdmin` e grava no `organization_id` da sessão;
- role built-in é imutável (PATCH → 403), enquanto role customizada aceita PATCH.

## NV

CI remota, Forgejo, produção e aprovações: **[NV]**. Cobertura idiomática: pt-BR (artefatos) + inglês (código/nomes de teste).
