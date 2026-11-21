# Validation: 098 — Recuperação determinística da senha do administrador bootstrap via env

**Data**: 2026-11-21 · **Branch**: `feature/098-password-recovery` @ `76f936e24` · **Baseline**: `76f936e24`
**Tipo**: correção de recuperação de credencial — reutilização do par `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD`, sobrescrita do hash a cada startup via `set_password_hash`, evento `Startup administrator password recovery` no target `flora_api` e contratos dos quatro estados. **Nenhuma alteração de gate, required check, constituição, ADR, schema, PR/merge/tag, `STATE.md` ou `PROGRESSO*` nesta validação.**
**Veredito R4**: **APROVADO**, com gates e verificações **medidos às 2026-11-21T02:56:49Z–03:02Z** (saídas reais coladas abaixo) e registro explícito de **[NV]** para CI remota, deploy/Coolify e login em produção até o deploy final. Esta R4 **não reexecutou** gates, build, testes, formatters nem comandos git de mutação; as saídas foram medidas pelo main agent e consolidadas aqui.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0`, **67 floors** — medido na janela 2026-11-21T02:56:49Z–03:02Z; único gate autoritativo sob ADR-0003, nenhum required check acrescentado ou relaxado neste ciclo (FR-005, SC-003; T008) |
| `git diff --check` | **exit=0** medido na janela 2026-11-21T02:56:49Z–03:02Z — diff limpo contra `76f936e24` (FR-005, SC-003; T008) |
| `cargo fmt --all -- --check` | **exit=0** medido na janela 2026-11-21T02:56:49Z–03:02Z — formatação limpa (FR-001..FR-005, SC-003; T008) |
| `cargo clippy --workspace --all-targets -- -D warnings` | **exit=0** medido na janela 2026-11-21T02:56:49Z–03:02Z — nenhum warning no workspace (FR-001..FR-004, SC-002, SC-003; T002..T006) |
| `cargo test -p flora-tests --lib` | **36 passed / 0 failed**, `exit=0` medido na janela 2026-11-21T02:56:49Z–03:02Z — cobre os quatro estados de startup, a sobrescrita com hash pré-existente, o evento em `flora_api`, o login 200 com a senha da env e o 401 genérico preservado (FR-003, FR-004, SC-001, SC-002; T003..T006) |
| 9 testes DB de bootstrap (Postgres) | **9 passed** medidos na janela 2026-11-21T02:56:49Z–03:02Z, incluindo `db_startup_password_overwrites_existing_hash` — sobrescrita da credencial já presente, além dos estados de startup e do 401 inalterado (FR-003, FR-004, SC-001; T005, T006) |
| `npm --prefix apps/frontend run check` | **0 errors** medido na janela 2026-11-21T02:56:49Z–03:02Z — nenhuma alteração de frontend neste ciclo; login local/pt-BR do ciclo 096 preservado (FR-004, SC-001; T006) |
| `npm run lint` (prettier + eslint) | **sem issues** medido na janela 2026-11-21T02:56:49Z–03:02Z — lint/formatação do frontend limpos (FR-004, FR-005, SC-003; T006, T008) |
| `cargo vet --locked --no-minimize-exemptions` | **exit=0** medido na janela 2026-11-21T02:56:49Z–03:02Z — auditoria de supply-chain sem achados (FR-005, SC-003; T008) |
| `git diff` / `git status` contra `76f936e24` | mudanças restritas a `flora-core` (config/bootstrap, config/mod, repositories/user_repository, traits/repository, services/password_service se necessário), `flora-api` (state/app_state), `flora-tests` (contratos bootstrap/auth), `.env.example` e artefatos do ciclo 098 (`intake`/`research`/`define`/`shape`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, este R4 e o R7); nenhum toque em gate, required check, constituição, ADR, OIDC, `POST /bootstrap`, advisory lock, rate limit, `validate/` pré-existente, `release/`, `STATE.md`, `PROGRESSO*` ou PR/merge/tag (FR-005, SC-003; T001, T008) |
| PRs no ciclo | nenhum PR, merge, tag ou deploy realizado por esta R4; promoção permanece manual e **[NV]** (SC-003) |

Nota de proveniência: todas as verificações da tabela foram **medidas pelo main agent na janela 2026-11-21T02:56:49Z–03:02Z**, com as saídas reais coladas no bloco abaixo; esta passagem R4 **não reexecutou** gates, build, testes, formatters, git de mutação, CI ao vivo, login em produção nem escrita no DB real. Nenhuma inferência além dessa evidência.

## Saídas coladas

Evidência **medida pelo main agent às 2026-11-21T02:56:49Z–03:02Z** e consolidada aqui, mais o diagnóstico lido diretamente no baseline (esta R4 não reexecutou nada):

```
$ # verificações reais medidas pelo main agent — 2026-11-21T02:56:49Z–03:02Z
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

