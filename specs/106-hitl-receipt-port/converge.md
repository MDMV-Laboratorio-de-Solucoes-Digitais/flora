# Converge: 106 — Port de recibo HITL

**Data**: 2026-11-22 · **Branch**: `feature/106-hitl-receipt-port` @ `0582883da`
**Modo**: append-only.

## Entrada 1 — 2026-11-22 (implementação e convergência)

**Status: Converged — sem novas tarefas.**

### Mudanças entregues

- `flora-core/src/models/hitl_receipt.rs`: `HitlDecision`, `HitlReceipt` (8 campos), allowlists, `validate_decision`, `metrics_payload` e `unsigned_payload` com JSON canônico.
- `flora-core/src/traits/hitl_receipt.rs`: `HitlReceiptPort` puro (`Send + Sync + Debug`).
- `flora-core/src/config/hitl.rs`: segredo `FLORA_HITL__HMAC_SECRET`, `Debug` redigido, `skip_serializing`, feature desligada quando vazio.
- `flora-api/src/adapters/hitl_receipt.rs`: HMAC-SHA256 (`hmac 0.13` + `sha2 0.11`), assinatura hex minúscula, `producer_system="flora"`, `status="verified"`.
- `flora-api/src/state/app_state.rs`: fábrica `hitl_receipt_adapter()` sem novo campo no `AppState` (evitou 16 literais de teste).
- `Cargo.toml` / `flora-api/Cargo.toml`: `hmac` 0.12 → 0.13 e declaração de `hmac`/`sha2` na API; `mime_guess` preservado.
- Testes unitários com vetor dourado do validador Python e casos de rejeição.

### Decisões e correções

1. Nenhum `#[ignore]` novo: pisos do gate preservados (10 legs / 77).
2. Nenhum segredo/PII em `Debug`, `tracing`, erro ou fixture — apenas o segredo de teste explícito no código de teste.
3. `producer_system` gravado literalmente como `"flora"` (vetor provou que `str()` da StrEnum devolve o valor).
4. Nenhum transporte HTTP novo: apenas o produtor determinístico; integração ao vivo permanece **[NV]**.

### Verificação observada

- `cargo check -p flora-api` exit 0; `cargo test -p flora-api hitl_receipt` → **2 passed**; assinatura do vetor confere.
- `cargo fmt --all -- --check` exit 0; gate do alvo e clippy registrados no handoff.

### Fronteira

Nenhum validador/secret/allowlist da Autônoma ou da factory alterado; nenhum gate/ADR/migration/credencial real tocado.
