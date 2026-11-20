# Intake: 062 — Escalation register

**Data:** 2026-11-18T04:27Z · **Âncora:** `github/dev@104c581f3` · branch `feature/062-escalation-register`

## Sinal

Os mesmos quatro itens bloqueados (ADR-0003/0006, N3, C5, C6) são reavaliados
em cada sessão autônoma, espalhados por dezenas de notas em `STATE.md`, sem um
registro consolidado de evidência, bloqueio e ação necessária. Isso gasta
orçamento de análise e aumenta o risco de um ciclo futuro iniciar trabalho
que exige autorização.

## Evidência (verificada)

- `grep -n escalad STATE.md` → ~10 menções repetidas; nenhuma tabela única.
- ADR-0003:19 e ADR-0006:10,52 citam "43"; gate imprime 50 (derivado de `FLOORS`).
- N3: FK composto com órfãos **[NV]**; ADR-0006:27-31 exige SE para deploy.
- C5/C6: decisão de dono / autorização SE (STATE.md, OBJETIVO.md).
- OSV (`gh api` 04:25Z): workflow `active`, id `420259367`, criado
  2026-11-16T11:04:49-04:00, **0 runs**; cron semanal `37 14 * * 1`.

## 80/20

`(I×C)/(E×R)` = (2×5)/(2×1) = **5.0**, maior candidato autônomo. Nenhum item
bloqueado será executado; este ciclo apenas consolida o registro.

## Fronteiras

Escopo: apêndice em `STATE.md`, artefatos do ciclo e notas datadas. Fora:
ADRs, migrações, código, CI, C5/C6, deploy. Factory fonte `4a5de2ce…`.
