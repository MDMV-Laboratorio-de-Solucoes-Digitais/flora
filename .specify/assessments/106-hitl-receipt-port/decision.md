# Decision: 106 — Port de recibo HITL com contrato bilateral

- **Slug**: 106-hitl-receipt-port
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `docs/inbox-receipt-2026-11-21` @ `b9d08f3904eab6aeed93d9d6732434356e583c65`
- **Âncora**: `.specify/feature.json` → `specs/106-hitl-receipt-port` (via `create-new-feature.sh --number 106 --short-name hitl-receipt-port`)
- **Evidência**: INBOX D15 (`.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9`) + bug `.specify/bugs/d15-hitl-receipt-contract-unilateral/` + inspeção determinística da factory e do baseline `b9d08f39`
- **Artefatos revisados**: intake | research | define | shape | bug (D15) | spec | plan | tasks

## Contexto / problema — causa raiz determinística

O contrato HITL Tier 4 existe apenas no consumidor. Na factory MDMV-Autonoma:

1. `src/contracts/delegation_contract.py:619-648` define `VerificationReceipt` com 8 campos; `:651-665` fixa JSON canônico (`sort_keys=True`, `separators=(",", ":")`, `ensure_ascii=False`, UTF-8) e HMAC-SHA256 hex com segredo obrigatório; `:723-793` valida envelope, `compare_digest`, `work_order_id`/`station` esperados e `status == "verified"`.
2. `src/contracts/flora_bridge.py:16-26` define `ALLOWED_RESUME_STAGES` em minúsculas; `:122-142` exige `operator_approved`, `operator_id`, `pii_redacted is True`, `real_external_send is False` e `next_stage` válido; `:145-200` verifica `producer_system == "flora"`, audita `flora_hitl_receipt_ingested` e só aprova se o operador aprovou. `specs/017-flora-hitl-receipt-gate/spec.md:20-21` descreve a mesma expectativa.
3. Na Flora, não há produtor: buscas por `receipt|work.?order|approval|operator|producer_system|external_send` e por `hitl|escalad|humano|aprova|tier.?4` em `flora-api/src` e `flora-core/src` retornam 0 correspondências; `flora-core/src/traits/mod.rs:3-9` não tem port HITL; `flora-api/src/routes/mod.rs:20-70` não tem rota de recibo.

Não há falha intermitente: é uma lacuna de contrato bilateral registrada como MAJOR D15. A correção não pode passar por flexibilizar o validador da Autônoma.

## Decisão

Registrar o ciclo 106 como **Port de recibo HITL com contrato bilateral**, com cinco compromissos rastreáveis (FR-001..FR-005), três critérios (SC-001, SC-002, SC-003) e oito tarefas (T001..T008; fila de implementação neste artefato):

