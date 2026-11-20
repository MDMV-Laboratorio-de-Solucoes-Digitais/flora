# Validation: 085 — First Seed Journey local one-time bootstrap

**Data**: 2026-11-20T03:17Z · **Branch**: `feature/085-local-first-seed` · **Baseline**: `38184fa50`
**Tipo**: nova rota pública one-time (`POST /api/v1/bootstrap`), transação atômica de provisionamento, emissão de sessão e ligação do botão "Sprout Workspace"; superfície não autenticada mitigada por guarda transacional, validação, rate limit e auditoria.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` (raiz) | **gate-integrity: PASS**, `exit=0` — `ok job frontend-build present and non-vacuous`, `ok job rust-build present and non-vacuous`, `ok job db-tests present and non-vacuous`, `ok job dependency-review present and non-vacuous`, `ok rust.yml contains 'cargo build --workspace'`, `ok rust.yml contains 'cargo test --workspace'`, `ok all 8 db contract floors intact (51 total)`; `gh` → `[]` (FR-006, SC-004) |
| `cargo fmt --all` + Prettier nos 2 fontes frontend | `fmt_exit=0`; `src/routes/(auth)/+page.svelte (unchanged)` e `src/routes/(auth)/login-error.svelte.spec.ts` formatados; `npm run lint` → `All matched files use Prettier code style!`, `lint_exit=0` (SC-004) |
| `cargo clippy --workspace --all-targets -- -D warnings` | `clippy_exit=0` — `Checking flora-api v0.1.0`, `Checking flora-tests v0.1.0`, `Finished \`dev\` profile [unoptimized] target(s) in 11.94s`; o `#[must_use]` redundante apontado na 1ª execução foi removido (SC-004) |
| `cargo test -p flora-tests db_bootstrap -- --nocapture` (DB `flora_demo_085`) | **4 contratos `db_bootstrap` PASS**: `db_bootstrap_concurrent_calls_create_single_organization ... ok`, `db_bootstrap_creates_org_admin_role_membership_workspace_and_session ... ok`, `db_bootstrap_rejects_second_call_without_writes ... ok`, `db_bootstrap_rejects_when_organization_exists ... ok`; `test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 88 filtered out; finished in 4.84s`; `contract_exit=0` (FR-003, FR-005, FR-006, SC-002, SC-003, SC-004) |
| `bunx vitest run --project client 'src/routes/(auth)/login-error.svelte.spec.ts'` | `Test Files 1 passed (1)` · `Tests 4 passed (4)` · `vitest_exit=0` — cobre OIDC 503/429, bootstrap 201 e bootstrap 409, com submissão duplicada (FR-008, FR-009, FR-010, FR-011, SC-004) |
| `npm run check` (svelte-check) | `1795144299942 COMPLETED 4760 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS`; `check_exit=0` (FR-007, SC-005) |
| `cargo test --workspace` (suíte completa) | `cargo test: 105 passed, 59 ignored (13 suites, 0.08s)` — sem regressão no workspace (SC-004, SC-005) |
| E2E real contra API em `flora_demo_085_e2e` | `health → 200`; `POST /api/v1/bootstrap` → **HTTP:201** com `{session_token, expires_in:3600, user_id, organization_id, workspace_id}`; segunda chamada → **HTTP:409** `{"error":"conflict","message":"duplicate entry: bootstrap already completed"}`; `GET /api/v1/tasks` com o token → **HTTP:200**. Log do servidor confirma `status=201`, `status=409` e `status=200` (FR-001, FR-004, FR-005, SC-001, SC-002) |
| Verificador isolado `BootstrapCodeVerifier` | Encontrou 3 findings (rota com prefixo duplicado, testes mascarando a topologia de produção e contrato JWT divergente); **todos fechados** antes deste R4 — ver *Nits fechados* |
| Cobertura documental | `specs/085-local-first-seed/spec.md` (FR-001..FR-012, SC-001..SC-005), `plan.md`, `contracts/bootstrap.md`, `data-model.md`, `quickstart.md`, `tasks.md` (T001..T023) e `checklists/requirements.md` + `checklists/security.md` presentes na branch (SC-004) |

### Saídas coladas

Gate autoritativo (2026-11-20T02:58:37Z):

```
=== GATE AUTORITATIVO 2026-11-20T02:58:37Z ===
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 8 db contract floors intact (51 total)
exit=0
=== worktree 073 status ===
worktree_exit=0
=== PRs gh ===
[]
gh_exit=0
=== main ancestry ===
main_ancestor_exit=0
```

