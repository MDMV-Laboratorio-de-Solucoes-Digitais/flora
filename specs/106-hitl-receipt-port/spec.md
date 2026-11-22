# Feature Specification: Port de recibo HITL com contrato bilateral

**Feature Branch**: `feature/106-hitl-receipt-port`
**Created**: 2026-11-22
**Status**: Specified — go para implementação
**Âncora**: `.specify/feature.json` → `specs/106-hitl-receipt-port`; bug `.specify/bugs/d15-hitl-receipt-contract-unilateral/`

**Input**: correção MAJOR D15 do INBOX autorizado pelo SE: a Autônoma valida `VerificationReceipt` HMAC, mas a Flora não tem emissor.

## User Scenarios & Testing

### User Story 1 — Emitir recibo verificável (P1)

A Flora produz um envelope de 8 campos que passa nos validadores `verify_verification_receipt` e `ingest_flora_hitl_receipt` da Autônoma sem alteração do lado consumidor.

**Acceptance Scenarios**:
1. **Given** uma decisão HITL válida e segredo HMAC configurado, **When** o emissor é chamado, **Then** retorna os 8 campos com `producer_system="flora"`, `status="verified"`, `signature_hmac` igual ao vetor canônico.
2. **Given** o mesmo input repetido, **When** emitido duas vezes, **Then** o envelope é idêntico (determinismo; sem nonce/hora).

### User Story 2 — Falhar fechado (P1)

Entradas que violam as invariantes do consumidor são rejeitadas antes de assinar.

**Acceptance Scenarios**:
1. **Given** `operator_approved` não-booleano, `operator_id` vazio, `pii_redacted=false`, `real_external_send=true`, `next_stage` fora da allowlist ou `station` inválida, **When** emitir, **Then** retorna erro de validação e nenhum recibo é produzido.
2. **Given** segredo HMAC vazio, **When** emitir, **Then** falha com `Error::Configuration` sem expor o valor.

### User Story 3 — Round-trip determinístico (P2)

O envelope emitido pela Flora é consumido pelo contrato Python sem dependência de rede.

**Acceptance Scenarios**:
1. **Given** o vetor dourado gerado pelo validador oficial (`work_order_id=wo-d15-001`, assinatura `ba563acc…`), **When** o teste Rust reconstrói o payload canônico e assina com o mesmo segredo de teste, **Then** a assinatura confere e a revalidação local aceita os 7 campos.
2. **Given** assinatura adulterada, **When** validada, **Then** rejeitada em tempo constante.

### Edge Cases

- `metrics_payload` com chaves em ordem diferente deve produzir a mesma assinatura (canonicalização por chave).
- `next_stage` deve ser sempre explícito, em minúsculas, dentro de `s0_radar…s6_telemetry`; nunca usar o default implícito do consumidor.
- `station` usa MAIÚSCULAS (`S0_RADAR…S6_TELEMETRY`); `next_stage` usa minúsculas.
- Nenhum segredo, `operator_id` ou payload entra em `tracing`, erro ou fixture versionada.

## Requirements

- **FR-001**: definir `HitlReceiptPort` puro em `flora-core/src/traits/` com validação fail-closed antes da emissão.
- **FR-002**: emitir envelope exato de 8 campos, com `producer_system="flora"`, `status="verified"` e HMAC-SHA256 sobre JSON canônico (chaves ordenadas, sem espaços, UTF-8).
- **FR-003**: carregar segredo de `FLORA_HITL__HMAC_SECRET` em config dedicada, sem default, `Debug` redigido e `skip_serializing`.
- **FR-004**: validar allowlists e as cinco regras de `metrics_payload` espelhando `flora_bridge.py:16-26,122-142`.
- **FR-005**: teste round-trip com vetor dourado do contrato Python e casos de rejeição; nenhum piso do gate alterado.

## Success Criteria

- **SC-001**: envelope gerado é aceito pelo validador oficial (vetor + revalidação local).
- **SC-002**: cada entrada inválida é rejeitada antes de assinar, sem segredo ou PII em log/erro.
- **SC-003**: gate do alvo permanece PASS (10 legs / 77 pisos) e nenhuma credencial real é versionada.

## Key Entities

- `HitlDecision` (input puro), `HitlReceipt` (8 campos), `HitlConfig` (segredo redigido), `HitlReceiptAdapter` (HMAC + canonicalização).

## Assumptions / [NV]

- `hmac 0.13` do lock é usado com `sha2 0.11` (digest 0.11); `hmac 0.12` transitivo é incompatível.
- Round-trip ao vivo com a Autônoma em execução, staging/produção e credenciais reais: **[NV]**.
- Nenhuma rota HTTP nova; transporte entre sistemas permanece responsabilidade da Autônoma.
