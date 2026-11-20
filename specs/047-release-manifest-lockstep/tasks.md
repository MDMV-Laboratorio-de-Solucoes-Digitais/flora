# Tasks: 047 — Release manifest lockstep

- [x] T001 Confirmar teste vermelho com o comando exato do workflow. Verify: `RED_TEST_EXIT=1` registrado em `intake.md`.
- [x] T002 Editar `.github/workflows/release.yml:53-61`: `length == 8` → `length == 6` + commentário. Verify: `git diff --stat` mostra 1 arquivo.
- [x] T003 Reexecutar o mesmo `jq -e` com versões alinhadas. Verify: exit 0 colado em `validate.md`.
- [x] T004 Reexecutar gate autoritativo do alvo. Verify: `python3 scripts/ci/check_gate_integrity.py` → PASS.
- [x] T005 Revalidar SDD nativo (completeness/IDs/DAG/converge). Verify: saídas coladas em `validate.md`.
- [x] T006 Registrar `converge.md` append-only + `validate.md` (R4) + `release.md` (R7). Verify: artefatos presentes.
