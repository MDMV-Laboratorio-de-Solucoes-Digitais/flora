# Decision: 007 — Org-Scoped Auth + Verifiable Core Slice

- **Slug**: 007-org-scoped-auth-e2e
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/007-org-scoped-auth-e2e/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O callback emitia `organization_id=nil` e não havia atualização de `organization_id/jti`, bloqueando as rotas endurecidas em 006 (`research.md:7-8,12-14`; `problem.md:9-11`). |
| Evidence strength | adequate | Auditorias de auth/sessão e frontend/contrato com achados convergentes, mais gates locais de 2026-11-15; smoke integrado com OIDC/DB/seed e duas sínteses de subagentes NV (`research.md:12-18,24-26`). |
| Value vs. inaction | strong | Sem promoção de escopo, o Seed autentica mas não funciona; adiar a checagem de membership abriria acesso a outras organizações (`problem.md:41-43`). |
| Feasibility / appetite | adequate | Opção B (`medium`) entregável por endpoint aditivo e revoke+create; toca no ponto mais sensível da sessão (`concept.md:18-21`). |
| Strategic fit | strong | Desbloqueio P0 para um Seed seguro/verificável em VPS barata, continuando 006 (`intake.md:7,13`; decisão original). |
| Risk posture | adequate | Risco médio de rotação/revogação mitigado por sessão ativa, membership, novo `jti` e escopo mínimo; testes dedicados do endpoint não adicionados (`spec.md:Deferred`). |

## Verdict & Rationale

Veredito histórico **`go (executed, partial slice)` preservado como `go`**: a decisão original de 2026-11-15 permanece coerente com os artefatos. A verificação de membership na promoção da org e a rotação com revogação respondem ao risco central; a execução integrada NV, o alinhamento de UI/WS deferido e as sínteses pendentes dos subagentes `ses_e1ea41e0fffeDgn1ojT3GM1BXp` e `ses_e1ea41e08ffeHj1Q40A2fgILJO` são follow-ups registrados, não contradição do `go`.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; perguntas abaixo ficam em aberto para o handoff).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Sessão pós-login sem escopo de organização (`organization_id=nil`, sem update de `organization_id/jti`) impedia o uso das rotas tenant-scoped e a verificação reproduzível do caminho `org → workspace → task` (`problem.md:7-11`).
- **Chosen approach**: Opção B — seleção explícita de org com sessão escopada e smoke no modelo Bearer (`concept.md:16-20,30-32`); Opção A (seleção automática) rejeitada por decidir pela org e poder burlar membership, Opção C (plataforma completa) fora do recorte.
- **In scope / out of scope**:
  - **In (executado em 2026-11-15)**: `POST /api/v1/auth/select-organization` em `flora-api/src/routes/auth.rs` — valida org/membership/sessão ativa, revoga a sessão atual e emite nova sessão+JWT org-scoped com `jti` rotacionado; `test-flora.sh` reescrito para Bearer/JWT com fluxo OIDC→callback→select-organization e chamadas tenant-scoped condicionadas a token/ambiente (`research.md:20-22`; `spec.md:Delivered`).
  - **Out/deferred/NV**: smoke e2e integrado com OIDC real/DB/seed e CI runtime remoto **NV**; alinhamento frontend/UI e contrato completo (seleção/persistência de contexto, WS/sync, endpoints/shapes) deferido; testes focados do endpoint não adicionados; recomendações dos dois subagentes pendentes na síntese podem gerar follow-ups.
- **Success metrics**: logins que resultam em sessão utilizável em rotas tenant-scoped (baseline: 0), promoção com `jti` rotacionado (baseline: 0), smoke de API no modelo Bearer/JWT executável (baseline: não), caminho `org → workspace → task` com evidência (baseline: NV); gates locais verdes — fmt/check, `flora-core --lib` (8), `flora-tests --lib` (54) — com runtime NV (`problem.md:33-39`; decisão original:21-26).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: reutilizar a mesma sessão com `jti` alterado ou revogar e criar nova, e qual política de refresh resultante?]; [NEEDS CLARIFICATION: contrato do ponto de seleção (payload, erros, resposta) e quem o consome?]; [NEEDS CLARIFICATION: existe ambiente com OIDC/DB/seed para validar o fluxo, ou o smoke permanece NV?] (`problem.md:45-49`) — ver também hardening adicional (rate limit, auditoria, refresh/rotation) conforme `research.md:27`.