$ cargo vet --locked --no-minimize-exemptions
exit=0

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS   exit=0   floors=67

$ # 9 testes DB de bootstrap — Postgres
9 passed   (inclui db_startup_password_overwrites_existing_hash)

$ # diagnóstico — leitura direta no baseline 76f936e24
$ sed -n '108,110p' flora-core/src/config/bootstrap.rs
if user.password_hash.is_some() { return Ok(false); }
-> grava via set_password_hash_if_absent(); skip silencioso com hash presente

$ sed -n '99,102p' flora-core/src/repositories/user_repository.rs
UPDATE users SET password_hash = $2, password_updated_at = $3
 WHERE id = $1 AND password_hash IS NULL

$ sed -n '93p' flora-api/src/state/app_state.rs
let _applied = apply_startup_password(...).await?;
-> bool descartado; nenhum sinal operacional

$ sed -n '43,64p' flora-core/src/config/mod.rs
Env::prefixed("FLORA_").split("__")   -> FLORA_BOOTSTRAP__ADMIN_* mapeado sem defeito

$ sed -n '83p;443,460p' flora-api/src/routes/auth.rs
.route("/password/login", post(password_login))  -> 401 generico/dummy hash inalterados

$ grep -n BOOTSTRAP .env.example
30: # --- Administrator password recovery (TEMPORARY — remove after use) ---
38: # FLORA_BOOTSTRAP__ADMIN_EMAIL=...
39: # FLORA_BOOTSTRAP__ADMIN_PASSWORD=...
-> par temporario ja configurado em producao com valores corretos; nenhuma chave nova

$ # correcao observavel — target flora_api (nao suprimido pelo filtro flora_core)
evento: "Startup administrator password recovery"

PRs no ciclo: 0   merge: 0   tag: 0   deploy executado por esta R4: 0
CI remota / deploy / login em producao: [NV] ate o deploy final
```

**Estado final confirmado**: a leitura direta no baseline confirma o diagnóstico — guard `password_hash IS NULL` em `bootstrap.rs:108-110`/`user_repository.rs:99-102`, resultado descartado em `app_state.rs:93`, rota de login íntegra em `auth.rs:83,443-460` e mapeamento Figment funcional em `config/mod.rs:43-64`. A decisão e os contratos seguem `intake`/`research`/`define`/`shape`, `spec.md`, `plan.md` e `tasks.md`: reutilizar o par temporário `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD`, sobrescrever o hash a cada startup via `set_password_hash` e emitir `Startup administrator password recovery` no target `flora_api`, sem senha em log ou resposta. `.env.example:32-39` registra que as variáveis são temporárias e devem ser removidas com restart após a recuperação, para que o segredo não seja reaplicado. As verificações locais foram **medidas às 2026-11-21T02:56:49Z–03:02Z**, todas com sucesso: `git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `check_gate_integrity.py` **PASS** exit=0 (67 floors), `cargo clippy --workspace --all-targets -- -D warnings` exit=0, `cargo test -p flora-tests --lib` **36 passed / 0 failed**, 9 testes DB de bootstrap **passed** (incluindo `db_startup_password_overwrites_existing_hash`), `npm run check` 0 errors, `npm run lint` sem issues e `cargo vet --locked --no-minimize-exemptions` exit=0. A validação do comportamento em produção depende do deploy e permanece **[NV]**.

