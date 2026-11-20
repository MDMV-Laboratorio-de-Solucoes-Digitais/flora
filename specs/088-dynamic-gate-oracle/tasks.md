# Tasks: Oráculo dinâmico de contratos ignorados no gate de CI

**Input**: Design documents from `/specs/088-dynamic-gate-oracle/`

**Prerequisites**: plan.md, spec.md, `.specify/assessments/088-dynamic-gate-oracle/decision.md`

**Tests**: incluídos porque a spec exige contagem descoberta e fail-closed (SC-001/SC-002).

**Status**: T001–T007 concluídas — evidência local registrada abaixo.

## Format: `[ID] [P?] Description`

- **[P]**: pode rodar em paralelo (arquivos distintos, sem dependência)

---

## Phase 1: Setup & Diagnosis

- [X] **T001** Confirmar âncora do ciclo 088.
  **Evidência**: `.specify/feature.json` =
  `{"feature_directory":"specs/088-dynamic-gate-oracle"}`; branch
  `feature/088-dynamic-gate-oracle` @ `cbc885383`; único arquivo de código no
  diff: `scripts/ci/check_gate_integrity.py`.

- [X] **T002** [P] Diagnosticar o espelho e validar a fonte de descoberta.
  **Evidência**: `check_gate_integrity.py` comparava só `FLOORS` ↔ `run_leg`
  do workflow; 086 (3 contratos `#[ignore]` em `rbac.rs`) e 087 (4 em
  `bootstrap.rs`) foram descobertos manualmente. `cargo test -p flora-tests
  -- --ignored --list` lista os 58 contratos; filtros `cargo test <filtro> --
  --list` não funcionaram de forma confiável. `FLOORS` = 10 entradas / 58
  (auth 13, messaging 4, websocket 8, search 8, notifications 5, files 6,
  tasks 4, workspace 3, rbac 3, bootstrap 4).

---

## Phase 2: Implementation

- [X] **T003** Descobrir os contratos ignorados no guarda.
  **Evidência**: `scripts/ci/check_gate_integrity.py` passou a chamar
  `subprocess.run(["cargo", "test", "-p", "flora-tests", "--", "--ignored",
  "--list"], cwd=ROOT, capture_output=True, text=True, check=False)` e a
  coletar de `stdout` somente os nomes de linhas terminadas em `: test`
  (FR-001/FR-002). `stderr` é usada apenas no detalhe do erro de falha do
  cargo.

- [X] **T004** Validar cada leg contra a descoberta.
  **Evidência**: regex de leg ancorada em linha
  (`run_leg <label> <filter> <floor>`) extrai as 10 legs de `rust.yml`; para
  cada uma o guarda exige `label in FLOORS`, `floor == FLOORS[label]`, uma
  única leg por rótulo e `count(name.startswith(filter)) == FLOORS[label]`
  (FR-002/FR-003). `startswith` cobre filtros com prefixo sobreposto, como
  `contract::auth::db_`.

- [X] **T005** [P] Fechar fail-closed e preservar o gate existente.
  **Evidência**: falhas de cargo (rc != 0), lista vazia, ausência de legs,
  rótulo desconhecido/duplicado e divergência de piso/contagem entram em
  `floor_failures` e viram exit 1. A mensagem de sucesso
  `all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)`
  foi mantida; `REQUIRED`, jobs, `continue-on-error`, path filters, comandos
  `cargo build/test --workspace` e `--remote` permanecem inalterados
  (FR-004/FR-005). Stdlib apenas, nenhuma dependência nova.

---

## Phase 3: Verification & Documentation

- [X] **T006** Validar o oráculo e a detecção de divergência.
  **Evidência**: `python3 scripts/ci/check_gate_integrity.py` → PASS com
  `all 10 db contract floors intact (58 total)` — 58 nomes descobertos, não
  digitados (SC-001). Divergência controlada (piso alterado em `FLOORS`,
  leg divergente ou contrato `#[ignore]` extra) gera falha de contagem/piso;
  cargo falhando ou lista vazia também falham (SC-002).

- [X] **T007** [P] Validar o escopo do diff e fechar os 4 artefatos 088.
  **Evidência**: `git diff --name-only cbc885383` contém apenas
  `scripts/ci/check_gate_integrity.py` e os artefatos 088 — zero arquivos em
  `.github/workflows/`, `validate/`, `release/` ou `STATE.md` (SC-003/FR-005).
  Artefatos: `decision.md` (verdict go, 2026-11-20), `spec.md`
  (FR-001..005, SC-001..003), `plan.md`, `tasks.md`.

---

## Dependencies

- T002 bloqueia T003/T004/T005 (diagnóstico antes da edição).
- T003 → T004 → T005 são incrementos no mesmo arquivo (ordem preservada).
- T006/T007 dependem de T003+T004+T005; T006 e T007 são paralelos
  (verificação funcional vs. escopo/documentação).
- Nenhuma tarefa depende de Postgres, rede, segredo, PR ou worktree espelho.

## Implementation Strategy

MVP = T001–T005 (um arquivo de código). T006/T007 fecham verificação e
documentação. O diff permanece cirúrgico: 1 arquivo de CI + 4 artefatos; a
descoberta dinâmica substitui o espelho sem criar novo required check.
