# Validation: 096 — Autenticação local (fallback de senha) e login pt-BR

**Data**: 2026-11-20 · **Branch**: `feature/096-auth-login-ptbr` @ `1910d1c7d` · **Baseline**: `1910d1c7d`
**Tipo**: correção de autenticação — credential local Argon2id, bootstrap e recuperação por env, endpoint de login por senha com rate limit e tela de login pt-BR. Nenhuma alteração de gate, required check, constituição, ADR, schema além da migration de credencial ou fluxo OIDC existente nesta validação.
**Veredito R4**: **APROVADO**, com gates e verificações **medidos às 2026-11-21T01:32:08Z** (saídas reais coladas abaixo) e registro explícito de **[NV]** para deploy/Coolify/E2E autenticado até o deploy final.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `git diff --check` | **exit=0** medido às 2026-11-21T01:32:08Z — diff limpo contra `1910d1c7d` (FR-001..FR-008, SC-003; T014) |
| `cargo fmt --all -- --check` | **exit=0** medido às 2026-11-21T01:32:08Z — formatação limpa (FR-001..FR-008, SC-003; T014) |
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0`, **10 floors / 63 testes** medido às 2026-11-21T01:32:08Z — único gate autoritativo sob ADR-0003, nenhum required check acrescentado ou relaxado neste ciclo (FR-008, SC-003; T014) |
| `cargo clippy -p flora-core -p flora-api --all-targets -- -D warnings` | **exit=0** medido às 2026-11-21T01:32:08Z — nenhum warning em `flora-core`/`flora-api` (FR-001..FR-006, SC-002, SC-003; T004, T006, T008, T010) |
| `cargo test -p flora-core --lib` | **25 passed**, `exit=0` medido às 2026-11-21T01:32:08Z — cobre migration `users.password_hash`/`users.password_updated_at`, `PasswordService` (Argon2id `m=19456, t=2, p=1`, PHC, dummy hash, `401` genérico) e recuperação por env com `password_hash IS NULL` (FR-001, FR-002, FR-005, SC-002, SC-003; T002, T003, T004, T005, T010, T011) |
| `cargo test -p flora-api --lib` | **36 passed**, `exit=0` medido às 2026-11-21T01:32:08Z — cobre `POST /bootstrap` com `admin_password` gravado na transação e `POST /api/v1/auth/password/login` com rate limit/`429` e nenhuma credencial em log ou resposta (FR-003, FR-004, FR-006, SC-001, SC-002, SC-003; T006, T007, T008, T009) |
| `npm --prefix apps/frontend run check` | **0 errors, 0 warnings**, `exit=0` medido às 2026-11-21T01:32:08Z — frontend "Sign In" local em pt-BR, tratamento de `429` e `app.html` com `lang="pt-BR"` (FR-007, FR-008, SC-002; T012, T013) |
| `cargo deny check bans licenses sources` | **exit=0** medido às 2026-11-21T01:32:08Z — `bans ok, licenses ok, sources ok`; deps `argon2`/`password-hash` pinadas em `Cargo.toml`/`Cargo.lock` (FR-001, FR-002; T001, T014) |
| revisor isolado | **7/7 achados P1/P2 corrigidos** confirmado às 2026-11-21T01:32:08Z — nenhum achado remanescente de enumeração de conta, vazamento de credencial ou rate limit (FR-002, FR-003, FR-006, SC-002, SC-003; T004..T011) |
| `git diff` / `git status` contra `1910d1c7d` | implementação restrita a deps/migration/modelo/repository/`PasswordService`/bootstrap/rotas de auth/config de startup/frontend de login e aos artefatos do ciclo (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, este R4 e o R7), além dos apêndices documentais de memória/governança; nenhum toque em gate, required check, constituição, ADR, fluxo OIDC, schema além da migration ou PR/merge/tag (SC-003; T014) |
| PRs no ciclo | nenhum PR, merge, tag ou deploy realizado por este documento; promoção permanece manual e **[NV]** (SC-003) |

### Saídas coladas

Evidência **medida pelo main agent às 2026-11-21T01:32:08Z** e consolidada aqui, mais o diagnóstico lido diretamente no baseline (esta R4 não reexecutou nada):

```
$ # diagnóstico — leitura direta no baseline 1910d1c7d
$ sed -n '20,33p' flora-core/src/config/oidc.rs
issuer_url  = ZITADEL_ISSUER_URL  || "http://localhost:8080"
client_id   = ZITADEL_CLIENT_ID   || "flora@localhost"
client_secret = ""   allowed_redirect_uris = Vec::new()

$ sed -n '70,140p' flora-api/src/routes/auth.rs
create_auth_router(): /login /callback /logout /refresh /select-organization
\-> GET /login inicia somente fluxo OIDC + PKCE; nenhuma rota de senha

$ grep -n 'password' flora-core/src/models/user.rs flora-api/src/routes/bootstrap.rs
Usuario bootstrap: User::new(email,"Admin"), oidc_subject=None, sem coluna de credencial
\-> conta luis.granja@mdmv.com.br nao consegue reautenticar

$ sed -n '1,140p' 'apps/frontend/src/routes/(auth)/+page.svelte'
handleLogin() ignora o campo Password e chama GET /api/v1/auth/login (OIDC)
strings: "Welcome to Flora" / "Sign In" / "First Seed Journey" (ingles)

