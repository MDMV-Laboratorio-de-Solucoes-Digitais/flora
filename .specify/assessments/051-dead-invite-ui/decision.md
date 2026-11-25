# Decision: Fim da affordance de convite morta (051)

- **Slug**: 051-dead-invite-ui
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Formulário faz `POST /api/v1/users/invite` (`UserManager.svelte:18`), mas `routes/mod.rs:20-68` não monta `/users`; `catch` só loga e a UI não mostra erro (research.md:5-14). |
| Evidence strength | strong | `rg 'UserManager|users/invite'` → 3 arquivos; leitura de `api/client.ts:130-131`, `specs/002/spec.md:34` e auditoria prévia com saída "delete the form" (intake.md:9-18). |
| Value vs. inaction | strong | Inação mantém falsa affordance e falha silenciosa em toda sessão autenticada, podendo induzir implementação apressada sem decisão de autoridade (problem.md:Cost of Inaction). |
| Feasibility / appetite | strong | Appetite small: componente isolado, sem estado compartilhado; remoção + marcação append-only do cenário obsoleto (concept.md:Option A). |
| Strategic fit | adequate | Corrige drift do contrato 002 sem criar backend; autoridade (`OrgAdmin` vs `RoleWrite`) permanece decisão futura do dono (research.md:19-20). |
| Risk posture | strong | Escopo de 3 arquivos, PR independente e PR #265 não tocado; intenção futura do dono sobre convites **[NV]** (decision.md anterior:NV). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard sustenta a decisão: a chamada morta e a falha silenciosa são verificadas, e a opção recomendada resolve o problema sem backend nem decisão de autorização. O veredito continua estritamente limitado: remover a affordance e marcar o cenário obsoleto na spec 002, sem e-mail, token, RBAC, constituição, ADRs ou workflows e sem tocar PR #265 (decision.md anterior:Delimitadores). Nenhum endpoint é criado e nenhuma decisão sobre convites futuros é tomada; a intenção do dono segue **[NV]**.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; as perguntas abertas sobre marcação exata e intenção futura são não-bloqueantes.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: a página de configurações exibe um formulário de convite que sempre retorna 404 e não mostra erro, enquanto a spec 002 segue descrevendo o cenário como entregue.
- **Chosen approach**: Option A — remover a affordance e marcar o drift: retirar o formulário e a chamada, mantendo o restante da página, e anotar em `specs/002/spec.md:34` que o cenário não foi implementado nesta baseline, preservando o texto original (concept.md:Recommendation).
- **In scope / out of scope**: in — `UserManager.svelte`, `settings/+page.svelte`, `specs/002/spec.md` + artefatos do ciclo; out — backend, e-mail, token, RBAC/autorização, constituição, ADRs, workflows, PR #265, esconder o erro sem remover a causa e apagar o texto original.
- **Success metrics**: `rg 'users/invite|UserManager' apps/frontend/src` → 0 ocorrências; cenário em `specs/002/spec.md:34` marcado como não implementado com texto preservado; grid de settings com as duas colunas restantes; gates frontend (check/lint/test/build) e Rust verdes; intenção futura do dono **[NV]**.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: qual marcação exata usar em `specs/002/spec.md:34` sem apagar o texto original?]; [NEEDS CLARIFICATION: o dono pretende implementar convites com e-mail/token em ciclo futuro?] **[NV]**; [NEEDS CLARIFICATION: algum outro cliente além da UI chama `/api/v1/users/invite`?].
