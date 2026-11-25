# Decision: 103 — RBAC na criação de workspace

- **Slug**: 103-workspace-create-rbac
- **Decided**: 2026-11-25
- **Verdict**: go (decisão original 2026-11-21, preservada)
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (anterior) | roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:184-188` | spec/plan/tasks 103

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Item 11 do roteiro exige 403 e nenhuma alteração de outra org; `SomenteLeituraQA` (`permissions=[]`, "NegadoQA") obtém 201 em `POST /api/v1/workspaces` (`intake.md:9-14`; `decisão anterior:13`). |
| Evidence strength | adequate | Reprodução em staging local zerado e cadeia determinística em um handler, com update/delete já implementando o padrão; gates e staging real **[NV]** (`research.md`; `decisão anterior:35-44`). |
| Value vs. inaction | strong | Fecha uma autorização ausente reaproveitando repositório, enum de permissões, erro 403 e convenção já presentes, sem nova superfície ou regra (`decisão anterior:48`). |
| Feasibility / appetite | adequate | Um bloco de consulta de papel e decisão idêntico ao de `update_workspace`; custo aceito é repetição intencional, sem abstração nova (`shape.md`; `decisão anterior:33,48`). |
| Strategic fit | strong | Mantém membership existente, tenant por `organization_id`, papéis do seed e fronteira de diff; nenhum serviço, flag ou schema novo (`define.md:FR-001..FR-005,SC-001..SC-003`). |
| Risk posture | adequate | `find_by_id_and_organization` barra papel de outra org e o 403 genérico de corpo constante evita vazamento; visibilidade da UI e RLS **[NV]** (`decisão anterior:52-55`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. Registro original de 2026-11-21, branch `feature/103-workspace-create-rbac` @ `14cb336173e01d63d6edb161a7b1ad09f520683b`, âncora `.specify/feature.json` → `specs/103-workspace-create-rbac`, evidência de reprodução em staging local zerado (roteiro item 11) e inspeção determinística do código.

Causa raiz: a rota `POST /api/v1/workspaces` (`flora-api/src/routes/mod.rs:39`, `workspace.rs:84-92`) não tem middleware de permissão; `create_workspace` obtém `org_id` da sessão e verifica apenas membership (`Forbidden("Not a member")`, `workspace.rs:126-137`) e em `:139-153` monta `CreateWorkspaceInput`, chama `workspace_service.create_workspace` e devolve `201`, sem consultar o papel — a lacuna está entre `:137` e `:139`. `WorkspaceService` valida e grava sem RBAC por desenho (`flora-core/src/services/workspace_service.rs:29-39`). Update e delete já carregam o papel com `PgRoleRepository::find_by_id_and_organization` (`flora-core/src/repositories/role_repository.rs:40-56`) e negam com `Error::Forbidden("Insufficient permissions")` (`workspace.rs:208-225,256-271`). `has_permission` reconhece `workspace:write`, `workspace:admin`, os aliases legados `WorkspaceWrite`/`WorkspaceAdmin` e `*` (`flora-core/src/models/role.rs:66-75,80-110,126-131,189-190`); com `permissions=[]`, ambas as checagens são `false`. `Forbidden(_)` já produz 403 com corpo constante (`flora-core/src/error.rs:291-294`).Isolamento entre organizações e checagem de membership continuam vigentes; falta exigir a permissão de escrita na criação.

| Compromisso rastreável | Conteúdo preservado |
|---|---|
| FR-001, FR-002 / T002 — consulta de papel | Após membership (`workspace.rs:132-137`), `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` (`:216-220`; `role_repository.rs:40-56`), com `Error::RoleNotFound` se o papel não pertence ao org, exatamente como `update_workspace`; repositório já importado em `:17-18`. |
| FR-001, FR-003 / T003 — decisão e negação | Exigir `WorkspaceWrite \|\| WorkspaceAdmin` (o par do update, não o `WorkspaceAdmin` exclusivo do delete, `:221-223`); sem ambas, `Error::Forbidden("Insufficient permissions")` (`:224`) antes da gravação (`:139-147`); corpo 403 constante (`error.rs:291-294`), sem log ou detalhe novo; `*` e aliases legados continuam valendo (`role.rs:66-110`). |
| FR-002 / T002–T003 — sucesso inalterado | `org_id` da sessão, `Forbidden("Not a member")`, `RoleNotFound`, `CreateWorkspaceInput`, `WorkspaceResponse`, `tracing::info!` e `201` idênticos (`:126-153`); nenhuma rota, flag, schema, migração, gate ou permissão nova. |
| FR-004 / T004–T007 — contratos | `flora-tests/src/contract/workspace.rs` ganha helper de seed com `&[Permission]` (padrão `rbac.rs:55-81`) e POST autenticado; 201 para `WorkspaceWrite`/`WorkspaceAdmin`/`*`; 403 genérico e nenhuma gravação para `permissions=[]` ("NegadoQA"); fail-closed para papel de outra org; 403 de membership ausente; listagem/isolamento atuais preservados (`:134-216`). |
| FR-005 / T008 — fronteira | Diff restrito a `flora-api/src/routes/workspace.rs`, `flora-tests/src/contract/workspace.rs` e artefatos do ciclo 103; sem `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição; factory somente leitura; sem commit. |

