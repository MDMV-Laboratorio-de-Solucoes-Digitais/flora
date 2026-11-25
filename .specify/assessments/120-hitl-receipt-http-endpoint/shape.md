# Shape — ciclo 120 (`120-hitl-receipt-http-endpoint`)

**Data**: 2026-11-24

## Desenho técnico enxuto

1. `flora-core/src/error.rs`: destacar `Error::ServiceUnavailable(_)` para HTTP `503 SERVICE_UNAVAILABLE` com corpo sanitizado `{"error":"service_unavailable","message":"Service temporarily unavailable"}` + teste unitário anti-vazamento.
2. `flora-api/src/routes/hitl.rs` + `flora-api/src/routes/mod.rs`:
   - Montar `.nest("/api/v1/hitl", hitl::create_hitl_router())` com `.route("/receipts", post(emit_hitl_receipt))`.
   - `EmitHitlReceiptRequest` (`#[serde(deny_unknown_fields)]`, sem `operator_id`/`organization_id`), helper puro `can_emit_hitl_receipt(role: &Role) -> bool`, helper puro `build_decision(req, operator_id: Uuid) -> HitlDecision`, guarda `require_hitl_operator(&state, &auth)` e conversão de `Error::Configuration(_)` para `Error::ServiceUnavailable(...)` após `validate_decision(&decision)?`.
   - `tracing::info!` sanitizado sem `signature_hmac`, `hmac_secret` ou `artifacts_hash`.
3. `flora-tests/src/contract/{mod.rs,hitl.rs}` + lockstep em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`: adicionar perna `hitl: 3` (`82 -> 85`).
