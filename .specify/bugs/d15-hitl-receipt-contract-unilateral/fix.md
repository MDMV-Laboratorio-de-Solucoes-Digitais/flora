# Bug Fix: D15 — contrato HITL unilateral

- **Slug**: d15-hitl-receipt-contract-unilateral
- **Proposto**: 2026-11-21
- **Assessment**: ./assessment.md
- **Status**: **proposta** — não aplicada (este ciclo é somente documental; implementação e verificação ficam no handoff)
- **Adjudicação**: ciclo 106 — `.specify/assessments/106-hitl-receipt-port/decision.md` (go)

## Summary

Dar à Flora o lado produtor que falta ao contrato HITL: um `HitlReceiptPort` puro no core, um adaptador emissor na API que gera o envelope exato de 8 campos com HMAC-SHA256 determinístico, validação fail-closed local espelhando a allowlist da Autônoma, ausência de credenciais/PII em log, e um teste round-trip com fixture consumida pelos validadores da factory (somente leitura).

## Envelope a emitir (espelho exato)

| # | Campo | Valor / regra | Origem na Autônoma |
|---|-------|---------------|--------------------|
| 1 | `work_order_id` | string não vazia; correlacionada ao `expected_work_order_id` | `delegation_contract.py:726-727,786-787` |
| 2 | `ativo_id` | string não vazia | `delegation_contract.py:727,746` |
| 3 | `station` | uma de `S0_RADAR` … `S6_TELEMETRY` (**MAIÚSCULAS**) | `delegation_contract.py:19-27,738-740` |
| 4 | `producer_system` | `"flora"` (`TargetSystem.FLORA_CORPORATE_SUITE`) | `delegation_contract.py:38`, `flora_bridge.py:160-166` |
| 5 | `status` | `"verified"` literal | `delegation_contract.py:790-791` |
| 6 | `artifacts_hash` | string não vazia (sha256 dos artefatos) | `delegation_contract.py:732,750` |
| 7 | `metrics_payload` | `dict` com `operator_approved: bool`, `operator_id: str` não vazio, `pii_redacted: true`, `real_external_send: false`, `next_stage` em `s0_radar` … `s6_telemetry` (**minúsculas**) | `flora_bridge.py:16-26,122-142` |
| 8 | `signature_hmac` | `hexdigest()` HMAC-SHA256 dos 7 campos não assinados | `delegation_contract.py:651-665,781-785` |

Canonicalização obrigatória: `json.dumps(payload, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")`, com segredo HMAC não vazio (`delegation_contract.py:658-665`). A ordem das chaves, a ausência de espaços e a preservação de UTF-8 fazem parte do contrato: qualquer desvio gera `invalid_signature`. O default de `next_stage`, quando ausente, é `s4_monetization` (`flora_bridge.py:128,179`) — o emissor DEVE sempre enviar o valor explícito.

## Changes propostos

| Arquivo | Mudança | Notas |
|---------|---------|-------|
| `flora-core/src/traits/hitl_receipt.rs` | novo | `HitlReceiptPort` + `HitlDecision`; validação pura e fail-closed antes da emissão; sem I/O, sem segredo, sem log. |
| `flora-core/src/traits/mod.rs` | + módulo e reexport | Segue o padrão de `storage_provider` (`mod.rs:3-9`). |
| `flora-core/src/config/hitl.rs` | novo | `HitlConfig` com `hmac_secret` lido de `FLORA_HITL__HMAC_SECRET` via figment (`config/mod.rs:49`); sem default inseguro; `Debug` redigido. |
| `flora-api/src/adapters/hitl_receipt.rs` | novo | Adaptador que serializa os 7 campos canônicos, calcula HMAC-SHA256 com `hmac 0.12`/`sha2 0.11` e devolve o envelope de 8 campos; nenhum `tracing` com segredo, `operator_id` ou payload. |
| `flora-api/src/` (wiring) | + injeção do adaptador no estado | Reaproveita `AppState`/config existentes; nenhuma rota HTTP nova. |
| `flora-tests/src/contract/hitl_receipt.rs` | novo | Vetor dourado de canonicidade/HMAC e casos de rejeição. |
| `scripts/contract/check_hitl_receipt_roundtrip.py` | novo | Harness somente leitura: lê a fixture emitida e aplica `verify_verification_receipt` + `ingest_flora_hitl_receipt` da factory; nenhuma credencial real. |
| `.specify/bugs/d15-hitl-receipt-contract-unilateral/*`, `.specify/assessments/106-hitl-receipt-port/*`, `specs/106-hitl-receipt-port/*` | novos | Rastreabilidade do ciclo 106. |

