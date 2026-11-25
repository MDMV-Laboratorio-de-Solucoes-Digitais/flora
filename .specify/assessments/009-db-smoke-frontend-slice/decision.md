# Decision: 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

- **Slug**: 009-db-smoke-frontend-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/009-db-smoke-frontend-slice/ | handoff de specs/008-select-org-tests-and-dev-smoke/ e assessment 008

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Três lacunas do ciclo 008: casos DB de `select-organization` não automatizados, smoke não executado pelo caminho documentado (5432 ocupada; `postgres:16-alpine` sem `uuidv7()`) e contrato frontend divergente (`problem.md:9,11`). |
| Evidence strength | adequate | Auditoria + cinco subagentes e gates SDD; smoke `select org → workspace → task` com **rc=0** em stack efêmera PG18 (15432/16379/17700) e `db-tests` validados localmente; CI remota e toolchain frontend NV (`research.md:4,10-18`; `intake.md:18-20`). |
| Value vs. inaction | strong | Sem cobertura DB, a autoridade da sessão pode regredir sem sinal; sem smoke oficial e contrato coerente, o Seed só demonstra valor em condições artesanais (`problem.md:40`; decisão original:7). |
| Feasibility / appetite | adequate | Opção B (`medium`): job separado com Postgres, smoke em portas alternativas e fatia mínima do frontend; Opção C (reconstruir compose/paridade total) fora do 80/20 (`concept.md:16-28,32`). |
| Strategic fit | strong | Verificabilidade do slice núcleo auth→org→workspace→task no caminho VPS-barata, mantendo tenancy e o fluxo de 006/007/008 (`research.md:6-12`; `intake.md:6`). |
| Risk posture | adequate | Risco controlado: job separado preserva o pipeline atual, testes `#[ignore]`/env-gated e fixtures únicas; não editar migração aplicada e registrar NV de compose/CI/frontend (`research.md:11-12,18`; decisão original:22-25). |

## Verdict & Rationale

Veredito histórico **`go (escopo implementado com NVs explícitos)` preservado como `go`**: os artefatos não contradizem a decisão de 2026-11-15. O scorecard sustenta a Opção B — cobertura DB e fatia mínima do frontend — sobre a Opção A (apenas registrar o smoke em stack equivalente) e a Opção C (reconstruir compose e buscar paridade total, risco de quebrar histórico de migração) (`concept.md:9-28,32`). Fatos preservados: testes `#[ignore]` + env-gated via `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`, com `PgPool::connect` + `run_migrations`, fixtures únicas, `--test-threads=1` e job `db-tests` separado com `services.postgres`/healthcheck mantendo `build` intacto (`research.md:10-12`; decisão original:12-13). O smoke `select org → workspace → task` passou com **rc=0**: select-org rotacionou a sessão (200), workspaces listados, task criada (201 com id/status); lista pós-criação vazia é esperada pela semântica assignee-scoped (`research.md:16-17`). A incompatibilidade observada — `postgres:16-alpine` falha na migração por `uuidv7()` inexistente, `postgres:18` funciona — fica registrada, sem tratar o tema como resolvido no caminho documentado (`research.md:18`; decisão original:22). Subagentes `ses_e1e8f0341ffeS9O37ZKM946HT1`, `ses_e1e8f0341ffdZw46QTHDDb5BhM`, `ses_e1e8f0341ffcqTrfH71HTjgehq`, `ses_e1e8f0340ffe4m9OxyO9VZASaF`, `ses_e1e8f0340ffdtK4QZLcUFWjhpp` permanecem rastreados (`intake.md:23-27`). A entrega (PR para `dev`, CI verde com `db-tests`, merge e sync `origin/dev`) estava pendente; CI remoto, checks completos do frontend e compose documentado permanecem **NV** (`intake.md:19,31-32`; decisão original:22-25).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Fluxo núcleo do ciclo 008 verificável apenas em parte — casos DB sem automação, smoke documentado bloqueado por porta/imagem e contrato frontend desalinhado (`problem.md:9,11`).
- **Chosen approach**: Opção B — cobertura DB e fatia mínima do frontend (`concept.md:16-20,32`), com distinção explícita entre evidência em stack alternativa e caminho documentado; Opção C rejeitada por risco de histórico/paridade.
- **In scope / out of scope**:
  - **In (decisão original:11-16)**: DB-backed tests em `flora-tests/src/contract/auth.rs` (sucesso com rotação/revogação, 403 não-membro, 401 sessão ausente/inativa/mismatch); job `db-tests` no `.github/workflows/rust.yml`; smoke integrado rc=0 em PG18:15432/valkey:16379/meili:17700 com seed + `scripts/dev-smoke-session.sql` + `test-flora.sh`; frontend mínimo (persistência de `state`/`redirect_uri`, callback com validação/envio de `state`, `AuthState` com org/workspace e rotação de token, rota `/select-org`, guard em `(app)`, tasks com seleção de workspace e create/list).
  - **Out/NV (decisão original:20-25)**: compose documentado na porta padrão, compatibilidade PG16↔`uuidv7()`, verificação frontend completa dependente de `bun`, CI remoto do PR, OIDC/JWKS e2e real, session authority no extrator, paridade frontend ampla e realtime/files/search avançados.
- **Success metrics**: casos DB automatizados (baseline: 0); smoke pelo caminho documentado com rc/logs (baseline: NV; rc=0 só em stack alternativa PG18); compatibilidade fresh install (baseline: falha em `postgres:16-alpine` por `uuidv7()`, PG18 funcional); etapas coerentes frontend↔API (baseline: divergentes em `state`, seleção de org, rotação, workspace/task); checks determinísticos frontend (baseline: NV/parcial) (`problem.md:35-39`; `research.md:16-18`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: versão/alvo de Postgres e tratamento de `uuidv7()` em fresh install sem alterar migrações aplicadas?]; [NEEDS CLARIFICATION: como evidenciar `db-tests` no CI remoto do PR, não observável localmente?]; [NEEDS CLARIFICATION: toolchain/`bun` disponível e escopo mínimo aceitável de alinhamento de contrato?] (`problem.md:45-48`).
