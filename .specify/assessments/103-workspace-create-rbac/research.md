# Research: 103 — RBAC na criação de workspace

**Data:** 2026-11-21 · **Evidência local determinística + reprodução em staging local zerado** · execução de gates/testes e staging real **[NV]** · branch `feature/103-workspace-create-rbac@14cb336173e01d63d6edb161a7b1ad09f520683b`

## Primária (local)

1. `docs/governance/2026-11-21-roteiro-staging-flora.md:184-188` — item 11 (`RBAC / isolamento`) pede, com o segundo usuário do fixture, uma ação administrativa e fixa o esperado: "ações administrativas bloqueadas (403) ou escondidas; nenhuma mudança de dados de outra organização" (`:187`), além de confirmar que IDs fixos de seed não permitem cross-org (`:188`). A reprodução relatada usa `SomenteLeituraQA` (`permissions=[]`) em `POST /api/v1/workspaces` → 201 ("NegadoQA").
2. `flora-api/src/routes/mod.rs:39` e `flora-api/src/routes/workspace.rs:84-92` — o grupo de workspaces é montado em `/api/v1/workspaces` e `POST /` aponta para `create_workspace`. Não é preciso rota nova: a correção é interna ao handler.
3. `flora-api/src/routes/workspace.rs:120-155` — `create_workspace` obtém `org_id` da sessão (`:126-129`), verifica membership com `find_by_user_and_organization` (`:133-137`), monta `CreateWorkspaceInput` (`:139-142`), chama `workspace_service.create_workspace` (`:144-147`) e devolve `201` com `WorkspaceResponse` (`:150-153`). Não há `PgRoleRepository`, `role_id` ou `has_permission` em nenhum ponto do handler: qualquer membro do org cria workspace.
4. `flora-api/src/routes/workspace.rs:208-225` — `update_workspace` é o precedente exato: após o guard de tenant do workspace (`:200-206`) e a membership (`:209-213`), cria `PgRoleRepository` (`:216`), chama `find_by_id_and_organization(membership.role_id, org_id)` com `Error::RoleNotFound` quando ausente (`:217-220`) e exige `WorkspaceWrite || WorkspaceAdmin` (`:221-223`), negando com `Error::Forbidden("Insufficient permissions")` (`:224`).
5. `flora-api/src/routes/workspace.rs:256-271` — `delete_workspace` repete o mesmo par de consulta de papel/erro (`:264-268`), variando apenas a exigência de `WorkspaceAdmin` (`:269-271`). A criação deve usar o par de permissões do update, não o do delete: criar é equivalente a escrever workspace, não a administrar exclusivamente.
6. `flora-api/src/routes/workspace.rs:17-18` — `PgRoleRepository` e `RoleRepository` já são importados no módulo; a correção não adiciona dependência, import novo nem abstração.
7. `flora-core/src/repositories/role_repository.rs:40-56` — `find_by_id_and_organization` filtra `WHERE id = $1 AND organization_id = $2`, portanto um papel de outro org não satisfaz a consulta (guard N3, ciclo 064). Reutilizar essa função impede cross-org na verificação de criação.
8. `flora-core/src/models/role.rs:66-75` — `has_permission` aceita `*` ou a forma canônica/legada da permissão; `:80-110` mapeia os aliases `PascalCase` usados pelo `dev-seed.sql` (`WorkspaceWrite`, `WorkspaceAdmin`), e `:126-131,189-190` define as variantes `Permission::WorkspaceWrite` / `WorkspaceAdmin` (`workspace:write`, `workspace:admin`). O cenário com `permissions=[]` retorna `false` para ambas — exatamente o caso "NegadoQA".
9. `flora-core/src/services/workspace_service.rs:29-39` — `create_workspace` apenas valida o `CreateWorkspaceInput` e grava via repositório; não há checagem de permissão na camada de domínio. A autorização é responsabilidade da rota, como em update/delete; não se altera o serviço.
10. `flora-core/src/error.rs:291-294` — `Error::Forbidden(_)` produz HTTP 403 com corpo fixo `{"error":"forbidden","message":"Insufficient permissions"}`, independentemente do texto interno. Usar a mesma mensagem de update/delete mantém o contrato de erro e não revela papel, usuário, org ou workspace. `RoleNotFound` vira 404 `not_found` (`:315-319`), comportamento existente a preservar.
11. `flora-tests/src/contract/workspace.rs:54-103` — `seed_member` cria papel com `Role::new` (que nasce com `permissions=[]`, `role.rs:35-47`) — ou seja, o fixture atual já representa `SomenteLeituraQA`, mas a suíte só exercita GET (`:134-216`). `flora-tests/src/contract/rbac.rs:55-81` mostra o padrão a seguir para semear papéis com permissões (`seed_member(..., &[Permission])` + `add_permission`).
12. `apps/frontend/src/lib/components/workspace/WorkspaceCreate.svelte:12-26` — a UI já chama `POST /api/v1/workspaces`; um 403 aparecerá no tratamento de erro existente do `ApiClient`. Nenhuma mudança de UI é necessária para fechar o bug de autorização; esconder o botão por papel exigiria superfície nova e está fora deste ciclo (o roteiro aceita "403 **ou** escondidas").

## Leitura da reprodução

- Item 11 (`roteiro:186-187`): a tentativa de ação administrativa com o usuário de QA deveria ser bloqueada com 403, mas `POST /api/v1/workspaces` devolve 201 e grava um workspace.
- `SomenteLeituraQA` com `permissions=[]`: `has_permission(WorkspaceWrite)` e `has_permission(WorkspaceAdmin)` são ambas `false` (`role.rs:66-75`); hoje essa informação nunca é consultada na criação.
- Nenhuma falha intermitente é necessária para explicar o fato: o handler não lê o papel, ponto. O isolamento entre organizações e o guard de membership continuam vigentes e não são postos em causa.
- O contrato de erro já é fail-closed e genérico (`error.rs:291-294`); basta chegar a ele.

## Diagnóstico

Cadeia fechada: membro do org com papel sem permissões (`workspace.rs:133-137`) → nenhuma consulta de papel no handler (`:120-155`) → serviço grava o workspace (`:144-147`) → resposta 201 (`:150-153`). `update_workspace` (`:208-225`) e `delete_workspace` (`:256-271`) já demonstram que a capacidade de RBAC existe e que o padrão é carregar o papel escopado ao org e exigir as permissões antes da escrita. A correção é inserir exatamente esse trecho em `create_workspace`, com as duas permissões do update, mantendo o erro 403 genérico e o restante do handler intacto.

## [NV]

- Execução do item 11 em staging real zerado após a correção, com `SomenteLeituraQA`/`NegadoQA`, e o estado do DB/staging de produção: **[NV]** — a reprodução relatada é a entrada; nenhuma requisição ou escrita ao vivo foi executada neste artefato.
- Resultado de gates, build, testes e formatters: **[NV]** neste artefato — serão consolidados no handoff; este documento apenas inspecionou código.
- Visibilidade/ocultação do botão de criação na UI por papel e RLS efetivo do PostgreSQL: **[NV]** — o roteiro aceita bloqueio por 403; a verificação de aplicação (`find_by_id_and_organization`) é mantida independentemente do RLS.
- Cobertura idiomática: pt-BR nos artefatos; inglês em nomes de código, rotas e mensagens de erro existentes, seguindo a convenção do repositório.