1. **Port puro (FR-001 / T002)**: `HitlReceiptPort` + `HitlDecision` em `flora-core/src/traits/hitl_receipt.rs`, reexportados em `mod.rs:3-9`; sem I/O, segredo ou log; validação antes da emissão.
2. **Adaptador e envelope exato (FR-002 / T003, T004)**: `flora-api/src/adapters/hitl_receipt.rs` emite os 8 campos com `producer_system="flora"`, `status="verified"`, HMAC-SHA256 sobre JSON canônico e segredo de `FLORA_HITL__HMAC_SECRET` via figment (`config/mod.rs:49`); `hmac 0.12`/`sha2 0.11` já existem (`Cargo.toml:171,192`); nenhuma credencial/PII em log, erro ou fixture; nenhuma rota HTTP nova.
3. **Allowlist e fail-closed (FR-003 / T002)**: `next_stage` minúsculo em `ALLOWED_RESUME_STAGES`, `station` maiúsculo em `VALID_STATIONS`, campos não vazios, `pii_redacted=true`, `real_external_send=false`, `operator_approved` bool e `operator_id` não vazio; rejeição local antes de assinar; determinismo para o mesmo input.
4. **Round-trip (FR-004 / T005)**: vetor dourado e harness somente leitura que aplica `verify_verification_receipt` e `ingest_flora_hitl_receipt` da factory à fixture emitida; caminho feliz (`verified`, `approved=True`) e um caso por reason de rejeição; sem rede, staging ou segredo real.
5. **Fronteira e rastreabilidade (FR-005 / T001, T006, T007, T008)**: diff restrito aos arquivos previstos e aos artefatos do ciclo 106; dívidas/**[NV]** registradas; factory somente leitura; sem gates, required checks, ADR, validate/release, STATE/PROGRESSO, `specs/106-proptest-invariants/`, dependência nova, rota nova, credencial em log ou commit.

## Evidência de entrada

- Baseline `b9d08f39`, branch `docs/inbox-receipt-2026-11-21`; `.specify/feature.json` passou a apontar para `specs/106-hitl-receipt-port` (o `spec.md` modelo foi substituído pela spec real).
- `.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9,13-15` — D15 e critério de aceite.
- `.specify/bugs/d15-hitl-receipt-contract-unilateral/{bug,assessment,fix,test}.md` — sintoma com file:line, causa/risco, proposta e plano round-trip.
- `MDMV-Autonoma/src/contracts/delegation_contract.py:19-27,38,619-793` e `src/contracts/flora_bridge.py:16-26,122-200` — envelope, allowlists e validação (somente leitura).
- `MDMV-Autonoma/specs/017-flora-hitl-receipt-gate/spec.md:18-25` — decisão HITL e endpoint receptor.
- `flora-core/src/traits/mod.rs:3-9`, `flora-core/src/traits/storage_provider.rs:12-31`, `flora-core/src/config/mod.rs:49`, `flora-api/src/routes/mod.rs:1-15,20-70`, `Cargo.toml:171,192`, `flora-tests/src/contract/` — padrões de port, config, roteador, dependências e testes.

## Racional

**go.** O custo está concentrado num produtor pequeno e determinístico, e o contrato já é explícito: 8 campos, HMAC canônico, duas allowlists com caixas distintas e cinco regras de métricas. Criar o port no core e o adaptador na API fecha o lado ausente sem alterar a autoridade da Autônoma, sem superfície HTTP nova e sem dependência nova (as crates já estão no workspace). O harness com fixture substitui com segurança o round-trip ao vivo, que depende de credenciais e da Autônoma em execução: o que é verificável aqui é o contrato, não o transporte. Registrar o transporte, o acionador humano e o staging como **[NV]** mantém a honestidade do ciclo. "Nada em log" é tratado como requisito funcional, não como observação: o segredo e o `operator_id` jamais entram em `tracing`, erro, fixture ou artefato.

## [NV]

- Execução do harness round-trip, do validador Python, de `cargo test`, build, gates e formatters: **[NV]** neste artefato — decisão documental; consolidação no handoff.
- Round-trip ao vivo, `MDMV_FLORA_URL`, `POST /receipts/flora-hitl`, segredo HMAC real, `FLORA_HITL__HMAC_SECRET` e credenciais de staging/produção: **[NV]** — nenhuma credencial, URL ou dado de produção foi lido ou registrado.
- Existência de UI/serviço de aprovação Tier 4 na Flora e mecanismo de transporte do recibo: **[NV]** — não afetam o envelope nem os testes determinísticos.
- Rotação de segredo e idempotência de reenvio: responsabilidade da Autônoma; **[NV]** neste ciclo.
- Números de linha referem-se a `b9d08f39` e ao snapshot da factory de 2026-11-21; uma implementação paralela pode deslocá-los.

## Fora de âmbito

Mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`; alteração na factory (validadores, allowlists, endpoint, testes), em `specs/017-flora-hitl-receipt-gate/`, em gates, required checks, workflows, ADRs, constituição ou changelog; criação de rota HTTP na Flora; envio externo real, transporte ao vivo, uso de segredo/URL de staging; acionador humano/UI de aprovação; escrita em `specs/106-proptest-invariants/`; commits.

## Handoff

`/speckit.specify` — especificar o produtor HITL como requisitos verificáveis (FR-001..FR-005 / SC-001..SC-003) e fila T001..T008: port puro + adaptador com envelope exato e HMAC canônico, allowlist de `next_stage`, travas fail-closed, zero credenciais em log e round-trip com fixture contra os validadores da factory. Harness, testes, gates, build e round-trip ao vivo ficam no handoff **[NV]**.

### Correção append-only — versão do HMAC

Onde este documento diz `hmac 0.12`, leia **`hmac 0.13`**: a série 0.12 usa `digest 0.10` e não compila com `sha2 0.11`. O lockfile já traz `hmac 0.13`/`digest 0.11`; a decisão e o escopo permanecem os mesmos.
