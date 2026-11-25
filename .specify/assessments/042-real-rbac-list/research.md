# Idea Research: 042 — Real RBAC role listing

- **Slug**: 042-real-rbac-list
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: medium

**Cobertura idiomática:** pt-BR_en (prosa em pt-BR; nomes de permissões, DTOs e rotas mantidos em inglês).

## Users & Demand

- O demandante é o maintainer/SE da MDMV; o tipo registrado é `new-capability` e o gatilho é o placeholder `GET /api/v1/roles`, apontado como primeiro comportamento RBAC concreto do Seed. [source: .specify/assessments/042-real-rbac-list/intake.md:6,10,19; specs/042-real-rbac-list/spec.md:6-9 | system] (confidence: high)
- O usuário implícito é membro de uma organização com `OrgRead`/`OrgAdmin` (ou `*`), que precisa listar os papéis da própria org sem confiar em header ou path; a tenancy vem apenas da sessão verificada via `require_org()`. [source: .specify/assessments/042-real-rbac-list/decision.md:5-9; flora-api/src/routes/rbac.rs:225-253 | system] (confidence: high)
- Não há evidência de cliente dependente de `organization_id` ou timestamps no DTO; a omissão foi registrada como decisão de contrato, não como resposta a consumo observado. [source: .specify/assessments/042-real-rbac-list/intake.md:24; specs/042-real-rbac-list/spec.md:15 | system] (confidence: medium)
- A demanda por assign/revoke/create foi adiada como decisão de owner, portanto não pode ser atribuída a este ciclo. [source: .specify/assessments/042-real-rbac-list/decision.md:11-14; specs/042-real-rbac-list/spec.md:19-23 | system] (confidence: high)

## Prior Art

- Antes do ciclo, os handlers de `rbac.rs` eram placeholders (`vec![]`/strings) sem `AuthExtractor`; o `GET /roles` passou a usar repositório real com gate de membership e permissão. [source: docs/research/2026-11-17-042-real-rbac.md:20-27; .specify/validations/042-real-rbac-list/validate.md:16-21 | system] (confidence: high)
- O padrão de autorização foi derivado de `workspace.rs`: `require_org()` → membership → role → `has_permission`, com `find_by_id_and_organization` como checagem de tenancy em profundidade. [source: flora-api/src/routes/workspace.rs:208-235; docs/research/2026-11-17-042-real-rbac.md:23-24 | system] (confidence: high)
- O repositório de papéis já expunha `find_by_organization_id` (ordenado por nome) e org sem roles retornava `[]`, contrato preservado pelo ciclo. [source: flora-core/src/repositories/role_repository.rs:58; docs/research/2026-11-17-042-real-rbac.md:26; .specify/validations/042-real-rbac-list/validate.md:18-21 | system] (confidence: high)
- O modelo de permissões usa formas canônicas (`org:read`, `org:admin`, `*`) e aliases `PascalCase` do seed (`OrganizationRead`, `OrganizationAdmin`) via `legacy_permission_name()`, sem migration; essa foi a correção do defeito D1. [source: flora-core/src/models/role.rs:60-90,185-208; dev-seed.sql:85; .specify/validations/042-real-rbac-list/validate.md:43-49 | system] (confidence: high)
- A Authorization Cheat Sheet mantida no repositório oficial da OWASP recomenda deny-by-default, validar permissão em toda requisição e cobrir a lógica com testes unitários/de integração — padrão consistente com o helper puro adotado. [source: https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Authorization_Cheat_Sheet.md | web] (confidence: high)

## Market & Context

- O ROADMAP registra o ciclo 042 como entregue (tenancy da sessão + gate `OrgRead|OrgAdmin` + DTO sem vazamento, helper puro e 3 testes), e o ciclo 066 como fechamento dos handlers placeholder com autoridade `RoleWrite || OrgAdmin`. [source: ROADMAP.md:60,240; .specify/memory/STATE.md:359-368 | system] (confidence: high)
- O merge é governado por ADR-0003, sob quatro required checks; o gate determinístico verifica `frontend-build`, `rust-build`, `db-tests`, `dependency-review`. [source: .specify/assessments/042-real-rbac-list/decision.md:21-23; scripts/ci/check_gate_integrity.py:28-32 | system] (confidence: high)
- RLS/role least-privilege segue **[NV]**: app conecta como superuser `postgres` e a migration habilita RLS sem policies, com autorização SE pendente. [source: .specify/assessments/042-real-rbac-list/decision.md:14; .env.example:24; flora-core/src/migrations/sql/20240818000001_init_schema.sql:200-214; .specify/memory/STATE.md:297 | system] (confidence: high)
- O protocolo de auditoria do 042 usa snapshot congelado, lição explícita do falso positivo por concorrência do ciclo 041. [source: specs/042-real-rbac-list/spec.md:25-28; .specify/validations/041-rust-dead-crates/validate.md:45-53 | system] (confidence: high)
- CI/proteções/merge remotos não observados nesta sessão: **[NV]**; nenhum resultado de runner foi colado como evidência. [source: specs/042-real-rbac-list/spec.md:30-33; docs/PROGRESSO.md:557 | system] (confidence: high)

