# Research: 106 — Port de recibo HITL com contrato bilateral

**Data:** 2026-11-21 · **Inspeção determinística do baseline `b9d08f39`** · round-trip ao vivo, validador Python, gates e build **[NV]** · branch `docs/inbox-receipt-2026-11-21`

## Primária (local)

1. `.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9` — adjudicação do MAJOR D15 e a forma da correção: `HitlReceiptPort` + adaptador que emita **exatamente** o envelope, com teste round-trip ponta a ponta e allowlist de `next_stage`. `:13-15` fixa o critério de aceite: PR reviewable, gate verde, **[NV]** em staging/produção e credenciais, nenhuma credencial em log/PR.
2. `MDMV-Autonoma/src/contracts/delegation_contract.py:619-630` — `VerificationReceipt` tem exatamente 8 campos; `metrics_payload` é `dict` (default `{}`) e `signature_hmac` é `str` (default `""`). `:632-648` mostra que `unsigned_payload()` contém os outros 7 e que `to_dict()` apenas acrescenta a assinatura.
3. `delegation_contract.py:651-665` — a assinatura é HMAC-SHA256 hex sobre `json.dumps(payload, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")`; `secret` vazio levanta `ValueError`. Canonicidade é parte do contrato, não detalhe de implementação.
4. `delegation_contract.py:19-27,38` — `VALID_STATIONS` usa MAIÚSCULAS (`S0_RADAR` … `S6_TELEMETRY`); `FLORA_CORPORATE_SUITE` vale `"flora"`. `:723-752` exige os 7 campos não vazios, `station` nessa tupla e `metrics_payload` do tipo `dict`.
5. `delegation_contract.py:755-793` — `verify_verification_receipt` é fail-closed: `missing_hmac_secret`, envelope inválido, `invalid_signature` via `compare_digest`, `work_order_mismatch`, `station_mismatch`, `receipt_not_verified`; `valid` só quando `reason == "verified"`. O `status` do recibo precisa ser literalmente `"verified"`.
6. `MDMV-Autonoma/src/contracts/flora_bridge.py:16-26` — `ALLOWED_RESUME_STAGES` usa **minúsculas** (`s0_radar` … `s6_telemetry`). A diferença de caixa em relação a `VALID_STATIONS` é o ponto mais provável de erro de integração.
7. `flora_bridge.py:122-142` — `validate_flora_hitl_metrics` devolve `ok` somente com `operator_approved: bool`, `operator_id: str` não vazio, `pii_redacted is True`, `real_external_send` bool e `False`, e `next_stage` na allowlist (default `s4_monetization` se ausente). Qualquer violação gera um reason específico e `approved=False`.
8. `flora_bridge.py:145-200` — `ingest_flora_hitl_receipt` verifica HMAC, exige `producer_system == "flora"`, valida metrics, grava `flora_hitl_receipt_ingested` quando há `audit_store` e devolve `approved` igual ao `operator_approved`. O gate não aceita "quase" recibo.
9. `MDMV-Autonoma/specs/017-flora-hitl-receipt-gate/spec.md:18-25` — confirma a decisão HITL Tier 4 esperada (com `resume()` ou transição para `FALHA`), a rejeição para PII não redigido/envio externo/produtor errado/assinatura inválida e o endpoint receptor `POST /receipts/flora-hitl`.
10. Buscas executadas neste ciclo em `flora-api/src` e `flora-core/src`: **0 correspondências** para `receipt|work.?order|approval|operator|producer_system|external_send` e para `hitl|escalad|humano|aprova|tier.?4`. Não existe tipo, trait, adaptador, handler, erro ou constante de domínio HITL na Flora.
11. `flora-core/src/traits/mod.rs:3-9` — o padrão de port seguido pelo repositório: módulo em `traits/`, reexport em `mod.rs`. `flora-core/src/traits/storage_provider.rs:12-31` mostra trait assíncrono com implementação separada em `flora-core/src/storage/local.rs:182-219`; para o emissor de recibo, a operação é pura e não precisa de `async`.
12. `flora-core/src/config/mod.rs:49` — configuração por figment com `Env::prefixed("FLORA_").split("__")`; `FLORA_HITL__HMAC_SECRET` segue a convenção já usada por `FLORA_BOOTSTRAP__ADMIN_*` (`flora-core/src/config/bootstrap.rs:3-8,77-92`). `Cargo.toml:171,192` já declara `sha2 = "0.11.0"` e `hmac = "0.12"`: nenhuma dependência nova é necessária.
13. `flora-api/src/routes/mod.rs:1-15,20-70` — o roteador é explícito; adicionar uma rota de recibo criaria superfície HTTP desnecessária, pois o receptor já vive na Autônoma (spec 017:23-25). `flora-tests/src/contract/` é o diretório natural para os testes de contrato; `scripts/ci/` mostra a convenção de scripts stdlib do repositório.

## Leitura da reprodução

- A falha é estrutural e determinística: a Autônoma espera um produtor que a Flora nunca implementou. Não há "caminho quase correto" a ser ajustado — é uma lacuna de contrato bilateral.
- A autoridade do formato está na Autônoma; a Flora deve **seguir** o envelope, não reimplementar o validador nem negociar versão mais fraca. A factory é somente leitura neste ciclo.
- O envelope é sensível a dois detalhes: ordem/compactação do JSON (assinatura) e caixa das enumerations (`station` maiúsculo, `next_stage` minúsculo). Um vetor dourado e um caso negativo por reason tornam ambos verificáveis.
- O round-trip é provável sem rede: a fixture emitida pela Flora é consumida em processo pelos validadores da factory, com segredo de teste. Transporte ao vivo, URL, credenciais e aprovação por operador humano ficam **[NV]**.
- Segurança: `operator_id` e o payload são dados sensíveis; o emissor não deve registrar segredo, `operator_id`, PII nem payload em `tracing`, mensagens de erro, fixtures ou artefatos. `real_external_send` deve permanecer `False` em todos os cenários.

## Diagnóstico

Cadeia fechada: INBOX D15 (`:9`) → contrato completo na Autônoma (`delegation_contract.py:619-793`, `flora_bridge.py:16-200`, spec 017:20-21) → nenhum produtor na Flora (0 ocorrências nas buscas, `traits/mod.rs:3-9`, `routes/mod.rs:20-70`) → gate HITL sem entrada válida. Forma escolhida: port puro no core + adaptador na API que emite os 8 campos com HMAC canônico e validação fail-closed local, provado por fixture round-trip com os validadores da factory. Sem rota nova, sem dependência nova, sem alteração na factory; testes/gates/build e round-trip ao vivo ficam **[NV]** no handoff.

## [NV]

- Execução do validador Python, do harness round-trip, de `cargo test`/gates/build/formatters e de qualquer requisição: **[NV]** neste artefato — apenas inspeção documental e as duas buscas executadas.
- Round-trip ao vivo, `MDMV_FLORA_URL`, `POST /receipts/flora-hitl`, segredo real, `FLORA_HITL__HMAC_SECRET` em staging/produção e identidade de operador: **[NV]** — nenhuma credencial, URL ou dado de produção foi lido ou registrado.
- A existência de UI/service de aprovação Tier 4 na Flora e o mecanismo de transporte do recibo até a Autônoma: **[NV]** — o contrato de emissão não depende deles.
- Números de linha referem-se a `b9d08f39` (Flora) e ao snapshot da factory de 2026-11-21; implementação paralela pode deslocá-los.
