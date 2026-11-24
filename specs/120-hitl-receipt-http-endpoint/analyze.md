# Analyze — ciclo 120 (`120-hitl-receipt-http-endpoint`)

**Data**: 2026-11-24 · **Modo**: read-only

- **Aderência `spec ↔ plan ↔ tasks ↔ issues`**: `FR-001..FR-004` e `SC-001..SC-003` cobrem integralmente `T-001` (`#411`), `T-002` (`#412`) e `T-003` (`#413`), sem tarefas órfãs nem alterações de UI em `apps/frontend`.
- **Invariantes críticos checados**:
  - `HitlReceiptAdapter` (`flora-api/src/adapters/hitl_receipt.rs`) retorna `Error::Configuration` quando `hmac_secret` está vazio; o handler `emit_hitl_receipt` deve validar `validate_decision(&decision)?` antes para preservar `422` em entradas inválidas e mapear `Error::Configuration` para `Error::ServiceUnavailable(...)` (`503` sanitizado), evitando falso `500` e vazamento de `FLORA_HITL__HMAC_SECRET`.
  - `#[serde(deny_unknown_fields)]` em `EmitHitlReceiptRequest` impede envio cliente de `operator_id` ou `organization_id` (Axum retorna `422`).
  - Atualização lockstep obrigatória `hitl: 3` (`82 -> 85`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml` para evitar falha em `rust-build` e `db-tests`.
- **Conclusão**: pronto para implementação (`0` conflitos).
