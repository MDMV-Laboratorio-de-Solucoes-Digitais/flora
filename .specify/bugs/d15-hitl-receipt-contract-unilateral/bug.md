# Bug Report: D15 — contrato HITL unilateral (Flora não emite `VerificationReceipt`)

- **Slug**: d15-hitl-receipt-contract-unilateral
- **Criado**: 2026-11-21
- **Fonte**: `.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9` (SE, via revisão interagentes `MDMV-Autonoma/.specify/assessments/ri-flora/relatorio.md`)
- **Veredito**: válido — **MAJOR D15**
- **Severidade**: alta — o gate HITL Tier 4 da Autônoma não pode ser liberado por uma decisão originada na Flora
- **Adjudicação**: ciclo 106 (`.specify/assessments/106-hitl-receipt-port/decision.md`, veredito **go**)

## Relato (verbatim / resumido)

> **MAJOR D15 — contrato HITL unilateral.** A Autônoma valida `VerificationReceipt` HMAC em `src/contracts/flora_bridge.py:130-195` (`producer_system=="flora"`, `operator_approved`, `operator_id`, `pii_redacted is True`, `real_external_send is False`, `next_stage` allowlist; `specs/017-flora-hitl-receipt-gate/spec.md:20-21`). O Flora não tem emissor/endpoint (`flora-api/src` + `flora-core/src`: 0 ocorrências de `receipt|work.?order|approval|operator|producer_system|external_send`). Implementar `HitlReceiptPort` + adaptador que emita exatamente esse envelope, com teste round-trip ponta a ponta e allowlist de `next_stage`.

## Sintoma

**Observado:** a Autônoma implementa o **lado consumidor** do contrato HITL de forma completa e fail-closed: valida o envelope de 8 campos, a assinatura HMAC-SHA256 canônica, `status == "verified"`, `producer_system == "flora"` e as cinco condições de `metrics_payload` (`operator_approved`, `operator_id`, `pii_redacted`, `real_external_send`, `next_stage`). A Flora não implementa o **lado produtor**: não existe traço, adaptador, tipo, rota ou vocabulário de recibo, work order, aprovação, operador, produtor ou envio externo em `flora-api/src` nem em `flora-core/src`. O par produtor/consumidor está pela metade: a Autônoma espera um recibo que a Flora nunca emite.

**Esperado:** a Flora emita, a partir de uma decisão humana Tier 4, um `VerificationReceipt` com o envelope exato que a Autônoma valida — incluindo HMAC determinístico, `next_stage` da allowlist e nenhuma credencial ou PII em log — e um teste round-trip prove que o recibo emitido é aceito pelo validador da Autônoma.

## Reprodução (determinística, somente leitura)

1. Busca executada neste artefato sobre `flora-api/src` e `flora-core/src` (baseline `b9d08f3904eab6aeed93d9d6732434356e583c65`), sem distinção de caixa:
   `receipt|work.?order|approval|operator|producer_system|external_send` → **0 correspondências**; `hitl|escalad|humano|aprova|tier.?4` → **0 correspondências**.
2. Em `flora-core/src/traits/mod.rs:3-9`, o inventário de ports é apenas `auth_provider`, `repository`, `storage_provider`. Não há port de emissão de recibo nem de decisão HITL.
3. Em `flora-api/src/routes/mod.rs:1-15,20-70`, o roteador registra health, auth, bootstrap, organizações, workspaces, canais, tarefas, arquivos, busca, papéis, notificações e websocket. Não há rota ou handler de recibo/HITL.
4. Do outro lado, `MDMV-Autonoma/src/contracts/flora_bridge.py:145-200` (`ingest_flora_hitl_receipt`) e `:122-142` (`validate_flora_hitl_metrics`) processam esse recibo; `specs/017-flora-hitl-receipt-gate/spec.md:20-21` descreve a decisão esperada. Sem emissor, qualquer tentativa de ingresso falha fechado (`approved=False`) ou nunca chega a ser disparada.
5. A execução do validador Python e do round-trip ao vivo **não foi feita** neste artefato: **[NV]**. A reprodução acima é por inspeção determinística do código e da busca.

## Causa raiz — arquivo:linha

**Lado consumidor — contrato completo (somente leitura, factory da Autônoma):**