## Data & Constraints

- Escopo registrado: `RoleResponse` com `id`, `name`, `description`, `permissions`, `is_builtin`; tenência exclusiva de `auth.require_org()`; membership + `find_by_id` com `can_list_roles()` puro (`OrgRead|OrgAdmin`, `*`); org sem roles retorna `[]`; demais handlers permanecem placeholders no recorte do ciclo. [source: .specify/assessments/042-real-rbac-list/decision.md:5-9,16-19; specs/042-real-rbac-list/spec.md:11-17 | system] (confidence: high)
- Na worktree atual, `can_list_roles()` está em `rbac.rs:136-140`, `list_roles()` em `:225-253` e o DTO em `:109-134`; 3 testes puros cobrem allow/deny e omissão de campos em `:535-547`. [source: flora-api/src/routes/rbac.rs:109-140,225-253,535-547 | system] (confidence: high)
- Gates do validate: fmt, clippy `-D warnings`, deny, `cargo test --workspace` com 3 novos testes rbac e `cargo check --workspace` em target isolado (330 crates, exit 0). [source: .specify/validations/042-real-rbac-list/validate.md:6-14 | system] (confidence: high)
- Auditoria sobre snapshot `ce42216`: D1 (ALTA) e D2 (médio) corrigidos — respectivamente 403 para todos os usuários semeados por incompatibilidade `PascalCase` vs `org:read`, e 422 em vez de 400 para nil-org; reconfirmação independente cobriu o payload exato do Admin do seed (32 strings). [source: .specify/validations/042-real-rbac-list/validate.md:36-57 | system] (confidence: high)
- Follow-ups não bloqueantes registrados: D3 (`memberships.role_id` sem FK e `find_by_id` sem filtro de org), D4 (teste de contrato DB A/B), D5 (docs/`llms.txt` ainda citam `X-Organization-ID` obsoleto), além de drift residual de permissões do seed sem equivalente no enum. [source: .specify/validations/042-real-rbac-list/validate.md:59-69 | system] (confidence: high)
- Teste de isolamento DB-gated não foi adicionado (helper module-private; nenhum piso o exige) e o comportamento live do banco permanece **[NV]**. [source: specs/042-real-rbac-list/spec.md:30-32; .specify/validations/042-real-rbac-list/validate.md:63-69 | system] (confidence: high)

## Evidence Against the Idea

