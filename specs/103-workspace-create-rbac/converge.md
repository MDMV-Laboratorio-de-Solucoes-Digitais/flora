# Converge: 103 — RBAC na criação de workspace

**Data**: 2026-11-21 · **Branch**: `feature/103-workspace-create-rbac` @ `14cb336173`
**Modo**: artefato append-only.

## Entrada 1 — 2026-11-21 (implementação e convergência)

**Status: Converged — sem novas tarefas.**

### Mudanças entregues

- `flora-api/src/routes/workspace.rs`: `create_workspace` troca `_membership` por `membership`, carrega o papel via `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` e retorna `Error::Forbidden("Insufficient permissions")` quando `can_create_workspace(&role)` é falso. Helper puro `can_create_workspace` centraliza a regra `WorkspaceWrite | WorkspaceAdmin`.
- `flora-api/src/routes/workspace.rs` (`#[cfg(test)] mod tests`): dois testes unitários — wildcard/aliases permitidos; `[]`, `workspace:read` e `WorkspaceRead` negados.
- Import de `Permission`/`Role` adicionado; as três qualificações longas pré-existentes em `update_workspace`/`delete_workspace` foram curtas para satisfazer `-D unused-qualifications`. Comportamento inalterado nesses handlers.

### Decisões

1. Reaproveitar a membership já carregada: uma consulta extra apenas para o papel, sem outra leitura de membership.
2. Mesmo erro opaco de `update`/`delete`: sem revelar qual permissão faltou.
3. Nenhum `#[ignore]` novo em `flora-tests`: pisos do gate (77) e `FLOORS["workspace"]=3` permanecem; regressão coberta por testes unitários + HTTP ao vivo.

### Verificação observada

- `cargo fmt --all` e `--check` exit 0.
- `cargo check -p flora-api` 0 erros; `cargo clippy -p flora-api --all-targets -- -D warnings` exit 0.
- `cargo test -p flora-api workspace::tests` → **2 passed**, 36 filtrados.
- API ao vivo: papel `SomenteLeituraQA` (`permissions=[]`) → `POST /api/v1/workspaces` **403** `{"error":"forbidden","message":"Insufficient permissions"}`; Admin → **201**.

### Fronteira preservada

Nenhum toque em gate, required checks, constituição, ADR, schema, migration, flags, rotas novas, other cycles' validate/release, `STATE.md` histórico ou `PROGRESSO*` histórico. Nenhuma credencial real registrada.
