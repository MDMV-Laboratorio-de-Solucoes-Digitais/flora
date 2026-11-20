# Implementation Plan: RBAC CI leg — fechar o confidence gap dos contratos DB

**Branch**: `feature/086-rbac-ci-leg` @ `9b05abb9d` | **Date**: 2026-11-20 | **Spec**: [spec.md](./spec.md)

**Input**: `/specs/086-rbac-ci-leg/spec.md`

## Summary

Registrar a lega `rbac` no job `db-tests` e seu piso no guarda de integridade,
para que os 3 contratos DB de RBAC — hoje `#[ignore]` e fora do CI — passem a
ser executados e protegidos contra drift. Duas linhas de configuração; nenhuma
mudança de aplicação.

## Technical Context

**Language/Version**: Rust 1.98.1 edition 2024 (workspace de contratos); Python 3
(stdlib) no guarda de CI; YAML no workflow do GitHub Actions.

**Primary Dependencies**: `cargo test -p flora-tests` com filtro
`contract::rbac::db::db_`, `--ignored --test-threads=1`; helper `run_leg`
(`rust.yml:90-104`) que soma contadores `N passed`; regex `run_leg\s+...db_...`
em `check_gate_integrity.py`.

**Storage**: PostgreSQL descartável já provisionado via
`FLORA_DATABASE__POSTGRES_URL` / `DATABASE_URL` (inalterado).

**Testing**: lega local 3 passed; `python3 scripts/ci/check_gate_integrity.py`
→ PASS `all 9 db contract floors intact (54 total)`.

**Target Platform**: GitHub Actions `ubuntu-latest`, job `db-tests` (required
check, ADR-0003).

**Project Type**: configuração de CI (não adiciona crate, rota ou serviço).

**Performance Goals**: custo marginal = 3 testes extras no mesmo job; sem
novo job, sem paralelização adicional.

**Constraints**: piso exatamente 3 (número de testes descobertos); não tocar
`validate/`, `release/`, `STATE.md`, outros pisos/legs ou `REQUIRED`/
`ALLOWED_JOB_IF`.

**Scale/Scope**: +1 linha no workflow, +1 linha no guarda, 4 artefatos 086.

## Constitution Check

*Gate*: **PASS**. Seed = Flora + PostgreSQL (nenhum serviço novo); helper
`run_leg` já fail-closed (variáveis `:?`); nenhum required check novo, apenas
reforço de um existente; ADR-0003 preservado. Nenhuma violação — tabela de
complexidade vazia.

## Arquivos tocados

| Arquivo | Mudança | Evidência |
|---|---|---|
| `.github/workflows/rust.yml` | +1: `run_leg rbac contract::rbac::db::db_ 3` em `db-tests` (:115) | diff `1 +` sobre `9b05abb9d` |
| `scripts/ci/check_gate_integrity.py` | +1: `"rbac": 3` em `FLOORS` (:51) | diff `1 +`; FLOORS agora 9 entradas / 54 |
| `.specify/assessments/086-rbac-ci-leg/decision.md` | novo — verdict go | data 2026-11-20 |
| `specs/086-rbac-ci-leg/spec.md` | preenchido (FR-001..004, SC-001..003) | antes template |
| `specs/086-rbac-ci-leg/plan.md` | novo | data 2026-11-20 |
| `specs/086-rbac-ci-leg/tasks.md` | novo — T001..T006 `[X]` | evidência local por tarefa |

**Fora de escopo**: `flora-tests/src/contract/rbac.rs` (testes já existem),
`validate/`, `release/`, `.specify/memory/STATE.md`, PRs, worktree espelho.

## Structure Decision

Nenhuma estrutura nova: reaproveitar o padrão de 8 legs existentes. A lega
`rbac` segue a mesma forma das demais, inclusive o piso numérico conferido pelo
guarda determinístico.

## Next Phase

Implementação concluída e verificada localmente; fechamento documental do ciclo
086. Nenhuma fase adicional.