## Fronteira

Contra a base `76f936e24`, as mudanças de implementação ficam restritas à configuração de bootstrap, ao mapeamento de config, ao repositório/traits de usuário, ao serviço de senha (se necessário), ao estado da app, aos contratos `flora-tests` de bootstrap/auth e a `.env.example`, além dos artefatos do próprio ciclo. Nada mais foi alterado pela implementação: sem gate, required check, constituição, ADR, schema, `STATE.md`, `PROGRESSO*`, OIDC/allowlist, `POST /bootstrap`, advisory lock, rate limit, PR, merge, tag ou deploy. `validate.md` (R4) e `release.md` (R7) são registros novos criados nas fases seguintes do mesmo ciclo e não afetam o sistema; por isso T008 os exclui do escopo de mudança de implementação.

## [NV]

- **Login em produção e DB real**: o 401 medido no ciclo 096 é o sintoma; a correção depende do deploy e da sobrescrita do hash no DB de produção com o par de env já configurado. Nenhuma escrita ou login ao vivo foi executado; permanece **[NV]** até o deploy final (SC-001).
- **Logs em produção**: o filtro que suprime `flora_core` impediu medir o evento antigo; o novo evento `Startup administrator password recovery` no target `flora_api` foi projetado para ser observável, mas sua medição em produção permanece **[NV]** até o deploy (SC-002).
- **CI/PR/produção**: verificações locais **medidas PASS** na janela 2026-11-21T02:56:49Z–03:02Z (saídas reais na tabela e nas saídas coladas), mas nenhuma CI remota, PR, merge, tag, aprovação de maintainer, proteção de branch ou ambiente de produção foi verificado neste ciclo; gate local não substitui CI remota para promoção (SC-003).
- **Credenciais**: nenhuma senha real foi fornecida, lida, registrada ou transmitida nesta passagem; a ausência de credencial em log, resposta e artefato é requisito SC-002, coberta por contratos e redação (`skip_serializing`/`Debug` redacted), não por observação ao vivo.
- **IdP/allowlist OIDC e Coolify/Forgejo/DNS**: configuração de provedor externo, publicação de imagem, resultado do deploy e E2E autenticado não foram observados; permanecem **[NV]** até o deploy final.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram reexecutados ao escrever este R4; as saídas foram medidas pelo main agent na janela 2026-11-21T02:56:49Z–03:02Z.

## Portarias

R1 `go` com causa raiz determinística, decisão materializada em `.specify/assessments/098-password-recovery/{intake,research,define,shape}.md` e em `docs/governance/2026-11-21-098-password-recovery.md` (2026-11-21, append-only); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `sc=3/3` e `tasks=8/8` (`specs/098-password-recovery/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`, com gates e verificações **medidos PASS às 2026-11-21T02:56:49Z–03:02Z** (`git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `check_gate_integrity.py` PASS exit=0 com 67 floors, `cargo clippy --workspace --all-targets -- -D warnings` exit=0, `cargo test -p flora-tests --lib` 36 passed/0 failed, 9 testes DB de bootstrap passed incluindo `db_startup_password_overwrites_existing_hash`, `npm run check` 0 errors, `npm run lint` sem issues, `cargo vet --locked --no-minimize-exemptions` exit=0) e **[NV]** explícito para CI remota, deploy/Coolify e login em produção até o deploy final; R7 registro de release na sequência, sem reexecução de verificações nesta passagem.
