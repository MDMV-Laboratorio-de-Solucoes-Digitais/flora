# Decision: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

- **Slug**: 010-compose-uuidv7-smoke-frontend-verify
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/010-compose-uuidv7-smoke-frontend-verify/ | validations/releases 010

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `docker-compose.yml` com `postgres:16-alpine` e `DEFAULT uuidv7()` na init migration quebrava o fresh install; CI `db-tests` já usava PG18 e o smoke documentado permanecia NV (`problem.md:9-11`, `research.md:10`). |
| Evidence strength | adequate | Causa-raiz localizada em `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, probe de ambiente e sete subagentes + skills; evidência de rc=0 só em stack alternativa do ciclo 009 (`research.md:4,10-19`; `intake.md:32-34`). |
| Value vs. inaction | strong | Sem coerência compose/docs/migração, cada operador repete a falha e o smoke oficial não pode ser defendido; checks frontend sem sinal agravam o drift (`problem.md:45`; decisão original:7-9). |
| Feasibility / appetite | adequate | Opção B (`medium`): alinhar compose/docs a PG18, evidenciar smoke compatível e executar gates; Opção A entrega só coerência, Opção C (polyfill/novo baseline PG16) tem risco `sqlx` maior (`concept.md:9-28,32`). |
| Strategic fit | strong | Caminho dev reproduzível para PME/VPS, sem alterar migração `sqlx` aplicada e sem OIDC/JWKS e2e, realtime/files/search ou paridade de UI (`concept.md:36-39`; `intake.md:15-17`). |
| Risk posture | adequate | Riscos ambientais explícitos (5432 ocupada externamente, volumes PG16, `.env` ausente) registraridos como NV; uso de projeto/volumes isolados e override de portas (`research.md:15,21-22`; decisão original:19-22). |

## Verdict & Rationale

Veredito histórico **`go` preservado como `go`**: os artefatos não contradizem a decisão de 2026-11-15. O scorecard sustenta a Opção B — caminho documentado evidenciado com gates do frontend — sobre a Opção A (alinhar compose/docs e ajustar testes, sem produzir evidência) e a Opção C (polyfill/novo baseline para PG16, que aumentaria a divergência com o CI e arriscaria histórico `sqlx`) (`research.md:11-12`; `concept.md:9-28,32`). Fatos preservados: `uuidv7()` é nativo em PostgreSQL 18+ e ausente no 16; `sqlx::migrate!` embute/valida checksums, então uma nova migration compat não resolve o fresh install sem bootstrap pré-migrate ou mudança de baseline; a decisão 80/20 foi alinhar o caminho documentado a **`postgres:18-alpine` sem editar a init migration**, com reset de volume PG16→18 (`docker compose down -v`) em ambiente descartável (`research.md:10-12`; decisão original:7,12). O probe encontrou `5432` ocupada por processo externo, `.env` ausente, portas `6379`/`7700`/`3000` livres, nenhum container/volume da Flora e imagem `postgres:18-alpine` disponível localmente (`intake.md:32-34`; `research.md:15`). Os subagentes `ses_e1e70c42bffeEeB6ykdz0lHRlg`, `ses_e1e70c429ffelej5ra63x4mjkr`, `ses_e1e70c421ffe8wdiTy72gocnXw`, `ses_e1e70c41effe44U20bkhSQ83U5`, `ses_e1e70c41cffe7ciYAWvFvne7bG`, `ses_e1e6f8d94ffeYNn6mu2GT06pKa`, `ses_e1e6f8d92ffeqp08mld5uEDEZy` e as skills `postgres-safe-migration`/`sqlx-code-review`/`svelte-testing` permanecem rastreados (`research.md:4`). Da entrega esperam-se PR para `dev`, CI verde, merge, sync `origin/dev` e manter apenas `dev`/`main`; onde não há execução observável — porta padrão, volumes PG16, CI remoto, toolchain frontend — o resultado é **NV** (`intake.md:30`; decisão original:19-22).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Fresh install pelo caminho documentado quebrava por `postgres:16-alpine` + `DEFAULT uuidv7()`; o smoke rc=0 existia apenas em stack alternativa e os checks determinísticos do frontend não estavam registrados (`problem.md:9,11,15`).
- **Chosen approach**: Opção B — caminho documentado evidenciado com gates do frontend (`concept.md:16-20,32`), preservando o limite de não editar migrações `sqlx` aplicadas; Opção C rejeitada por risco de checksum/histórico e divergência com o CI.
- **In scope / out of scope**:
  - **In (decisão original:11-14)**: `docker-compose.yml` → `postgres:18-alpine` e `docs/dev-smoke.md` com requisito PG18, nota de reset PG16→18, conflito de portas e correção da nota de chave do Meilisearch; smoke `select org → workspace → task` executado/evidenciado no caminho compatível, com override isolado e NV se 5432 estiver ocupada; inclusão de `tests/**` no projeto Vitest `server` e execução de `bun run check`, `lint`, `test:unit -- --run --project server`, `build`, com correções mínimas in-scope.
  - **Out**: editar migração `sqlx` aplicada, polyfill/extensão `uuidv7` para PG16 no runtime, OIDC/JWKS e2e real, expansão ampla de produto/UI e CI remoto além do PR (decisão original:16-17).
- **Success metrics**: fresh install documentado conclui migração/startup (baseline: falha em PG16); smoke com rc/logs registrados (baseline: NV no caminho documentado; rc=0 só em stack alternativa PG18); coerência de imagem/volume entre compose, docs e CI (baseline: divergente — compose PG16, `db-tests` PG18); cobertura do projeto `server` sobre `tests/**` (baseline: parcial); quatro checks frontend executados (baseline: NV/parcial) (`problem.md:39-43`; `research.md:16-19`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: versão alvo de Postgres e tratamento aceitável de `uuidv7()` sem alterar migrações aplicadas?]; [NEEDS CLARIFICATION: como evidenciar o smoke com 5432 ocupada por processo externo não descartável?]; [NEEDS CLARIFICATION: volumes PG16 e nota de chave do Meilisearch sem afetar bancos não descartáveis?]; [NEEDS CLARIFICATION: toolchain/`bun` disponível e falha mínima considerada em escopo?] (`problem.md:51-54`).
