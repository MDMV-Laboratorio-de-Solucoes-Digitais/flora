# Bug Assessment: D15 — contrato HITL unilateral

- **Slug**: d15-hitl-receipt-contract-unilateral
- **Created**: 2026-11-21
- **Source**: `.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9` + `MDMV-Autonoma/.specify/assessments/ri-flora/relatorio.md` (via revisão interagentes do SE)
- **Verdict**: valid (MAJOR D15)
- **Severity**: high
- **Adjudicação**: ciclo 106 — `.specify/assessments/106-hitl-receipt-port/decision.md`

## Report

A revisão interagentes encontrou um contrato implementado de um lado só. A Autônoma possui o validador fail-closed completo (`src/contracts/delegation_contract.py:619-793`, `src/contracts/flora_bridge.py:122-200`, `specs/017-flora-hitl-receipt-gate/spec.md:20-21`), mas a Flora não possui nenhum emissor: buscas por `receipt|work.?order|approval|operator|producer_system|external_send` e por `hitl|escalad|humano|aprova|tier.?4` em `flora-api/src` e `flora-core/src` retornam 0 correspondências no baseline `b9d08f39`. `flora-core/src/traits/mod.rs:3-9` lista apenas os ports de auth, repositório e storage; `flora-api/src/routes/mod.rs:1-15,20-70` não registra rota de recibo. A consequência é que o gate HITL Tier 4 não tem entrada produtora: a decisão humana da Flora não vira um `VerificationReceipt` que a Autônoma possa aceitar.

## Symptom

**Observed:** ausência estrutural de domínio HITL na Flora; nenhum tipo, port, adaptador ou handler que construa o envelope. A Autônoma, por sua vez, rejeita qualquer payload que não satisfaça os 8 campos, a assinatura HMAC canônica, `status == "verified"` e as cinco regras de `metrics_payload`.

**Expected:** emissor determinístico na Flora, com validação local fail-closed e HMAC canônico idêntico ao da Autônoma, acompanhado de teste round-trip que prove aceitação pelo validador e rejeição para cada violação — sem credenciais/PII em log.

## Reproduction

1. `grep -rniE 'receipt|work.?order|approval|operator|producer_system|external_send' flora-api/src flora-core/src` → 0 resultados; mesmo resultado para `hitl|escalad|humano|aprova|tier.?4`.
2. `flora-core/src/traits/mod.rs:3-9` — inventário de ports sem HITL; `flora-api/src/routes/mod.rs:20-70` — roteador sem recibo.
3. `MDMV-Autonoma/src/contracts/flora_bridge.py:145-200` — ingestão existente, mas sem produtor: chamá-la com qualquer payload local inventado.resulta em `approved=False` (fail-closed correto, fluxo parado).
4. Round-trip ao vivo e validador Python não executados neste artefato: **[NV]**.

## Suspected Code Paths

- **Consumidor (somente leitura, factory)**: `MDMV-Autonoma/src/contracts/delegation_contract.py:19-27` (`VALID_STATIONS` maiúsculas), `:38` (`producer_system="flora"`), `:619-648` (8 campos, `unsigned_payload`/`to_dict`), `:651-665` (JSON canônico + HMAC), `:723-793` (envelope, `compare_digest`, `status`, `reason`); `src/contracts/flora_bridge.py:16-26` (`ALLOWED_RESUME_STAGES` minúsculas), `:122-142` (5 regras de metrics), `:145-200` (ingestão/auditoria); `specs/017-flora-hitl-receipt-gate/spec.md:18-25`.
- **Produtor (alvo, hoje vazio)**: `flora-core/src/traits/` (novo `hitl_receipt.rs` + reexport em `mod.rs:3-9`), `flora-core/src/config/mod.rs:49` (figment `FLORA_` + `__` para o segredo), `flora-api/src/` (adaptador emissor), `flora-tests/src/contract/` (vetores e round-trip). `Cargo.toml:171,192` já traz `sha2 0.11` e `hmac 0.12`: sem dependência nova.

## Root Cause Hypothesis

**Confidence: high.** Não é uma falha intermitente nem um bug de linha: é um contrato bilateral que foi especificado e implementado apenas no consumidor. A Autônoma codifica a forma exata do envelope, mas a Flora nunca recebeu a obrigação correspondente — o INBOX D15 registra explicitamente o desequilíbrio. Qualquer "correção" que flexibilize o validador para aceitar o que a Flora tem hoje (nada) seria errada: a autoridade do contrato está na Autônoma e permanece intacta.

