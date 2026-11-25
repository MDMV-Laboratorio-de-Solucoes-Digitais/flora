# Tasks: 120 — Endpoint HTTP RBAC para emissão de recibo HITL (`POST /api/v1/hitl/receipts`)

**Data**: 2026-11-24 · **Spec**: `specs/120-hitl-receipt-http-endpoint/spec.md`

- T-001 (Issue #411): Mapear `Error::ServiceUnavailable` para `503` sanitizado em `flora-core/src/error.rs` e implementar `flora-api/src/routes/hitl.rs` (`POST /api/v1/hitl/receipts`) montado em `flora-api/src/routes/mod.rs` com testes unitários [FR-001 / FR-002 / SC-001 / SC-002]
- T-002 (Issue #412): Implementar `flora-tests/src/contract/hitl.rs` (`3` contratos PostgreSQL 18 `db_hitl_*` via `oneshot`) registrado em `flora-tests/src/contract/mod.rs` e atualizar trava lockstep `hitl: 3` (`82 -> 85`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml` [FR-003 / SC-003]
- T-003 (Issue #413): Preencher Briefing de Compreensão (`auth/rbac/hitl`), governança, docs append-only (`BF-03 -> fechado`), R4 (`test-roadmap.md`, `evidence.md`, `validate.md`), R7 (`0.12.0` MINOR), ACR independente e promoção `dev -> main` [FR-004 / SC-003]