$ # verificacoes reais medidas pelo main agent — 2026-11-21T01:32:08Z
$ git diff --check
exit=0

$ cargo fmt --all -- --check
exit=0

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS  exit=0  (10 floors / 63 testes)

$ cargo clippy -p flora-core -p flora-api --all-targets -- -D warnings
exit=0

$ cargo test -p flora-core --lib
25 passed

$ cargo test -p flora-api --lib
36 passed

$ npm --prefix apps/frontend run check
0 errors  0 warnings

$ cargo deny check bans licenses sources
exit=0  (bans ok, licenses ok, sources ok)

revisor isolado: 7/7 achados P1/P2 corrigidos

PRs no ciclo: 0   merge: 0   tag: 0   deploy executado por esta R4: 0
```

Nota de proveniência: as verificações acima foram **medidas pelo main agent às 2026-11-21T01:32:08Z**, com as saídas coladas literalmente na seção "Saídas coladas"; esta passagem R4 **não reexecutou** gates, build, testes, formatters, git de mutação, CI ao vivo nem o login real em produção. O diagnóstico vem da leitura direta do `oidc.rs`, de `auth.rs`, do `bootstrap.rs`, do modelo `User` e da tela de login no baseline `1910d1c7d`. Nenhuma inferência além dessa evidência.

**Estado final confirmado**: a leitura direta confirma o diagnóstico no baseline — defaults OIDC em `localhost:8080`, allowlist vazia, ausência de rota de senha e de coluna de credencial, e tela de login em inglês com o campo de senha decorativo. Os valores de implementação (`m=19456, t=2, p=1`, nomes de variáveis e o endpoint de senha) seguem `decision.md`/`spec.md`/`plan.md`/`tasks.md`. As verificações locais foram **medidas às 2026-11-21T01:32:08Z**, todas `exit=0`: `git diff --check`, `cargo fmt --all -- --check`, `check_gate_integrity.py` (**PASS**, 10 floors / 63 testes), `cargo clippy -p flora-core -p flora-api --all-targets -- -D warnings`, `cargo test -p flora-core --lib` (**25 passed**), `cargo test -p flora-api --lib` (**36 passed**), `npm --prefix apps/frontend run check` (**0 errors, 0 warnings**) e `cargo deny check bans licenses sources` (`bans ok, licenses ok, sources ok`); revisor isolado confirmou **7/7 achados P1/P2 corrigidos**.

## Fronteira

Contra a base `1910d1c7d`, as mudanças de implementação ficam restritas a dependências, migration de credencial, modelo/repositório de usuário, `PasswordService`, bootstrap, rotas de autenticação, configuração de startup, frontend de login e aos artefatos do próprio ciclo. Nada mais foi alterado pela implementação: sem gate, required check, constituição, ADR, fluxo OIDC, schema além da migration, `STATE.md`, PR, merge, tag ou deploy. `validate.md` (R4) e `release.md` (R7) são registros novos criados nas fases seguintes do mesmo ciclo e não afetam o sistema; por isso T014 os exclui do escopo de mudança de implementação.

## [NV]

- **Login da conta bootstrap até validação final**: a autenticação de `luis.granja@mdmv.com.br` em produção não foi executada; depende da migration aplicada e das variáveis `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` configuradas no ambiente de destino quando `password_hash` estiver nulo.
- **CI/PR/produção**: gates locais **medidos PASS** às 2026-11-21T01:32:08Z (saídas reais na tabela e nas saídas coladas), mas nenhum PR, merge, tag, aprovação de maintainer, proteção de branch ou ambiente de produção foi verificado neste ciclo; gate local não substitui CI remota para promoção.
- **Credenciais**: nenhuma senha real foi fornecida, lida, registrada ou transmitida nesta passagem; a ausência de credencial em log e resposta é requisito SC-003, coberta pelos testes medidos (`flora-core` 25 passed + `flora-api` 36 passed) e pela revisão isolada (7/7 P1/P2), não por observação ao vivo em produção.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram reexecutados ao escrever este R4; as saídas coladas foram medidas pelo main agent às 2026-11-21T01:32:08Z.
- **Deploy/Coolify/E2E autenticado**: configuração de DNS/FQDN, app no Coolify, publicação de imagem, resultado do deploy e o fluxo E2E de login com credencial real permanecem **[NV]** até o deploy final.

## Portarias

R1 `go` com causa raiz determinística, decisão materializada em `.specify/assessments/096-auth-login-ptbr/decision.md` (2026-11-20); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=8/8`, `fr=8/8`, `sc=3/3` e `tasks=14/14` (`specs/096-auth-login-ptbr/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`, com gates **medidos PASS às 2026-11-21T01:32:08Z** (`git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `check_gate_integrity.py` PASS 10 floors/63 testes, `cargo clippy ... -D warnings` exit=0, `cargo test` 25 + 36 passed, `npm run check` 0 errors/0 warnings, `cargo deny check bans licenses sources` ok, revisor isolado 7/7 P1/P2 corrigidos) e **[NV]** explícito para deploy/Coolify/E2E autenticado até o deploy final; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com bump MINOR `0.5.0 → 0.6.0`, changelog FR-001..FR-008 / T001..T014 e rollback = reapontar o app/web, reverter a migration e a config se necessário. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos documentais do ciclo, não comandos executados.