Critérios preservados: **SC-001** reprodução fechada (`permissions=[]` → 403 com `{"error":"forbidden","message":"Insufficient permissions"}` e nenhum workspace criado; `WorkspaceWrite`/`WorkspaceAdmin` → 201 com `WorkspaceResponse` idêntico); **SC-002** consistência RBAC e isolamento (mesmo par de update nos três handlers, guard cross-org); **SC-003** contrato estável e fronteira preservada. Fila **T001–T008**. Racional histórico: a lacuna está em um único handler; a correção reaproveita repositório, enum, mensagem e convenção sem nova superfície; escolher o par do update mantém `Member`/`WorkspaceWrite` capazes de criar; o 403 constante não vaza papel, usuário ou existência; `find_by_id_and_organization` cobre o risco cross-org do item 11; FR/SC verificáveis por contratos DB e inspeção, diff reversível. Nomes `SomenteLeituraQA`/`NegadoQA` vêm do cenário relatado; nenhum segredo, token ou dado de produção foi lido.

**[NV]** preservados: execução do item 11 em staging real zerado e estado do DB/staging de produção (a reprodução é a entrada; nenhuma requisição/escrita ao vivo); gates, build, testes e formatters (consolidados no handoff); visibilidade do botão de criação na UI e RLS efetivo do PostgreSQL (o roteiro aceita bloqueio por 403; checagem de aplicação mantida).

Fora de âmbito preservado: mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`; alteração de required checks, workflows, ADRs, constituição ou changelog; mudança em update/delete, `WorkspaceService`, repositórios, `has_permission`, modelo de erros, listagem/switch de workspaces ou UI/erro do frontend; migração, schema, RLS, role fixo, flag, rota ou permissão nova; deploy, escrita em staging/produção e commits.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante registrada nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `POST /api/v1/workspaces` retorna 201 para um papel sem permissões (`SomenteLeituraQA`/`NegadoQA`, `permissions=[]`) porque a criação verifica apenas membership, enquanto update e delete já exigem RBAC (`roteiro:184-188`; `workspace.rs:126-153,208-225,256-271`).
- **Chosen approach**: Inserir no handler de criação o mesmo bloco de `update_workspace` — `find_by_id_and_organization` e `WorkspaceWrite || WorkspaceAdmin` — com `Error::Forbidden("Insufficient permissions")` antes da gravação, preservando o corpo 403 constante e os aliases legados/`*`.
- **In scope / out of scope**:
  - **In**: FR-001..FR-005 / SC-001..SC-003, fila T001..T008; `workspace.rs`, contratos `workspace.rs` com helper `&[Permission]`, cenários 201/403/cross-org/membership e regressão de listagem/isolamento.
  - **Out/NV**: update/delete, `WorkspaceService`, repositórios, `Role`/`has_permission`, modelo de erros, UI/frontend, migração/schema/RLS/flag/rota/permissão nova; `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog, constituição; deploy/escrita ao vivo/commits; staging real, gates e RLS **[NV]**.
- **Success metrics**: `permissions=[]` → 403 genérico e nenhum workspace gravado; `WorkspaceWrite`, `WorkspaceAdmin` e papéis com `*` → 201 com `WorkspaceResponse` idêntico; papel de outra org e membership ausente → falha fechada; contratos `db_workspace_list_returns_seeded_workspace`, `db_n3_cross_org_role_lookup_rejected` e `db_workspace_isolation_between_orgs` permanecem PASS.
- **Carried-forward open questions**: `[NV]` item 11 em staging real zerado e estado do DB; `[NV]` gates/build/testes/formatters; `[NV]` visibilidade do botão na UI e RLS efetivo — nenhuma bloqueia o `go`.
