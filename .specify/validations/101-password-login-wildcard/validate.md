# Validation: 101 — Wildcard de login local com opt-in explícito em `POST /api/v1/auth/password/login`

**Data**: 2026-11-21 · **Branch**: `feature/101-password-login-wildcard` @ `5890471be` · **Baseline**: `5890471be`
**Tipo**: correção de autenticação — `email='*'` (exatamente `*`, após `trim()`) aceito em `POST /api/v1/auth/password/login` somente quando `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`config.bootstrap.admin_email` também for `*`; nesse modo a rota resolve o primeiro usuário ativo (`is_active = TRUE`, `ORDER BY created_at ASC, id ASC`, `LIMIT 1`, sem filtro de role/sistema) e valida a senha com `verify_or_dummy`, preservando o 401 genérico e a sessão em formato OIDC. **Nenhuma alteração de constituição, ADR, contratos de API, valores de status, schema, PR/merge/tag, `STATE.md` ou `PROGRESSO*` nesta validação.** O script do gate, o workflow e `AGENTS.md` foram atualizados pelo main agent para refletir os 77 floors e o login wildcard; nenhum required check foi acrescentado ou relaxado sob ADR-0003.
**Veredito R4**: **APROVADO**, com gates e verificações **medidos pelo main agent em 2026-11-21** (saídas reais coladas abaixo: `git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `cargo clippy --workspace --all-targets -- -D warnings` exit=0, `cargo test -p flora-tests --lib` 36 passed/0 failed e `check_gate_integrity.py` PASS exit=0 com 77 floors — auth 23, bootstrap 13) e registro explícito de **[NV]** até o deploy final para testes DB locais (não concluídos por timeout/encerramento do Postgres temporário — **não é falha**; CI/db-tests é a verdade remota), CI remota, Coolify/Forgejo/DNS, aplicação da mudança, login wildcard com a credencial de recuperação, remoção das env temporárias e medição em navegador real. Esta R4 **não reexecutou** gates, build, testes, formatters nem comandos git de mutação; as saídas foram medidas pelo main agent e consolidadas aqui.

