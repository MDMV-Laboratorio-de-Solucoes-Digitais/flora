# Idea Intake: Auditoria viva de governança do CI

- **Slug**: 091-governance-ci-audit
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/091-governance-ci-audit/decision.md` (+ `specs/091-governance-ci-audit/spec.md`)
- **Type**: compliance

## Idea (as captured)

Os ciclos 086–090 reforçaram o CI de forma determinística (leg RBAC, leg bootstrap, oráculo dinâmico do gate, espelho Forgejo advisory e bump de `rustls`); a emenda de 2026-11-20 à ADR-0003, confirmada literalmente pelo SE, estabelece a regra vigente: CI/CD GREEN basta para merge autônomo em `dev/` e, se o CI não inspirar confiança, o próximo 80/20 imediato é reforçá-lo — não escalar para aprovação humana (`decision.md:12-18`). O artefato registra a falta de uma **auditoria viva** que confirme que a governança praticada coincide com a documentada: required checks e proteção reais de `dev`/`main`, emenda aplicada sem aprovador humano, deploy fora do gate de merge, explicações auditáveis e omissão dos 4 scripts SDD da factory em relatórios, conforme decisão 089 (`decision.md:20-25`). A decisão registrada classifica o ciclo como auditoria documental e não mutante (`decision.md:31-34`); o `specs/091-governance-ci-audit/spec.md:9-13` registra o mesmo input. Estado ao vivo do CI, da proteção de branch e do deploy permanece **[NV]** (`decision.md:92-95`).

## Restated

Propõe-se conferir, de forma documental e sem mutação, se o estado real do CI e da proteção de branch corresponde à governança registrada, mantendo a regra de merge autônomo sob CI green. O registro também abrange como explicar fatos verificados e itens diferidos sem alterar gates, workflows ou decisões.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: Emenda de 2026-11-20 à ADR-0003 e a sequência dos ciclos 086–090, que reforçaram o CI (`decision.md:12-25`); o estado ao vivo que a auditoria pretendia confirmar está marcado como **[NV]** em `decision.md:92-95`.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Qual é o estado ao vivo dos required checks e da proteção de `dev`/`main` no momento da auditoria, além do registrado como [NV]?]
- [NEEDS CLARIFICATION: Qual divergência concreta entre documentação e prática motivou a auditoria, se houver alguma?]
- [NEEDS CLARIFICATION: Qual é o destino e a reavaliação das demais emendas pendentes registradas como escaladas [NV] a maintainer?]
- [NEEDS CLARIFICATION: Qual é o estado atual do deploy em produção conduzido pela ligação MDMV-Autonoma, incluindo health e imagem de rollback?]
- [NEEDS CLARIFICATION: Como os itens diferidos (Tauri/OTLP, leg RBAC) devem ser acompanhados até uma eventual reavaliação?]
- [NEEDS CLARIFICATION: Qual audiência e periodicidade são esperadas para o relatório de auditoria viva?]
