# Validation: 012 — Session Authority Revalidation & Contract Confidence (80/20)

**Date:** 2026-11-16  
**Base:** `dev@598f08152` (cycle 011 merged)  
**Verdict:** **Approved (local deterministic + DB-backed evidence green; remote CI/merge pending as NV)**

## Deterministic checks (local)

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo clippy -p flora-api -p flora-core -p flora-tests --all-targets -- -D warnings` | pass |
| `cargo check -p flora-api -p flora-core -p flora-tests` | pass |
| `cargo test -p flora-core --lib` | pass (8 passed) |
| `cargo test -p flora-tests --lib` | pass (55 passed, 12 ignored) |
| `cargo test -p flora-api` | pass (7 passed, 1 ignored) |
| `FLORA_DATABASE__POSTGRES_URL=postgresql://postgres:postgres@127.0.0.1:15434/flora cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` | **pass (12 passed: 5 select-org + 7 session authority)** |
| `cd apps/frontend && bun run check` | pass (0 errors, 0 warnings) |
| `cd apps/frontend && bun run lint` | pass |
| `cd apps/frontend && bun run test:unit -- --run --project server` | pass (7 files, 17 tests) |
| `cd apps/frontend && bun run build` | pass |

## DB environment

- PostgreSQL **18.7** (Debian) descartável via `postgres:18`, host `127.0.0.1:15434`, banco `flora`; migrações aplicadas por `run_migrations`.
- Porta alternativa porque `127.0.0.1:5432` está ocupada por processo externo não-Flora neste ambiente (**NV ambiental** para o caminho padrão).
- Testes cobrem: sessão válida (200), revogada (401), expirada (401), ausente (401), user mismatch (401), org mismatch (401), nil-org passa a autenticação e é barrada por `require_org` (422).

## Changes validated

- `flora-api/src/extractors/auth.rs`: `authenticate_bearer` (JWT + sessão `jti`), usado por `AuthExtractor`, `UserContext`, `OrgContext`; mensagens genéricas; log detalhado; teste de assinatura inválida sem DB.
- `apps/frontend/src/lib/api/client.ts`: parse do corpo antes do 401; `apiErrorFromResponse` preservando `code`/`field`/`message`/`body`; fallback `unauthorized`.
- `apps/frontend/tests/integration/api-interceptor.test.ts`: 401 com corpo JSON e fallback sem corpo.
- `flora-tests/src/contract/auth.rs`: módulo `db_session_authority` (7 casos) + remoção do teste não-DB obsoleto de nil-org; imports/ajustes de lint.
- `.github/workflows/rust.yml`: `db-tests` agora executa `contract::auth::db_`.
- `OBJETIVO.md`/`ROADMAP.md`: ciclo 011 registrado; ciclo 012 definido.
- `docs/research/2026-11-16-012-session-authority-international.md` e `docs/research/2026-11-15-012-frontend-api-contract-audit.md`.

## NV / limitations

- Remote CI required checks e merge/sync: **NV** até a execução do PR.
- Smoke integrado `select org → workspace → task`: não re-executado neste ciclo; fluxo inalterado, evidenciado em 010 (PG18, alt-port) e DB tests rodam no mesmo PG18 descartável.
- WebSocket `/ws`: ainda valida JWT sem autoridade de sessão (follow-up documentado).
- Latência sob carga com lookup por request: não medida; sem cache neste ciclo.
- OIDC/JWKS e2e real: NV.
