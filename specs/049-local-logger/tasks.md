# Tasks: 049 — Local logger

- [x] T001 Registrar evidência do 404 e do contrato obsoleto. Verify: `intake.md`/`research.md`.
- [x] T002 Reescrever `logger.ts` como local-only (buffer 200 + `snapshot()`). Verify: `rg` sem `/api/v1/telemetry/logs`.
- [x] T003 Atualizar contrato 002 (`flush()` e ingestão backend removidos). Verify: diff explícito registrado.
- [x] T004 Criar `logger.spec.ts` no projeto `server`. Verify: `bun run test:unit -- --run --project server` verde.
- [x] T005 Reexecutar gates frontend/Rust + gate autoritativo. Verify: exits colados em `validate.md`.
- [x] T006 Registrar `converge.md`, `validate.md` (R4), `release.md` (R7). Verify: artefatos presentes.
