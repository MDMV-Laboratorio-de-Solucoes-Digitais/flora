# Problem Definition: Contagens verificadas defasadas em PROGRESSO

- **Slug**: 060-refresh-verified-counts
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

A nota de "estado verificado" em `docs/PROGRESSO.md:11-22` foi escrita em 02:23Z, logo após o ciclo 056, com specs 47 dirs/202 arquivos, assessments 44/175, validations 42/43 e releases 42/42 (intake.md:9-12; research.md:5). Os ciclos 057–059 acrescentaram artefatos e um `find` atual mostra specs 50/224, assessments 47/192, validations 46/47 e releases 46/46; também bugs 4/12, docs/research 135 (sem índice naquele momento) e docs/adr 6 (intake.md:15-17; research.md:6). Como `index.md:15` defere explicitamente a contagem a `PROGRESSO.md`, a nota defasada passa a ser o "estado verificado" citado pela entrada raiz. A nota anterior é histórico, não erro de fato — não deve ser reescrita (research.md:8; shape.md:11). Ranqueamento 80/20 = 5.0 (intake.md:24-28).

## Affected Users & Stakeholders

- **Users**: leitores que tomam as contagens em `PROGRESSO.md` como estado verificado do repositório (intake.md:11-14; research.md:11).
- **Stakeholders**: mantenedores que mantêm `PROGRESSO.md` e `index.md` como fonte de estado datado, com append-only aprovado no ciclo 056 (`decision.md:9-13`; `shape.md:11`).

## Goals

- Acrescentar nova seção datada em `docs/PROGRESSO.md` com contagens atuais obtidas por `find` (`define.md:7`; `research.md:13-15`).
- Preservar integralmente a seção anterior (append-only) e declarar âncora `262ac042f` e **[NV]** para estado remoto (`define.md:8-9`; `shape.md:5-7`).
- Não alterar código, migration, ADR ou gate (`define.md:10`; `decision.md:11`).
- Validar cada número com `find`, `git diff` (só acréscimos) e verificador isolado (`define.md:16-19`).

## Non-Goals

- Não corrigir ADRs — aprovação de maintainer pendente (`define.md:13`; `decision.md:17`).
- Não executar N3 — risco de migração e ADR-0006 (`define.md:14`; `research.md:20`).
- Não reescrever a nota de 02:23Z nem atualizar `docs/research/index.md` (contagem de research não mudou) (`research.md:16-19`; `shape.md:11`).
- Não editar `index.md`: não contém números próprios, apenas deferência (`research.md:18`).

## Success Metrics

- Cada número da nova tabela confere com `find` (baseline: contagens da nota 02:23Z defasadas em 4 categorias — define.md:16; intake.md:15-17).
- `git diff` mostra apenas acréscimo ao final de `PROGRESSO.md` (baseline: nenhuma alteração — define.md:17; shape.md:6).
- Verificador isolado confirma contagens e preservação da nota anterior (baseline: nota antiga única — define.md:18).
- Gates do alvo permanecem verdes (baseline: gate PASS 8 pisos/50 derivados de `FLOORS` — intake.md:19; define.md:19).
- Estado remoto/CI: **[NV]**; ADR-0003/0006 e N3 seguem escalados (`decision.md:17`).

## Cost of Inaction

Leitores continuam obtendo números antigos de uma seção rotulada como "estado verificado", e `index.md` herda o erro por deferência. A dívida cresce a cada ciclo que acrescenta artefatos; corrigi-la depois exigiria reconstruir qual contagem correspondia a qual data (intake.md:11-19; research.md:8-11).

## Open Questions

- [NEEDS CLARIFICATION: qual será a periodicidade das próximas atualizações da nota de contagens?] (`shape.md:5-7`)
- [NEEDS CLARIFICATION: a aprovação para corrigir ADR-0003/0006 chegará em qual ciclo?] (`decision.md:17`; **[NV]**)
- [NEEDS CLARIFICATION: o estado remoto/CI será verificado em qual PR?] (`decision.md:17`; **[NV]**)
