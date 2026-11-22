# Intake: 106 — Port de recibo HITL com contrato bilateral

**Data:** 2026-11-21T00:00Z · **Âncora:** `docs/inbox-receipt-2026-11-21@b9d08f3904eab6aeed93d9d6732434356e583c65` · `.specify/feature.json` → `specs/106-hitl-receipt-port`

## Sinal

O item 1 do INBOX de correções da revisão interagentes (`.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9`) adjudica o **MAJOR D15 — contrato HITL unilateral**: a Autônoma valida `VerificationReceipt` HMAC de forma completa e fail-closed, enquanto a Flora não possui emissor algum. Buscas executadas neste intake em `flora-api/src` e `flora-core/src` confirmam **0 ocorrências** de `receipt|work.?order|approval|operator|producer_system|external_send` e de `hitl|escalad|humano|aprova|tier.?4`; `flora-core/src/traits/mod.rs:3-9` inventaria apenas auth, repositório e storage; `flora-api/src/routes/mod.rs:1-15,20-70` não registra rota de recibo. O bug está registrado em `.specify/bugs/d15-hitl-receipt-contract-unilateral/bug.md` e avaliado em `assessment.md` do mesmo diretório, com proposta em `fix.md` e plano round-trip em `test.md`.

O contrato que falta reproduzir é preciso e já existe do outro lado: envelope de 8 campos (`MDMV-Autonoma/src/contracts/delegation_contract.py:619-648`), JSON canônico com `sort_keys=True`, `separators=(",",":")`, `ensure_ascii=False` e HMAC-SHA256 (`:651-665`), verificação fail-closed (`:723-793`), `producer_system="flora"` (`:38`), `station` em maiúsculas (`:19-27`), `next_stage` em minúsculas (`flora_bridge.py:16-26`) e cinco regras de `metrics_payload` (`:122-142`). A correção é criar o lado produtor na Flora — `HitlReceiptPort` + adaptador — sem tocar na factory, sem nova rota HTTP e sem nenhuma credencial em log.

## Evidência

```
INBOX / ADJUDICAÇÃO:
.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md:9   MAJOR D15, emissor ausente;
:13-15  critério de aceite (round-trip, gate verde, [NV], nenhuma credencial em log/PR).
.specify/bugs/d15-hitl-receipt-contract-unilateral/{bug,assessment,fix,test}.md

AUSÊNCIA NO PRODUTOR (buscas executadas, 0 resultados):
flora-api/src + flora-core/src: receipt|work.?order|approval|operator|producer_system|external_send;
flora-api/src + flora-core/src: hitl|escalad|humano|aprova|tier.?4.
flora-core/src/traits/mod.rs:3-9          ports existentes: auth_provider, repository, storage_provider;
flora-core/src/traits/                   arquivos: auth_provider.rs, repository.rs, storage_provider.rs;
flora-api/src/routes/mod.rs:1-15,20-70    nenhuma rota de recibo/HITL;
flora-api/src/routes/                     13 módulos, nenhum de HITL.

CONSUMIDOR (factory MDMV-Autonoma, somente leitura):
src/contracts/delegation_contract.py:19-27    VALID_STATIONS (S0_RADAR..S6_TELEMETRY, maiúsculas);
:38                                           TargetSystem.FLORA_CORPORATE_SUITE = "flora";
:619-630                                      8 campos do VerificationReceipt;
:632-648                                      unsigned_payload() (7) / to_dict() (8);
:651-665                                      json.dumps(sort_keys, separators, ensure_ascii=False) + HMAC-SHA256;
:723-752                                      envelope obrigatório + station + metrics_payload dict;
:755-793                                      compare_digest, status == "verified", reason/valid.
src/contracts/flora_bridge.py:16-26           ALLOWED_RESUME_STAGES (s0_radar..s6_telemetry, minúsculas);
:122-142                                      operator_approved/operator_id/pii_redacted/real_external_send/next_stage;
:145-200                                      ingest: producer, metrics, auditoria, approved.
specs/017-flora-hitl-receipt-gate/spec.md:18-25  decisão HITL e endpoint receptor esperados.

BASE E PADRÕES DA FLORA:
flora-core/src/config/mod.rs:49             figment Env::prefixed("FLORA_").split("__");
Cargo.toml:171,192                          sha2 0.11.0 e hmac 0.12 já presentes (sem dependência nova);
flora-tests/src/contract/                   diretório de testes de contrato existente;
flora-core/src/traits/storage_provider.rs:12-31  padrão de trait + implementação em adapter.
```

Baseline observado: `b9d08f39` na Flora; fact finding documental, sem build, teste, gate, formatter ou requisição executada neste artefato. Nenhuma credencial, URL de staging ou dado de produção foi lido ou registrado.

## Autorização

Modo autônomo. Decisão `go` em `.specify/assessments/106-hitl-receipt-port/decision.md` antes da implementação. Escopo documental deste artefato: os cinco arquivos de assessoria, `specs/106-hitl-receipt-port/{spec,plan,tasks}.md` (com substituição do `spec.md` modelo gerado pelo script) e os quatro arquivos do bug D15. Escopo previsto para a implementação posterior: (`1`) `HitlReceiptPort` + `HitlDecision` puros no core; (`2`) adaptador emissor na API com envelope exato de 8 campos e HMAC canônico; (`3`) allowlist de `next_stage` e travas fail-closed espelhando a Autônoma; (`4`) round-trip com fixture e casos negativos, sem credencial em log; (`5`) dívidas e **[NV]** registrados. Nenhuma mudança na factory, em gates, required checks, rotas HTTP, ADRs, validate/release, STATE/PROGRESSO; sem commit.

## Fronteira

Artefatos novos do ciclo 106: `.specify/bugs/d15-hitl-receipt-contract-unilateral/{bug,assessment,fix,test}.md`, `.specify/assessments/106-hitl-receipt-port/{intake,research,define,shape,decision}.md` e `specs/106-hitl-receipt-port/{spec,plan,tasks}.md`, além da atualização de `.specify/feature.json` via `create-new-feature.sh --number 106 --short-name hitl-receipt-port`. Somente leitura: `MDMV-Autonoma/` (factory), `.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md`, contratos e spec 017 da Autônoma, `flora-core/src/traits/*`, `flora-core/src/config/mod.rs`, `flora-api/src/routes/*`, `Cargo.toml` e `.specify/bugs/gate-integrity-unguarded/*`. Sem toques em `specs/106-proptest-invariants/`, `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição. Nenhum código, gate ou commit neste artefato.
