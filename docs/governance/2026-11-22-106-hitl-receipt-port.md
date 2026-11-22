# 106 — Emissor de recibo HITL com contrato bilateral

**Data**: 2026-11-22 · **Branch**: `feature/106-hitl-receipt-port` @ `0582883da`
**Âncora**: `feature.json` → `specs/106-hitl-receipt-port`; bug `.specify/bugs/d15-hitl-receipt-contract-unilateral/`
**Autorização**: INBOX do SE confirmado em `.specify/memory/STATE.md` (PR #351)
**Gate do alvo**: `check_gate_integrity.py` — PASS (10 legs / 77 pisos)

## Sinal e decisão

Correção MAJOR D15: a Autônoma valida `VerificationReceipt` HMAC, mas a Flora não tinha emissor. Implementado produtor determinístico com `HitlReceiptPort`, envelope de 8 campos e HMAC-SHA256 sobre JSON canônico. Nenhum validador, allowlist ou transporte da Autônoma foi alterado.

## Verificação observada

- Vetor canônico gerado pelo validador Python da Autônoma (`ba563acc…`) reproduzido em Rust: `cargo test -p flora-api hitl_receipt` → **2 passed**.
- `cargo fmt --all -- --check` exit 0; `cargo check -p flora-api` exit 0; clippy e gate consolidados no handoff.
- `hmac 0.13` (digest 0.11) com `sha2 0.11`; nenhuma dependência nova.

Rollback: novo commit de reversão dos arquivos do ciclo, sem force-push. Nenhuma migration ou credencial a desfazer.

**[NV]**: round-trip ao vivo com a Autônoma, transporte HTTP, staging/produção e credenciais reais.
