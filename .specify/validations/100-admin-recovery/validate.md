# Validation: 100 — Recuperação da senha do administrador real via opt-in explícito `*`

**Data**: 2026-11-21 · **Branch**: `feature/100-admin-recovery` @ `9cf6775a7` · **Baseline**: `9cf6775a7`
**Tipo**: correção de recuperação de credencial — opt-in explícito por `FLORA_BOOTSTRAP__ADMIN_EMAIL=*`, seleção do primeiro usuário ativo (`is_active = TRUE`, `ORDER BY created_at ASC, id ASC`, `LIMIT 1`), sobrescrita idempotente do hash, evento `Startup administrator password recovery` no target `flora_api` sem segredo e contratos wildcard/e-mail exato/e-mail ausente. **Nenhuma alteração de gate, required check, constituição, ADR, contratos de API, valores de status, schema, PR/merge/tag, `STATE.md` ou `PROGRESSO*` nesta validação.**
**Veredito R4**: **APROVADO**, com gates e verificações **medidos às 2026-11-21T04:27:20Z–04:33Z** (saídas reais coladas abaixo) e registro explícito de **[NV]** para CI remota, Coolify/Forgejo/DNS, deploy, login em produção e remoção das env temporárias **até o deploy final**. Esta R4 **não reexecutou** gates, build, testes, formatters nem comandos git de mutação; as saídas foram medidas pelo main agent e consolidadas aqui.

## Gates e verificações

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0`, **71 floors** (bootstrap 13), medido às `2026-11-21T04:27:20Z–04:33Z` — único gate autoritativo sob ADR-0003, nenhum required check acrescentado ou relaxado neste ciclo (FR-005, SC-003; T008) |
| `git diff --check` | **exit=0**, medido às `2026-11-21T04:27:20Z–04:33Z` — diff limpo contra `9cf6775a7`; mudanças restritas a `flora-core/src/config/bootstrap.rs`, `flora-tests/src/contract/bootstrap.rs`, `.env.example`, artefatos do ciclo 100 e apêndices de memória/governança (FR-005, SC-003; T008) |
| `cargo fmt --all -- --check` / `cargo clippy --workspace --all-targets -- -D warnings` | **exit=0 / exit=0**, medidos às `2026-11-21T04:27:20Z–04:33Z` — formatação limpa e nenhum warning no workspace (FR-001..FR-005, SC-003; T008) |
| `cargo test -p flora-tests --lib` e testes DB de bootstrap (Postgres) | **36 passed / 0 failed** e **13 testes DB de bootstrap passaram**, medidos às `2026-11-21T04:27:20Z–04:33Z`, incluindo wildcard, exact email, missing email no-op e no active user; contratos em `bootstrap.rs`/`flora-tests/src/contract/bootstrap.rs`: `:340`, `:409`, `:485`, `:522`, `:569`, além dos testes unitários de `BootstrapConfig` (`:179-271`). Execução em produção/DB real permanece **[NV]** até o deploy (FR-003, FR-005, SC-001, SC-002; T006, T008) |
| `npm --prefix apps/frontend run check` / `npm run lint` | **0 errors / sem issues**, medidos às `2026-11-21T04:27:20Z–04:33Z` — nenhuma alteração de frontend neste ciclo, login pt-BR do ciclo 099 preservado (SC-001; T008) |
| seletor wildcard (`is_wildcard_selector()`) e ausência de fallback | **confirmado por leitura direta** em `flora-core/src/config/bootstrap.rs:56-59,69-91,129-143`: só o token exato `*` troca a busca por e-mail pela seleção do primeiro ativo; qualquer outro valor segue `repo.find_by_email(&email)` com warn e `Ok(false)`. Sem usuário ativo, há aviso próprio e `Ok(false)` (`:145-154`) (FR-001, FR-002, SC-002; T002, T003, T006) |
| escrita e observabilidade | **confirmado por leitura direta** em `bootstrap.rs:158-171` (`UPDATE users … WHERE id = $1`, `rows_affected()`, `user_id` no evento) e `flora-api/src/state/app_state.rs:93-99` (`applied` no target `flora_api`); nenhuma senha ou hash em log/resposta (FR-003, FR-004, SC-002, SC-003; T004, T005) |
| `.env.example` | **confirmado por leitura do diff**: e-mail exato como caminho preferido, `*` documentado como recuperação explícita e temporária do primeiro usuário ativo, com instrução de remover ambas as env e reiniciar; nenhum valor real de senha versionado (FR-005, SC-002; T007) |
| `git diff` / `git status` contra `9cf6775a7` | mudanças restritas aos arquivos de implementação do ciclo e aos artefatos do próprio ciclo (`.specify/assessments/100-admin-recovery/`, `docs/governance/2026-11-21-100-admin-recovery.md`, `specs/100-admin-recovery/`, este R4 e o R7), além dos apêndices de `STATE.md`/`PROGRESSO.md`; nenhum toque em gate, required check, constituição, ADR, contratos de API, valores de status, schema, `validate/` pré-existente, `release/` ou PR/merge/tag (FR-005, SC-003; T001, T008) |
| PRs no ciclo | nenhum PR, merge, tag ou deploy realizado por esta R4; promoção permanece manual e **[NV]** (SC-003) |

Nota de proveniência: todas as verificações da tabela foram **medidas pelo main agent às 2026-11-21T04:27:20Z–04:33Z**, com as saídas reais coladas no bloco abaixo; esta passagem R4 **não reexecutou** gates, build, testes, formatters, comandos git de mutação, CI ao vivo, navegador real nem ambiente de produção. Nenhuma inferência além dessa evidência.

## Saídas coladas

Evidência **medida pelo main agent às 2026-11-21T04:27:20Z–04:33Z** e consolidada aqui, mais o diagnóstico lido diretamente no baseline (esta R4 não reexecutou nada):

```
$ # verificações reais medidas pelo main agent — 2026-11-21T04:27:20Z–04:33Z
$ git diff --check
exit=0

