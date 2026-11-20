# Requirements Checklist: 012 — Session Authority Revalidation & Contract Confidence (80/20)

- [x] Assessment R1 (intake/research/problem/concept/decision `go`) criado.
- [x] Helper de autenticação valida `jti` no PostgreSQL (ativa, não expirada, user/org coerentes).
- [x] `AuthExtractor`/`UserContext`/`OrgContext` usam o helper; 401 genérico e 500 para falha de banco.
- [x] `ApiClient` preserva status/corpo/código/campo/mensagem em 401 e mantém `lockSession()`.
- [x] Testes DB-backed: válida, revogada, expirada, ausente, user mismatch, org mismatch, nil-org.
- [x] Job CI `db-tests` executa `contract::auth::db_` com Postgres 18.
- [x] Verificações Rust (`fmt --check`, `check`, `clippy -D warnings`, testes) e frontend (`check`, `lint`, `test:unit -- --run --project server`, `build`) verdes.
- [x] DB tests executados em Postgres 18 descartável (12 passed, porta alternativa; NV porta padrão).
- [x] Validação/release SDD finalizados; PR para `dev` com CI verde, merge e sync `origin/dev`.
