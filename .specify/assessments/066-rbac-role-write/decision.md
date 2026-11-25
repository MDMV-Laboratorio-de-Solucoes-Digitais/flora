# Decision: 066 — RBAC RoleWrite

- **Slug**: 066-rbac-role-write
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (anterior) | specs/066-rbac-role-write/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Quatro handlers em `rbac.rs:131-164` são placeholders que só retornam strings; a autoridade para mutar papéis não estava definida (intake.md:9-17). |
| Evidence strength | adequate | `research.md:10-13` marcava presença de `RoleWrite` e assinaturas de repositórios como pendentes no mapeamento; o ciclo fechou com 5 testes unitários PASS, verificador e gates verdes, mas execução em DB real **[NV]** (converge.md). |
| Value vs. inaction | strong | Transforma no-ops em CRUD real no tenant da sessão e materializa a autoridade decidida pelo SE; I=4 (intake.md:27). |
| Feasibility / appetite | adequate | Quatro operações, ajuste no modelo de permissões e testes; `converge.md` registra correções de lint (`const fn`, `String::from`, import) antes dos gates verdes. |
| Strategic fit | strong | Segue a decisão SE (permissão `RoleWrite`, `OrgAdmin` habilitado por padrão, builtins imutáveis) sem tocar RLS (C6), ADRs, constituição ou deploy. |
| Risk posture | adequate | Endpoints antes inofensivos passam a mutar dados (risco médio), mas são reversíveis por `git revert` e não há deploy/RLS; execução real em DB/CI **[NV]**. |

## Verdict & Rationale

Veredito histórico **`go (scoped — autorização SE em STATE 14:38Z)` preservado**: a decisão SE é explícita quanto à autoridade e à imutabilidade de `is_builtin`, e nenhuma autorização adicional era necessária. A implementação ficou em `flora-api/src/routes/rbac.rs` com DTOs, `can_manage_roles = RoleWrite || OrgAdmin`, guarda de builtin e assign/revoke via repositório de membership; `revoke_role` = remover a membership do usuário na org e `assign_role` = delete+create (assunções registradas). Âncora `github/dev@8d0cb64b0`, branch `feature/066-rbac-role-write`; factory fonte `c81f0f81…`. Nenhum ADR/RLS/constituição/migration/deploy tocado.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go` e a autorização SE está registrada.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Handlers de escrita de papéis RBAC retornam strings sem gravar nada e a autoridade para mutar papéis não estava definida.
- **Chosen approach**: Option A de `concept.md` — CRUD real com permissão `RoleWrite` dedicada, `OrgAdmin` por padrão e papéis `is_builtin` imutáveis; DTOs serde e handlers no contexto da org da sessão (`shape.md`).
- **In scope / out of scope**: 1 arquivo de rota (`rbac.rs`), modelo de permissão se necessário, serviços/repositórios, testes e artefatos; fora RLS/produção (C6), convites/email/token, ADRs, constituição e deploy.
- **Success metrics**: testes unitários cobrem `RoleWrite` e builtin; fmt/clippy/deny/build/test + frontend verdes; CI verde e diff revisável; nenhuma mudança em RLS/ADRs/constituição (define.md SC-001–SC-004).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: `RoleWrite` já existe no modelo `Permission` ou a variante precisou ser adicionada]; [NEEDS CLARIFICATION: assinaturas de `RoleRepository`/`MembershipRepository` e a semântica de `revoke_role`]; [NEEDS CLARIFICATION: em qual PR a execução real em DB/CI será observada — **[NV]**].