Union final de gates (2026-11-20T03:14:22Z):

```
=== fmt 2026-11-20T03:14:22Z ===
fmt_exit=0
=== clippy ===
    Checking flora-api v0.1.0 (/home/luis/development/MDMV/projetos/flora/flora-api)
    Checking flora-tests v0.1.0 (/home/luis/development/MDMV/projetos/flora/flora-tests)
    Finished `dev` profile [unoptimized] target(s) in 11.94s
clippy_exit=0
=== contratos ===
   Compiling flora-api v0.1.0 (/home/luis/development/MDMV/projetos/flora/flora-api)
   Compiling flora-tests v0.1.0 (/home/luis/development/MDMV/projetos/flora/flora-tests)
    Finished `test` profile [unoptimized] target(s) in 23.68s
     Running unittests src/lib.rs (target/debug/deps/flora_tests-916281a641960093)

running 4 tests
test contract::bootstrap::tests::db_bootstrap_concurrent_calls_create_single_organization ... ok
test contract::bootstrap::tests::db_bootstrap_rejects_second_call_without_writes ... ok
test contract::bootstrap::tests::db_bootstrap_creates_org_admin_role_membership_workspace_and_session ... ok
test contract::bootstrap::tests::db_bootstrap_rejects_when_organization_exists ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 88 filtered out; finished in 4.84s

contract_exit=0
=== vitest ===
 RUN  v4.1.11 /home/luis/development/MDMV/projetos/flora/apps/frontend
 Test Files  1 passed (1)
      Tests  4 passed (4)
   Start at  23:15:13
   Duration  6.45s (transform 0ms, setup 0ms, import 1.92s, tests 578ms, environment 0ms)

vitest_exit=0
```

Frontend `check`/`lint` (gates em paralelo, 2026-11-20T03:11Z):

```
=== frontend check ===
> frontend@0.0.1 check
> svelte-kit sync && svelte-check --tsconfig ./tsconfig.json
1795144299863 START "/home/luis/development/MDMV/projetos/flora/apps/frontend"
1795144299942 COMPLETED 4760 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
check_exit=0
=== frontend lint ===
> frontend@0.0.1 lint
> prettier --check . && eslint .
Checking formatting...
All matched files use Prettier code style!
lint_exit=0
```

E2E real (2026-11-20T03:16:14Z–03:16:15Z; token truncado no `session_token` — valor completo apenas na resposta viva do DB descartável):

```
=== health ===
200
=== bootstrap 1 ===
{"session_token":"eyJ0eXAiOiJKV1QiLCJhbGci...","expires_in":3600,"user_id":"01a1f6f4-6725-7e01-8e11-5abf3fa786e6","organization_id":"01a1f6f4-6725-7e01-8e11-5aae3dc4769d","workspace_id":"01a1f6f4-6725-7e01-8e11-5adeb87943d3"}
HTTP:201
=== bootstrap 2 ===
{"error":"conflict","message":"duplicate entry: bootstrap already completed"}
HTTP:409
=== tasks com token ===
HTTP:200
=== token prefixo ===
eyJ0eXAiOiJKV1QiLCJhbGci...
```

Log do servidor de teste (trecho literal):

```
2026-11-20T03:16:14.817922Z  INFO request{method=POST uri=/api/v1/bootstrap version=HTTP/1.1}: flora_api::routes::bootstrap: Bootstrap completed: first organization and administrator created organization_id=01a1f6f4-6725-7e01-8e11-5aae3dc4769d user_id=01a1f6f4-6725-7e01-8e11-5abf3fa786e6
2026-11-20T03:16:14.818715Z DEBUG request{method=POST uri=/api/v1/bootstrap version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=64 ms status=201
2026-11-20T03:16:14.936129Z DEBUG request{method=POST uri=/api/v1/bootstrap version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=12 ms status=409
2026-11-20T03:16:15.026979Z DEBUG request{method=GET uri=/api/v1/tasks version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=20 ms status=200
```

### Nits fechados

