# Research — ciclo 120 (`120-hitl-receipt-http-endpoint`)

**Data**: 2026-11-24 · **Escopo**: diagnóstico local determinístico em `github/dev@e47983874`

## Achados no código-fonte

1. **Port e adaptador já prontos e verificados**:
   - `flora-core/src/models/hitl_receipt.rs`: define `HitlDecision`, `HitlReceipt`, `VALID_STATIONS`, `ALLOWED_RESUME_STAGES`, `validate_decision`, `unsigned_payload` e `metrics_payload`.
   - `flora-core/src/traits/hitl_receipt.rs`: define `HitlReceiptPort::emit(&self, decision: &HitlDecision) -> Result<HitlReceipt>`.
   - `flora-api/src/adapters/hitl_receipt.rs`: `HitlReceiptAdapter::emit` valida a decisão, exige `FLORA_HITL__HMAC_SECRET` não vazio (erro `Error::Configuration` se vazio) e assina o JSON canônico com HMAC-SHA256.
   - `flora-api/src/state/app_state.rs:62-64`: expõe `AppState::hitl_receipt_adapter(&self)`.
2. **Lacuna HTTP e mapeamento de erro**:
   - `flora-api/src/routes/mod.rs`: não monta rota HITL (`hitl`).
   - `flora-core/src/error.rs:360`: `Error::ServiceUnavailable(_)` caía no ramo `500 INTERNAL_SERVER_ERROR` expondo `self.to_string()`.
3. **Sem alteração de schema SQL**: emissão de recibos é stateless e usa exclusivamente as tabelas já existentes (`users`, `memberships`, `roles`, `sessions`).
4. **Sem alteração de UI**: nenhuma mudança em `apps/frontend`; evidência será CLI/HTTP/DB conforme Bloco 0.3.