- A afirmação "apenas `GET /roles` é real" é específica do ciclo 042: no checkout atual o módulo registra que os demais handlers são reais desde o ciclo 066 e o router registra create/get/update/delete/assign/revoke. [source: flora-api/src/routes/rbac.rs:5-6,29-40; ROADMAP.md:240; .specify/memory/STATE.md:365-368 | system] (confidence: high)
- O defeito D1 mostrou que a implementação inicial, mesmo com os 3 testes verdes, dava 403 para todos os usuários do seed (inclusive Admin) — testes originais tinham ponto cego no payload real. [source: .specify/validations/042-real-rbac-list/validate.md:43-49 | system] (confidence: high)
- O defeito D2 mostrou divergência de status HTTP (422 vs 400) em nil-org em relação ao contrato registrado. [source: .specify/validations/042-real-rbac-list/validate.md:51-57; specs/042-real-rbac-list/spec.md:12-14 | system] (confidence: high)
- O gate de listagem aceita curinga `*` além de `OrgRead`/`OrgAdmin` (via `has_permission`), o que é mais amplo que a descrição "OrgRead | OrgAdmin" usada em parte dos artefatos; não há evidência de que todos os clientes esperem esse alargamento. [source: flora-api/src/routes/rbac.rs:136-140; flora-core/src/models/role.rs:66-74; .specify/validations/042-real-rbac-list/validate.md:25 | system] (confidence: high)
- Sem RLS e com superuser, a segurança de tenancy do endpoint depende só do código da aplicação; D3 registra ausência de FK em `memberships.role_id` e `find_by_id` sem filtro de org como defesa em profundidade incompleta. [source: .env.example:24; flora-core/src/migrations/sql/20240818000001_init_schema.sql:200-214; .specify/validations/042-real-rbac-list/validate.md:59-61 | system] (confidence: high)
- Não há teste DB-gated A/B de isolamento; a afirmação de tenant-safety live não foi observada nesta sessão. [source: specs/042-real-rbac-list/spec.md:30-32; .specify/assessments/042-real-rbac-list/decision.md:21-23 | system] (confidence: medium)
- A documentação residual cita header `X-Organization-ID` obsoleto, o que pode induzir clientes a enviar tenancy por fonte não autorizada. [source: .specify/validations/042-real-rbac-list/validate.md:59-61 | system] (confidence: medium)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Quem decide a semântica de create/get/{id}/assign/revoke e a proteção last-Owner, diferidas em c041-02?] [source: .specify/assessments/042-real-rbac-list/intake.md:23; .specify/assessments/041-rust-dead-crates/decision.md:11-14 | system] (confidence: high)
- [NEEDS CLARIFICATION: Algum cliente depende de `organization_id` ou timestamps hoje omitidos do DTO `RoleResponse`?] [source: .specify/assessments/042-real-rbac-list/intake.md:24 | system] (confidence: high)
- [NEEDS CLARIFICATION: Como a checagem de associação se comporta em sessão expirada ou após troca de organização?] [source: .specify/assessments/042-real-rbac-list/intake.md:25; flora-api/src/routes/rbac.rs:232-241 | system] (confidence: high)
- [NEEDS CLARIFICATION: O retorno `[]` para organizações sem papéis é contrato estável para todos os clientes atuais?] [source: .specify/assessments/042-real-rbac-list/intake.md:26; specs/042-real-rbac-list/spec.md:9 | system] (confidence: high)
- [NEEDS CLARIFICATION: Um teste de isolamento com DB real será adotado, dado o helper module-private e o comportamento live [NV]?] [source: .specify/assessments/042-real-rbac-list/intake.md:27; specs/042-real-rbac-list/spec.md:30-32 | system] (confidence: high)
- [NEEDS CLARIFICATION: Além de `OrgRead`/`OrgAdmin` e `*`, quais papéis a checagem de listagem deve aceitar?] [source: .specify/assessments/042-real-rbac-list/intake.md:28; flora-api/src/routes/rbac.rs:136-140 | system] (confidence: high)

## Sources

- `system` — `.specify/assessments/042-real-rbac-list/intake.md:6-28` e `decision.md:5-23` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `specs/042-real-rbac-list/spec.md:6-33` e `tasks.md:3-9` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `.specify/validations/042-real-rbac-list/validate.md:6-69` — lido diretamente 2026-11-25; policy: source local primária.
- `system` — `docs/research/2026-11-17-042-real-rbac.md:20-34`, `docs/research/index.md:160`, `ROADMAP.md:60,240` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `flora-api/src/routes/rbac.rs:5-6,29-40,109-140,225-253,535-547`, `flora-api/src/routes/workspace.rs:208-235`, `flora-core/src/models/role.rs:60-90,185-208`, `flora-core/src/repositories/role_repository.rs:58`, `dev-seed.sql:85` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `.env.example:24`, `flora-core/src/migrations/sql/20240818000001_init_schema.sql:200-214`, `.specify/memory/STATE.md:292-297,359-368`, `scripts/ci/check_gate_integrity.py:28-32`, `docs/PROGRESSO.md:557` — lidos diretamente 2026-11-25; policy: source local primária.
- `web` — `https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Authorization_Cheat_Sheet.md` — host `github.com`, policy `allowlisted`, acesso 2026-11-25 (o leitor serviu o espelho `raw.githubusercontent.com` do mesmo arquivo; nenhum link da página foi seguido).
- `auto-refused` — `https://owasp.org/www-project-cheat-sheets/`, `https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-162.pdf` — hosts não allowlisted; **[UNVERIFIED — fetch skipped: host not allowlisted]**; referência equivalente obtida no repositório GitHub allowlisted.
- `auto-refused` — `https://docs.rs/axum` e `https://github.blog/` — hosts não allowlisted / fora do escopo; **[UNVERIFIED — fetch skipped: host not allowlisted]**; nenhum dado tomado desses hosts.
