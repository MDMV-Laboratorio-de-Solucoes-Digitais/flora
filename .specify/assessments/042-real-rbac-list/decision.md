# Decision: 042 — Listagem real de papéis RBAC

- **Slug**: 042-real-rbac-list
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `GET /api/v1/roles` é placeholder (`vec![]`/strings) sem extrator de autenticação, então o primeiro comportamento RBAC verificável do Seed não existe (intake.md:10,19; research.md:18; problem.md:9). |
| Evidence strength | adequate | Padrão derivado de `workspace.rs`, OWASP e auditoria sobre snapshot congelado que encontrou e corrigiu D1 (403 para todos os seed) e D2 (422 vs 400), mas confiança overall `medium`, sem teste DB-gated de isolamento e comportamento live [NV] (research.md:5,37-39,44-45). |
| Value vs. inaction | strong | Entrega o primeiro comportamento RBAC verificável com tenancy vinda da sessão e evita que clientes continuem a tratar papéis como lista vazia ou fonte não autorizada (problem.md:38-40). |
| Feasibility / appetite | adequate | Medium: regra pura e DTO acordados, mas o comportamento com banco real segue sem observação [NV] (concept.md:11). |
| Strategic fit | strong | Primeiro passo concreto de autorização, com operações de escrita e decisões de Owner deliberadamente diferidas e RLS mantido [NV] (research.md:14,26; concept.md:12). |
| Risk posture | adequate | D1 mostrou ponto cego nos testes originais, D2 divergência de status, gate aceita `*` além de `OrgRead|OrgAdmin`, RLS inerte com superuser e docs citam `X-Organization-ID` obsoleto (research.md:46-49; concept.md:12-13). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: validade e valor são `strong`, evidência/apetite/risco `adequate` pelas ressalvas de auditoria e pelo comportamento live não observado. A Option A de `concept.md` — listagem real derivada da sessão — responde às metas de tenancy, regra pura, DTO sem vazamento e correção das divergências de status; a Option B entrega só a regra e repete o ponto cego que deixou todos os usuários semeados, inclusive Admin, receberem 403 (concept.md:27-29; research.md:44). O recorte continua limitado a `GET /roles`: `create/get/{id}/assign/revoke`, proteção de último Owner e RLS seguem como decisões de dono/SE, e a afirmação de tenant-safety live permanece `[NV]` (decision.md antigo:11-23).

## If needs-clarification

N/A — veredito `go`. Incertezas registradas como `[NEEDS CLARIFICATION:]`/`[NV]` no handoff abaixo; nenhuma pergunta bloqueante do recorte aprovado.

## If go — Handoff to `/speckit.specify`

- **Problem**: A listagem de papéis é um placeholder sem autenticação, sem nenhum comportamento RBAC verificável e com risco de tenancy derivada de fonte não autorizada.
- **Chosen approach**: Option A — listagem real derivada da sessão verificada (`concept.md:9-12,29`).
- **In scope / out of scope**: In — DTO `RoleResponse` (`id`, `name`, `description`, `permissions`, `is_builtin`), autorização restrita a `GET /roles` com tenancy exclusiva de `auth.require_org()`, membership + `OrgRead|OrgAdmin` (ou `*`) via helper puro `can_list_roles()`, `[]` para org sem papéis; 3 testes puros e correção de D1/D2. Out — RBAC completo em todas as rotas, `PermissionService` no `AppState`, RLS/role change, ADR/constituição/required checks, deploy/tag/release; não afirmar que mais de um endpoint é real neste recorte; não aceitar `X-Organization-ID` como fonte de tenancy (decision.md antigo:5-19).
- **Success metrics**: Três testes puros cobrem allow/deny/omissão de campos; usuário semeado com permissão válida, inclusive Admin, recebe 200 com o payload esperado; org nula resulta no status registrado no contrato; org sem papéis retorna `[]` e a doc deixa de citar `X-Organization-ID`; CI/proteções remotas [NV] (baseline: handlers placeholder, 403 para todos os seed, 422 em nil-org — problem.md:30-36; research.md:37-39).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quem decide semântica de `create/get/{id}/assign/revoke` e proteção de último Owner, diferidas em c041-02]; [NEEDS CLARIFICATION: algum cliente depende de `organization_id` ou timestamps omitidos do DTO]; [NEEDS CLARIFICATION: comportamento da checagem em sessão expirada ou após troca de organização]; [NEEDS CLARIFICATION: `[]` para org sem papéis é contrato estável para todos os clientes?]; [NEEDS CLARIFICATION: adotar teste de isolamento com DB real, dado helper module-private e comportamento live [NV]?]; [NEEDS CLARIFICATION: quais papéis além de `OrgRead`/`OrgAdmin` e `*` a checagem deve aceitar]; [NV: CI/proteções/merge remotos]; ADR-0003 mantém merge como decisão de máquina sob 4 required checks verdes (decision.md antigo:21-23).
