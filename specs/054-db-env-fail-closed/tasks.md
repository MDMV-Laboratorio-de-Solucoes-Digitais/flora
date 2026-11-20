# Tasks: 054 — DB env fail-closed

- [x] T001 Registrar vermelho: 9 `Ok(None)` e 50 `Ok(())` sem env; guardião cego. Verify: `research.md`.
- [x] T002 Adicionar expansões `:?` em `run_leg` no workflow. Verify: diff + teste sem env.
- [x] T003 Exigir env no guardião determinístico. Verify: mutação temporária → RED.
- [x] T004 Rodar 8 legs com DB real isolado e gates locais. Verify: saídas em `validate.md`.
- [x] T005 Registrar `converge.md`, `validate.md` (R4) e `release.md` (R7). Verify: artefatos presentes.
