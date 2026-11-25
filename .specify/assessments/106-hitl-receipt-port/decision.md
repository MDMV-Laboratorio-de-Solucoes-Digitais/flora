# Decision: 106 — Port de recibo HITL com contrato bilateral

- **Slug**: 106-hitl-receipt-port
- **Decided**: 2026-11-25
- **Verdict**: go (decisão original 2026-11-21, preservada)
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (anterior) | bug D15 (`bug/assessment/fix/test.md`) | INBOX 2026-11-21 | contratos da factory (somente leitura) | spec/plan/tasks 106

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | MAJOR D15: a Autônoma valida `VerificationReceipt` fail-closed, mas a Flora não emite nenhum recibo (0 correspondências nas buscas, nenhum port/rota HITL), deixando o gate Tier 4 sem entrada (`intake.md:5-13`; `INBOX:9`). |
| Evidence strength | adequate | Contrato bilateral explícito na factory (8 campos, HMAC canônico, allowlists, regras de métricas) e buscas determinísticas no produtor; harness, validador Python e round-trip ao vivo **[NV]** (`research.md:1-13`). |
| Value vs. inaction | strong | Fecha o lado ausente com produtor pequeno e determinístico, sem flexibilizar o validador da Autônoma e sem rota ou dependência nova (`decisão anterior:42`). |
| Feasibility / appetite | adequate | Port puro no core + adaptador na API + harness com fixture; `hmac`/`sha2` já no workspace ("0.13" na correção append-only), segredo via figment (`define.md:FR-001..FR-004`). |
| Strategic fit | strong | Mantém a Autônoma como autoridade única do contrato, factory somente leitura e fronteira de artefatos; nenhuma credencial, migração ou gate novo (`define.md:FR-005,SC-003`). |
| Risk posture | adequate | Fail-closed local antes de assinar, caixas distintas de `station`/`next_stage`, "nada em log" como requisito funcional; transporte, acionador humano e staging **[NV]** (`decisão anterior:42,44-50`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. Registro original de 2026-11-21, branch `docs/inbox-receipt-2026-11-21` @ `b9d08f3904eab6aeed93d9d6732434356e583c65`, âncora `.specify/feature.json` → `specs/106-hitl-receipt-port` (via `create-new-feature.sh --number 106 --short-name hitl-receipt-port`), evidência do INBOX D15 (`.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9`), do bug `.specify/bugs/d15-hitl-receipt-contract-unilateral/` e da inspeção determinística da factory e do baseline `b9d08f39`.

Causa raiz: o contrato HITL Tier 4 existe apenas no consumidor. `MDMV-Autonoma/src/contracts/delegation_contract.py:619-648` define `VerificationReceipt` com 8 campos; `:651-665` fixa JSON canônico (`sort_keys=True`, `separators=(",", ":")`, `ensure_ascii=False`, UTF-8) e HMAC-SHA256 hex com segredo obrigatório; `:723-793` valida envelope, `compare_digest`, `work_order_id`/`station` esperados e `status == "verified"`. `flora_bridge.py:16-26` define `ALLOWED_RESUME_STAGES` em minúsculas; `:122-142` exige `operator_approved`, `operator_id`, `pii_redacted is True`, `real_external_send is False` e `next_stage` válido; `:145-200` exige `producer_system == "flora"`, audita `flora_hitl_receipt_ingested` e só aprova se o operador aprovou; `specs/017-flora-hitl-receipt-gate/spec.md:20-21` descreve a mesma expectativa. Na Flora, buscas por `receipt|work.?order|approval|operator|producer_system|external_send` e por `hitl|escalad|humano|aprova|tier.?4` em `flora-api/src` e `flora-core/src` retornam **0 correspondências**; `flora-core/src/traits/mod.rs:3-9` não tem port HITL e `flora-api/src/routes/mod.rs:20-70` não tem rota de recibo. Não é falha intermitente; a correção não pode passar por flexibilizar o validador da Autônoma.

| Compromisso rastreável | Conteúdo preservado |
|---|---|
| FR-001 / T002 — port puro | `HitlReceiptPort` + `HitlDecision` em `flora-core/src/traits/hitl_receipt.rs`, reexportados em `mod.rs:3-9`; sem I/O, segredo ou log; validação antes da emissão. `HitlDecision` com `work_order_id`, `ativo_id`, `station`, `artifacts_hash`, `operator_approved`, `operator_id`, `pii_redacted`, `real_external_send`, `next_stage`. |
| FR-002 / T003–T004 — adaptador/envelope | `flora-api/src/adapters/hitl_receipt.rs` emite exatamente 8 campos (`work_order_id`, `ativo_id`, `station`, `producer_system="flora"`, `status="verified"`, `artifacts_hash`, `metrics_payload`, `signature_hmac`), JSON canônico e HMAC-SHA256 com `FLORA_HITL__HMAC_SECRET` via figment (`config/mod.rs:49`); nenhuma credencial/PII em log, erro ou fixture; nenhuma rota HTTP nova. |
| FR-003 / T002 — allowlist/fail-closed | `next_stage` minúsculo em `ALLOWED_RESUME_STAGES`, `station` maiúsculo em `VALID_STATIONS`, campos não vazios, `pii_redacted=true`, `real_external_send=false`, `operator_approved` bool, `operator_id` não vazio, segredo não vazio; rejeição local antes de assinar; determinismo para o mesmo input; `next_stage` sempre explícito (default `s4_monetization` na Autônoma). |
| FR-004 / T005 — round-trip | Vetor dourado e harness somente leitura (`flora-tests/src/contract/hitl_receipt.rs`, `scripts/contract/check_hitl_receipt_roundtrip.py`) aplicando `verify_verification_receipt` e `ingest_flora_hitl_receipt` da factory à fixture emitida; caminho feliz (`valid=True`, `reason="verified"`, `approved=True`, `reason="verified_flora_hitl_decision"`) e um caso por reason de rejeição; sem rede, staging ou segredo real. |
| FR-005 / T001, T006–T008 — fronteira | Diff restrito aos arquivos previstos e artefatos do ciclo 106; dívidas/**[NV]** registradas; factory somente leitura; sem gates, required checks, ADR, validate/release, STATE/PROGRESSO, `specs/106-proptest-invariants/`, dependência nova, rota nova, credencial em log ou commit. |

Critérios preservados: **SC-001** round-trip positivo sem alteração na Autônoma e assinatura idêntica em emissões repetidas; **SC-002** fail-closed com reason determinístico, `real_external_send` sempre `False` e varredura de 0 ocorrências de segredo/`operator_id`/PII/payload; **SC-003** nenhuma dependência, rota HTTP, gate, flag, schema, ADR, artefato de outro ciclo ou commit; factory intacta. Fila **T001–T008**. Racional histórico: custo concentrado num produtor determinístico com contrato explícito (8 campos, HMAC canônico, duas allowlists com caixas distintas, cinco regras de métricas); criar port no core e adaptador na API fecha o lado ausente sem alterar a autoridade da Autônoma; o harness com fixture substitui com segurança o round-trip ao vivo, que depende de credenciais e da Autônoma em execução — o verificável aqui é o contrato, não o transporte; "nada em log" é requisito funcional.

**Correção append-only — versão do HMAC (preservada)**: onde o registro histórico diz `hmac 0.12`, leia **`hmac 0.13`**: a série 0.12 usa `digest 0.10` e não compila com `sha2 0.11`; o lockfile já traz `hmac 0.13`/`digest 0.11`; a decisão e o escopo permanecem os mesmos.

**[NV]** preservados: execução do harness, do validador Python, `cargo test`, build, gates e formatters (decisão documental; consolidação no handoff); round-trip ao vivo, `MDMV_FLORA_URL`, `POST /receipts/flora-hitl`, segredo HMAC real, `FLORA_HITL__HMAC_SECRET` e credenciais de staging/produção (nenhuma credencial, URL ou dado de produção lido ou registrado); existência de UI/serviço de aprovação Tier 4 na Flora e mecanismo de transporte; rotação de segredo e idempotência de reenvio (responsabilidade da Autônoma); números de linha referem-se a `b9d08f39` e ao snapshot da factory de 2026-11-21.

Fora de âmbito preservado: mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`; alteração na factory (validadores, allowlists, endpoint, testes), em `specs/017-flora-hitl-receipt-gate/`, gates, required checks, workflows, ADRs, constituição ou changelog; criação de rota HTTP na Flora; envio externo real, transporte ao vivo, uso de segredo/URL de staging; acionador humano/UI de aprovação; escrita em `specs/106-proptest-invariants/`; commits.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante registrada nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: A Autônoma valida um `VerificationReceipt` HMAC fail-closed, mas a Flora não emite nenhum recibo (MAJOR D15 / INBOX `:9`), deixando o gate Tier 4 sem entrada válida.
- **Chosen approach**: Port puro no core + adaptador na API com envelope exato de 8 campos, JSON canônico e HMAC-SHA256 (`hmac 0.13`/`sha2 0.11` já no lockfile), allowlists e travas fail-closed, provados por fixture round-trip contra os validadores inalterados da factory.
- **In scope / out of scope**:
  - **In**: FR-001..FR-005 / SC-001..SC-003, fila T001..T008; `traits/hitl_receipt.rs` (novo), `traits/mod.rs`, `config/hitl.rs` (novo), `adapters/hitl_receipt.rs` (novo), wiring na API, `contract/hitl_receipt.rs` (novo), `scripts/contract/check_hitl_receipt_roundtrip.py` (novo), `feature.json`; dívidas/**[NV]** registradas.
  - **Out/NV**: factory/spec 017, gates/required checks/workflows/ADRs/constituição/changelog, rota HTTP, envio externo, segredo/URL de staging, acionador humano/UI Tier 4, `specs/106-proptest-invariants/`, dependência nova, `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, commits; harness/testes/gates/round-trip ao vivo **[NV]**.
- **Success metrics**: recibo emitido aceito sem alteração na Autônoma (`valid=True`/`reason="verified"`, `approved=True`/`reason="verified_flora_hitl_decision"`, envelope com exatamente 8 campos, assinatura determinística); cada violação rejeitada com reason correspondente sem `resume()`; `real_external_send=False` sempre; varredura de 0 ocorrências de segredo/`operator_id`/PII/payload em log, fixture e artefatos.
- **Carried-forward open questions**: `[NV]` harness/validador/`cargo test`/build/gates; `[NV]` round-trip ao vivo, URL, segredo real e credenciais; `[NV]` existência de UI de aprovação Tier 4 e transporte; `[NV]` rotação de segredo e idempotência (Autônoma) — nenhuma bloqueia o `go`.
