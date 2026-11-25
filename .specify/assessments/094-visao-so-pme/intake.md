# Idea Intake: Visão SO-PME

- **Slug**: 094-visao-so-pme
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/094-visao-so-pme/decision.md` (+ `specs/094-visao-so-pme/spec.md`)
- **Type**: exploration

## Idea (as captured)

A nota registra a visão do SE de Flora como "Sistema Operacional" da PME/MDMV: ponto único de trabalho e integração entre sistemas (Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma), pessoas (sócios/funcionários/clientes) e agents MDMV (Autonoma, dark-factory, marketing, monetization, seller), com dogfooding na MDMV (`decision.md:8-13`). O artefato registra que se trata de nota de governança **NÃO-normativa**, documental e append-only, que não cria obrigação, não autoriza execução e não altera autoridade; o gate continua `check_gate_integrity.py` sob ADR-0003 (`decision.md:15-18`). O `specs/094-visao-so-pme/spec.md:9` registra como input a própria frase "Flora como Sistema Operacional da PME/MDMV — nota de governança, sem mudança de gate". A evidência de produção citada (health, deploy, refs, advisories) é registrada como fatos da sessão, não revalidada (`decision.md:26-36`); afirmações sobre estado remoto/CI além dela permanecem **[NV]**.

## Restated

Propõe-se apenas registrar, com data e sem força normativa, a visão de Flora como ponto de integração entre sistemas, pessoas e agents MDMV. O registro não autoriza mudanças de autoridade, gate, workflow ou código.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: Visão do SE registrada como input do ciclo (`specs/094-visao-so-pme/spec.md:9`) e nota de governança datada de 2026-11-20 (`decision.md:3-5`); [NEEDS CLARIFICATION: qual evento ou discussão precedeu essa visão?]

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Qual evento ou discussão precedeu a formulação da visão SO-PME?]
- [NEEDS CLARIFICATION: Como a visão deve ser revisitada ou atualizada sem alterar o registro append-only?]
- [NEEDS CLARIFICATION: Quem são os destinatários pretendidos da nota de governança além dos contribuidores futuros citados em `spec.md:28-35`?]
- [NEEDS CLARIFICATION: A evidência de produção registrada em `decision.md:26-31` deve ser reconsultada antes de servir como referência, dado o estado remoto [NV]?]
- [NEEDS CLARIFICATION: Existe algum plano de dogfooding na MDMV com escopo ou prazo definidos?]
- [NEEDS CLARIFICATION: A inclusão de novos sistemas, pessoas ou agents no mapa segue algum critério além do registro documental?]
