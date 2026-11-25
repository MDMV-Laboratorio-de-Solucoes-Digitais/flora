# Decision: Visão SO-PME

- **Slug**: 094-visao-so-pme
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | A visão do SE como “Sistema Operacional” da PME/MDMV era uma afirmação isolada, sem registro datado e rastreável; contribuidores podem inferir dependências ou autoridade que os artefatos não suportam (`problem.md:7-9`, `research.md:26`). |
| Evidence strength | strong | A frase literal está em `decision.md:8-13` e `specs/094-visao-so-pme/spec.md:9`; estados heterogêneos e a nota histórica estão em `research.md:24-27,34`; a evidência de produção é fotografia datada, registrada mas não revalidada (`decision anterior:26-31`). |
| Value vs. inaction | adequate | Sem registro, a visão fica sujeita a interpretações divergentes e pode ser lida como prioridade normativa, deslocando decisões de ciclos próprios sob ADR-0003 (`problem.md:37-39`, `concept.md:12`). |
| Feasibility / appetite | strong | Opção A: documentação datada no formato de notas de governança já usado em ciclos anteriores; apetite small (`concept.md:9-11`). |
| Strategic fit | strong | Nota explicitamente NÃO-normativa, append-only, que preserva autoridade inalterada e o gate `check_gate_integrity.py` sob ADR-0003 (`decision anterior:15-18`, `concept.md:29`). |
| Risk posture | strong | Risco de “escopo normativo acidental” mitigado pela declaração não-normativa e pela distinção explícita de estados; nenhum PR, merge, tag ou deploy promovido pela nota (`concept.md:12,29`, `problem.md:35`). |

## Verdict & Rationale

Veredito histórico `go (registro documental NÃO-normativo)` de 2026-11-20 preservado como `go`: nenhum critério é `weak`/`unknown` e não há contradição nos artefatos — as incógnitas listadas são não-bloqueantes para um registro documental. A Opção A é a recomendada em `concept.md:27-29`: entrega nota datada cobrindo sistemas, pessoas e agents, com estados explícitos, sem criar obrigação ou autorização; a B não entrega o mapa nem a distinção de estados (`:15-19`) e a C contradiz a natureza não-normativa e exigiria autorização ausente (`:21-25`). Estado operacional, adoção e reconsulta de produção são `[NV]`; nenhuma autorização é simulada. Branch `feature/094-visao-so-pme` @ `fd25d8f8b` (`decision anterior:5`).

## If needs-clarification

- **Blocking questions**: N/A — nenhuma; as seis perguntas em aberto não bloqueiam o registro não-normativo e seguem listadas no handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: visão SO-PME isolada e sem registro datado permite inferências divergentes sobre o que Flora integra e o que autoriza.
- **Chosen approach**: Opção A — nota de governança datada, não-normativa e append-only com mapa único de sistemas (Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma), pessoas (sócios/funcionários/clientes) e agents MDMV (Autonoma, dark-factory, marketing, monetization, seller), classificando cada item como implementado/verificado, deferido ou ausente no alvo (`concept.md:9-11`, `decision anterior:9-18`).
- **In scope / out of scope**: dentro = apenas os 4 artefatos do ciclo 094 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`) e a nota de governança; fora = `validate.md`, `release.md`, `STATE.md`, constituição, ADRs, CI, deploy, obrigação/autorização de execução, roadmap, plano de dogfooding, critério de inclusão de novos itens, reescrita de registros históricos (inclusive a nota “deploy em curso, health `[NV]`”), afirmação de estado operacional sem `[NV]` e PR/merge/tag/deploy (`decision anterior:20-24`, `concept.md:31-37`). PR independente; nenhum trabalho alheio tocado.
- **Success metrics**: nota datada e rastreável cobrindo sistemas, pessoas e agents; zero emendas, gates, workflows ou código criados a partir da nota; distinção explícita entre implementado/verificado, deferido e ausente; nenhum PR aberto/mesclado, merge, tag ou deploy promovido pela nota (`problem.md:30-35`, `concept.md:29`). Baseline: estados heterogêneos em `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:13-22` (`problem.md:34`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: qual evento ou discussão precedeu a visão SO-PME?]; [NEEDS CLARIFICATION: como revisitá-la sem alterar o registro append-only?]; [NEEDS CLARIFICATION: quem são os destinatários além dos contribuidores futuros?]; [NEEDS CLARIFICATION: a evidência de produção de `decision.md:26-31` deve ser reconsultada dado o estado remoto `[NV]`?]; [NEEDS CLARIFICATION: existe plano de dogfooding na MDMV com escopo/prazo?]; [NEEDS CLARIFICATION: há critério de inclusão de novos sistemas, pessoas ou agents?] (`intake.md:23-28`, `problem.md:43-48`).
