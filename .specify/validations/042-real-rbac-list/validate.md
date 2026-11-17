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

## Auditoria author-blind (snapshot congelado `ce42216`)

Ver `/tmp/opencode/dark-factory-80-20/c042-02-author-blind-audit.md`. O auditor
confirmou: fmt/clippy/deny/`cargo test --workspace` (**107 passed, 0 failed**)/3 testes
rbac/`check_gate_integrity.py` todos verdes; tenancy vem só da sessão; DTO sem vazamento;
sem regressão dos required checks.

**🔴 D1 (ALTA) — CORRIGIDO.** O seed canônicoUSA `PascalCase` (`"OrganizationRead"`,
`dev-seed.sql:85`) mas `has_permission` comparava só `org:read` — todos os usuários
semeados, inclusive Admin, recebiam 403. Correção: `has_permission` agora aceita
ambos os formatos via `legacy_permission_name()` (`role.rs`), sem migration e sem
mudar dados. 3 novos testes em `role.rs` usam o payload **exato** do Admin do seed
(ponto cego que os testes originais tinham). Verificado:Admin e org:read/org:admin
passam, `*`, negativos e kebab-case também.

**🟡 D2 — CORRIGIDO.** nil-org retornava 422 (`Error::Validation`); trocado por
`Error::OrganizationContextRequired` → 400, conforme spec/validate.

**🟡/🔵 Follow-ups (não bloqueantes):** D3 `memberships.role_id` sem FK + `find_by_id`
sem filtro de org (defesa em profundidade, sem vazamento); D4 teste de contrato DB A/B;
D5 docs/`llms.txt` ainda citam `X-Organization-ID` (obsoleto).

## NV

- Teste de contrato DB-gated de isolamento A/B não adicionado (helper privado em `auth`;
  nenhum piso o exige) — segurança de tenancy provada por código + testes com dados reais.
- RLS/role least-privilege [NV]; assign/revoke/create deferidos ao dono.
- Drift residual do seed: permissões como `ChannelAdmin`, `UserRead`, `SearchRead` não
  têm equivalente no enum `Permission` — fora do escopo deste endpoint, registrado.
- Forgejo/remote CI [NV].
