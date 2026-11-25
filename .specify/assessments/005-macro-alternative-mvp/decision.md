# Decision: 005 — Macro Alternative MVP Reframe

- **Slug**: 005-macro-alternative-mvp
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/005-macro-alternative-mvp/ | docs/research/2026-11-15-macro-alternative-80-20-research.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Sem recorte 80/20 explícito, a Flora tende a replicar a complexidade do Macro (monorepo AGPL-3.0, muitos crates Rust e TS/JS pesado) incompatível com VPS barata e com a constituição (`problem.md:9`, `research.md:14-16`). |
| Evidence strength | adequate | Constituição, `OBJETIVO.md`/`ROADMAP.md`, specs 001-004, verificações prévias e evidência pública inicial do Macro; priorizações e hardware são hipóteses, pesquisa de subagentes em andamento marcada NV (`research.md:4,17,58`; decisão original:10). |
| Value vs. inaction | strong | Um norte recortado reduz desperdício e orienta contribuições antes de investir em email completo, calls, CRM/databases ou agentes avançados (`problem.md:40`; decisão original:13). |
| Feasibility / appetite | adequate | Opção B (`medium`): reframe + MVP slice + docs/roadmap + spec do próximo incremento; só artefatos de baixo risco, sem infra/segredos/deploy (`concept.md:16-21`; decisão original:22-25). |
| Strategic fit | strong | Debloat/vendor-unlock, "VPS test", Rust+Postgres, Svelte 5 + Tauri futuro, tiers Seed→Forest e AGPL; Opção C (paridade ampla) incompatível (`research.md:8,48-50`; `intake.md:16`). |
| Risk posture | adequate | Riscos de super-simplificação, suposição de demanda e drift com pesquisa pendente mitigados por NV, não-objetivos claros e fatias verticais verificáveis; sem reuso AGPL-incompatível (`concept.md:21`; decisão original:19,33-35). |

## Verdict & Rationale

Veredito histórico **`go (ciclo 005, escopo de reframe estratégico + planejamento 80/20)` preservado como `go`**: os artefatos não contradizem a decisão de 2026-11-15. O scorecard sustenta a Opção B — reframe estratégico com MVP Seed verificável — sobre a Opção A (recorte documental mínimo, que não aproxima o Seed de um fluxo utilizável) e a Opção C (paridade ampla com o Macro, "não fazer") (`research.md:48-50`; `concept.md:9-28,32`). O conteúdo material preservado inclui o destino de cada capacidade do Macro: **core Seed** — chat/canais/DMs + notificações básicas, tarefas/projetos leves, arquivos básicos + permissões e busca com Postgres FTS primeiro (Meilisearch opcional); **fase seguinte** — docs colaborativos leves sem CRDT pesado; **adiar/substituir** — email multi-conta completo, calls/transcrição/gravação, CRM/databases genéricas e calendar/booking; agentes/IA apenas **opcionais por feature flag**, nunca requisito Seed, com preferência por BYO API (`research.md:23-32,38-42`). Demanda real de PME, limites de ~2 vCPU/4 GB e conformidade de reimplementação limpa sob AGPL seguem como hipóteses não medidas/NV (`problem.md:35-38,46-48`; `concept.md:43-46`).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Ausência de recorte 80/20 e de debloat explícito para uma alternativa self-hosted ao Macro em "qualquer VPS barata"; sem norte, o esforço replica a complexidade operacional que a proposta promete evitar (`problem.md:9,11`).
- **Chosen approach**: Opção B — reframe estratégico com MVP Seed verificável (`concept.md:16-20,32`), poucas jornadas utilizáveis, componentes opcionais degradando graciosamente e Postgres como gravidade; paridade ampla rejeitada (`concept.md:23-28`).
- **In scope / out of scope**:
  - **In (decisão original:22-29)**: registrar assessment 005 (go); atualizar `OBJETIVO.md`/`ROADMAP.md` com o norte "alternativa self-hosted ao Macro para PMEs em VPS barata" e o MVP (chat/mensagens + tarefas + arquivos básicos + org/usuários/auth, busca Postgres-first); criar `specs/005-macro-alternative-mvp/`; consolidar pesquisa em `docs/research/2026-11-15-macro-alternative-80-20-research.md` (incorporação incremental dos subagentes, marcando NV).
  - **Out**: implementação ampla, migrações de arquitetura de alto risco, CI/infra remota, reuso AGPL-incompatível, RE não autorizada e promessas de paridade/carga sem medição (decisão original:32-35).
- **Success metrics**: jornadas verticais verificáveis ponta a ponta em dev (baseline: 0/NV); consumo em VPS de referência ~2 vCPU/4 GB (baseline: unknown — hipótese não medida); capacidades do Macro com destino e justificativa de custo (baseline: hipótese em tabela, sem validação de demanda); alinhamento OBJETIVO/ROADMAP ao estado observável (baseline: parcial) (`problem.md:35-38`; `research.md:21-32`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: existe evidência de demanda PME por chat/tarefas/arquivos ou é hipótese?]; [NEEDS CLARIFICATION: limites reais de CPU/RAM/disco e alvo de "VPS barata"?]; [NEEDS CLARIFICATION: como tratar conformidade AGPL em reimplementação limpa face a código/assets do Macro?] (`problem.md:46-48`).
