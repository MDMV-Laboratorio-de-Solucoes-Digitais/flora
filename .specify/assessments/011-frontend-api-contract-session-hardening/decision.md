# Decision: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

- **Slug**: 011-frontend-api-contract-session-hardening
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/011-frontend-api-contract-session-hardening/ | validations/releases 010

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `ApiClient` descarta status/corpo estruturado, `POST /tasks` ignora `description`/`assignee_id`, `/tasks` sem guard e `setOrganization` não limpa `isInvalidated` — contrato frágil no slice núcleo (`problem.md:3-4`, `research.md:10-13`). |
| Evidence strength | adequate | Base `dev@556b78cc8` (pós-010, PR #213 mergeado, CI verde) com smoke rc=0 e checks verdes; achados de auditoria de código/CI e pesquisa multilíngue, com CI remoto/porta 5432 NV (`intake.md:4,8`, `research.md:4-5,9-26`). |
| Value vs. inaction | strong | Correções contratuais de alto valor e baixo risco consolidam auth→org→workspace→task; sem elas, atribuição/listagem e tratamento de 400/403/422/409 permanecem imprevisíveis (`problem.md:3`; decisão original:7-8). |
| Feasibility / appetite | adequate | Recorte 80/20: contrato/API, estado/guards/testes frontend e ergonomia do smoke; hardening global de sessão fica follow-up por risco/perf (`concept.md:3-8`; decisão original:9,12-16). |
| Strategic fit | strong | Continuidade direta de 006-010 no Seed seguro/verificável para VPS barata, mantendo escopo mínimo e sem expansão de produto (`intake.md:7-9`, `research.md:28`). |
| Risk posture | adequate | Revalidação DB global no `AuthExtractor` **não** adotada neste ciclo: sessão pré-seleção `org=nil`, consistência de extractors, perf e verificação DB extensiva — registrada como follow-up fundamentado (`research.md:19`; decisão original:9,19). |

## Verdict & Rationale

Veredito histórico **`go` preservado como `go`**: os artefatos não contradizem a decisão de 2026-11-15. O scorecard sustenta o recorte de alto valor e baixo risco sobre o slice já evidenciado no ciclo 010 (`dev@556b78cc8`, PR #213 mergeado, CI verde, smoke `rc=0` em PG18 com NV da porta padrão) (`intake.md:4,8`). Os fatos materiais preservados: em `apps/frontend/src/lib/api/client.ts` um `401` vira `Error('Unauthorized')` após lock e outros erros viram string, dificultando 400/403/422/409 e o envelope `{error,message,field?}`; `POST /api/v1/tasks` aceita `description`/`assignee_id` mas usa apenas `Task::new(..., title)`; `/tasks` não tem guard auth/org; `AuthState.setOrganization` não limpa `isInvalidated`; `AuthState.test.ts`/`api-interceptor.test.ts` estão desalinhados; `AuthExtractor` valida JWT/claims sem lookup de sessão ativa/revogada por `jti` no DB, enquanto `select-organization`/`refresh` fazem checks parciais (`research.md:9-22`; `problem.md:3-7`). A decisão prioriza corrigir esses pontos e a ergonomia do smoke (override de portas), mas **não** introduz revalidação DB global no extrator neste ciclo, mantendo-a como follow-up priorizado com base em auditoria/pesquisa (`concept.md:4-8`; decisão original:7-9,18-20). Sem `git add -A`, sem auto-merge cego e com claims remotos/ambientais não observados marcados **NV** (`intake.md:20-22`; decisão original:22-23).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Contrato frontend↔API frágil no slice auth→org→workspace→task, estado/guards e testes determinísticos insuficientes, autoridade de sessão parcialmente inconsistente e smoke documentado dependente de portas livres (`problem.md:3-7`).
- **Chosen approach**: Correções contratuais e determinísticas de baixo risco (`concept.md:4-8,32`), com hardening de sessão como follow-up; sem revalidação DB global no `AuthExtractor` neste ciclo (`research.md:19`; decisão original:9).
- **In scope / out of scope**:
  - **In (decisão original:11-16)**: `POST /api/v1/tasks` aplica `description`/`assignee_id` mantendo semântica org/membership; `ApiClient` preserva status e corpo (JSON/texto) com erro tipado e mantém `lockSession()` no 401; `AuthState.setOrganization` limpa `isInvalidated`; guards auth/org em `/tasks` e ajustes de uso de contrato/erros; alinhar `AuthState`/interceptor tests; `docs/dev-smoke.md` com exemplo de override de portas; fmt/check/tests Rust, frontend check/lint/test:unit(server)/build, smoke compatível se ambiente permitir e artefatos SDD (spec/plan/tasks/checklists/validate/release).
  - **Out/follow-up (decisão original:18-20)**: revalidação DB global de sessão no extrator/middleware, expansão ampla de testes DB/CI, OIDC/JWKS e2e real, realtime/files/search/notificações wiring e mudanças de migração aplicada.
- **Success metrics**: criação de task com `description`/`assignee_id` persistidos e listagem assignee-scoped coerente (baseline: campos ignorados); erro tipado com status/corpo e 401 com lock preservado (baseline: descartados); `setOrganization` sem `isInvalidated` residual (baseline: não limpava); guard em `/tasks` e testes frontend alinhados (baseline: ausentes/desalinhados); gates Rust/frontend e smoke compatível com evidência, com NV explícito (`problem.md:3-7`; `research.md:9-16`; decisão original:15-16).
- **Carried-forward open questions**: follow-up de hardening — extrator único com verificação `jti` ativa/não expirada atrelada a user/org no Postgres, tratando a sessão pré-seleção `org=nil` e exigindo rollout consistente (`research.md:19,25-26`; decisão original:18-19). NV: porta `5432` ocupada externamente, CI remoto/merge/sync verificados apenas no PR e volume PG16→18 em ambientes não descartáveis (`intake.md:22`; decisão original:22-23).