1. `MDMV-Autonoma/src/contracts/delegation_contract.py:619-630` — `VerificationReceipt` tem exatamente 8 campos: `work_order_id`, `ativo_id`, `station`, `producer_system`, `status`, `artifacts_hash`, `metrics_payload`, `signature_hmac`. `:632-648` define `unsigned_payload()` (7 campos, sem assinatura) e `to_dict()` (8 campos).
2. `delegation_contract.py:651-665` — canonicalização: `json.dumps(payload, sort_keys=True, separators=(",", ":"), ensure_ascii=False)` em UTF-8, HMAC-SHA256 com segredo obrigatório e `hexdigest()`.
3. `delegation_contract.py:723-752` (`_validate_receipt_envelope`) e `:755-793` (`verify_verification_receipt`) — exigem os 7 campos não vazios, `station ∈ VALID_STATIONS`, `metrics_payload` do tipo `dict`, assinatura igual via `hmac.compare_digest`, `status == "verified"`; `valid` só é `True` quando `reason == "verified"`.
4. `delegation_contract.py:19-27` — `VALID_STATIONS` são **MAIÚSCULAS** (`S0_RADAR`..`S6_TELEMETRY`); `:38` fixa `TargetSystem.FLORA_CORPORATE_SUITE = "flora"`.
5. `MDMV-Autonoma/src/contracts/flora_bridge.py:16-26` — `ALLOWED_RESUME_STAGES` são **minúsculas** (`s0_radar`..`s6_telemetry`); `:122-142` valida as cinco chaves de `metrics_payload`; `:145-200` checa `producer_system`, audita `flora_hitl_receipt_ingested` e devolve `approved`. `specs/017-flora-hitl-receipt-gate/spec.md:20-21` registra a mesma expectativa.

**Lado produtor — ausência total (alvo Flora):**

6. `flora-core/src/traits/mod.rs:3-9` — nenhum `HitlReceiptPort`; `flora-core/src/traits/` contém apenas `auth_provider.rs`, `repository.rs`, `storage_provider.rs`.
7. `flora-api/src/routes/mod.rs:1-15,20-70` — nenhuma rota de recibo; `flora-api/src/routes/` não tem módulo de HITL.
8. Buscas dos itens 1–3 acima: nenhum símbolo de domínio (recibo, work order, aprovação, operador, produtor, envio externo) em `flora-api/src` ou `flora-core/src`. A ausência é estrutural, não um bug de uma linha.

## Correção adjudicada (ciclo 106)

`HitlReceiptPort` (port puro em `flora-core`, sem I/O e sem segredo) + `HitlDecision` com validação fail-closed; adaptador emissor em `flora-api` que produz o envelope exato de 8 campos, com `status="verified"`, `producer_system="flora"`, `station` em `VALID_STATIONS` (maiúsculas), `next_stage` em `ALLOWED_RESUME_STAGES` (minúsculas), `pii_redacted=true`, `real_external_send=false`, e HMAC-SHA256 sobre JSON canônico (`sort_keys`, `separators=(',',':')`, `ensure_ascii=False`). Segredo HMAC via configuração `FLORA_HITL__HMAC_SECRET` (padrão figment `FLORA_` + `__`, `flora-core/src/config/mod.rs:49`), nunca em log, erro, fixture ou artefato; `hmac 0.12` e `sha2 0.11` já estão em `Cargo.toml:171,192` (nenhuma dependência nova). Round-trip determinístico: a Flora emite uma fixture e um harness somente leitura aplica `verify_verification_receipt` e `ingest_flora_hitl_receipt` da factory (que permanece somente leitura), cobrindo o caminho positivo e cada reason de rejeição. Ver `.specify/assessments/106-hitl-receipt-port/decision.md` e `specs/106-hitl-receipt-port/spec.md` (FR-001..FR-005 / SC-001..SC-003, T001..T008).

## [NV]

- A disponibilidade do endpoint receptor `POST /receipts/flora-hitl`, o valor real de `MDMV_FLORA_URL`, do segredo HMAC e de `FLORA_HITL__HMAC_SECRET`, e o round-trip ao vivo: **[NV]** — nenhuma credencial, URL de staging ou requisição foi lida ou executada; nenhuma credencial aparece neste artefato.
- A existência de um operador humano e da interface de aprovação Tier 4 na Flora, bem como o transporte do recibo até a Autônoma: **[NV]** — o contrato de emissão e o teste com fixture são verificáveis sem eles.
- Os números de linha se referem a `b9d08f39` (Flora) e ao snapshot da factory examinado em 2026-11-21; uma implementação paralela pode deslocá-los.
- Este artefato é documental: nenhum código, gate, teste, build ou formatter foi executado ou alterado aqui.
