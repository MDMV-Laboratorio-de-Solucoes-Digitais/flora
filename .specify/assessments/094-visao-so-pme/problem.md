# Problem Definition: Registro da visão de integração da PME

- **Slug**: 094-visao-so-pme
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

A visão do SE de Flora como "Sistema Operacional" da PME/MDMV — ponto único de trabalho e integração entre sistemas (Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma), pessoas (sócios/funcionários/clientes) e agents MDMV (Autonoma, dark-factory, marketing, monetization, seller), com dogfooding na MDMV — era uma afirmação isolada, registrada como input do ciclo (`decision.md:8-13`; `specs/094-visao-so-pme/spec.md:9`). Sem um registro datado e rastreável, contribuidores futuros não dispõem de um mapa único e podem inferir dependências ou autoridade que os artefatos não suportam (`research.md:26`). A forma escolhida é a de nota de governança **NÃO-normativa**, documental e append-only: não cria obrigação, não autoriza execução e não altera autoridade; o gate continua `check_gate_integrity.py` sob ADR-0003 (`decision.md:15-18`). O mapa descreve intenção, não capacidade instalada: Grafana+VictoriaMetrics e Uptime Kuma estão deferidos e três agents estão ausentes no alvo (`research.md:72-75`). Não há registro de evento ou discussão que tenha precedido a visão (`intake.md:19`; `research.md:27`), e o estado operacional remoto é **[NV]** (`decision.md:33-35`).

## Affected Users & Stakeholders

- **Users**: SE/dono da MDMV, autor da visão registrada como input literal (`research.md:24`); futuro contribuidor, destinatário explícito do mapa de integrações, pessoas e agents (`research.md:26`).
- **Stakeholders**: maintainers que leem notas de governança; dono do gate ADR-0003 (autoridade inalterada); [ASSUMPTION] operadores das integrações listadas, não consultados diretamente.

## Goals

- Preservar a visão como registro datado, rastreável e append-only, sem força normativa (`decision.md:20-24`).
- Dar aos contribuidores futuros um mapa único de sistemas, pessoas e agents sem inferir dependências inexistentes (`research.md:26`).
- Distinguir explicitamente o que está implementado e verificado do que está deferido ou ausente no alvo (`research.md:74`).
- Manter o gate, a constituição, a autoridade de merge e os fluxos de execução inalterados (`decision.md:15-18`).

## Non-Goals

- Criar obrigação, autorização de execução, emenda constitucional, gate, workflow, migração ou código a partir da nota.
- Promover a visão a prioridade normativa: o risco de "escopo normativo acidental" é registrado no plano, não observado como efeito (`research.md:79`).
- Reescrever registros históricos, incluindo a nota anterior que dizia "deploy em curso, health **[NV]** até confirmação" (`research.md:76`).
- Afirmar estado operacional de sistemas, agents, CI, Coolify ou Forgejo sem **[NV]** (`research.md:77`).

## Success Metrics

- Nota datada e rastreável cobrindo os três grupos do mapa — sistemas, pessoas e agents (baseline: visão presente apenas como sinal único do SE, sem registro — `research.md:24-27`).
- Zero emendas, gates, workflows ou código criados a partir da nota (baseline: diff confinado aos quatro artefatos do ciclo e à nota de governança — `research.md:77`).
- Distinção explícita entre implementado/verificado, deferido e ausente no alvo (baseline: estados heterogêneos registrados em `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md:13-22` — `research.md:74`).
- Nenhum PR aberto ou mesclado, nenhum merge/tag/deploy promovido pela nota (baseline: registrado no fechamento do ciclo — `research.md:76`).

## Cost of Inaction

Sem registro datado, a visão permanece implícita e sujeita a interpretações divergentes sobre o que Flora integra e o que autoriza. Contribuidores podem tratar sistemas deferidos ou agents ausentes como capacidades instaladas, ou ler a nota como prioridade normativa, deslocando decisões que hoje pertencem a ciclos próprios sob ADR-0003.

## Open Questions

- [NEEDS CLARIFICATION: qual evento ou discussão precedeu a formulação da visão SO-PME?] (`intake.md:23`; `research.md:27`)
- [NEEDS CLARIFICATION: como a visão deve ser revisitada ou atualizada sem alterar o registro append-only?] (`intake.md:24`)
- [NEEDS CLARIFICATION: quem são os destinatários pretendidos da nota de governança além dos contribuidores futuros citados em `spec.md:28-35`?] (`intake.md:25`)
- [NEEDS CLARIFICATION: a evidência de produção registrada em `decision.md:26-31` deve ser reconsultada antes de servir como referência, dado o estado remoto **[NV]**?] (`intake.md:26`; `research.md:78`)
- [NEEDS CLARIFICATION: existe algum plano de dogfooding na MDMV com escopo ou prazo definidos?] (`intake.md:27`; `research.md:79`)
- [NEEDS CLARIFICATION: a inclusão de novos sistemas, pessoas ou agents no mapa segue algum critério além do registro documental?] (`intake.md:28`)
