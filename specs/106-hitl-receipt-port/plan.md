# Implementation Plan: 106 — Port de recibo HITL

**Branch**: `feature/106-hitl-receipt-port` · **Baseline**: `0582883da`

## Technical Context

- Rust workspace edition 2024, `flora-core` (domínio/traits/config) + `flora-api` (adaptador) + `flora-tests` (contrato).
- `serde_json::Map` sem `preserve_order` ⇒ `BTreeMap`, ordem canônica equivalente a `sort_keys=True`.
- `hmac 0.13` (digest 0.11) com `sha2 0.11`; `hmac 0.12` transitivo é incompatível e não deve ser usado no adaptador.
- Segredo lido por Figment `Env::prefixed("FLORA_").split("__")` → `FLORA_HITL__HMAC_SECRET`.

## Steps

- **T001** Confirmar bug report D15 e decisão `go` (já materializados).
- **T002** Modelo `HitlReceipt`/`HitlDecision` + port `HitlReceiptPort` em `flora-core` com validação fail-closed.
- **T003** Config `HitlConfig` com segredo redigido, sem default, validação não-vazia.
- **T004** Adaptador em `flora-api`: canonicalização JSON, HMAC-SHA256, envelope de 8 campos.
- **T005** Teste com vetor dourado Python e casos de rejeição; nenhum `#[ignore]` novo.
- **T006** Wiring do adaptador sem rota HTTP nova, sem segredo em `Debug`/`tracing`.
- **T007** Gates determinísticos (fmt/clippy/test/gate) e documentação append-only.
- **T008** analyze/converge/R4/R7 e PR para `dev/`.

## Constitution Check

Sem mudança de gate/required check, sem unsafe/unwrap, sem serviço obrigatório novo, sem segredo versionado, compatível com ADR-0001 e com a autoridade da Autônoma sobre o consumidor.

## Complexity Tracking

Uma trait, uma struct de config, um adaptador e um módulo de teste; nenhuma abstração adicional.
