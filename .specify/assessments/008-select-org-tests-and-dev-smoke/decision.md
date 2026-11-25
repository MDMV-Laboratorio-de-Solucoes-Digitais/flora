# Decision: 008 — Select-Organization Tests & Reproducible Dev Smoke

- **Slug**: 008-select-org-tests-and-dev-smoke
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/008-select-org-tests-and-dev-smoke/ | handoff de specs/007-org-scoped-auth-e2e/

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Ponto crítico do ciclo 007 sem testes próprios e sem smoke reproduzível sem OIDC; CI (`.github/workflows/rust.yml`) não fornece Postgres, então casos DB não são verificados (`research.md:7-9`; `problem.md:9`). |
| Evidence strength | adequate | Auditoria de código (`flora-api/src/routes/auth.rs:325-389`) e seis subagentes catalogados, com achados convergentes; execução integrada e testes DB em CI marcados NV (`intake.md:16-22`, `research.md:4,23-26`). |
| Value vs. inaction | strong | Testes determinísticos CI-safe + bootstrap documentado elevam verificabilidade do ponto de autenticação/tenancy com baixo risco; sem eles, regressão de membership/revogação passa até incidente (`problem.md:41`; decisão original:8). |
| Feasibility / appetite | adequate | Opção A (`small`): contratos sem DB e smoke bootstrável; sem mudança de runtime e sem infra nova (`concept.md:9-14`; decisão original:11-14). |
| Strategic fit | strong | Continua 006/007 e o Seed seguro/verificável em VPS ~2 vCPU/4 GB, Postgres-first e Valkey opcional; não escopo OIDC/JWKS completo (`research.md:17-18`; `intake.md:13`). |
| Risk posture | adequate | Risco baixo: mudanças em testes/docs/scripts de suporte dev, sem alterar comportamento da API; segredos reais vetados no bootstrap e nuance assignee-scoped documentada (`concept.md:13-14`; decisão original:9). |

## Verdict & Rationale

Veredito histórico **`go (escopo acotado, condicionado por NV de runtime/DB integrado)` preservado como `go`**: os artefatos não trazem contradição. O scorecard sustenta a Opção A — testes CI-safe e smoke dev documentado — sobre a Opção B (cobertura DB-backed na CI, próximo passo natural mas não pré-condição) e a Opção C (alinhamento frontend amplo, explicitamente deferido) (`concept.md:9-28,32`; decisão original:6,11-14). Fatos preservados: `POST /api/v1/auth/select-organization` rejeita `organization_id` nil com `422` via `Error::Validation`, exige `AuthExtractor`, valida sessão por `jti` ativa/mesmo usuário e membership, revoga a sessão atual e emite nova com `jti` rotacionado (`research.md:7`; decisão original:12). O seed traz org `11111111-1111-1111-1111-111111111111`, Alice `22222222-...`, workspaces `88888888-...`/`99999999-...`, memberships e tasks, mas **sem sessões**; o bootstrap usa JWT HS256 assinado com o mesmo `FLORA_JWT_SECRET` (`research.md:12-13`). `GET /api/v1/tasks` é assignee-scoped, então task criada sem assignee pode não aparecer na listagem — comportamento esperado, não falha (`research.md:14`; `concept.md:45`). Subagentes `ses_e1e9a5437ffeDtndGc5NhfHNUB`, `ses_e1e9a5435ffexYwl8z3bRWNTW3`, `ses_e1e9a542bffe1iScHCp3nDbNe7`, `ses_e1e9a5428ffesJ4ikPt7V732Nm`, `ses_e1e9a5426ffeFHRHkBZGVF8s6o` e `ses_e1ea41e08ffeHj1Q40A2fgILJO` permanecem rastreados (`intake.md:17-22`). Onde não há evidência observável — testes DB automatizados, execução integrada do smoke — o resultado é **NV** (`research.md:24-26`; decisão original:16-19).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `select-organization` era o ponto mais sensível do fluxo sem testes dedicados, e o seed não permitia smoke sem OIDC real nem automação DB na CI (`problem.md:9`, `research.md:7-9`).
- **Chosen approach**: Opção A — testes de contrato CI-safe e smoke dev documentado (`concept.md:9-13,32`); cobertura DB-backed na CI fica como próximo 80/20, e alinhamento frontend amplo adiado.
- **In scope / out of scope**:
  - **In (decisão original:11-14)**: testes CI-safe para auth ausente e `organization_id` nil rejeitado (`422`); smoke sem OIDC via docker compose + migrations pela API + `dev-seed.sql` + `scripts/dev-smoke-session.sql` + bootstrap JWT determinístico para `select org → workspace → task`, com nuances documentadas; artefatos SpecKit 008 e NV explícito.
  - **Out/NV (decisão original:16-19)**: testes DB-backed automatizados em CI sem Postgres, execução integrada real do smoke, OIDC/JWKS completo, alinhamento frontend (`apps/frontend`), deploy e serviços pagos.
- **Success metrics**: casos dedicados cobrindo `select-organization` (baseline: 0); cenários executáveis na CI atual sem serviços externos (baseline: só casos sem DB); etapas de smoke local sem OIDC (baseline: não documentadas/bootstráveis); leitura incorreta da listagem assignee-scoped (baseline: nuance observada); gates determinísticos locais (baseline: check/fmt e testes de biblioteca ok, DB integrado NV) (`problem.md:35-39`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: cobertura DB-backed entra na CI com Postgres dedicado ou `#[ignore]`/env-gated com job separado?]; [NEEDS CLARIFICATION: forma aceitável de bootstrap de sessão/JWT sem segredos reais?]; [NEEDS CLARIFICATION: alinhamento mínimo de frontend neste ciclo ou deferido?] (`problem.md:47-49`). Próximo 80/20: job `db-tests` com Postgres, fatia frontend (callback `state`, `select-organization`, troca de token, workspace/task reais) e JWKS/authority por `jti` (`research.md:24-26`; decisão original:21-24).
