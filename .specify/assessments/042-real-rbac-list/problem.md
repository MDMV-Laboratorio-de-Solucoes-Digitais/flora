# Problem Definition: Listagem de papéis RBAC ainda placeholder no Seed

- **Slug**: 042-real-rbac-list
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O endpoint `GET /api/v1/roles` do Seed ainda é um placeholder (`vec![]`/strings) sem extrator de autenticação, de modo que o primeiro comportamento RBAC concreto não existe e os handlers não oferecem uma listagem confiável (intake.md:10,19; research.md:18). O problema central é de tenancy: a lista de papéis precisa derivar da sessão verificada, não de header ou path, e o padrão existente em `workspace.rs` já mostra a cadeia `require_org()` → membership → role → `has_permission` (research.md:12,19). A auditoria sobre snapshot congelado revelou que uma implementação inicial com três testes verdes ainda devolvia 403 para todos os usuários semeados, inclusive Admin, por incompatibilidade entre `PascalCase` e `org:read` (D1), e 422 em vez de 400 para organização nula (D2) (research.md:37,44-45). Além disso, sem RLS e com conexão como superuser, o isolamento depende apenas do código da aplicação, e a documentação residual ainda cita o header obsoleto `X-Organization-ID` (research.md:28,49). O problema é entregar uma listagem efetiva e tenant-safe sem reivindicar que o RBAC inteiro está pronto.

## Affected Users & Stakeholders

- **Users**: membro de uma organização com permissão `OrgRead`/`OrgAdmin` (ou `*`), que precisa listar os papéis da própria org sem confiar em header ou path (research.md:12).
- **Stakeholders**: mantenedores e responsáveis pela segurança — precisam de gate de autorização com testes determinísticos; dono/SE — precisa decidir a semântica dos demais handlers e a proteção de último Owner, adiadas (decision.md, "Deferred").

## Goals

- Fazer a listagem de papéis retornar dados reais da organização com tenancy derivada exclusivamente da sessão verificada.
- Exigir associação à organização e permissão adequada, com regra pura e testável, preservando o contrato de `[]` para organizações sem papéis.
- Evitar vazamento de campos não acordados no DTO e corrigir divergências de status HTTP em relação ao contrato registrado.
- Delimitar explicitamente o que é real neste ciclo, deixando operações de escrita e decisões de owner fora.

## Non-Goals

- Não afirmar RBAC completo em todas as rotas, não incluir `PermissionService` no `AppState`, não alterar RLS/role, ADR, constituição, required checks, deploy, tag ou release (decision.md, "Non-goals / non-claims").
- Não afirmar que mais de um endpoint é real neste recorte; `create`, `get/{id}`, `assign` e `revoke` permanecem sujeitos a decisão de owner (c041-02).
- Não adicionar teste de isolamento com banco real como requirement do ciclo, dado o helper module-private e o comportamento live [NV] (research.md:39).
- Não aceitar `X-Organization-ID` como fonte de tenancy.

## Success Metrics

- Três testes puros cobrem allow, deny e omissão de campos do DTO (baseline: handlers placeholder, nenhum teste de listagem real).
- Usuário semeado com permissão válida, incluindo Admin, recebe 200 com o payload esperado (baseline: 403 para todos os usuários do seed, defeito D1).
- Organização nula resulta no status HTTP registrado no contrato (baseline: 422, defeito D2).
- Organização sem papéis retorna `[]` e a documentação deixa de citar `X-Organization-ID` (baseline: contrato preservado no placeholder; docs com header obsoleto).
- Resultado de CI/proteções remotas não foi observado nesta sessão [NV].

## Cost of Inaction

Enquanto o endpoint for placeholder, o Seed não possui nenhum comportamento RBAC verificável e clientes podem continuar a tratar papéis como lista vazia; sem gate baseado na sessão, a tenancy poderia depender de fonte não autorizada. Documentação com header obsoleto reforça esse risco. O custo é capacidade bloqueada e risco de isolamento; o comportamento live do banco permanece sem observação [NV].

## Open Questions

- [NEEDS CLARIFICATION: quem decide a semântica de `create/get/{id}/assign/revoke` e a proteção de último Owner, diferidas em c041-02?]
- [NEEDS CLARIFICATION: algum cliente depende de `organization_id` ou timestamps hoje omitidos do DTO `RoleResponse`?]
- [NEEDS CLARIFICATION: como a checagem de associação se comporta em sessão expirada ou após troca de organização?]
- [NEEDS CLARIFICATION: o retorno `[]` para organizações sem papéis é contrato estável para todos os clientes atuais?]
- [NEEDS CLARIFICATION: um teste de isolamento com DB real será adotado, dado o helper module-private e o comportamento live [NV]?]
- [NEEDS CLARIFICATION: além de `OrgRead`/`OrgAdmin` e `*`, quais papéis a checagem de listagem deve aceitar?]
