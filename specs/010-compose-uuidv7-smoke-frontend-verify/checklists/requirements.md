# Requirements Checklist: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

- [x] Assessment 010 com intake/research/decision `go`.
- [x] `docker-compose.yml` alinhado para PostgreSQL 18 (`uuidv7` nativo) sem edição de migração aplicada.
- [x] `docs/dev-smoke.md` atualizado (requisito PG18, notas de volume/porta, consistência operacional).
- [x] Smoke integrado `select org → workspace → task` executado com evidência (`smoke_rc=0`, PG18 stack; alt-port override por conflito de `5432` registrado como NV ambiental).
- [x] Verificações determinísticas frontend executadas/registradas (`check`, `lint`, `test:unit -- --run --project server`, `build`) com ajustes mínimos in-scope.
- [x] Verificações Rust determinísticas (fmt/check/tests lib) executadas localmente conforme aplicável.
- [~] Artefatos SDD 010 finalizados (validação/release criados) — PR/CI/merge/sync pendentes.
