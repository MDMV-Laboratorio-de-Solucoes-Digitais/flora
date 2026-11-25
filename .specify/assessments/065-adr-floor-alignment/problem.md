# Problem Definition: ADRs de governança com pisos obsoletos

- **Slug**: 065-adr-floor-alignment
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

`docs/adr/0003:19` descreve os pisos do `db-tests` como `12/4/8/8/5/6 — total 43` e `docs/adr/0006:10,52` cita "piso 43" (research.md:5-7). O guardião determinístico, porém, usa `FLOORS` em `scripts/ci/check_gate_integrity.py:42-51` com auth 13, messaging 4, websocket 8, search 8, notifications 5, files 6, tasks 4 e workspace 3 — total 51, impresso dinamicamente (intake.md:11-17; research.md:9-10). O SE autorizou o patch documental citando "total 50", valor vigente antes do ciclo 064; a divergência 50→51 foi registrada em `STATE.md` e o alinhamento segue o valor real do gate (intake.md:9; decision.md:7-9). A correção pós-verificação de 13:12Z registrou que o detalhamento correto é `13/4/8/8/5/6/4/3 — total 51`: a citação anterior `/4/2` somava 50 e contradizia o total, sendo corrigida no ADR-0003:19 (define.md:26-29; research.md:23-25). Ranqueamento 80/20 = 10.0 (intake.md:23-27).

## Affected Users & Stakeholders

- **Users**: leitores das ADRs de governança que tomam as cifras de pisos como autoridade para decisões (intake.md:11-17; research.md:5-7).
- **Stakeholders**: SE, que autorizou o patch documental, e mantenedores de governança, que precisam preservar numeração/versão dos ADRs e o registro da divergência (`decision.md:7-13`; `research.md:12`).

## Goals

- Fazer `docs/adr/0003:19` refletir `13/4/8/8/5/6/4/3 — total 51` (`define.md:7`, com a correção pós-verificação em `:26-29`).
- Fazer as duas citações em `docs/adr/0006:10,52` refletirem `piso 51` (`define.md:8`).
- Registrar a autorização SE e a divergência 50→51 (`define.md:11`; `decision.md:9`).
- Preservar o texto restante, a numeração/versão dos ADRs e o histórico append-only de `STATE.md`/converge (`research.md:12`; `define.md:13`).

## Non-Goals

- Não mudar o gate para 50: o valor real é 51 (`define.md:14`; `shape.md:10`).
- Não reescrever histórico de `STATE.md`/converge (`define.md:13`; `research.md:21`).
- Não alterar constituição, código, CI, C5/C6 (`define.md:12`; `decision.md:12`).
- Não editar a constituição nem tratar a autorização como implícita fora do registro (`research.md:21-22`).

## Success Metrics

- `grep -n 43` nas citações de piso de `docs/adr/0003*`/`0006*` → 0 (baseline: 3 citações com 43 — define.md:15; research.md:5-7).
- Cifra impressa pelo guardião = 51 (baseline: 51 no gate, 43 nas ADRs — define.md:16; intake.md:17).
- Detalhamento `13/4/8/8/5/6/4/3` soma 51 e substitui `/4/2` (baseline: citação anterior somava 50 — define.md:26-29).
- Diff = 2 ADRs + artefatos do ciclo; gates locais verdes (baseline: nenhuma mudança de código — define.md:17-18; decision.md:11).
- Estado remoto/CI: **[NV]** (decision.md:17).

## Cost of Inaction

Documentação de governança continua contradizendo o gate determinístico, e a autorização SE baseada em "50" fica sem contexto para quem ler depois. Decisões futuras podem usar a cifra 43 ou o detalhamento `/4/2` que soma 50, ambos incorretos frente a `FLOORS` (intake.md:11-17; define.md:26-29).

## Open Questions

- [NEEDS CLARIFICATION: a divergência 50→51 exige nova aprovação de maintainer além da já registrada?] (`intake.md:9`; `decision.md:17`; **[NV]**)
- [NEEDS CLARIFICATION: as três citações em `0003:19` e `0006:10,52` são as únicas com "43"?] (`define.md:15`; `research.md:5-7`)
- [NEEDS CLARIFICATION: em qual PR o estado remoto/CI será observado?] (`decision.md:17`; **[NV]**)