## Gates e verificações

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | **PASS**, `exit=0`, **77 floors** (auth 23, bootstrap 13), medido pelo main agent em 2026-11-21 após atualização do script/workflow e de `AGENTS.md` com os floors e o login wildcard; único gate autoritativo sob ADR-0003, nenhum required check acrescentado ou relaxado (FR-004, SC-003; T008) |
| `git diff --check` | **exit=0**, medido pelo main agent em 2026-11-21; diff limpo, mudanças restritas a `flora-api/src/routes/auth.rs` (inclusive a remoção da qualificação redundante em `:265`), `flora-tests/src/contract/auth.rs`, `.env.example`, script/workflow do gate, `AGENTS.md` e artefatos do próprio ciclo 101 (FR-004, SC-003; T008) |
| `cargo fmt --all -- --check` / `cargo clippy --workspace --all-targets -- -D warnings` | **exit=0 / exit=0**, medidos pelo main agent em 2026-11-21 — formatação limpa e nenhum warning no workspace após remover a qualificação redundante em `flora-api/src/routes/auth.rs:265` (FR-001..FR-004, SC-003; T008) |
| `cargo test -p flora-tests --lib` e contratos `mod db_password_login` | **36 passed / 0 failed**, medido pelo main agent em 2026-11-21. Contratos em `flora-tests/src/contract/auth.rs`: opt-in `*` → 200/sessão do primeiro ativo, sem opt-in → 401, variantes → 401, senha errada com `*` → 401 idêntico a desconhecido, nenhum ativo → 401, formato de sessão preservado e nenhum segredo em log/resposta. Os testes DB locais **não foram concluídos por timeout/encerramento do Postgres temporário — registrado como não verificado localmente, não como falha**; CI/db-tests é a verdade remota e o DB de produção permanece **[NV]** até o deploy (FR-004, SC-001, SC-002, SC-003; T006, T008) |
| `npm --prefix apps/frontend run check` / `npm run lint` | **Não executados por esta R4**; nenhuma alteração de frontend neste ciclo, login pt-BR do ciclo 099 preservado — **[NV]** até o handoff (SC-001; T008) |
| seletor wildcard (`is_wildcard_selector()`) e ausência de fallback | **Confirmado por leitura direta no baseline** em `flora-core/src/config/bootstrap.rs:56-60,69-91,129-143`: só o token exato `*` após `trim()` troca a busca pelo primeiro usuário ativo; `auth.rs:451-460` mostra o caminho por e-mail que permanece sem opt-in. A implementação na rota é revisitada no handoff; medição ao vivo **[NV]** (FR-001, FR-002, SC-002; T002, T003, T006) |
| 401 genérico e sessão idêntica | **Confirmado por leitura direta no baseline** em `flora-api/src/routes/auth.rs:457-492`: `verify_or_dummy`, `Error::InvalidCredentials`, `Session`, `FloraClaims::new`, `encode_token` e o JSON `session_token`/`expires_in`/`user_id`/`next_step`. Preservação verificada por contratos executados no `cargo test -p flora-tests --lib` (**36 passed/0 failed**); testes DB locais **não verificados por timeout do Postgres temporário — não é falha**, CI/db-tests é a verdade remota (FR-003, SC-001, SC-003; T004, T005) |
| `.env.example` | **Confirmado por leitura do diff medido pelo main agent**: documentação de `email="*"` enquanto `ADMIN_EMAIL=*` estiver ativa e da remoção das duas variáveis com restart após o login, sem valor real (FR-004, SC-003; T007) |
| `git diff` / `git status` contra `5890471be` | **Medido pelo main agent em 2026-11-21**: mudanças restritas aos arquivos de implementação do ciclo (`flora-api/src/routes/auth.rs`, `flora-tests/src/contract/auth.rs`, `.env.example`), ao script do gate/workflow e a `AGENTS.md` (floors e login wildcard), e aos artefatos próprios (`.specify/assessments/101-password-login-wildcard/`, `docs/governance/2026-11-21-101-password-login-wildcard.md`, `specs/101-password-login-wildcard/`, apêndices de `STATE.md`/`PROGRESSO.md`, este R4 e o R7); nenhum toque em constituição, ADR, contratos de API, valores de status, schema, `validate/` pré-existente, `release/` ou PR/merge/tag (FR-004, SC-003; T001, T008) |
| PRs no ciclo | nenhum PR, merge, tag ou deploy realizado por esta R4; promoção permanece manual e **[NV]** (SC-003) |

Nota de proveniência: todas as verificações assinaladas como medidas foram executadas pelo main agent em 2026-11-21, com as saídas reais coladas no bloco abaixo; os testes DB locais não concluídos por timeout/encerramento do Postgres temporário são registrados como **não verificados localmente, não como falha** (CI/db-tests é a verdade remota). Esta R4 não reexecutou gates, build, testes, formatters, comandos git de mutação, CI ao vivo, navegador real nem ambiente de produção. Nenhuma inferência além dessa evidência.

## Saídas coladas

Evidência **medida pelo main agent em 2026-11-21** e consolidada aqui, mais o diagnóstico lido diretamente no baseline (esta R4 não reexecutou nada):

