# Decision: Gestão de membros e papéis RBAC em `/settings`

- **Slug**: 122-org-members-rbac-management
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (original 2026-11-24) | specs/122-org-members-rbac-management/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `research.md:5-11` mostra `assign_role` com `delete`+`create` (perda de `joined_at`/`metadata`), `revoke_role` com auto-lockout e ausência de `GET/POST /api/v1/roles/members`. |
| Evidence strength | adequate | file:line determinísticos em `research.md:5-17`, repositórios já existentes e seed real (`dev-seed.sql:28-73,153-185`) para E2E; execução de contratos e Playwright fica no handoff. |
| Value vs. inaction | strong | É a última entrega acionável do `backlog_de_fechamento`: encerra `BF-02` integralmente (`intake.md:7-11`); sem ela, restam apenas itens bloqueados por decisão externa. |
| Feasibility / appetite | adequate | `update_role` atômico via `UPDATE ... RETURNING` e `find_by_ids` em lote evitam nova migration; lockstep `rbac: 3 → 5` (`85 → 87`) e UI segue o padrão `RoleStore` (`shape.md:1-5`). |
| Strategic fit | strong | Fecha a ponta a ponta a gestão RBAC em `/settings` iniciada em 119 e deixa o backlog com apenas `BF-05` (RLS runtime) e `BF-06` (expiração) pendentes (`intake.md:11`). |
| Risk posture | adequate | `403` contra auto-rebaixamento/auto-revogação, DTO sem campos sensíveis, `deny_unknown_fields`, rotas `/members` antes de `/{id}` e autoridade da sessão mitigam enumeração e lockout (`shape.md:2-4`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**: os artefatos não contradizem a decisão de 2026-11-24. O scorecard sustenta o `go` pelo escopo remanescente claramente observável e pelas evidências de código; a decisão original registra o ciclo como última entrega acionável, encerrando **`BF-02 → fechado`** (gestão de membros + atribuição/revogação de papéis em `/settings` ponta a ponta). Restrições transportadas: Briefing de Compreensão (`auth`/`rbac`) comprovando preservação multi-tenant, proteção contra auto-rebaixamento/auto-revogação e ausência de enumeração anônima; lockstep de pisos DB (`rbac: 3 → 5`, total `85 → 87`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`; zero migrations SQL novas; zero binários/PNGs no git; abertura de 4 issues R6 (`#420`–`#423`) sem `dark-factory:eligible`.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma incógnita bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `/api/v1/roles/{id}/assign` e `revoke` permitem auto-lockout e perdem metadados, e não há como listar ou adicionar membros da organização ativa (`intake.md:7`, `research.md:5-11`).
- **Chosen approach**: `MembershipRepository::update_role` atômico preservando `joined_at`/`metadata`, rotas `/roles/members` montadas antes de `/{id}`, hardening de `assign`/`revoke` com `403` de auto-ação e `MemberManager.svelte` seguindo o padrão `RoleStore`/`syncContext` (`shape.md:1-5`).
- **In scope / out of scope**: **In** — `GET/POST /api/v1/roles/members`, `update_role`, `assign`/`revoke` endurecidos, 2 contratos PG 18 (`rbac: 5`, total `87`), `MemberStore`, `MemberManager.svelte`, `encodeURIComponent(roleId)`, TDD e E2E com WCAG 2.2 AA. **Out** — envio de e-mail transacional SMTP/`POST /users/invite` para usuários sem conta; a adição opera sobre usuários ativos já registrados (`define.md:23`).
- **Success metrics**: FR-001..FR-004 em `specs/122-org-members-rbac-management/spec.md` — DTO sem `password_hash`/`oidc_subject`/`profile`, `201`/`409`/`404` conforme o caso, `403` nas auto-ações, `409` em divergência de papel, `87` pisos DB, TDD `RED→GREEN` backend/frontend e Passo 9 E2E com `reload()` e Axe sem achados.
- **Carried-forward open questions**: `[NV]` execução de contratos DB, Playwright real e ACR neste artefato — decisão documental; `[NEEDS CLARIFICATION: ao remover o último administrador de uma organização por ação de outro administrador, a operação deve ser bloqueada por regra de último admin ou apenas a auto-revogação permanece proibida?]` (`define.md:11-14`); `[NEEDS CLARIFICATION: a promoção `dev → main` e o PR `#410` dependem de aprovação externa — registrar como `[NV]` até confirmação]` (`spec.md:FR-004`).
