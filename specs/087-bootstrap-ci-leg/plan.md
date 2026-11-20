# Implementation Plan: Bootstrap CI leg — fechar o confidence gap dos contratos DB

**Branch**: `feature/087-bootstrap-ci-leg` @ `367a50e79` | **Date**: 2026-11-20 | **Spec**: [spec.md](./spec.md)

**Input**: `/specs/087-bootstrap-ci-leg/spec.md`

## Summary

Registrar a lega `bootstrap` no job `db-tests` e seu piso no guarda de
integridade, para que os 4 contratos DB de bootstrap — hoje `#[ignore]` e
fora do CI — passem a ser executados e protegidos contra drift. Duas linhas de
configuração; nenhuma mudança de aplicação.

## Technical Context

**Language/Version**: Rust 1.98.1 edition 2024 (workspace de contratos); Python 3
(stdlib) no guarda de CI; YAML no workflow do GitHub Actions.

**Primary Dependencies**: `cargo test -p flora-tests` com filtro
`contract::bootstrap::tests::db_`, `--ignored --test-threads=1`; helper `run_leg`
(`rust.yml:90-105`) que soma contadores `N passed`; regex em
`check_gate_integrity.py:118` exige filtro terminado em `::db_`.

**Storage**: PostgreSQL descartável já provisionado via
`FLORA_DATABASE__POSTGRES_URL` / `DATABASE_URL` (inalterado).

**Testing**: lega local 4 passed; `python3 scripts/ci/check_gate_integrity.py`
→ PASS `all 10 db contract floors intact (58 total)`.

**Target Platform**: GitHub Actions `ubuntu-latest`, job `db-tests` (required
check, ADR-0003).

**Project Type**: configuração de CI (não adiciona crate, rota ou serviço).

**Performance Goals**: custo marginal = 4 testes extras no mesmo job; sem
novo job, sem paralelização adicional.

**Constraints**: piso exatamente 4 (número de testes descobertos); filtro no
formato aceito pelo guarda; não tocar `validate/`, `release/`, `STATE.md`,
outros pisos/legs ou `REQUIRED`/`ALLOWED_JOB_IF`.

**Scale/Scope**: +1 linha no workflow, +1 linha no guarda, 4 artefatos 087.

## Constitution Check

*Gate*: **PASS**. Seed = Flora + PostgreSQL (nenhum serviço novo); helper
`run_leg` já fail-closed (variáveis `:?`); nenhum required check novo, apenas
reforço de um existente; ADR-0003 preservado. Nenhuma violação — tabela de
complexidade vazia.

## Arquivos tocados

| Arquivo | Mudança | Evidência |
|---|---|---|
| `.github/workflows/rust.yml` | +1: `run_leg bootstrap contract::bootstrap::tests::db_ 4` em `db-tests` (:116) | diff `1 +` sobre `367a50e79` |
| `scripts/ci/check_gate_integrity.py` | +1: `"bootstrap": 4,` em `FLOORS` (:52), 10ª entrada; `"rbac": 3` preservado | diff `1 +`; FLOORS agora 10 entradas / 58 |
| `.specify/assessments/087-bootstrap-ci-leg/decision.md` | novo — verdict go | data 2026-11-20 |
| `specs/087-bootstrap-ci-leg/spec.md` | preenchido (FR-001..004, SC-001..003) | antes template |
| `specs/087-bootstrap-ci-leg/plan.md` | novo | data 2026-11-20 |
| `specs/087-bootstrap-ci-leg/tasks.md` | novo — T001..T006 `[X]` | evidência local por tarefa |

**Fora de escopo**: `flora-tests/src/contract/bootstrap.rs` (testes já
existem), `validate/`, `release/`, `.specify/memory/STATE.md`, PRs, worktree
espelho.

## Structure Decision

Nenhuma estrutura nova: reaproveitar o padrão das 9 legs existentes. A lega
`bootstrap` segue a mesma forma das demais, inclusive o piso numérico conferido
pelo guarda determinístico. O filtro usa `::db_` (não `::db_bootstrap_`) para
ser aceito pela regex do guarda sem alterá-la.

## Next Phase

Implementação concluída e verificada localmente; fechamento documental do ciclo
087. Nenhuma fase adicional.
