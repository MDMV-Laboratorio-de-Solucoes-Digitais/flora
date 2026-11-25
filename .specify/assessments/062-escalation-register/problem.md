# Problem Definition: Registro canônico dos escalonamentos

- **Slug**: 062-escalation-register
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

Os mesmos quatro itens bloqueados por governança — ADR-0003/0006, N3, C5 e C6 — são reavaliados em cada sessão autônoma e espalhados por dezenas de notas em `STATE.md`, sem um registro consolidado de evidência, bloqueio e ação necessária: `grep -n escalad STATE.md` encontrou ~10 menções repetidas e nenhuma tabela única (intake.md:9-16; research.md:5). Ao mesmo tempo, o estado do scan OSV permanece **[NV]** sem evidência do porquê, embora `gh api` às 04:25Z mostre workflow `active`, id `420259367`, criado em 2026-11-16T11:04:49-04:00, **0 runs** e cron semanal `37 14 * * 1` (intake.md:19-21; research.md:5-8). ADR-0003:19 e ADR-0006:10,52 citam "43" enquanto o gate imprime 50 derivado de `FLOORS`; N3 tem órfãos **[NV]** e exige SE para deploy; C5/C6 dependem de dono/SE (intake.md:17-19; research.md:10-12). Ranqueamento 80/20 = 5.0 (intake.md:27-29).

## Affected Users & Stakeholders

- **Users**: agentes e sessões autônomas que precisam saber rapidamente o que está bloqueado e por que, sem reavaliar cada item do zero (intake.md:9-14).
- **Stakeholders**: dono, SE e maintainers que devem autorizar C5/C6, ADRs e N3; não se deve simular nenhuma autorização (`decision.md:9-13`; `research.md:16`).

## Goals

- Incluir em `STATE.md` uma tabela append-only com os 4 itens escalados e colunas de evidência, bloqueio, ação necessária e status (`define.md:7`; `shape.md:5-7`).
- Registrar o estado OSV verificado ao vivo: `active`, 0 runs, cron semanal e pin ainda não validado (`define.md:8`; `research.md:13-15`).
- Não executar nenhum item bloqueado (`define.md:9`; `decision.md:11`).
- Não alterar código, ADR, migração ou gate (`define.md:10`; `decision.md:12`).

## Non-Goals

- Não alterar ADRs, N3, C5/C6, constituição ou CI (`define.md:13`; `decision.md:12`).
- Não disparar o workflow OSV: efeito remoto desnecessário neste ciclo (`define.md:14`; `research.md:18`).
- Não forjar validate/release para dívidas de fase (`research.md:17`).
- Não criar documento novo fora de `STATE.md`: fragmentaria o registro (`shape.md:11`).

## Success Metrics

- Tabela com 4 linhas e colunas completas (baseline: nenhuma tabela; ~10 menções repetidas — define.md:15; intake.md:16).
- Números/estados do OSV conferem com `gh api` (baseline: 0 runs, `active`, id `420259367`, cron `37 14 * * 1` — define.md:16; research.md:5-8).
- Verificador isolado PASS (baseline: sem registro canônico para verificar — define.md:17).
- Gates do alvo verdes (baseline: verdes; mudança apenas em arquivo de memória — define.md:18).
- Aprovação de maintainer/SE/dono e execução futura do OSV: **[NV]** (decision.md:17).

## Cost of Inaction

Cada sessão gasta orçamento de análise reavaliando os mesmos quatro bloqueios, e a ausência de registro aumenta o risco de um ciclo futuro iniciar trabalho que exige autorização. O **[NV]** do OSV permanece sem contexto — 0 runs é esperado antes da primeira segunda-feira, mas isso só fica explícito com a evidência consolidada (intake.md:11-14; research.md:13-15).

## Open Questions

- [NEEDS CLARIFICATION: quem aprova cada um dos quatro itens e em qual canal a autorização será registrada?] (`decision.md:17`; `intake.md:19`; **[NV]**)
- [NEEDS CLARIFICATION: quando ocorrerá a primeira execução real do OSV após a criação do workflow?] (`research.md:13-15`; **[NV]**)
- [NEEDS CLARIFICATION: a tabela em `STATE.md` deve permanecer append-only também para mudanças de status?] (`shape.md:6-7`)
