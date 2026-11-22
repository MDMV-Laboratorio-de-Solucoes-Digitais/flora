# Analyze: 106 — Port de recibo HITL (READ-ONLY)

**Data**: 2026-11-22 · **Branch**: `feature/106-hitl-receipt-port` @ `0582883da` · **Baseline**: `0582883da`
**Entradas**: bug D15, INBOX autorizado, `specs/106-hitl-receipt-port/*`, `flora-core`/`flora-api`, contrato Python da Autônoma (somente leitura).

## Diagnóstico

- A Autônoma valida `VerificationReceipt` HMAC em `flora_bridge.py:130-195` com 8 campos, `producer_system="flora"`, `status="verified"`, station em MAIÚSCULAS, `next_stage` em minúsculas e 5 travas de métricas. O Flora não tinha emissor.
- Vetor canônico gerado pelo validador oficial: payload de 7 campos com `json.dumps(sort_keys=True, separators=(",", ":"), ensure_ascii=False)` e assinatura `ba563acce4ed017e817584be22f6a70fe40748b2ae16a57b01398dfd8bb80dbb`.
- `hmac 0.12` (digest 0.10) é incompatível com `sha2 0.11` (digest 0.11); o lock já traz `hmac 0.13` via `hkdf`/`sqlx`.
- `serde_json::Map` sem `preserve_order` é `BTreeMap` — ordem de chaves equivalente à canonicalização Python.

## Checagem determinística

```
artifacts=8  critical=0  fr=5/5  sc=3/3  tasks=8/8  go=True  divergences=1
```

Divergência corrigida na fonte: artifactos de assessoria diziam `hmac 0.12`; apêndices append-only registram `hmac 0.13`.

## Rastreabilidade

| # | Requisito | Evidência |
|---|---|---|
| R1 | port puro + validação fail-closed | `flora-core/src/traits/hitl_receipt.rs`, `models/hitl_receipt.rs::validate_decision` |
| R2 | envelope de 8 campos e `producer_system="flora"` | `adapters/hitl_receipt.rs` + teste com vetor |
| R3 | segredo redigido, sem default inseguro | `config/hitl.rs` (`Debug` manual, `skip_serializing`) |
| R4 | allowlists e métricas | `VALID_STATIONS`, `ALLOWED_RESUME_STAGES`, validação de PII/envio |
| R5 | round-trip determinístico | `cargo test -p flora-api hitl_receipt` → 2 passed, assinatura confere |

## [NV]

Round-trip ao vivo com a Autônoma, transporte HTTP, staging/produção e credenciais reais. Nenhuma rota HTTP nova foi adicionada.

## Resultado

**PASS — 5/5 FR, 3/3 SC, 8/8 tarefas, 0 críticos.** Nenhuma edição nesta fase.
