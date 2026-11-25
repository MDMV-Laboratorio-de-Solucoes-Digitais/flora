# Problem Definition: Identidade e tenant não verificados no slice central

- **Slug**: 006-secure-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md

## Problem Statement

O P0 registrado em OBJETIVO/ROADMAP e nas auditorias do ciclo 006 era a confiança em cabeçalhos crus (`X-User-ID`, `X-Organization-ID`) em rotas críticas (workspaces, channels, messages, tasks, files, search, notifications). A auditoria de rotas mostrou que `AuthExtractor` (JWT validado, com `sub/org_id/email` e `x-workspace-id`) existia, mas não era usado pelos handlers; o websocket aceitava headers primeiro e só depois caía para JWT; `list_tasks` filtrava por assignee sem filtro de org; `create_channel` não validava `workspace_id ∈ org`; atualização/exclusão de workspace não checavam tenancy do recurso. Identidade e organização, portanto, eram derivadas do cliente sem verificação criptográfica nem checagem consistente de membership — um risco direto de spoofing e vazamento entre tenants.

Havia um segundo bloqueio: o callback OIDC emitia sessão/JWT com `organization_id = nil` e sinalizava `next_step=select_or_create_organization`, mas o frontend ignorava esse estado. Com as rotas tenant-scoped passando a exigir uma organização concreta, o fluxo pós-login ficava inutilizável. Research.md também aponta divergência de contrato frontend↔backend (estado do login não round-tripado, org create sem `slug`, WS em `/ws` vs `/api/v1/ws`, envelopes e campos distintos, ausência de listagem de canais e de `/messages/sync`, tasks UI estática) e marca o E2E de UI como NV. O problema é a ausência de um contexto de autenticação/tenant verificável no servidor e de um fluxo núcleo mínimo coerente entre as pontas — não a existência isolada de uma rota insegura.

## Affected Users & Stakeholders

- **Users**: membros de organizações que usam o Seed (dono da org e usuários comuns), que dependem de isolamento entre tenants e de um login que os leve a um workspace utilizável; administradores de tenant, responsáveis por criar workspaces/canais sem vazar escopo.
- **Stakeholders**: operador do repositório e responsáveis por segurança/governança, que precisam eliminar a confiança em headers; mantenedores de backend e frontend, que precisam de um contrato coerente para o caminho feliz; operadores de VPS, que arcam com o risco de um fluxo de auth inconsistente.

## Goals

- Caracterizar a derivação de identidade/tenant a partir de dados controlados pelo cliente como risco de segurança e de isolamento entre organizações.
- Explicitar a dependência entre o token com `organization_id = nil` do callback e a inutilização das rotas tenant-scoped.
- Delimitar a divergência de contrato frontend↔backend como obstáculo a qualquer jornada verificável ponta a ponta.
- Esclarecer o que significa "slice seguro e verificável" em termos de comportamento observável (identidade validada, membership e tenancy do recurso checados) sem prescrever a implementação.
- Distinguir o que é verificável com comandos determinísticos locais do que permanece NV por depender de runtime integrado.

## Non-Goals

- Não propor neste artefato a refatoração de handlers, assinatura de extratores ou desenho de endpoints.
- Não resolver neste definition a seleção de organização, a rotação de sessão/token ou o wiring completo de UI/WS — são problemas vizinhos tratados em ciclos próprios.
- Não afirmar que o E2E de UI ou a integração com Postgres/Valkey/Meilisearch/OIDC foram executados sem evidência (NV).
- Não eliminar abruptamente clientes header-based sem registrar o risco de quebra e a necessidade de migração compatível.

## Success Metrics

- Rotas críticas que derivam identidade/tenant de cabeçalhos não verificados (baseline: múltiplas — auditoria em research.md).
- Rotas com checagem de membership e tenancy do recurso antes da operação (baseline: parcial; casos explícitos citados em research.md).
- Sessões pós-callback utilizáveis em rotas tenant-scoped (baseline: 0 — callback emite `organization_id = nil`).
- Caminho E2E de UI verificável para o fluxo núcleo (baseline: NV — contrato desalinhado).
- Gates determinísticos locais executados (baseline: fmt/check ok, 8 testes `flora-core --lib` e 54 `flora-tests --lib` ok, conforme decision.md; runtime integrado NV).

## Cost of Inaction

Manter headers spoofáveis como fonte de identidade deixa qualquer tenant capaz de agir como outro usuário ou organização, comprometendo a premissa de isolamento de uma ferramenta colaborativa self-hosted. A inconsistência entre o token pós-login e as rotas protegidas bloqueia o uso real do Seed, enquanto a divergência de contrato faz o frontend e o backend evoluírem em direções opostas. O custo é duplo: risco de segurança em produção e retrabalho crescente na integração das pontas.

## Open Questions

- [NEEDS CLARIFICATION: qual estratégia de migração é aceitável para clientes header-based existentes (`test-flora.sh`, frontend atual) — fallback explícito em dev ou quebra imediata?]
- [NEEDS CLARIFICATION: qual fatia núcleo deve ser priorizada para ser verificável primeiro (`workspace → tarefa` ou `workspace → canal → mensagem`), dado o custo adicional de discovery/WS?]
- [NEEDS CLARIFICATION: que ambiente e serviços estão disponíveis para validar o fluxo com runtime integrado, ou o resultado permanece NV?]
