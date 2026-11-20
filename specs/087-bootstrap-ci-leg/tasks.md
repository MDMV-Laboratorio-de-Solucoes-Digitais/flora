# Tasks: Bootstrap CI leg — fechar o confidence gap dos contratos DB

**Input**: Design documents from `/specs/087-bootstrap-ci-leg/`

**Prerequisites**: plan.md, spec.md, `.specify/assessments/087-bootstrap-ci-leg/decision.md`

**Tests**: incluídos porque a spec exige piso verificável (SC-001/SC-002).

**Status**: T001–T006 concluídas — evidência local registrada abaixo.

## Format: `[ID] [P?] Description`

- **[P]**: pode rodar em paralelo (arquivos distintos, sem dependência)

---

## Phase 1: Setup & Diagnosis

- [X] **T001** Confirmar âncora do ciclo 087.
  **Evidência**: `.specify/feature.json` =
  `{"feature_directory":"specs/087-bootstrap-ci-leg"}`; branch
  `feature/087-bootstrap-ci-leg` @ `367a50e79`.

- [X] **T002** [P] Inventariar os contratos bootstrap `@ignore` fora do CI e a
  ausência da lega no workflow base.
  **Evidência**: `flora-tests/src/contract/bootstrap.rs:137` (`#[ignore =
  "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]`) com funções em
  :138, :188, :211, :229 — 4 testes `db_bootstrap_*` no módulo `tests` (:7),
  prefixo confirmado via `cargo test -- --list`:
  `contract::bootstrap::tests::db_bootstrap_`; `rust.yml` base sem nenhuma
  linha `run_leg bootstrap`.

---

## Phase 2: Implementation (duas linhas)

- [X] **T003** Adicionar a lega ao job `db-tests` em `.github/workflows/rust.yml`.
  **Evidência**: linha :116
  `run_leg bootstrap contract::bootstrap::tests::db_ 4`, imediatamente após
  `rbac`; diff `+1 -0` sobre `367a50e79`. Filtro `::db_` cobre os mesmos 4
  testes `db_bootstrap_*` e casa a regex do guarda (linha 118); a forma longa
  `::db_bootstrap_` foi descartada por não casar (match=False).

- [X] **T004** [P] Registrar o piso no guarda
  `scripts/ci/check_gate_integrity.py`.
  **Evidência**: linha :52 `"bootstrap": 4,` dentro de `FLOORS` (10ª entrada),
  com `"rbac": 3,` preservado na linha :51; diff `+1 -0`; regex do guarda
  passa a cobrir a lega (FR-002).

---

## Phase 3: Verification & Documentation

- [X] **T005** Validar o guarda de integridade do gate.
  **Evidência**: `python3 scripts/ci/check_gate_integrity.py` → PASS com
  `all 10 db contract floors intact (58 total)`; nenhuma falha
  `db floor for bootstrap` (SC-002). Soma:
  13+4+8+8+5+6+4+3+3+4 = 58.

- [X] **T006** [P] Validar a lega local e o escopo do diff; fechar os 4
  artefatos do ciclo.
  **Evidência**: lega `contract::bootstrap::tests::db_` local = **4 passed**
  (SC-001); `git diff --name-only 367a50e79` contém apenas `rust.yml`,
  `check_gate_integrity.py` e os artefatos 087 — zero arquivos em `validate/`,
  `release/` ou `STATE.md` (SC-003/FR-004). Artefatos: `decision.md`
  (verdict go, 2026-11-20), `spec.md` (FR-001..004, SC-001..003), `plan.md`,
  `tasks.md`.

---

## Dependencies

- T002 bloqueia T003/T004 (diagnóstico antes da edição).
- T005/T006 dependem de T003+T004; T005 e T006 são paralelos (gate vs. lega/diff).
- Nenhuma tarefa depende de rede, segredo, PR ou worktree espelho.

## Implementation Strategy

MVP = T001–T004 (duas linhas); T005/T006 fecham a verificação e a
documentação. O diff permanece cirúrgico: 2 linhas de CI + 4 artefatos.
