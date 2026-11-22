# Plan: 107 — wildcard one-shot

**Baseline**: `df8c1e185` · **Branch**: `feature/107-wildcard-one-shot`

## Steps

- T001 bug report e artefatos de assessoria (concluídos).
- T002 migration singleton `bootstrap_wildcard_state`.
- T003 helper `find_wildcard_admin` (papel) e `try_consume_wildcard` (CAS) em `flora-core`.
- T004 startup usa helper e respeita marcador.
- T005 login usa helper e consome antes da sessão.
- T006 ADR 0007 com expiração 2026-11-29.
- T007 testes unitários e extensão de contratos existentes (sem novos `#[ignore]`).
- T008 gates, analyze/converge/R4/R7 e PR.

## Constitution Check

Nenhum gate/required check novo; migration aditiva; fail-closed; compatível com ADR-0001.
