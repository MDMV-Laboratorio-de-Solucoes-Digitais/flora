# Shape: 106 — Port de recibo HITL com contrato bilateral

**Data:** 2026-11-21 · **Branch**: `docs/inbox-receipt-2026-11-21@b9d08f3904eab6aeed93d9d6732434356e583c65`

## Forma escolhida

Criar apenas o lado produtor que falta, mantendo a Autônoma como autoridade única do contrato. Um port puro no core descreve a decisão HITL e valida antes de emitir; um adaptador na API serializa e assina o envelope exato; um harness somente leitura prova o round-trip contra os validadores da factory. Nenhuma rota HTTP nova, nenhuma dependência nova, nenhuma reimplementação do validador, nenhuma credencial em log. O padrão segue `traits/storage_provider.rs:12-31` + `storage/local.rs:182-219` (trait no core, implementação fora), simplificado para operação pura.

1. **Causa e rastro (T001 / intake, research, bug D15)**: fixar o INBOX D15 (`:9`), o contrato na Autônoma (`delegation_contract.py:619-793`, `flora_bridge.py:16-200`, spec 017:20-21) e a ausência total na Flora (0 ocorrências nas buscas; `traits/mod.rs:3-9`; `routes/mod.rs:20-70`). Registrar dívidas/**[NV]**. Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).
2. **Port e tipos (T002 / FR-001)**: `flora-core/src/traits/hitl_receipt.rs` com `HitlReceiptPort`, `HitlDecision` e o recibo de 8 campos; reexport em `mod.rs`. Sem I/O, segredo ou log.
3. **Travas fail-closed (T002 / FR-003)**: validar `next_stage` minúsculo contra `ALLOWED_RESUME_STAGES`, `station` maiúsculo contra `VALID_STATIONS`, os quatro campos não vazios, os quatro flags/métricas de `metrics_payload` e o segredo não vazio antes de assinar. `status="verified"` e `producer_system="flora"` são constantes do emissor.
4. **Adaptador e segredo (T003 / FR-002)**: `flora-api/src/adapters/hitl_receipt.rs` monta os 7 campos, serializa com `sort_keys`/`separators`/`ensure_ascii=False`, assina HMAC-SHA256 hex com `hmac 0.12`/`sha2 0.11` e devolve o 8º campo. Segredo em `FLORA_HITL__HMAC_SECRET` via figment (`config/mod.rs:49`); `Debug` redigido; nenhuma rota nova.
5. **Sigilo (T004 / FR-005)**: proibir `tracing` de segredo, `operator_id`, PII e payload; mensagens de erro genéricas; fixture com dados sintéticos. Varredura de 0 ocorrências em stdout/stderr/artefatos.
6. **Round-trip (T005 / FR-004, SC-001, SC-002)**: vetor dourado no teste de contrato da Flora e harness `scripts/contract/check_hitl_receipt_roundtrip.py` que importa `verify_verification_receipt` e `ingest_flora_hitl_receipt` da factory, sem modificá-la e sem HTTP. Caminho feliz + um caso por reason de rejeição.
7. **Dívidas e handoff (T006, T007 / FR-005, SC-003)**: registrar transporte ao vivo, acionador humano, staging/credenciais e idempotência como **[NV]**; confrontar FR/SC com o baseline; deixar build, testes e gates para o handoff.
8. **Fecho documental (T008 / FR-005, SC-003)**: rastrear T001..T008 em decision/spec/plan/tasks, verificar fronteira proibida e ausência de commit.

- **Contrato unilateral (motivo do ciclo)**: a Autônoma valida, a Flora não emite — mitigado criando o produtor exato, sem tocar no consumidor.
- **Canonicidade da assinatura**: qualquer mudança em `sort_keys`, `separators` ou `ensure_ascii` (`delegation_contract.py:651-665`) invalida o HMAC sem alterar o conteúdo lógico; vetor dourado protege.
- **Caixa das enumerations**: `station` maiúsculo (`delegation_contract.py:19-27`) × `next_stage` minúsculo (`flora_bridge.py:16-26`); casos RT-004 e RT-011 no plano de teste protegem contra inversão.
- **Vazamento de segredo/PII**: risco LGPD e quebra do critério de aceite do INBOX (`:15`); mitigado por config redigida, erro genérico e varredura do harness.
- **Tentação de duplicar o validador**: reimplementar a checagem na Flora criaria duas autoridades; o port apenas valida a elegibilidade do input e o harness usa o validador real da factory.
- **Tentação de abrir rota HTTP**: o receptor já existe na Autônoma (spec 017:23-25); nova rota aumentaria superfície sem benefício contratual.
- **Dependências**: `hmac`/`sha2` já estão no workspace (`Cargo.toml:171,192`); nenhuma inclusão é feita.
- **Resultado de testes/gates e round-trip ao vivo**: **[NV]** neste artefato — não executamos harness, `cargo test`, build, gates nem requisições aqui.

## Entrega

Artefatos do ciclo 106 (assessoria, spec/plan/tasks e bug D15) e atualização de `.specify/feature.json` via `create-new-feature.sh`. Diff de implementação previsto (fora deste artefato): `flora-core/src/traits/hitl_receipt.rs` (novo), `flora-core/src/traits/mod.rs`, `flora-core/src/config/hitl.rs` (novo), `flora-api/src/adapters/hitl_receipt.rs` (novo), wiring em `flora-api/src/`, `flora-tests/src/contract/hitl_receipt.rs` (novo) e `scripts/contract/check_hitl_receipt_roundtrip.py` (novo). Arquivos lidos, não modificados: todos os contratos e a spec 017 da factory, INBOX, `traits/*`, `config/mod.rs`, `routes/*`, `Cargo.toml`. Verificação documental cobre T001–T008; harness, gates, build e round-trip ao vivo ficam no handoff **[NV]**. Nenhum commit.