```
$ # diagnóstico — leitura direta no baseline 5890471be
$ sed -n '93,99p' flora-api/src/state/app_state.rs
let applied = apply_startup_password(...).await?;
tracing::info!(applied = applied, "Startup administrator password recovery");
-> producao mediu applied=true: hash gravado no primeiro usuario ativo pelo ciclo 100

$ sed -n '451,460p' flora-api/src/routes/auth.rs
let email = req.email.trim().to_lowercase();
let user = user_repo.find_by_email(&email).await?;
let stored_hash = user.as_ref().and_then(|found| found.password_hash.as_deref());
let password_ok = PasswordService::shared().verify_or_dummy(&req.password, stored_hash);
if !password_ok || user.as_ref().is_none_or(|found| !found.is_active) {
    return Err(Error::InvalidCredentials);
}
-> find_by_email("*") -> None -> 401: e-mail real do bootstrap desconhecido

$ sed -n '56,60p;69,91p;129,143p' flora-core/src/config/bootstrap.rs
fn is_wildcard_selector(&self) -> bool { admin_email.trim() == "*" }   # token exato
validate(): aceita endereco ou '*'; nenhuma chave nova
if config.is_wildcard_selector() { SELECT ... WHERE is_active = TRUE
                                  ORDER BY created_at ASC, id ASC LIMIT 1 }
-> mesma consulta reutilizada pelo wildcard de login

$ sed -n '463,492p' flora-api/src/routes/auth.rs
Session { ... }; FloraClaims::new(...); encode_token(...);
json!({ "session_token": jwt, "expires_in": ttl, "user_id": ..., "next_step": ... })
-> resposta de sessao identica; nenhum segredo em log/resposta

$ # verificacoes reais medidas pelo main agent — 2026-11-21
$ git diff --check
exit=0

$ cargo fmt --all -- --check
exit=0

$ cargo clippy --workspace --all-targets -- -D warnings
exit=0   # apos remover qualificacao redundante em flora-api/src/routes/auth.rs:265

$ cargo test -p flora-tests --lib
36 passed   0 failed

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS   exit=0   floors=77   auth=23   bootstrap=13

$ # script/workflow do gate e AGENTS.md atualizados com os 77 floors e o login wildcard;
$ # nenhum required check novo adicionado ou relaxado sob ADR-0003

$ # testes DB locais: nao concluidos por timeout/encerramento do Postgres temporario
$ # -> NAO VERIFICADO localmente, NAO e falha; CI/db-tests e a verdade remota

$ # contratos entregues (nomes lidos diretamente do arquivo)
db_password_login_wildcard_with_opt_in_returns_session_for_first_active_user
db_password_login_wildcard_without_opt_in_returns_401
db_password_login_exact_email_is_preserved_under_wildcard_opt_in
db_password_login_only_exact_wildcard_token_is_accepted
db_password_login_wildcard_wrong_password_matches_unknown_user_401
db_password_login_wildcard_without_active_user_returns_401

PRs no ciclo: 0   merge: 0   tag: 0   deploy executado por esta R4: 0
CI remota / deploy / login wildcard em producao / remocao das env: [NV] ate o deploy final
```

**Estado final confirmado**: a leitura direta do baseline e as verificações medidas pelo main agent confirmam o diagnóstico — `applied=true` com hash gravado, mas 401 na rota por e-mail desconhecido — e o contrato a preservar: `*` é opt-in exato verificado no servidor, o caminho wildcard resolve o primeiro usuário ativo sem fallback, e o 401/sessão permanecem idênticos ao login por e-mail. A decisão e os contratos seguem `intake`/`research`/`define`/`shape`/`decision`, `spec.md`, `plan.md` e `tasks.md`. O desfecho em produção (login wildcard com a credencial de recuperação, conta alvo e remoção das env) permanece **[NV]** até o deploy final.

## Fronteira

Contra a base `5890471be`, as mudanças de implementação ficam restritas a `flora-api/src/routes/auth.rs` (inclusive a limpeza da qualificação redundante em `:265`), `flora-tests/src/contract/auth.rs` e `.env.example`, além dos artefatos do próprio ciclo e dos apêndices de memória/governança. O script do gate, o workflow e `AGENTS.md` foram atualizados pelo main agent para refletir os 77 floors (auth 23, bootstrap 13) e o login wildcard, sem acrescentar ou relaxar required checks sob ADR-0003; nenhum outro arquivo foi alterado: sem constituição, ADR, contratos de API, valores de status `TODO`/`IN_PROGRESS`/`DONE`, schema, `STATE.md` histórico, `PROGRESSO*`, OIDC/allowlist, `POST /bootstrap`, advisory lock, rate limit, PR, merge, tag ou deploy. `validate.md` (R4) e `release.md` (R7) são registros novos criados nas fases seguintes do mesmo ciclo e não afetam o sistema; por isso T008 os exclui do escopo de mudança de implementação.

