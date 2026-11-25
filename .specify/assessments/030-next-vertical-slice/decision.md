# Decision: 030 — Next vertical slice (smoke de navegador e integridade documental)

- **Slug**: 030-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | auditoria pós-029 (via intake/research)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Nenhum smoke determinístico em navegador: `.spec.ts` órfãos fora do `testMatch`, e `select_organization` revoga o `jti` injetado (`auth.rs:384,387`), tornando repetições instáveis (`research.md:3`; `problem.md:9`). |
| Evidence strength | adequate | Auditoria pós-029 (clippy/ADR-0002/smoke rc=0 VERIFICados), edit list exata e seletores semânticos mapeados; execução remota em CI/Forgejo NV (`intake.md:8`; `research.md:3-8,45`). |
| Value vs. inaction | strong | Sem a lane de navegador, regressões no happy path passam até a release e a documentação sugere verificações inexistentes (`problem.md:37-39`). |
| Feasibility / appetite | adequate | Opção A (`medium`, incerto quanto à CI remota): bin feature-gated, um smoke e job não-required; sensível a mudanças de UI (`concept.md:9-13`). |
| Strategic fit | strong | Fecha a última lane de verificação do produto sem novos serviços e sem promover checks obrigatórios (`research.md:10-11`). |
| Risk posture | adequate | `retries: 0`/`workers: 1` obrigatórios, sem static tokens, sem acesso a produção e evidência informativa distinguida de gate de merge (decisão anterior:16-19). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O recorte S1 é GO condicional nos artefatos e a Opção A é a que satisfaz smoke determinístico com sessão emitida, artefato rastreável e documentação honesta; a Opção B não entrega evidência reproduzível e a Opção C traria serviço externo proibido (`concept.md:27-29`). O caráter não-required do job de CI e a NV de CI remota estavam explícitos na decisão anterior.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; perguntas abaixo seguem em aberto).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Ausência de verificação determinística em navegador do happy path `select-organization → canais/chat → tarefas`, agravada por prosa de clippy e artefato de evidência incompleto (`problem.md:7-13`).
- **Chosen approach**: S1 / Opção A — "Um smoke de navegador + documentação honesta" (`concept.md:9-11,27-29`; decisão anterior:6).
- **In scope / out of scope**:
  - **In**: bin `flora-mint-session` feature-gated em `flora-core` (`dev-mint`, `required-features`) emitindo storageState do Playwright a partir de `FloraClaims` literal via `Config::load()`; reescrita da config do Playwright (`retries: 0`, `workers: 1`, gates de `url` no webServer, projects setup + chromium), `seed.setup.ts` (seed + mint + guard de drift positivo) e um `seed-flow.e2e.ts`; gitignore do arquivo de auth e script `test:e2e`; job `e2e-browser` não-required no CI (Chromium, retenção curta de artefatos, sem `dev-mint` em outro lugar); corrigir prosa de clippy (README, `rust.yml` comentário/nome do step, OBJETIVO, banners do research 029) e completar artefato de evidência do smoke; PR/CI verde nos checks required, merge/sync e planejar 031 (decisão anterior:8-13).
  - **Out/NV (031)**: tag/release + SBOM, allowlist CORS, load/restore, cargo-vet required e OTLP. Smoke em CI pode ser não-required e registrado como evidência se verde. Não converter em gate de merge, não afirmar cobertura de todos os fluxos, não criar serviços, não reverter trabalho paralelo e não acessar produção; não resolver ADR-0002 reentrada #2 nem Documentos FASE 7 (decisão anterior:15-19; `problem.md:23-28`).
- **Success metrics**: pelo menos um smoke do happy path com `retries: 0`/`workers: 1` verde em execuções repetidas, artefato com revisão fixada/impressão digital do segredo/transcript versionado, prosa e banners sem afirmações obsoletas e nenhum required check regredido (`problem.md:30-35`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: que critério torna o smoke não-required "evidência suficiente" para aprovar o slice?]; [NEEDS CLARIFICATION: quais seletores semânticos congelar?]; [NEEDS CLARIFICATION: artefato versionado no repo ou apenas referenciado externamente?]; [NEEDS CLARIFICATION: quem valida que a correção da prosa de clippy não conflita com edições alheias?]; resultado de CI remoto e Forgejo [NV] (`problem.md:41-45`; `concept.md:45`).
