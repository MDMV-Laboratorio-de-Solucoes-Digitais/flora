# Feature Specification: Endpoint HTTP RBAC para emissão de recibo HITL (`POST /api/v1/hitl/receipts`)

**Feature ID**: 120 · **Status**: specified · **Data**: 2026-11-24

## User Scenario

Como administrador autenticado (`Permission::OrgAdmin`) de uma organização no Flora atuando como operador humano Tier 4, quero submeter uma decisão HITL via `POST /api/v1/hitl/receipts` e receber o recibo canônico assinado (`HitlReceipt`) em `201 Created`, com `operator_id` derivado exclusivamente da minha sessão autenticada, validação fail-closed da decisão e resposta `503 Service Unavailable` sanitizada quando a assinatura HMAC não estiver configurada.

## Acceptance Scenarios

1. **SC-001 (Autorização RBAC e anti-spoofing de `operator_id`)**: **Given** requisição para `POST /api/v1/hitl/receipts`, **when** enviada sem Bearer JWT (`401`), por membro sem `Permission::OrgAdmin` (`403`) ou contendo campo extra `operator_id` (`422` por `deny_unknown_fields`), **then** a API rejeita a emissão; **when** enviada por `OrgAdmin` autenticado com decisão válida, **then** vincula `operator_id = auth.user_id.to_string()`, assina com `HitlReceiptAdapter` e retorna `201 Created` com os 8 campos canônicos (`producer_system == "flora"`, `status == "verified"`, `signature_hmac`).
2. **SC-002 (Fail-closed em decisão inválida e segredo HMAC ausente sem vazamento)**: **Given** decisão com `pii_redacted: false`, `real_external_send: true`, `station` ou `next_stage` fora da allowlist, **when** submetida por `OrgAdmin`, **then** retorna `422 Unprocessable Entity` antes de checar o segredo; **given** decisão válida quando `FLORA_HITL__HMAC_SECRET` está vazio, **then** retorna `503 Service Unavailable` (`{"error":"service_unavailable","message":"Service temporarily unavailable"}`) sem expor nomes de variáveis ou segredos.
3. **SC-003 (Contratos PostgreSQL 18 e portões lockstep `82 -> 85`)**: **Given** suíte `flora-tests`, **when** rodam os portões locais e remotos, **then** `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo deny`, `cargo test --workspace`, `check_gate_integrity.py` (`85` pisos com `hitl: 3`), `check_deploy_workflows.py` e `contract::hitl::db::db_` (`3 passed; 0 failed`) passam integralmente.

## Requirements

- **FR-001**: Mapear `Error::ServiceUnavailable(_)` em `flora-core/src/error.rs` para HTTP `503 Service Unavailable` sanitizado com teste unitário dedicado.
- **FR-002**: Criar `flora-api/src/routes/hitl.rs` (`create_hitl_router`, `EmitHitlReceiptRequest`, `can_emit_hitl_receipt`, `build_decision`, `require_hitl_operator`, `emit_hitl_receipt`) com testes unitários e montá-lo em `/api/v1/hitl` em `flora-api/src/routes/mod.rs`.
- **FR-003**: Adicionar `flora-tests/src/contract/hitl.rs` (`flora-tests/src/contract/mod.rs`) com 3 contratos DB `db_hitl_*` e atualizar a trava lockstep `hitl: 3` (`82 -> 85`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
- **FR-004**: Produzir Briefing de Compreensão (`auth / rbac / hitl`), issues R6 (`#411`–`#413`), R4 (`test-roadmap.md`, `evidence.md`, `validate.md`), R7 (`0.12.0` MINOR), `converge.md`, docs append-only e ACR independente.

## [NV]

Deploy externo em staging/produção no Coolify e interface visual específica para operadores Tier 4 em `apps/frontend` permanecem fora deste ciclo.
