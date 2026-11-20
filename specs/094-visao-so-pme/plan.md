# Implementation Plan: 094 — Visão SO-PME

**Data:** 2026-11-20 · **Baseline:** `feature/094-visao-so-pme` @ `fd25d8f8b`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| decision | veredito go + natureza NÃO-normativa + evidência | — |
| spec | FR-001..FR-005, SC-001..SC-003, nota/mapa/status | decision |
| plano | ordem e delimitadores | spec |
| tasks | T001..T006 marcadas | plano |

Ordem: decision → spec → plano → tasks.

## Abordagem

1. Registrar em `decision.md` o veredito go para registro documental
   não-normativo e a evidência de produção datada.
2. Escrever `spec.md` com a nota de visão (FR-001), mapa de integrações,
   pessoas e agents (FR-002), status de produção (FR-003), append-only
   (FR-004) e zero mudança de gate (FR-005), mais SC-001..SC-003.
3. Plano curto e `tasks.md` com T001..T006 concluídas.

## Riscos

- Evidência de produção datada pode envelhecer — mitigado por data/hora e **[NV]**.
- Risco de escopo normativo acidental — mitigado por FR-004/FR-005 e delimitadores explícitos.

## Verificação

Leitura dos 4 artefatos; `git diff` confinado; `check_gate_integrity.py` PASS
(executado pelo dono do gate, não por este ciclo).

## Fora do plano

Emenda constitucional, ADRs, gate/workflow/migração/código, CI, deploy,
`STATE.md`, `validate.md`, `release.md`.
