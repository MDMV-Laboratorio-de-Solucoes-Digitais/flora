# Plan: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Assessment:** `.specify/assessments/010-compose-uuidv7-smoke-frontend-verify/`  
**Spec:** `specs/010-compose-uuidv7-smoke-frontend-verify/spec.md`

## Approach
1. **Compose/docs:** atualizar `docker-compose.yml` para `postgres:18-alpine`; revisar `docs/dev-smoke.md` (PG18, volume PG16→18, conflito de portas, consistência de chaves/URLs) sem alterar fluxo lógico.
2. **Smoke:** validar caminho documentado/compatível. Se `5432` indisponível no host, usar projeto compose isolado com portas alternativas (override/args equivalentes) e registrar desvio/NV; executar seed + bootstrap + `test-flora.sh`.
3. **Frontend determinístico:** ajustar `apps/frontend/vite.config.ts` para incluir `tests/**/*.{test,spec}.{js,ts}` no projeto `server`; executar `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build`; corrigir falhas mínimas in-scope.
4. **Verificação Rust:** `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core -p flora-tests`, `cargo test -p flora-core --lib`, `cargo test -p flora-tests --lib`.

## Evidence / artifacts
- Atualizar `research/decision/spec/plan/tasks/checklist`, registrar comandos/resultados/NV, preparar PR para `dev`.
