# Problem Definition: Formulário de convite sem backend

- **Slug**: 051-dead-invite-ui
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

A página de configurações renderiza `UserManager.svelte`, que faz `POST /api/v1/users/invite` (`:18-21`), mas `flora-api/src/routes/mod.rs:20-68` não monta nenhum módulo `/users`; o `catch` do componente apenas chama `logger.error(...)` (`:24-25`) e a UI não mostra erro. Como `api/client.ts:130-131` lança em resposta não-2xx, `successMessage` nunca é exibido na prática: não há falso sucesso, há falha silenciosa e uma falsa affordance. `specs/002-svelte-frontend-rewrite/spec.md:34` ainda lista o cenário de convite como entregue; auditoria prévia já apontava "delete the form" como saída documentada (intake.md:9-18; research.md:5-16). Ranqueamento 80/20 = 10.0; implementar convites exigiria e-mail/token e decisão de autoridade (`OrgAdmin` vs `RoleWrite`), fora de escopo (intake.md:23-27; research.md:18-20).

## Affected Users & Stakeholders

- **Users**: administradores que veem o formulário de convite nas configurações e não recebem sucesso nem erro — a ação nunca pode funcionar neste checkout (intake.md:9-12; research.md:5-8).
- **Stakeholders**: mantenedores do frontend, responsáveis pelo contrato 002, e dono/SE, que decidiriam a autoridade caso convites venham a ser implementados (`research.md:19-20`; `decision.md:9-11`).

## Goals

- Remover o formulário de convite da página de settings e a chamada a `/api/v1/users/invite` do frontend (`define.md:7-8`; `shape.md:5-7`).
- Marcar o cenário obsoleto em `specs/002-.../spec.md:34` como não implementado, preservando o texto original (`define.md:9`; `decision.md:9`).
- Manter `OrganizationManager`, `WorkspaceCreate` e o layout geral inalterados (`define.md:10`).
- Não tomar decisão de autorização: nenhum endpoint é criado (`decision.md:11`).

## Non-Goals

- Não implementar convites, e-mail ou token (`define.md:13`; `shape.md:10`).
- Não alterar backend, autorização/RBAC, constituição, ADRs ou workflows (`define.md:14`; `decision.md:12`).
- Não manter formulário desabilitado nem esconder o erro: ambos preservariam a falha silenciosa (`shape.md:11-12`).
- Não tocar PR #265 (`decision.md:13`).

## Success Metrics

- `rg 'users/invite|UserManager' apps/frontend/src` → 0 ocorrências (baseline: componente + settings; a terceira ocorrência está em `specs/050`, fora do frontend — intake.md:13-15; define.md:18).
- Cenário em `specs/002/spec.md:34` marcado como não implementado com texto original preservado (baseline: listado como entregue — research.md:14).
- Página de settings mantém o grid com as duas colunas restantes (baseline: três colunas com o formulário — shape.md:6).
- Gates frontend (check/lint/test/build) e Rust verdes (baseline: verdes; remoção de componente isolado — define.md:19-21).
- Intenção futura do dono sobre convites: **[NV]** (research.md:22; decision.md:16).

## Cost of Inaction

Usuários continuam preenchendo um formulário que retorna 404 e não exibe erro, enquanto a spec 002 segue descrevendo o cenário como entregue. A falsa affordance consome atenção e pode induzir uma implementação apressada de convites sem a decisão de autoridade necessária (intake.md:11-12; research.md:18-20).

## Open Questions

- [NEEDS CLARIFICATION: qual marcação exata usar em `specs/002/spec.md:34` sem apagar o texto original?] (`define.md:9`; `shape.md:6`)
- [NEEDS CLARIFICATION: o dono pretende implementar convites com e-mail/token em ciclo futuro?] (`research.md:22`; **[NV]**)
- [NEEDS CLARIFICATION: algum outro cliente além da UI chama `/api/v1/users/invite`?] (`intake.md:13-15`)
