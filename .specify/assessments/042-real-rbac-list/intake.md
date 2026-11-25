# Idea Intake: 042 — Real RBAC role listing

- **Slug**: 042-real-rbac-list
- **Created**: 2026-11-25
- **Source**: repositório local — `.specify/assessments/042-real-rbac-list/decision.md` (+ `specs/042-real-rbac-list/spec.md`)
- **Type**: new-capability

## Idea (as captured)

O ciclo 042 registra em `decision.md:5-9` e `specs/042-real-rbac-list/spec.md:6-17` a aprovação do DTO `RoleResponse` (id, name, description, permissions, is_builtin) e a autorização restrita a `GET /roles`: tenancy obtida exclusivamente da sessão verificada via `require_org()`, com associação e permissão `OrgRead | OrgAdmin` (ou `*`), e retorno `[]` para organizações sem papéis. O artefato registra que os demais handlers (create/get/{id}/assign/revoke) seguem como placeholders e que não se deve afirmar que mais de um endpoint é real (`decision.md:16-19`; `spec.md:19-23`).

## Restated

Propõe-se substituir o placeholder de listagem de papéis por um endpoint real e tenant-safe, usando a sessão verificada como única fonte de tenancy. A iniciativa fica limitada a esse endpoint, com operações de escrita diferidas.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: Placeholder `GET /api/v1/roles` registrado em `spec.md:7-9` como primeiro comportamento RBAC concreto do Seed; `decision.md:11-14` registra que assign/revoke/create dependem de decisão de owner e que RLS segue [NV].

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Quem decide a semântica de create/get/{id}/assign/revoke e a proteção last-Owner, diferidas em c041-02?]
- [NEEDS CLARIFICATION: Algum cliente depende de `organization_id` ou timestamps hoje omitidos do DTO `RoleResponse`?]
- [NEEDS CLARIFICATION: Como a checagem de associação se comporta em sessão expirada ou após troca de organização?]
- [NEEDS CLARIFICATION: O retorno `[]` para organizações sem papéis é contrato estável para todos os clientes atuais?]
- [NEEDS CLARIFICATION: Um teste de isolamento com DB real será adotado, dado o helper module-private e o comportamento live [NV]?]
- [NEEDS CLARIFICATION: Além de `OrgRead`/`OrgAdmin` e `*`, quais papéis a checagem de listagem deve aceitar?]
