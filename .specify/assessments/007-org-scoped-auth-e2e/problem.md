# Problem Definition: Sessão pós-login sem escopo de organização

- **Slug**: 007-org-scoped-auth-e2e
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md

## Problem Statement

Após o ciclo 006, as rotas críticas passaram a exigir `AuthExtractor::require_org()` (organização não-nil) e membership validada, e o websocket passou a aceitar apenas Bearer JWT com org concreta. O callback OIDC, porém, continuava criando sessão e JWT com `organization_id = nil` e sinalizando `select_or_create_organization`, sem que existisse um mecanismo para esclarecer essa seleção. O repositório de sessão/serviço não tinha operação de atualização de `organization_id`/`jti`; o refresh reemitia token com o mesmo `jti` e o mesmo `session.organization_id`, então não havia como promover a sessão para uma organização escolhida. Na prática, o login autenticava, mas não produzia um contexto utilizável nas rotas tenant-scoped — o pré-requisito de todo o fluxo núcleo do Seed.

Agrava-se o quadro com a ausência de um caminho mínimo reproduzível para `org → workspace → task`: `test-flora.sh` ainda usava `X-User-ID`/`X-Organization-ID` e o path `/auth/login`, incompatíveis com o modelo Bearer/JWT; o frontend não round-tripava `state`, não chamava seleção de org, não trocava token rotacionado e tinha payloads/estado divergentes. Auditorias de auth/session e de frontend/contrato sugeriram que um slice verificável em nível de API seria o mais próximo do estado real, e research.md marca a execução e2e integrada com OIDC/DB/seed como NV neste ambiente. A incógnita é como obter, após o login, uma sessão com organização válida e verificável — e como provar isso de forma reproduzível — não qual interface de usuário implementar.

## Affected Users & Stakeholders

- **Users**: usuário recém-autenticado que precisa escolher uma organização e passar a operar dentro dela; administradores de organização, que precisam que workspaces e tarefas fiquem corretamente escopados; desenvolvedores que executam o smoke de API do Seed.
- **Stakeholders**: operador do repositório e mantenedores de API, responsáveis pela segurança do fluxo de sessão; mantenedores de frontend, que dependem de um contrato estável; responsáveis por CI, que precisam de um caminho verificável sem depender de OIDC real; usuários PME, que só percebem valor depois do login.

## Goals

- Caracterizar o token com `organization_id = nil` como bloqueio funcional das rotas tenant-scoped endurecidas no ciclo anterior.
- Explicitar a ausência de atualização/rotação de sessão (`organization_id`/`jti`) como limitação estrutural do modelo atual.
- Delimitar a necessidade de verificação de membership como requisito de qualquer promoção de contexto de organização.
- Reconhecer a quebra do smoke header-based como parte do mesmo problema de verificabilidade do fluxo pós-login.
- Distinguir o que pode ser provado em nível de API do que depende de alinhamento de UI/WS e permanece deferido ou NV.

## Non-Goals

- Não desenhar neste artefato o endpoint de seleção, o formato do contrato ou a política exata de rotação de sessão/token.
- Não especificar a UX de seleção de organização, persistência de contexto no frontend ou wiring de websocket/sync.
- Não declarar smoke e2e integrado com OIDC real, Postgres e seed como executado sem evidência (NV).
- Não propor auditoria de ponta a ponta de OIDC/JWKS ou expansão de RBAC fino como parte deste problema imediato.

## Success Metrics

- Logins que resultam em uma sessão utilizável em rotas tenant-scoped (baseline: 0 — callback emite `organization_id = nil`).
- Sessões com promoção de escopo de organização e `jti` rotacionado (baseline: 0 — sem update de `organization_id/jti` no repositório/serviço).
- Smoke de API executável no modelo Bearer/JWT após o login (baseline: não — `test-flora.sh` header-based e path incompatível).
- Caminho `org → workspace → task` verificado com evidência (baseline: NV — integração com OIDC/DB não executada neste ambiente).
- Gates determinísticos locais (baseline: fmt/check ok, 8 e 54 testes ok em decision.md; runtime NV).

## Cost of Inaction

Enquanto o callback emitir um token sem organização, todas as rotas protegidas pelo ciclo 006 permanecem inalcançáveis em uso real: o Seed autentica, mas não funciona. Adiar a verificação de membership na promoção de contexto abriria uma brecha de acesso a organizações alheias, e manter o smoke header-based mascara esse bloqueio com um caminho que já não reflete o runtime. Sem evidência reproduzível, cada ajuste de frontend ou API é feito às cegas, aumentando retrabalho e o risco de uma sessão com escopo inconsistente em produção.

## Open Questions

- [NEEDS CLARIFICATION: a rotação deve reutilizar a mesma sessão com `jti` alterado ou revogar a atual e criar nova sessão, e qual é a política de refresh resultante?]
- [NEEDS CLARIFICATION: qual é o contrato esperado do ponto de seleção de organização (payload, códigos de erro, forma de resposta) e quem o consome?]
- [NEEDS CLARIFICATION: existe ambiente com OIDC/DB/seed disponível para validar o fluxo, ou o smoke deve permanecer documentado como NV?]
