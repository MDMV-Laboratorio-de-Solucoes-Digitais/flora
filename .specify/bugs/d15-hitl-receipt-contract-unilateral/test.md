# Bug Verification: D15 — contrato HITL unilateral

- **Slug**: d15-hitl-receipt-contract-unilateral
- **Plano**: 2026-11-21
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md (proposta, não aplicada)
- **Result**: **plano de teste round-trip** — nenhuma verificação executada neste artefato; handoff **[NV]**

## Summary

Plano determinístico e sem credenciais para provar que o recibo emitido pela Flora é aceito pelo contrato da Autônoma e que toda violação é rejeitada fail-closed. O harness lê uma fixture emitida pelo adaptador e aplica, em processo, `verify_verification_receipt` (`MDMV-Autonoma/src/contracts/delegation_contract.py:755-793`) e `ingest_flora_hitl_receipt` (`MDMV-Autonoma/src/contracts/flora_bridge.py:145-200`) da factory somente leitura. Não há requisição HTTP, staging, segredo real nem envio externo. Status de cada linha abaixo: **planejado**.

## Cenários round-trip

| ID | Cenário | Ação | Resultado esperado | Status |
|----|---------|------|--------------------|--------|
| RT-001 | Caminho feliz | Emitir decisão válida (station `S3_VERIFICATION`, `next_stage` `s4_monetization`, `pii_redacted=true`, `real_external_send=false`, `operator_approved=true`, `operator_id` não vazio) e validar a fixture | `verify`: `valid=True`, `reason="verified"`, `producer_system="flora"`; `ingest`: `valid=True`, `approved=True`, `reason="verified_flora_hitl_decision"`, `next_stage="s4_monetization"` | planejado |
| RT-002 | Canonicidade/HMAC | Repetir emissão do mesmo input; comparar bytes JSON e assinatura | JSON com chaves ordenadas, sem espaços, UTF-8 (`sort_keys`, `separators=(",",":")`, `ensure_ascii=False`); `signature_hmac` idêntico, 64 hex minúsculos; 8 campos, nenhum extra | planejado |
| RT-003 | Assinatura inválida | Alterar 1 byte de um campo não assinado sem recalcular HMAC | `valid=False`, `reason="invalid_signature"`, `approved=False` | planejado |
| RT-004 | `next_stage` fora da allowlist | Usar `s7_billing` e, separadamente, `S4_MONETIZATION` (caixa errada) | `reason="invalid_next_stage"`; nenhum `resume()` | planejado |
| RT-005 | PII não redigido | `pii_redacted=false` | `reason="unredacted_pii_rejected_tier4"`, `approved=False` | planejado |
| RT-006 | Envio externo | `real_external_send=true` | `reason="unapproved_external_send_rejected_tier4"`, `approved=False` | planejado |
| RT-007 | Operador ausente | `operator_approved` não bool; depois `operator_id=""` | `reason="missing_operator_approved_flag"`; depois `reason="missing_operator_id"` | planejado |
| RT-008 | Status divergente | `status="pending"` | `reason="receipt_not_verified"`, `valid=False` | planejado |
| RT-009 | Produtor divergente | `producer_system="mdmv-autonoma"` | `reason="invalid_producer_system"`, `approved=False` | planejado |
| RT-010 | Envelope incompleto | Remover `ativo_id` e `artifacts_hash` | `reason="missing_required_fields"` com `missing_fields` contendo ambos | planejado |
| RT-011 | Estação inválida | `station="s3_verification"` (minúscula) | `reason="invalid_station"` — contrasta com RT-001, onde a maiúscula é aceita | planejado |
| RT-012 | Correlação | `expected_work_order_id` diferente do emitido; depois `expected_station` diferente | `reason="work_order_mismatch"`; depois `reason="station_mismatch"` | planejado |
| RT-013 | Segredo ausente | Chamar validadores com `secret=""` | `reason="missing_hmac_secret"`, `valid=False` | planejado |
| RT-014 | Auditoria | Fornecer `audit_store` no caso feliz | 1 evento `flora_hitl_receipt_ingested` com `work_order_id`, `ativo_id`, `operator_id`, `operator_approved`, `next_stage` (`flora_bridge.py:181-191`) | planejado |

## Cenários de segurança e escopo

| ID | Cenário | Ação | Resultado esperado | Status |
|----|---------|------|--------------------|--------|
| RT-015 | Zero credenciais em log | Capturar stdout/stderr do emissor e do harness; varrer segredo de teste, `operator_id` e PII | 0 ocorrências; logs contêm apenas identificadores não sensíveis | planejado |
| RT-016 | Zero credenciais em artefato | Varrer fixture, diff e artefatos do ciclo | 0 ocorrências de segredo, token, URL de staging ou dado de produção | planejado |
| RT-017 | Fronteira de escrita | Conferir o diff | Somente os arquivos previstos em `fix.md`; factory somente leitura; nenhum gate/ADR/validate/release/STATE/PROGRESSO/commit | planejado |
| RT-018 | Reprodução determinística | Repetir RT-001..RT-014 com o mesmo input | Mesmos reasons, sem flakiness e sem rede | planejado |

## Output esperado (ilustrativo)

```
verify:   valid=True reason=verified producer_system=flora station=S3_VERIFICATION
ingest:   valid=True approved=True reason=verified_flora_hitl_decision next_stage=s4_monetization
negative: invalid_signature | invalid_next_stage | unredacted_pii_rejected_tier4 |
          unapproved_external_send_rejected_tier4 | receipt_not_verified |
          invalid_producer_system | missing_required_fields | invalid_station | work_order_mismatch
secrets:  0 occurrences in stdout/stderr/fixtures
```

## Procedimento

1. Implementar `HitlReceiptPort` + adaptador conforme `fix.md` (FR-001..FR-003).
2. Gerar a fixture com um segredo de **teste** definido apenas no ambiente do harness; nunca versioná-lo.
3. Executar `scripts/contract/check_hitl_receipt_roundtrip.py`: o script importa os validadores da factory, não os modifica e não faz HTTP.
4. Executar os testes Rust de contrato (`flora-tests/src/contract/hitl_receipt.rs`) para vetor dourado e rejeições locais.
5. Conferir RT-015..RT-018 e registrar o resultado no handoff do ciclo 106; gates do alvo ficam **[NV]** neste artefato.

## Residual Risks

- O harness usa os validadores da factory **como estão**: se a Autônoma mudar o formato canônico ou a allowlist, o round-trip quebra — isso é desejável (fail-closed) e deve ser tratado num ciclo de sincronização, não flexibilizando a Flora.
- Case sensitivity (`station` maiúsculo × `next_stage` minúsculo) continua sendo o ponto mais provável de regressão; RT-004 e RT-011 protegem.
- O transporte real até o endpoint da Autônoma, `MDMV_FLORA_URL`, credenciais de staging, aprovação por operador humano e envio externo real não são exercitados: **[NV]**.

## Recommendation

Bug **aberto até que o round-trip acima seja executado na implementação do ciclo 106**. Este artefato registra apenas o plano: nenhum teste, build, gate ou validação ao vivo foi executado aqui, e nenhuma credencial foi lida ou registrada.
