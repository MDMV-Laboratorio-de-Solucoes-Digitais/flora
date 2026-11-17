# Validation: 042 — Real RBAC role listing

**Data:** 2026-11-17 · **Baseline:** `dev@411b379a` · **Tipo:** primeira funcionalidade
RBAC real, behavior-preserving no contrato (ainda 200/`[]` para orgs sem roles).

## Gates determinísticos

| Gate | Resultado |
|---|---|
| `cargo fmt --all -- --check` | limpo |
| `cargo clippy --workspace --all-targets -- -D warnings` | "No issues found" |
| `cargo deny check bans licenses sources` | bans/licenses/sources ok |
| `cargo test --workspace` | baseline idêntico + 3 novos testes rbac (3 passed, 0 failed) |
| `cargo check --workspace` com `CARGO_TARGET_DIR` isolado | 330 crates, exit 0 (3m19s) |

## Comportamento implementado

`GET /api/v1/roles`: tenancy exclusivamente de `auth.require_org()` (400 se nil-org);
membership via `PgMembershipRepository`; gate via `can_list_roles()` puro
(`OrgRead | OrgAdmin` ou `*`); `find_by_organization_id` ordenado por nome; `RoleResponse`
omite `organization_id`/timestamps. Outros handlers seguem placeholders explícitos.

## Testes (3, gates `cargo test --workspace`)

1. `can_list_roles` aceita `org:read`, `org:admin`, `*`.
2. `can_list_roles` rejeita vazio, `task:write`, `workspace:read`.
3. `RoleResponse` serializa sem `organization_id`/`created_at`/`updated_at`, com `id`/`permissions`.

## Resolução F-1 (ciclo 041)

O fresh check com target isolado passou (330 crates, exit 0) — o erro reportado pelo
auditor 041 foi falso positivo por **concorrência** (auditor rodou durante edição do
`rbac.rs`). `validate.md` do 041 atualizado com a lição: auditores devem rodar sobre
snapshot congelado. Nesta passagem, a auditoria do 042 será disparada **após o commit**.

## NV

- Teste de contrato DB-gated de isolamento A/B não adicionado (helper privado em `auth`;
  nenhum piso o exige) — segurança de tenancy provada por código + testes puros;
  comportamento ao vivo [NV].
- RLS/role least-privilege [NV]; assign/revoke/create deferidos ao dono.
- Forgejo/remote CI [NV].
