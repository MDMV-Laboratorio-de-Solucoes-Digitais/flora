# Tasks: 059 — DB tests fail locally

- [x] T001 Registrar vermelho: 4 passed em 0.00s sem env. Verify: `intake.md`.
- [x] T002 Substituir 9 retornos `Ok(None)` por `anyhow::bail!`. Verify: `git diff`.
- [x] T003 Validar sem env (exit != 0), `--list` (50) e DB real (50 passed). Verify: saídas em `validate.md`.
- [x] T004 Rodar gates locais e observar CI do PR. Verify: saídas em `validate.md`.
- [x] T005 Registrar `converge.md`, `validate.md` (R4) e `release.md` (R7). Verify: artefatos presentes.
