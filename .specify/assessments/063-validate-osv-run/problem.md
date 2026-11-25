# Problem Definition: Validar o scan OSV em execução real

- **Slug**: 063-validate-osv-run
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

O ciclo 052 fixou o workflow reutilizável de OSV scan em `@v2.7.1`, mas a validação foi apenas estática: sem uma execução real, o sinal advisory de supply-chain não tem evidência de funcionamento (intake.md:9-11; research.md:5; define.md:5-7). `gh api` às 04:43Z confirma `state=active`, id `420259367` e `total_count: 0` de runs, enquanto o próprio workflow declara `workflow_dispatch` além do cron semanal `37 14 * * 1`, com `fail-on-vuln: false` e nenhum segredo lido — há um caminho autorizado para validar o pin sem esperar a segunda-feira (intake.md:13-19; research.md:6-10). A tabela canônica em `STATE.md` marca o [NV] do OSV (intake.md:21). Ranqueamento 80/20 = 2.5, único candidato autônomo restante (intake.md:23-27).

## Affected Users & Stakeholders

- **Users**: mantenedores que dependem do sinal advisory de supply-chain produzido pelo scan OSV (intake.md:9-11).
- **Stakeholders**: responsáveis por registrar evidência em `STATE.md`, sem alterar código, ADRs ou gates; nenhuma autorização adicional é necessária porque o gatilho já faz parte do workflow (`decision.md:9-13`).

## Goals

- Disparar uma execução via `workflow_dispatch` do gatilho já declarado (`define.md:7`; `research.md:14-15`).
- Registrar em `STATE.md`, com apêndice datado, o resultado determinístico: `id`, `status`, `conclusion` e timestamp (`define.md:8`; `shape.md:5-7`).
- Registrar uma falha como ela é, sem maquiagem, sem alterar o workflow no mesmo movimento (`define.md:11`; `intake.md:11`).
- Manter o diff sem código e fechar o `[NV]` aberto no ciclo 052 (`define.md:10`; `decision.md:9`).

## Non-Goals

- Não alterar o workflow, o pin ou qualquer configuração se a execução falhar — ciclo separado (`define.md:13`; `shape.md:10`).
- Não disparar outros workflows nem tocar branches/PRs remotos (`define.md:14`; `shape.md:11`).
- Não modificar código, ADRs, constituição ou gates (`intake.md:29-31`; `decision.md:12`).
- Não fazer múltiplos disparos: um é suficiente e evita ruído em security-events (`shape.md:12`).

## Success Metrics

- Run criado e observado até conclusão (baseline: `total_count: 0` em `gh api` às 04:43Z — define.md:14; intake.md:15-16).
- `conclusion` e timestamp colados em `STATE.md` (baseline: `[NV]` registrado — define.md:15; intake.md:21).
- Verificador e gates locais verdes (baseline: verdes; mudança sem código — define.md:16).
- Diff sem código (baseline: nenhuma alteração de código — define.md:17; shape.md:7).
- Resultado da execução antes de obtido: **[NV]** — preenchido com evidência real (decision.md:17).

## Cost of Inaction

O `[NV]` do ciclo 052 permanece aberto por dias até o próximo cron e o pin continua sem evidência de funcionamento; se o job estiver silenciosamente quebrado, nenhum required check acusa. O registro append-only preserva o desfecho mesmo se a execução falhar, sem esconder a falha (research.md:16-17; shape.md:15-17).

## Open Questions

- [NEEDS CLARIFICATION: qual será o desfecho real do run — `conclusion` e timestamp — ainda **[NV]**?] (`decision.md:17`)
- [NEEDS CLARIFICATION: o upload de SARIF é comportamento esperado do workflow e não efeito colateral indevido?] (`shape.md:15-16`)
- [NEEDS CLARIFICATION: o apêndice em `STATE.md` preserva integralmente as notas anteriores?] (`shape.md:17`)
