# Define — ciclo 120 (`120-hitl-receipt-http-endpoint`)

**Data**: 2026-11-24

- **Problema**: operadores administradores autenticados na Flora e integradores Tier 4 não possuem endpoint HTTP em `flora-api` para assinar e emitir o envelope `HitlReceipt` implementado por `HitlReceiptAdapter` (`BF-03` / D15).
- **Fronteira de segurança**:
  - Somente membros ativos da organização com `Permission::OrgAdmin` (ou `*`) podem emitir recibos HITL.
  - `organization_id` vem estritamente de `auth.require_org()`.
  - `operator_id` é vinculado estritamente no servidor via `auth.user_id.to_string()`; payload cliente não pode enviar `operator_id` nem campos desconhecidos (`#[serde(deny_unknown_fields)]`).
  - Se `FLORA_HITL__HMAC_SECRET` estiver ausente, decisões inválidas falham primeiro com `422 Unprocessable Entity` e decisões válidas falham fechado com `503 Service Unavailable` (`{"error":"service_unavailable","message":"Service temporarily unavailable"}`) sem vazar nomes de variáveis de ambiente ou segredos.
- **Critério de sucesso**: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo deny`, `cargo test --workspace`, `check_gate_integrity.py` (`85` pisos com `hitl: 3`), `check_deploy_workflows.py` + `--self-test` e `contract::hitl::db::db_` (`3/3`) verdes.
