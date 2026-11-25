# Decision: Endpoint HTTP RBAC para emissão de recibo HITL

- **Slug**: 120-hitl-receipt-http-endpoint
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (original 2026-11-24) | specs/120-hitl-receipt-http-endpoint/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `intake.md:5-9` e `research.md:7-12` mostram `HitlReceiptPort`/`HitlReceiptAdapter` já prontos desde o ciclo 106 (D15), porém sem rota HTTP no roteador principal (`BF-03`). |
| Evidence strength | adequate | Diagnóstico determinístico com file:line em `research.md:5-13` (`models`, `traits`, `adapters`, `app_state.rs:62-64`, `error.rs:360`, `routes/mod.rs`); execução de gates e contratos fica no handoff. |
| Value vs. inaction | strong | Fecha `BF-03` com consumidor real (`flora-api` + `MDMV-Autonoma/src/contracts/flora_bridge.py` + `db-tests`) e elimina o contrato unilateral de recibos (`decisão original`). |
| Feasibility / appetite | adequate | Entrega focada (~250–350 LOC de código + testes), sem alteração de schema SQL nem do `Cargo.lock`, reaproveitando o adaptador existente (`research.md:13`). |
| Strategic fit | strong | Completa a ponte HTTP/RBAC do contrato bilateral HITL iniciado em 106, reforçando os invariantes de servidor frente à Autônoma. |
| Risk posture | adequate | `OrgAdmin`, `operator_id` via `auth.user_id`, `deny_unknown_fields`, `422` antes da checagem do segredo e `503` sanitizado mitigam spoofing e vazamento (`define.md:7-11`, `shape.md:1-7`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**: nada nos artefatos contradiz a decisão de 2026-11-24. O scorecard sustenta o `go` pelo problema de contrato bilateral observável e pela evidência local determinística; o score original **2000 / 3125** (`impacto=5 × confiança=5 × fechamento=5 × (6−esforço=2)=4 × (6−risco=2)=4`) é mantido, com os comparativos registrados: `BF-04` `1500/3125` e `BF-02` remanescente `384/3125`. As restrições originais seguem válidas: domínio crítico `auth/rbac/hitl`, Briefing de Compreensão, `3` contratos DB (`contract::hitl::db::db_*`) com trava lockstep `82 → 85`, `R4`/`R7` (`0.12.0` MINOR) e ACR independente (`intake.md:9`).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma incógnita bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`
- **Problem**: Operadores administrativos e integradores Tier 4 não têm endpoint HTTP em `flora-api` para assinar e emitir o envelope `HitlReceipt` já implementado internamente (`intake.md:5`).
- **Chosen approach**: Montar `.nest("/api/v1/hitl", hitl::create_hitl_router())` com `POST /receipts`, helpers puros (`can_emit_hitl_receipt`, `build_decision`), guarda `require_hitl_operator`, conversão de `Error::Configuration(_)` em `503` sanitizado e mapeamento de `Error::ServiceUnavailable(_)` em `flora-core/src/error.rs` (`shape.md:1-7`).
- **In scope / out of scope**: **In** — rota autenticada por Bearer com `Permission::OrgAdmin`, `organization_id` de `auth.require_org()`, `operator_id` vinculado no servidor, `deny_unknown_fields`, validação fail-closed, HMAC-SHA256, `503` sem vazar segredo, testes unitários, 3 contratos DB e lockstep `hitl: 3` (`82 → 85`). **Out** — alteração de schema SQL, mudanças em `apps/frontend`, transporte ao vivo/segredo real e credenciais (`research.md:13-15`, `spec.md:[NV]`).
- **Success metrics**: SC-001..SC-003 em `specs/120-hitl-receipt-http-endpoint/spec.md` — `401` sem JWT, `403` sem `OrgAdmin`, `422` para campo extra/decisão inválida, `201` com os 8 campos canônicos (`producer_system="flora"`, `status="verified"`, `signature_hmac`), `503` com corpo `{"error":"service_unavailable","message":"Service temporarily unavailable"}`, e portões `fmt`/`clippy`/`deny`/`test --workspace`/`check_gate_integrity.py` (`85` com `hitl: 3`) verdes.
- **Carried-forward open questions**: `[NV]` execução de gates, contratos DB e ACR neste artefato — decisão documental; `[NV]` deploy externo (Coolify) e interface visual para operadores Tier 4 (`spec.md:[NV]`); `[NEEDS CLARIFICATION: o acionador humano/transporte do recibo para a Autônoma permanece responsabilidade externa ou será endereçado em ciclo posterior?]` (escopo em `intake.md:9` cobre apenas a emissão local).