$ cargo fmt --all -- --check
exit=0

$ cargo clippy --workspace --all-targets -- -D warnings
exit=0

$ cargo test -p flora-tests --lib
36 passed   0 failed

$ npm --prefix apps/frontend run check
0 errors

$ npm run lint        # prettier + eslint
sem issues

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS   exit=0   floors=71   bootstrap=13

$ # 13 testes DB de bootstrap (Postgres)
13 passed   (inclui wildcard, exact email, missing email no-op e no active user)

$ # diagnóstico — leitura direta no baseline 9cf6775a7
$ sed -n '111,115p' flora-core/src/config/bootstrap.rs
repo.find_by_email(&email).await?
tracing::warn!("FLORA_BOOTSTRAP__ADMIN_EMAIL does not match an existing user; startup password skipped")
return Ok(false)
-> producao mediu applied=false: e-mail configurado ausente do DB (causa refinada do ciclo 098)

$ sed -n '93,99p' flora-api/src/state/app_state.rs
let applied = apply_startup_password(...).await?;
tracing::info!(applied = applied, "Startup administrator password recovery");
-> target flora_api, nao suprimido pelo filtro de producao

$ # implementacao entregue — leitura direta do diff
$ sed -n '56,59p;69,91p;129,171p' flora-core/src/config/bootstrap.rs
fn is_wildcard_selector(&self) -> bool { ... }        # exatamente '*' apos trim
if config.is_wildcard_selector() { SELECT ... WHERE is_active = TRUE
                                  ORDER BY created_at ASC, id ASC LIMIT 1 }
else { repo.find_by_email(&email) }                   # nenhum fallback
warn: "matched no active user" | "does not match an existing user"
UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1
tracing::info!(user_id = %user.id, "Startup administrator password overwritten")
-> opt-in exato; nenhum fallback para outro valor; nenhuma senha/hash/e-mail real em log

$ # contratos entregues
$ grep -n 'fn db_startup_password' flora-tests/src/contract/bootstrap.rs
340: db_startup_password_overwrites_existing_hash
409: db_startup_password_wildcard_targets_first_active_user
485: db_startup_password_exact_email_still_applies
522: db_startup_password_missing_email_is_noop
569: db_startup_password_wildcard_without_active_user_is_noop
$ # testes unitarios de BootstrapConfig: wildcard, typo, senha invalida, redacao (bootstrap.rs:179-271)

$ # .env.example — diff
+ # Explicit, TEMPORARY recovery when the address is unknown: set ADMIN_EMAIL to
+ # exactly `*` to apply the password to the FIRST ACTIVE user (oldest by
+ # created_at). There is no admin-role filter: confirm which account received
+ # the password, then REMOVE both variables and restart immediately.

$ # fronteira
flora-core/src/config/bootstrap.rs, flora-tests/src/contract/bootstrap.rs, .env.example
+ artefatos do ciclo 100; nenhum gate/workflow/ADR/API/schema/PR/merge/tag tocado

