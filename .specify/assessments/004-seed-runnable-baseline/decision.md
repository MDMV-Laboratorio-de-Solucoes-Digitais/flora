# Decision: 004 — Seed Runnable Baseline

- **Slug**: 004-seed-runnable-baseline
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/004-seed-runnable-baseline/ | docs/research/2026-11-15-flora-seed-80-20-research.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Sem compose de dependências nem `OBJETIVO.md`/`ROADMAP.md` no início, um checkout limpo não conseguia verificar a jornada ponta a ponta; gaps como WS fanout/indexação/arquivos/RBAC/quota/health são secundários (`problem.md:9,11`). |
| Evidence strength | adequate | Verificações locais de 2026-11-15 (`cargo check -p flora-api -p flora-core`, fmt check, `test --workspace --no-run`, 8 `flora-core --lib` + 54 `flora-tests --lib`) e pesquisa em fontes primárias/oficiais; integração com Postgres/Valkey/Meilisearch/OIDC NV (`intake.md:Context`, `research.md:6-17`). |
| Value vs. inaction | strong | Sem ambiente reproduzível e direção consolidada, correções não podem ser validadas e o drift se reproduz; o ciclo destrava onboarding e fatias verticais (`problem.md:40`; decisão original:13). |
| Feasibility / appetite | adequate | Opção B (`medium`): compose dev + config/instruções/checklists e consolidação documental, baixo a médio esforço, reversível, sem segredos (`concept.md:16-21`; decisão original:16). |
| Strategic fit | strong | Constituição Seed/VPS-first, Postgres-first, rigor de verificação e debloat; sem paridade Macro, sem REA/Craft-Anything como dependência core (`research.md:8`; decisão original:19). |
| Risk posture | adequate | Risco principal: validação incompleta sem Docker/serviços/OIDC; mitigado por documentação explícita, artefatos verificáveis e marcação NV (`concept.md:43-46`; decisão original:19). |

## Verdict & Rationale

Veredito histórico **`go (ciclo 004, escopo restrito e verificável)` preservado como `go`**: os artefatos não trazem contradição. O scorecard sustenta o recorte de maior alavancagem — consolidar direção e tornar o Seed local reproduzível, sem feature de usuário final como meta (decisão original:6,22-24). A Opção B venceu a Opção A (smoke manual com NV explícito) e a Opção C (ambiente completo de produção/CI remota, fora do escopo autônomo) (`concept.md:9-28,32`). A decisão original preserva as quatro frentes — `OBJETIVO.md`/`ROADMAP.md`, assessment 004, `specs/004-seed-runnable-baseline/*` e `docs/research/2026-11-15-flora-seed-80-20-research.md` — com os limites de não pedir OIDC real/JWKS completo, segredos reais, deploy ou reuso AGPL-incompatível. Verificações parciais de 2026-11-15 são reutilizáveis como evidência, mas não substituem integração runtime: onde Docker/serviços/OIDC não rodam, o resultado é **NV** (`concept.md:43-46`; decisão original:37-39).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Seed v0.1 com implementação/specs substanciais (001-003, assessment 003), mas sem baseline local reproduzível; sem objetivo/roadmap e sem compose de dependências, apenas `dev-seed.sql` para smoke manual e integração runtime não exercitada (`problem.md:9,11`).
- **Chosen approach**: Opção B — baseline local reproduzível do Seed (`concept.md:16-20,32`), com perfil dev mínimo (infra essencial, sem mock OIDC/Zitadel "por completude"), smoke acordado e direção alinhada ao estado observável; Opção C (ambiente completo de produção/CI remota) fora do escopo autônomo.
- **In scope / out of scope**:
  - **In (decisão original:22-24,26-29)**: consolidar `OBJETIVO.md`/`ROADMAP.md` com estado real, gaps, não-objetivos e próximos passos; registrar pesquisa e assessment 004; criar `specs/004-seed-runnable-baseline/*` e materiais dev de baixo risco (compose dev, instruções, checklists) com validação estática/documentada.
  - **Out**: deploy/infra de produção, proteções remotas, merges/releases, segredos reais, serviços pagos/proprietários, RE não autorizada e reuso AGPL-incompatível; implementações amplas sem evidência razoável (decisão original:31-35).
- **Success metrics**: etapas para Seed local funcional a partir de checkout limpo (baseline: unknown — sem compose/docs no início); serviços externos integrados com evidência (baseline: 0/NV); comandos determinísticos registrados (baseline: parcial — check/fmt e 8+54 testes ok em 2026-11-15, integração/frontend inconclusivos); documentação de direção alinhada ao observável (baseline: documentos inexistentes) (`problem.md:35-38`; `intake.md:Context`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: perfil dev mínimo — infra essencial ou incluir mock OIDC/Zitadel, e critério de aceite?]; [NEEDS CLARIFICATION: Docker/serviços/credenciais OIDC disponíveis para smoke integrado ou resultado NV?]; [NEEDS CLARIFICATION: critérios de smoke suficientes para declarar o Seed verificável?] (`problem.md:46-48`).
