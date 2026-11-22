# Release: 106 — Emissor de recibo HITL

**Ciclo**: 106 · **Data**: 2026-11-22 · **Branch**: `feature/106-hitl-receipt-port` @ `0582883da`

## Entregue

`HitlReceiptPort` + adaptador HMAC-SHA256 que emite o envelope de 8 campos exigido pela Autônoma, com allowlists, travas de privacidade, config redigida e teste com vetor dourado do validador Python. Nenhum transporte HTTP novo; nenhum gate/ADR/migration/credencial tocado.

## SemVer

**Proposta:** PATCH `0.7.7` → `0.7.8`. Nova porta e adaptador sem mudança de contrato existente, rota, schema ou config obrigatória (feature desligada sem segredo).

## Changelog (FR / T / bug)

- `FR-001` port puro e validação fail-closed (T002).
- `FR-002` envelope exato, `producer_system="flora"`, `status="verified"` (T004).
- `FR-003` segredo redigido em `FLORA_HITL__HMAC_SECRET` (T003).
- `FR-004` allowlists de station/next_stage e métricas (T002, T004).
- `FR-005` round-trip com vetor `ba563acc…` (T005).
- **Bug**: D15 do INBOX autorizado pelo SE (contrato HITL unilateral). Bug report em `.specify/bugs/d15-hitl-receipt-contract-unilateral/`.

## Evidência

R4 APROVADO · analyze/converge do ciclo · `docs/governance/2026-11-22-106-hitl-receipt-port.md` · `cargo test -p flora-api hitl_receipt` → 2 passed · gate PASS.

## Rollback

Novo commit de reversão dos arquivos do ciclo, sem force-push; reapontar a API para o build anterior se publicado. Nenhuma migration/variável/dado a desfazer.

## [NV]

Round-trip ao vivo, CI/deploy e credenciais reais.