PRs no ciclo: 0   merge: 0   tag: 0   deploy executado por esta R4: 0
CI remota / deploy / login em producao / remocao das env: [NV] ate o deploy final
```

**Estado final confirmado**: a leitura direta confirma o diagnóstico — `applied=false` com warn de e-mail inexistente, sem escrita — e a implementação entregue: `*` é opt-in exato para o primeiro usuário ativo, e-mail exato e e-mail ausente mantêm o comportamento anterior, a escrita é restrita a um `user_id` e o evento permanece observável em `flora_api` sem segredo. A decisão e os contratos seguem `intake`/`research`/`define`/`shape`/`decision`, `spec.md`, `plan.md` e `tasks.md`. O desfecho em produção (qual conta foi alvo, login com a senha recuperada e remoção das env) permanece **[NV]** até o deploy final.

## Fronteira

Contra a base `9cf6775a7`, as mudanças de implementação ficam restritas a `flora-core/src/config/bootstrap.rs`, `flora-tests/src/contract/bootstrap.rs` e `.env.example`, além dos artefatos do próprio ciclo e dos apêndices de memória/governança. Nada mais foi alterado pela implementação: sem gate, required check, constituição, ADR, contratos de API, valores de status `TODO`/`IN_PROGRESS`/`DONE`, schema, `STATE.md` histórico, `PROGRESSO*`, OIDC/allowlist, `POST /bootstrap`, advisory lock, rate limit, PR, merge, tag ou deploy. `validate.md` (R4) e `release.md` (R7) são registros novos criados nas fases seguintes do mesmo ciclo e não afetam o sistema; por isso T008 os exclui do escopo de mudança de implementação.

## [NV]

- **Login em produção e DB real**: o `available=false` e o `applied=false` são medições do relato/governança; a seleção da conta administradora real por `*`, a escrita do hash no DB de produção e o login com a senha recuperada não foram executados — **[NV]** até o deploy final (SC-001).
- **Conta alvo**: o wildcard não filtra por role; com vários usuários ativos, seleciona o mais antigo por `created_at`. Qual conta recebeu a senha e se é a administradora esperada só pode ser confirmado no ambiente real — **[NV]** (SC-001, SC-002).
- **Logs em produção**: o evento em `flora_api` e o warn de e-mail ausente foram projetados para serem observáveis; a medição nos logs com o filtro atual de produção permanece **[NV]** até o deploy (SC-002).
- **Env temporárias**: a remoção de `FLORA_BOOTSTRAP__ADMIN_EMAIL=*`/`ADMIN_PASSWORD` no Coolify após o login bem-sucedido é passo operacional do deploy, não executado aqui — **[NV]** até o deploy final (SC-002, SC-003).
- **CI/PR/produção**: verificações locais **medidas PASS às 2026-11-21T04:27:20Z–04:33Z** (saídas reais na tabela e no bloco acima), mas nenhuma CI remota, PR, merge, tag, aprovação de maintainer, proteção de branch ou ambiente de produção foi verificado neste ciclo; gate local não substitui CI remota para promoção (SC-003).
- **Coolify/Forgejo/DNS**: publicação de imagem, reapontamento da API, resultado do deploy e E2E autenticado em navegador real não foram observados — **[NV]** até o deploy final (SC-001, SC-003).
- **Credenciais**: nenhuma senha real foi fornecida, lida, registrada ou transmitida nesta passagem; a ausência de credencial em log, resposta e artefato é requisito SC-003, coberta por contratos e redação, não por observação ao vivo.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram reexecutados ao escrever este R4; as saídas foram medidas pelo main agent às 2026-11-21T04:27:20Z–04:33Z.

## Portarias

R1 `go` com diagnóstico determinístico, decisão materializada em `.specify/assessments/100-admin-recovery/{intake,research,define,shape,decision}.md` e em `docs/governance/2026-11-21-100-admin-recovery.md` (2026-11-21, append-only); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `sc=3/3` e `tasks=8/8` (`specs/100-admin-recovery/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`, com gates e verificações **medidos PASS às 2026-11-21T04:27:20Z–04:33Z** (`git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `cargo clippy --workspace --all-targets -- -D warnings` exit=0, `cargo test -p flora-tests --lib` 36 passed/0 failed, `npm --prefix apps/frontend run check` 0 errors, `npm run lint` sem issues, `check_gate_integrity.py` PASS exit=0 com 71 floors/bootstrap 13 e 13 testes DB de bootstrap) e **[NV]** explícito para CI remota, Coolify/Forgejo/DNS, deploy, login em produção e remoção das env temporárias até o deploy final; R5/R6 sem alterações de gate/required check — nenhum check novo adicionado ou relaxado sob ADR-0003; R7 registro de release PATCH `0.7.1` → `0.7.2` com rollback (remover `*` da env e reapontar, sem force-push), publicação/promoção manuais e **[NV]** até o deploy final.