Nenhuma dependência nova: `sha2 = "0.11.0"` e `hmac = "0.12"` já estão em `Cargo.toml:171,192`. A factory `mdmv-dark-factory`/MDMV-Autonoma permanece somente leitura; nenhum validador, allowlist ou teste dela é alterado.

## Diff Highlights (ilustrativo)

```rust
// flora-core/src/traits/hitl_receipt.rs — port puro, sem I/O e sem segredo
pub trait HitlReceiptPort {
    fn emit(&self, decision: &HitlDecision) -> Result<HitlReceipt>;
}
// HitlDecision: work_order_id, ativo_id, station, artifacts_hash,
//              operator_approved, operator_id, pii_redacted,
//              real_external_send, next_stage
```

```python
# scripts/contract/check_hitl_receipt_roundtrip.py — somente leitura
verdict = verify_verification_receipt(fixture, secret=test_secret)
ingest = ingest_flora_hitl_receipt(fixture, secret=test_secret)
assert verdict["reason"] == "verified" and ingest["approved"] is True
```

## Regras locais de fail-closed (antes de assinar)

Rejeitar sem emitir: `station` fora de `VALID_STATIONS`; `next_stage` fora de `ALLOWED_RESUME_STAGES` (comparação em minúsculas); `work_order_id`, `ativo_id` ou `artifacts_hash` vazios; `operator_approved` não bool; `operator_id` vazio; `pii_redacted != true`; `real_external_send != false`; segredo vazio. `status` e `producer_system` são constantes do emissor (`"verified"`, `"flora"`). Nenhum campo extra é acrescentado ao envelope.

## Tests Added or Updated

- Round-trip positivo: recibo emitido pela Flora → `reason="verified"` no validador → `approved=True`, `reason="verified_flora_hitl_decision"`, `next_stage` da allowlist na ingestão.
- Vetor dourado: mesmo payload produz sempre o mesmo `signature_hmac` (64 hex minúsculos), provando `sort_keys`, `separators` e `ensure_ascii=False`.
- Um caso por reason de rejeição: `invalid_signature`, `invalid_next_stage`, `unredacted_pii_rejected_tier4`, `unapproved_external_send_rejected_tier4`, `missing_operator_approved_flag`, `missing_operator_id`, `receipt_not_verified`, `invalid_producer_system`, `missing_required_fields`, `invalid_station`, `work_order_mismatch`.
- Varredura de log/artefatos: 0 ocorrências do segredo de teste, de `operator_id` ou de PII em stdout/stderr/fixtures.
- Nenhum teste de rede, staging, IdP ou envio externo real.

## Local Verification

Não executada — proposta documental. A verificação completa está em `./test.md` e nos artefatos do ciclo 106; gates, build e testes do alvo ficam no handoff **[NV]**.

## Deviations from Assessment

- Nenhuma. A forma escolhida é a opção `Preferred` do assessment; o transporte ao vivo para `POST /receipts/flora-hitl` (spec 017:23-25) continua **[NV]** e fora do apetite.

## Follow-ups

- Definir o acionador real da decisão Tier 4 na Flora e o transporte até a Autônoma **[NV]**.
- Rodar o harness com o segredo de teste em CI e integrar o round-trip ao gate existente — somente após decisão explícita; nenhum required check é mudado neste ciclo.
- Rotação do segredo HMAC e reenvio idempotente são responsabilidade da Autônoma; a Flora garante apenas determinismo para o mesmo input.

### Correção de dependência (2026-11-22, append-only)

As menções a `hmac 0.12` acima estavam erradas: `hmac 0.12.1` usa `digest 0.10.7`, incompatível com `sha2 0.11.0` (`digest 0.11.3`). O `Cargo.lock` já contém `hmac 0.13.0` (`digest 0.11.3`, via `hkdf`/`sqlx`). A implementação usa `hmac = "0.13"` com `hmac.workspace = true` em `flora-core`; nenhuma dependência nova é baixada. O valor serializado de `producer_system` é literalmente `"flora"`, não o nome do enum Python.

### Fechamento (2026-11-22, append-only)

Implementação mesclada no ciclo 106 (`HitlReceiptPort` + `HitlReceiptAdapter` + modelo). O rótulo "proposta" no cabeçalho é histórico; status verificado em 2026-11-22: `cargo test -p flora-api hitl_receipt` → **2 passed**, exit 0.
