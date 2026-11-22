# Tasks: 106 — Port de recibo HITL

**Date**: 2026-11-22 · **Input**: `specs/106-hitl-receipt-port/{spec,plan}.md`, bug D15, INBOX autorizado.

**Format**: `[ID] Description` — T001..T008.

- [ ] T001 Fixar bug report D15 (bug/assessment/fix/test) e veredito `go`.
- [ ] T002 Criar `flora-core/src/models/hitl_receipt.rs` (8 campos + `HitlDecision`) e `flora-core/src/traits/hitl_receipt.rs` (`HitlReceiptPort` fail-closed), registrando módulos em `mod.rs`.
- [ ] T003 Criar `flora-core/src/config/hitl.rs` com `FLORA_HITL__HMAC_SECRET`, `Debug` redigido, `skip_serializing`, sem default inseguro; registrar em `Config::validate()`.
- [ ] T004 Criar adaptador HMAC em `flora-api` com `hmac 0.13` + `sha2 0.11`, JSON canônico e allowlists (station MAIÚSCULA, next_stage minúscula).
- [ ] T005 Teste com vetor dourado (`ba563acc…`) e rejeições; sem `#[ignore]`, sem segredo real.
- [ ] T006 Wiring no estado da API, sem rota HTTP nova e sem exposição de segredo em `Debug`/`tracing`.
- [ ] T007 Rodar `cargo fmt/check/clippy`, `cargo test`, `check_gate_integrity.py` e `git diff --check`.
- [ ] T008 Escrever analyze (READ-ONLY), converge (append-only), validate R4, release R7 e nota de governança; abrir PR para `dev/`.

**Dependencies**: T002→T003→T004→T005→T006→T007→T008. Fronteira: nenhum gate, required check, ADR, migration, staging, credencial ou validador da Autônoma é alterado.
