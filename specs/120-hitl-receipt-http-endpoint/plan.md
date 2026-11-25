# Plan — ciclo 120 (`120-hitl-receipt-http-endpoint`)

**Data**: 2026-11-24 · **Spec**: `specs/120-hitl-receipt-http-endpoint/spec.md`

| Requisito / SC | Arquivos | Verificação |
|---|---|---|
| FR-001 / FR-002 / SC-001 / SC-002 | `flora-core/src/error.rs`, `flora-api/src/routes/hitl.rs`, `flora-api/src/routes/mod.rs` | `cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` |
| FR-003 / SC-003 | `flora-tests/src/contract/mod.rs`, `flora-tests/src/contract/hitl.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml` | `python3 scripts/ci/check_gate_integrity.py` (`85` pisos) e `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` (`3/3`) |
| FR-004 | `docs/governance/2026-11-24-120-briefing-compreensao.md`, `docs/governance/2026-11-24-120-hitl-receipt-http-endpoint.md`, R4/R7 (`0.12.0`), docs append-only | ACR independente (`research`, `pro`) + CI remoto PR para `dev` + promoção `dev -> main` |
