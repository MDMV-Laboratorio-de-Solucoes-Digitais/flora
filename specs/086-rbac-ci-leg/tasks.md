# Tasks: RBAC CI leg — fechar o confidence gap dos contratos DB

**Input**: Design documents from `/specs/086-rbac-ci-leg/`

**Prerequisites**: plan.md, spec.md, `.specify/assessments/086-rbac-ci-leg/decision.md`

**Tests**: incluídos porque a spec exige piso verificável (SC-001/SC-002).

**Status**: T001–T006 concluídas — evidência local registrada abaixo.

## Format: `[ID] [P?] Description`

- **[P]**: pode rodar em paralelo (arquivos distintos, sem dependência)

---

## Phase 1: Setup & Diagnosis

- [X] **T001** Confirmar âncora do ciclo 086.
  **Evidência**: `.specify/feature.json` =
  `{"feature_directory":"specs/086-rbac-ci-leg"}`; branch
  `feature/086-rbac-ci-leg` @ `9b05abb9d`.
- [X] **T002** [P] Inventariar os contratos RBAC `@ignore` fora do CI e a
  ausência da lega no workflow base.
  **Evidência**: `flora-tests/src/contract/rbac.rs:143,171,222` (`#[ignore =
  "requires Postgres; ..."]`) com funções em :144, :172, :223 — 3 testes
  `db_`; `rust.yml` base sem nenhuma linha `run_leg rbac`.

---

## Phase 2: Implementation (duas linhas)

- [X] **T003** Adicionar a lega ao job `db-tests` em `.github/workflows/rust.yml`.
  **Evidência**: linha :115 `run_leg rbac contract::rbac::db::db_     3`,
  imediatamente após `workspace`; diff `+1 -0` sobre `9b05abb9d`.
- [X] **T004** [P] Registrar o piso no guarda
  `scripts/ci/check_gate_integrity.py`.
  **Evidência**: linha :51 `"rbac": 3,` dentro de `FLOORS` (9ª entrada);
  diff `+1 -0`; regex do guarda passa a cobrir a lega (FR-002).

---

## Phase 3: Verification & Documentation

- [X] **T005** Validar o guarda de integridade do gate.
  **Evidência**: `python3 scripts/ci/check_gate_integrity.py` → PASS com
  `all 9 db contract floors intact (54 total)`; nenhuma falha
  `db floor for rbac` (SC-002). Soma: 13+4+8+8+5+6+4+3+3 = 54.
- [X] **T006** [P] Validar a lega local e o escopo do diff; fechar os 4
  artefatos do ciclo.
  **Evidência**: lega `contract::rbac::db::db_` local = **3 passed** (SC-001);
  `git diff --name-only 9b05abb9d` contém apenas `rust.yml`,
  `check_gate_integrity.py` e os artefatos 086 — zero arquivos em `validate/`,
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
