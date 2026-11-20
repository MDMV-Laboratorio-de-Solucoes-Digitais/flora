# Decision: First Seed Journey local one-time bootstrap

- **Slug**: 085-local-first-seed
- **Decided**: 2026-11-20T01:14:00Z
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Logs e UI confirmam que a tela inicial não provisiona nada sem IdP; usuário reportou e escolheu opção explícita. |
| Evidence strength | adequate | Código, logs locais e docs internos citados; demanda quantitativa ausente, marcada como risco. |
| Value vs. inaction | strong | Sem a feature, a entrada depende de injeção manual de sessão; com ela, `/tasks` é alcançável pela UI. |
| Feasibility / appetite | adequate | Repositórios, modelos, JWT e AuthState já existem; contrato + guarda atômica são medium, não large. |
| Strategic fit | adequate | Seed é Postgres-first e o IdP é opcional para dev; porém a constituição cita identidade OIDC, registrada como tensão. |
| Risk posture | adequate | Superfície não autenticada é real, mas limitada por guarda one-time, 409 e testes de concorrência; sem mitigação ficaria weak. |

## Verdict & Rationale

**go** para a Opção A. O problema é observável, a evidência técnica é suficiente e a opção recomendada usa apenas componentes já presentes. O único risco não mitigado em conceito — ausência de verificação de e-mail — é aceito porque o cenário é explicitamente "instalação nova sem IdP"; a especificação deve tornar isso explícito e exigir guarda atômica e testes de rejeição.

## If needs-clarification

- **Blocking questions**: nenhuma; as quatro perguntas abertas são resolvidas na especificação/clarificação.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: instalação nova sem IdP não consegue criar a primeira org/admin/workspace nem entrar na app.
- **Chosen approach**: endpoint único de bootstrap, permitido somente com banco vazio, que cria os objetos mínimos e devolve sessão válida; UI usa esse endpoint no "Sprout Workspace".
- **In scope / out of scope**: in — contrato API, guarda atômica, Role/Membership/Workspace, JWT/sessão, UI, testes; out — mudar OIDC, verificar e-mail, perfis pesados, instalações com dados.
- **Success metrics**: entrada em `/tasks` com `organizationId`; 409 em reexecução e em banco não vazio; cobertura automatizada de sucesso e rejeições.
- **Carried-forward open questions**: critério exato de banco vazio; emissão de token; nomes de papel/workspace; descoberta da disponibilidade pelo frontend.
