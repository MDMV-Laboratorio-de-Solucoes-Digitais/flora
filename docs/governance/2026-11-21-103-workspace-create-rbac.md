# 103 — RBAC na criação de workspace

**Data**: 2026-11-21 · **Branch**: `feature/103-workspace-create-rbac` @ `14cb336173`
**Âncora**: `.specify/feature.json` → `specs/103-workspace-create-rbac`
**Gate do alvo**: `python3 scripts/ci/check_gate_integrity.py` — **PASS** (`exit=0`, 10 legs / 77 pisos)
**Fonte factory (somente leitura)**: `b1deab15f482dd85871cd6c6a857ad710b89e2a6`

## Sinal e evidência

Roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` item 11 exige que ações administrativas sejam bloqueadas para usuário sem privilégio. Em staging local zerado, papel `SomenteLeituraQA` (`permissions=[]`) atribuído a `qa-user2@flora.test` conseguiu `POST /api/v1/workspaces` → **201** (`NegadoQA`), enquanto `GET /api/v1/roles` → 403. `create_workspace` (`flora-api/src/routes/workspace.rs:121-154`) verificava apenas a existência de membership; `update_workspace` e `delete_workspace` já verificavam permissões.

## Decisão

Carregar o papel da organização da sessão via `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` e exigir `WorkspaceWrite` ou `WorkspaceAdmin` antes da gravação. Negação com `Error::Forbidden("Insufficient permissions")`, idêntica à de `update`/`delete`, sem vazar qual permissão faltou. Wildcard `*` e aliases legados continuam válidos (`Role::has_permission`).

Fora de escopo: gate, required checks, constituição, ADR, schema, migration, novas rotas/permissões, RBAC de outros handlers, staging real e deploy.

## Verificação observada

- `cargo fmt --check`, `cargo check -p flora-api`, `cargo clippy -p flora-api --all-targets -- -D warnings`: exit 0.
- `cargo test -p flora-api workspace::tests`: 2 passed (wildcard/aliases permitidos; vazio/somente leitura negados).
- Gate do alvo: PASS, 10 legs / 77 pisos.
- API ao vivo: `SomenteLeituraQA` → **403** `{"error":"forbidden","message":"Insufficient permissions"}`; Admin → **201**.

Rollback: novo commit de reversão do arquivo `flora-api/src/routes/workspace.rs`, sem force-push. Nenhuma migration ou variável a desfazer.

**[NV]**: staging real, CI remota, branch protection, PR/merge e deploy. Nenhum segredo registrado.
