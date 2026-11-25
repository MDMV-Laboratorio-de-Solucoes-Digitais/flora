# Decision: 064 — N3 org-scoped roles

- **Slug**: 064-n3-org-scoped-roles
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md (problem equivalente) | shape.md (concept equivalente) | decision.md (anterior) | specs/064-n3-org-scoped-roles/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `memberships.role_id` sem FK (`init_schema.sql:33`) e resolução por `find_by_id(id)` permitem membership da org A usar papel da org B — escalada cross-tenant latente (intake.md:9-12). |
| Evidence strength | strong | Auditoria local: 0 órfãos, 0 cross-org, 3 memberships, 3 roles; FK testada em DB temporário limpo (rejeita cross-org/órfão, `RESTRICT`); teste contratual PASS; verificador 7/7 (converge.md). |
| Value vs. inaction | strong | Fecha ponteiro inválido entre tenants com autorização SE explícita (STATE 11:13Z); I=4 (intake.md:23-25). |
| Feasibility / appetite | adequate | 9 call sites trocados, assinaturas públicas de serviço alteradas (sem chamadores externos) e 2 migrations — esforço médio (shape.md; converge.md). |
| Strategic fit | strong | Endurecimento de isolamento multi-tenant alinhado a ADR-0006 e à tabela de escalonamento; exclui explicitamente deploy, limpeza, ADRs, C5 e C6. |
| Risk posture | adequate | Pré-voo fail-closed e `ON DELETE RESTRICT` protegem dados, mas produção não foi auditada nem deployada **[NV]**; papéis legados passam a `RoleNotFound` (fail-closed correto) e assinaturas públicas mudam. |

## Verdict & Rationale

Veredito histórico **`go (scoped — autorização SE explícita em STATE 11:13Z)` preservado**: a autorização cobria código e migrations, não produção. A auditoria local limpa permitiu a FK composta com pré-voo `RAISE EXCEPTION`; o banco agora rejeita pares cross-org e órfãos e o deploy em DB sujo aborta fechado (converge.md). Nenhum dado foi alterado, nenhuma limpeza feita e nenhum ADR/C5/C6 tocado. Âncora `github/dev@448090327`, branch `feature/064-n3-org-scoped-roles`; factory fonte `ce3d4213…`. Estado de produção e aprovação de deploy permanecem **[NV]** — voltar ao SE antes de qualquer deploy.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go` e a autorização SE está registrada.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Resolução de papel por id puro permite que membership da org A use papel da org B, sem impedimento no banco.
- **Chosen approach**: Duas camadas na ordem autorizada pelo SE — (1) `find_by_id_and_organization(id, org)` no trait/impl e troca dos call sites (`permission_service`, `rbac.rs`, `workspace.rs`, `org.rs`), com `RbacService::get_role` recebendo a org e validando o cache; (2) migrations `UNIQUE(organization_id, id)` em `roles` e FK composta com `ON DELETE RESTRICT` e pré-voo fail-closed (`shape.md`).
- **In scope / out of scope**: repositórios/serviços/rotas, 2 migrations, testes e artefatos; fora deploy em produção, limpeza de dados, auditoria em produção, ADRs, RLS (C6) e RoleWrite (C5).
- **Success metrics**: cross-org rejeitado na aplicação (teste); migrations em DB limpo e FK rejeitando insert inválido; fmt/clippy/deny/build/test + frontend verdes; auditoria local registrada e produção **[NV]** (define.md SC-001–SC-004).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: auditoria de produção e aprovação de deploy — **[NV]**, exigem retorno ao SE (ADR-0006:27-31)]; [NEEDS CLARIFICATION: destino dos papéis legados sem org, que agora viram `RoleNotFound`]; [NEEDS CLARIFICATION: se houver sujeira em produção, qual limpeza o SE escolhe].