## [NV]

- **Login em produção e DB real**: `applied=true` e o 401 são medições do relato/governança; o login com `email="*"` e a credencial de recuperação, a emissão de sessão e `/tasks` → 200 não foram executados — **[NV]** até o deploy final (SC-001).
- **Conta alvo**: o wildcard não filtra por role; com vários usuários ativos, seleciona o mais antigo por `created_at`. Qual conta recebeu o hash do ciclo 100 e se é a administradora esperada só pode ser confirmado no ambiente real — **[NV]** (SC-001, SC-002).
- **Logs em produção**: a ausência de senha, hash e e-mail real em log/resposta foi projetada e coberta por contratos; a medição nos logs com o filtro atual de produção permanece **[NV]** até o deploy (SC-003).
- **Env temporárias**: a remoção de `FLORA_BOOTSTRAP__ADMIN_EMAIL=*`/`ADMIN_PASSWORD` no Coolify após o login bem-sucedido é passo operacional do deploy, não executado aqui — **[NV]** até o deploy final (SC-002, SC-003).
- **CI/PR/produção**: gate do alvo **PASS medido localmente** (`check_gate_integrity.py` exit=0, 77 floors — auth 23, bootstrap 13), mas nenhuma CI remota, PR, merge, tag, aprovação de maintainer, proteção de branch ou ambiente de produção foi verificado nesta R4; gate local não substitui CI remota para promoção (SC-003).
- **Coolify/Forgejo/DNS**: publicação de imagem, reapontamento da API, reinício/deploy, resultado do login em navegador real e medição de `/health`/CORS não foram observados — **[NV]** até o deploy final (SC-001, SC-003).
- **Credenciais**: nenhuma senha real foi fornecida, lida, registrada ou transmitida nesta passagem; a ausência de credencial em log, resposta e artefato é requisito SC-003, coberta por contratos e redação, não por observação ao vivo.
- **Testes DB locais**: não concluídos por timeout/encerramento do Postgres temporário — registrado como **não verificado localmente, não como falha** (FR-004, SC-001, SC-002; T006, T008); CI/db-tests é a verdade remota e o DB de produção permanece **[NV]** até o deploy final.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram reexecutados ao escrever este R4; as saídas consolidadas acima foram medidas pelo main agent em 2026-11-21.

## Portarias

R1 `go` com diagnóstico determinístico, decisão materializada em `.specify/assessments/101-password-login-wildcard/{intake,research,define,shape,decision}.md` e em `docs/governance/2026-11-21-101-password-login-wildcard.md` (2026-11-21, append-only); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=4/4`, `fr=4/4`, `sc=3/3` e `tasks=8/8` (`specs/101-password-login-wildcard/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`, com gate do alvo **PASS medido localmente** (`check_gate_integrity.py` exit=0, 77 floors — auth 23, bootstrap 13) e verificações medidas pelo main agent (`git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `cargo clippy --workspace --all-targets -- -D warnings` exit=0, `cargo test -p flora-tests --lib` 36 passed/0 failed), testes DB locais **não verificados por timeout do Postgres temporário — não é falha**, CI/db-tests como verdade remota, e **[NV]** explícito para CI remota, Coolify/Forgejo/DNS, deploy, login wildcard em produção e remoção das env temporárias até o deploy final; R5/R6 sem alterações de gate/required check — nenhum check novo adicionado ou relaxado sob ADR-0003; R7 registro de release PATCH `0.7.2` → `0.7.3` com rollback (remover `*` da env, reapontar a app da API para a imagem/build anterior, reiniciar, sem force-push e sem reescrita de histórico), publicação/promoção manuais e **[NV]** até o deploy final.
