# Intake — ciclo 120 (`120-hitl-receipt-http-endpoint`)

**Data**: 2026-11-24 · **Baseline**: `github/dev@e47983874` · **Modo**: Autônomo 80/20 (v4.9)

- **Solicitação / lacuna**: fechar `BF-03` expondo endpoint HTTP autenticado `POST /api/v1/hitl/receipts` sobre `HitlReceiptAdapter` / `HitlReceiptPort` (`flora-api/src/adapters/hitl_receipt.rs`, `flora-core/src/traits/hitl_receipt.rs`), já implementado internamente no ciclo 106 (Bug D15), porém sem rota HTTP consumidora no roteador principal.
- **Domínio crítico**: `auth / rbac / hitl` (autenticação por Bearer JWT, contexto de organização via `auth.require_org()`, permissão `Permission::OrgAdmin`, anti-spoofing de `operator_id`, emissão HMAC-SHA256 e resposta `503 Service Unavailable` sanitizada).
- **Meta de entrega**: endpoint `POST /api/v1/hitl/receipts` funcional em `flora-api`, `503` sanitizado em `flora-core/src/error.rs`, testes unitários, `3` contratos DB (`contract::hitl::db::db_*`) com trava lockstep `82 -> 85`, Briefing `auth/rbac/hitl`, R4/R7 (`0.12.0` MINOR) e ACR independente.
