# Requirements Checklist: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

- [x] Assessment R1 (intake/research/problem/concept/decision `go`) criado.
- [x] `POST /api/v1/tasks` persiste `description` e `assignee_id` do request.
- [x] `ApiClient` preserva status/corpo de erro em erro tipado e mantém `lockSession()` em 401.
- [x] `AuthState.setOrganization` limpa `isInvalidated`; guard auth/org em `/tasks` alinhado.
- [x] Testes frontend determinísticos atualizados (AuthState, interceptor) e passando no modo CI-equivalente.
- [x] `docs/dev-smoke.md` inclui exemplo concreto de override de portas (conflito de host).
- [x] Verificações Rust (`fmt --check`, `check`, `clippy -D warnings`, `test -p flora-core --lib`, `test -p flora-tests --lib`, `test -p flora-api`) e frontend (`check`, `lint`, `test:unit -- --run --project server`, `build`) executadas com evidência.
- [~] Smoke compatível evidenciado ou NV ambiental registrado (NV: `5432` externo no ambiente atual; caminho/fluxo documentado e validado em 010; sem alteração funcional do fluxo de smoke).
- [x] Validação/release SDD finalizados; PR #214 para `dev` com CI remoto 6/6 verde, merge e sync `origin/dev` concluídos (`dev@598f08152`).