## Proposed Remediation

**Preferred:** port puro `HitlReceiptPort` em `flora-core` (tipos de decisão + validação fail-closed, sem I/O, sem segredo) e adaptador emissor em `flora-api` que serialize os 7 campos não assinados em JSON canônico (`sort_keys=True`, `separators=(",", ":")`, `ensure_ascii=False`, UTF-8), calcule HMAC-SHA256 hex com `hmac 0.12`/`sha2 0.11` e produza o 8º campo `signature_hmac`. Regras locais espelhadas: `station` em `VALID_STATIONS` (maiúsculas), `next_stage` em `ALLOWED_RESUME_STAGES` (minúsculas), `status="verified"`, `producer_system="flora"`, `work_order_id`/`ativo_id`/`artifacts_hash` não vazios, `operator_approved` bool, `operator_id` não vazio, `pii_redacted is True`, `real_external_send is False`. Segredo via `FLORA_HITL__HMAC_SECRET`, sem default inseguro; logging restrito a identificadores não sensíveis (nunca segredo, `operator_id`, PII nem payload). Teste round-trip com fixture emitida e validadores da factory aplicados por harness somente leitura, mais um caso negativo por reason.

**Alternatives:** (a) endpoint HTTP novo na Flora — aumenta superfície de ataque e não é necessário para provar o contrato; (b) reimplementar o validador na Flora — duplica a autoridade e cria drift; (c) enfraquecer o gate da Autônoma — inaceitável (Tier 4/LGPD). Nenhuma alternativa é adotada.

**Files likely to change:** `flora-core/src/traits/hitl_receipt.rs` (novo), `flora-core/src/traits/mod.rs`, `flora-core/src/config/hitl.rs` (novo), `flora-api/src/adapters/hitl_receipt.rs` (novo), `flora-tests/src/contract/hitl_receipt.rs` (novo), `scripts/contract/check_hitl_receipt_roundtrip.py` (novo), além dos artefatos do ciclo 106.

**Tests to add or update:** round-trip positivo (`reason="verified"`, `approved=True`, `reason="verified_flora_hitl_decision"`), vetor canônico/HMAC e um caso por reason de rejeição (`invalid_signature`, `invalid_next_stage`, `unredacted_pii_rejected_tier4`, `unapproved_external_send_rejected_tier4`, `missing_operator_approved_flag`, `missing_operator_id`, `receipt_not_verified`, `invalid_producer_system`, `missing_required_fields`, `invalid_station`, `work_order_mismatch`), mais asserção de que nenhum segredo/PII aparece em log.

## Risks & Considerations

- **Case sensitivity**: `station` é maiúsculo na Autônoma (`delegation_contract.py:19-27`) e `next_stage` é minúsculo (`flora_bridge.py:16-26`). Inverter qualquer um dos dois gera `invalid_station` / `invalid_next_stage` apesar de um payload "parecido". Risco alto de drift; mitigam vetores explícitos.
- **Correlação**: `expected_work_order_id` e `expected_station` são checados em `:786-789`; o emissor precisa propagar o identificador da ordem sem inventá-lo.
- **Segredos e PII**: `operator_id` e o payload são dados sensíveis; logar segredo, payload ou identificador do operador viola o critério de aceite do INBOX e a postura LGPD do gate. Falha em qualquer log é falha do ciclo.
- **Autoridade unilateral preservada**: a factory é somente leitura; nenhum validador, allowlist ou teste da Autônoma é alterado para "facilitar" a integração.
- **Transporte ao vivo**: fora do apetite; o round-trip usará fixture/harness determinístico. URL, credenciais e staging ficam **[NV]**.
- **Nenhum envio externo real**: `real_external_send` deve permanecer `False` em todos os cenários — o gate bloqueia `True` por design (`flora_bridge.py:138-139`).

## Open Questions

- Quem aciona a emissão no fluxo real (UI de aprovação Tier 4 na Flora ou serviço interno)? **[NV]** — o contrato de emissão é independente do acionador.
- O transporte até `POST /receipts/flora-hitl` (spec 017:23-25) será feito pelo bridge da Autônoma ou por integração futura? **[NV]** — não altera o envelope nem os testes.
- Rotação do segredo HMAC e idempotência de reenvio do mesmo recibo são tratadas na Autônoma; a Flora apenas precisa ser determinística para o mesmo input.
