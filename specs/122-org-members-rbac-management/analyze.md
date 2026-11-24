# Analyze: `122-org-members-rbac-management`

- **Roteamento Axum**: Em `flora-api/src/routes/rbac.rs`, `.route("/members", ...)` precisa ser declarada explicitamente (como rota literal no router `{id}` não captura `"members"`, e em versões do `matchit`/Axum rotas estáticas precedem parâmetros tipados; ainda assim posicionar `/members` antes de `/{id}` torna a precedência inequívoca).
- **Segurança em `assign_role` / `revoke_role`**: O `assign_role` anterior removia e recriava a linha em `memberships`, apagando `joined_at` e `metadata` e permitindo inserir usuário alheio à organização sem usar o fluxo explícito de adição; trocar para `update_role` garante atomicidade (`UPDATE ... WHERE user_id = $1 AND organization_id = $2`).
- **Auto-lockout**: Bloquear `req.user_id == auth.user_id` em `assign_role` e `revoke_role` com `403 Forbidden` impede que o único administrador rebaixe ou remova a si mesmo via UI ou API.
- **Lockstep DB**: Subir `rbac` de `3` para `5` (`85 -> 87`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml` simultaneamente no mesmo PR.
