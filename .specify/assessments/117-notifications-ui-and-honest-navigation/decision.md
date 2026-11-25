# Decision: Central de notificações escopada por organização e navegação honesta

- **Slug**: 117-notifications-ui-and-honest-navigation
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (original 2026-11-23) | specs/117-notifications-ui-and-honest-navigation/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `intake.md:9-21` e `research.md:7-14` documentam rotas sem filtro de `organization_id`, DTO sem `target_url`, `mark_as_read` com `403` cross-org e atalhos enganosos em `+layout.svelte:136-143,241-299`. |
| Evidence strength | adequate | file:line determinísticos em `flora-api/src/routes/notifications.rs:20-108`, schema/migração de dedup e 5 contratos existentes; execução E2E em navegador real e staging externo permanece `[NV]` (`spec.md:[NV]`). |
| Value vs. inaction | strong | A inação preserva oráculo de existência cross-tenant e UX quebrada (`/dms` com `channelId` inexistente, gaveta de IA sem backend); o escopo fecha as lacunas #1, #2 e #4 (`intake.md:3`). |
| Feasibility / appetite | adequate | `shape.md:5-14` preserva os métodos legados e adiciona variantes `*_and_organization`, com TDD `RED→GREEN` no frontend e lockstep de piso; escopo médio, não trivial. |
| Strategic fit | strong | Alinhado às issues R6 `#391`..`#395` e ao backlog priorizado em `STATE.md` (`intake.md:3`, decisão original), reforçando o Seed multi-tenant. |
| Risk posture | adequate | `404` fail-closed antes da checagem de `user_id`, `resolveNotificationHref` contra URLs externas/`javascript:` e limpeza do store na troca de org mitigam o risco médio aceito (`shape.md:5-14`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**: nada nos artefatos contradiz a decisão de 2026-11-23. O scorecard sustenta o `go` pelo P0 de isolamento multi-tenant e pela evidência técnica determinística; o score original **240 / 625** (`impacto=4 × confiança=5 × (6−esforço=3)=3 × (6−risco=2)=4`) permanece registrado. As restrições originais seguem válidas e são transportadas ao handoff: Briefing de Compreensão (`tenant`, `.specify/templates/briefing-compreensao.md`, ciclo 109) comprovando isolamento cross-organization e proteção contra enumeração; lockstep obrigatório de `FLOORS["notifications"] = 6` em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml` (total `80`); reuso das issues `#391`..`#395` sem duplicação e sem `dark-factory:eligible`; sem `.unwrap()`/`.expect()` em produção; SemVer `MINOR` (`0.9.0 → 0.10.0`).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma incógnita bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `/api/v1/notifications` não filtra pela organização da sessão, omite `target_url` e não oferece marcação em lote, enquanto a UI exibe um `/dms` quebrado e um painel de IA sem backend (`intake.md:9-21`).
- **Chosen approach**: Tenant boundary fail-closed no backend + normalização segura de `target_url` no frontend + badge reativo na sidebar e navegação honesta (`shape.md:5-14`); manter os métodos antigos do repositório intactos para callers legados.
- **In scope / out of scope**: **In** — `GET /api/v1/notifications`, `POST /{id}/read` e `POST /read-all` escopados por `ensure_member`, `404` cross-org, `target_url` opcional, contrato DB (`5 → 6`), tipos/mapper/utilitários/`NotificationStore` em TDD, rota `/notifications`, badge, remoção do atalho `/dms` e do drawer de IA, `/dms` informativo, E2E Passos 6/7, WCAG 2.2 AA e Briefing `tenant`. **Out** — backend real de DMs, assistente de IA/LLM, WebSocket push e alterações de schema SQL (`intake.md:23-24`).
- **Success metrics**: SC-001..SC-005 em `specs/117-notifications-ui-and-honest-navigation/spec.md` — isolamento Org A/Org B com `404`/`403`, `target_url`, `read-all` escopado, piso `notifications: 6` (total `80`), mapeamento dos 9 eventos para `pt-BR`, zero Links fantasma e zero violações WCAG 2.2 AA nos Passos 6/7.
- **Carried-forward open questions**: `[NV]` execução local direta contra Postgres de staging real, tema escuro em navegador real e staging/produção externos (`spec.md:[NV]`); `[NEEDS CLARIFICATION: qual evidência substitui a contagem de não lidas de Alice, que começa com 0 notificações em dev-seed.sql:315, no cenário E2E do badge?]`; `[NEEDS CLARIFICATION: o cartão honesto de /dms deve ser o único destino da URL direta ou também haverá redirecionamento para /channels?]` (`shape.md:14`).