- **CRITICAL — rota inalcançável em produção**: `flora-api/src/routes/mod.rs:32` registrava `.nest("/api/v1/bootstrap", bootstrap::create_bootstrap_router())`, produzindo `POST /api/v1/bootstrap/bootstrap` porque o router interno já expõe `/bootstrap`. Corrigido para `.nest("/api/v1", bootstrap::create_bootstrap_router())` — o mesmo padrão de `auth` —, alinhando contrato, quickstart, frontend e servidor real. Confirmado pelo E2E `HTTP:201` em `/api/v1/bootstrap` (FR-001, FR-008, SC-001). **Fechado**.
- **MEDIUM — topologia dos testes de contrato**: o helper montava `.nest("/api/v1", create_bootstrap_router())` enquanto a produção usava o prefixo duplicado; com a correção do `mod.rs`, teste e produção passaram a exercitar o mesmo caminho `uri("/api/v1/bootstrap")`. Confirmado pelos 4 contratos PASS e pelo E2E real (SC-004). **Fechado**.
- **LOW — contrato JWT divergente**: `specs/085-local-first-seed/contracts/bootstrap.md:32` dizia `sub=jti`; a implementação e `flora-core/src/utils/jwt.rs` usam `sub=user_id`, `jti=<id da sessão>`. Contrato ajustado para `sub=user_id, jti=<id da sessao>, organization_id e email`; o JWT real decodificado no E2E contém `sub`/`jti`/`organization_id` coerentes (FR-004). **Fechado**.
- **Lint/compilação do workspace**: 7 `unused result of type PgQueryResult` em `bootstrap.rs`, 2 resultados não usados no teste e 25 `.unwrap()`/`panic!` proibidos por `-D clippy::all` foram corrigidos (`anyhow::Result` + `?`, `.ok_or_else(...)`, `anyhow::bail!`), além de um `#[must_use]` redundante removido. `cargo fmt`, `cargo clippy -D warnings` e os 4 contratos estão verdes na execução final (SC-004). **Fechado**.

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git nem inspeção visual. As saídas acima são a evidência literal consolidada do handoff do main agent (gates entre 2026-11-20T02:58Z e 03:16Z) e da revisão do verificador isolado `BootstrapCodeVerifier`, com os findings fechados antes deste documento. Nenhuma inferência além dela.

## Fronteira

Mudanças de código contra a base `38184fa50`: `flora-api/src/routes/bootstrap.rs` (novo), `flora-api/src/routes/mod.rs` (`.nest("/api/v1", ...)` + `pub mod bootstrap`), `flora-api/src/middleware/rate_limit.rs` (classe `Bootstrap`, 5/60 s, chave `bootstrap:{ip}`), `flora-tests/src/contract/bootstrap.rs` (novo, 4 contratos), `flora-tests/src/contract/mod.rs` (registro do módulo), `apps/frontend/src/routes/(auth)/+page.svelte` (`handleBootstrap`, botão "Sprout Workspace") e `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts` (novo, 4 testes). Mais os artefatos do ciclo 085 em `specs/085-local-first-seed/`, `.specify/assessments/085-local-first-seed/`, `docs/` e este R4/R7. O fluxo OIDC/"Sign In", a migration `system`, a constituição, o gate/scripts CI, workflows, ADRs alheias, PRs, produção e DBs fora de `flora_demo_085*`/`flora_demo_085_e2e` não foram alterados.

## [NV]

- **Merge/produção**: zero PRs no snapshot; CI ao vivo, Forgejo, proteção de branch, aprovações, merge e deploy não verificados neste ciclo.
- **Verificação de e-mail**: FR-012 aceita explicitamente a ausência; o bootstrap prova apenas a posse do campo informado, não do e-mail — não usar como mecanismo de autenticação em instalação estabelecida.
- **Concorrência em produção**: coberta por contrato local (`pg_advisory_xact_lock`) e por uma execução real sequencial; não há teste de carga em ambiente de produção.
- **UI visual em browser**: o E2E real foi via HTTP com token (`201/409/200`); não houve captura de tela nem clique manual no navegador para conferir renderização/toast.
- **Reexecução nesta passagem**: gates/formatters/git não foram reexecutados neste documento de validação; cobertura determinística atribuída ao handoff do main agent.
- **DBs descartáveis**: `flora_demo_085` e `flora_demo_085_e2e` existem apenas para validação; não representam estado de produção.

## Portarias

R1 `go` antes da implementação (`.specify/assessments/085-local-first-seed/decision.md`, 2026-11-20T01:14Z); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer minor `0.3.5 → 0.4.0`, changelog FR-001..FR-012 / SC-001..SC-005 / T001..T023 e rollback dos commits do ciclo com remoção da rota pública. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos escritos diretamente. Nenhuma tag, deploy ou merge executados; PR a critério do maintainer.
