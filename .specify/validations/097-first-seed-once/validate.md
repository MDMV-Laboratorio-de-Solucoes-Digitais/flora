# Validation: 097 — Primeira configuração condicionada ao estado da instalação

**Data**: 2026-11-20 · **Branch**: `feature/097-first-seed-once` @ `696849934` · **Baseline**: `696849934`
**Tipo**: correção de exibição — endpoint read-only de estado da instalação, frontend condicional com fail-closed e guardião 409 do bootstrap preservado. Nenhuma alteração de gate, required check, constituição, ADR, schema ou PR/merge/tag nesta validação.
**Veredito R4**: **APROVADO**, com gates e verificações **medidos às 2026-11-21T02:41:52Z e 02:43Z** (saídas reais coladas abaixo; 9 testes DB de bootstrap com Postgres 18 às 02:21Z) e registro explícito de **[NV]** para CI remota/deploy/E2E de produção até o deploy final. Esta R4 **não reexecutou** gates, build, testes, formatters nem comandos git de mutação; as saídas foram medidas pelo main agent e consolidadas aqui.

## Gates e verificações

| Gate | Resultado |
|---|---|
| `git diff --check` | **exit=0** medido às 2026-11-21T02:41:52Z — diff limpo (FR-006, SC-003; T010) |
| `cargo fmt --all -- --check` | **exit=0** medido às 2026-11-21T02:41:52Z — formatação limpa (FR-001..FR-006, SC-003; T010) |
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0`, **67 floors** (auth 17, bootstrap 9) medido às 2026-11-21T02:43Z — único gate autoritativo sob ADR-0003, nenhum required check acrescentado ou relaxado neste ciclo (FR-006, SC-003; T010) |
| `cargo clippy --workspace --all-targets -- -D warnings` | **exit=0** medido às 2026-11-21T02:43Z — nenhum warning no workspace (FR-001..FR-006, SC-002, SC-003; T001..T010) |
| `cargo test -p flora-tests --lib` | **36 passed / 0 failed**, `exit=0` medido às 2026-11-21T02:43Z — cobre `GET /api/v1/bootstrap/status` nos estados fresh/configurado/erro e o guardião 409 do `POST /bootstrap` inalterado (FR-001..FR-003, SC-002; T001..T004) |
| 9 testes DB de bootstrap (Postgres 18) | **9 passed** medidos localmente às 2026-11-21T02:21Z — contagem de organizações não-sistema, advisory lock e 409 com Postgres 18 (FR-002, SC-002; T002, T003) |
| `npm --prefix apps/frontend run check` | **0 errors** medido às 2026-11-21T02:43Z — tela de login condicional e contrato de status sem quebrar login local/SSO (FR-001, FR-004, FR-005, FR-006; T005, T006, T007, T010) |
| `npm run lint` (prettier + eslint) | **sem issues** medido às 2026-11-21T02:43Z — lint/formatação do frontend limpos (FR-004..FR-006, SC-003; T005..T007, T010) |
| `git diff` / `git status` contra `696849934` | mudanças restritas à rota de leitura, ao helper de contagem compartilhado, ao frontend de login e aos artefatos do ciclo (`spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, este R4 e o R7); nenhum toque em gate, required check, constituição, ADR, schema, PR/merge/tag (SC-003; T009, T010) |
| CI remota / PRs / deploy / E2E de produção | nenhum PR, merge, tag ou deploy realizado por esta R4; **CI remota, deploy e E2E em produção permanecem [NV]** até o deploy final (SC-003) |

Nota de proveniência: todas as verificações da tabela foram **medidas pelo main agent às 2026-11-21T02:21Z, 02:41:52Z e 02:43Z**, com as saídas coladas literalmente no bloco abaixo; esta passagem R4 **não reexecutou** gates, build, testes, formatters, git de mutação, CI ao vivo nem E2E em produção. Nenhuma inferência além dessa evidência.

## Evidência documental (leitura direta no baseline)

```
$ # verificações medidas pelo main agent — 2026-11-21T02:21Z / 02:41:52Z / 02:43Z
$ git diff --check
exit=0                                              (02:41:52Z)

$ cargo fmt --all -- --check
exit=0                                              (02:41:52Z)

$ cargo clippy --workspace --all-targets -- -D warnings
exit=0                                              (02:43Z)

$ cargo test -p flora-tests --lib
36 passed   0 failed                                (02:43Z)

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS   exit=0   floors=67 (auth 17, bootstrap 9)   (02:43Z)

$ # 9 testes DB de bootstrap — Postgres 18, execução local
9 passed                                            (02:21Z)

$ npm --prefix apps/frontend run check
0 errors                                            (02:43Z)

$ npm run lint        # prettier + eslint
sem issues                                          (02:43Z)

$ # diagnóstico — leitura direta no baseline 696849934
$ sed -n '238,243p' 'apps/frontend/src/routes/(auth)/+page.svelte'
É uma instalação nova?  ->  Fazer primeira configuração   (branch {:else}, incondicional)
toggleSetup():174-177   handleBootstrap():123-172  409 -> "Este workspace já foi configurado.":140-143

$ sed -n '57,60p' flora-api/src/routes/bootstrap.rs
create_bootstrap_router(): somente POST /bootstrap  -> nenhum sinal de "instalação vazia"

$ sed -n '28,29p;238,244p;287,291p' flora-api/src/routes/bootstrap.rs
BOOTSTRAP_LOCK_KEY + pg_advisory_xact_lock
org_count = COUNT(*) FROM organizations WHERE id <> '00000000-0000-0000-0000-000000000000'
if org_count > 0 -> Error::DuplicateEntry("bootstrap already completed")   (409)

PRs no ciclo: 0   merge: 0   tag: 0   deploy: 0
CI remota / E2E produção: [NV]
```

**Estado final confirmado**: as verificações locais foram **medidas às 2026-11-21T02:21Z, 02:41:52Z e 02:43Z**, todas com sucesso: `git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `check_gate_integrity.py` **PASS** exit=0 (67 floors: auth 17, bootstrap 9), `cargo clippy --workspace --all-targets -- -D warnings` exit=0, `cargo test -p flora-tests --lib` **36 passed / 0 failed**, 9 testes DB de bootstrap com Postgres 18 (02:21Z), `npm --prefix apps/frontend run check` 0 errors e `npm run lint` sem issues. A leitura direta no baseline confirma o diagnóstico — link incondicional na tela de login e único sinal de "instalação vazia" confinado ao `POST /bootstrap` — e a decisão segue `spec.md`/`plan.md`/`tasks.md` e a nota de governança do ciclo. CI remota, deploy e E2E em produção permanecem **[NV]** até o deploy final.

## Fronteira

Contra a base `696849934`, as mudanças de implementação ficam restritas à rota de status, ao helper de contagem de organizações não-sistema compartilhado com o guardião, ao frontend da tela de login e aos artefatos do próprio ciclo. Nada mais foi alterado pela implementação: sem gate, required check, constituição, ADR, schema, `STATE.md`, PR, merge, tag ou deploy. `validate.md` (R4) e `release.md` (R7) são registros novos criados nas fases seguintes do mesmo ciclo e não afetam o sistema; por isso T010 os exclui do escopo de mudança de implementação.

## [NV]

- **Estado real da instalação em produção**: se a instalação de destino está vazia ou configurada não foi medido; permanece **[NV]** até o deploy final (SC-001).
- **CI remota / PRs / produção**: verificações locais **medidas PASS** (saídas reais na tabela e nas saídas coladas), mas nenhuma CI remota, PR, merge, tag, aprovação de maintainer, proteção de branch ou ambiente de produção foi verificado neste ciclo; gate local não substitui CI remota para promoção.
- **Deploy / Coolify / Forgejo / DNS**: configuração, publicação de imagem, resultado do deploy e medição em navegador real dos dois estados da tela permanecem **[NV]** até o deploy final, conforme a nota de governança.
- **E2E de produção**: o fluxo bootstrap → sessão → `/api/v1/tasks`, o link visível em instalação vazia/oculto em configurada e o 409 preservado em chamada direta/concorrente não foram observados ao vivo em produção; permanecem **[NV]** até o deploy final.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram reexecutados ao escrever este R4; as saídas foram medidas pelo main agent às 2026-11-21T02:21Z, 02:41:52Z e 02:43Z.

## Portarias

R1 `go` com causa raiz determinística, decisão materializada em `docs/governance/2026-11-21-097-first-seed-once.md` (2026-11-21, append-only); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=6/6`, `fr=6/6`, `sc=3/3` e `tasks=10/10` (`specs/097-first-seed-once/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`, com gates e verificações **medidas PASS às 2026-11-21T02:21Z/02:41:52Z/02:43Z** (`git diff --check` exit=0, `cargo fmt --all -- --check` exit=0, `check_gate_integrity.py` PASS exit=0 com 67 floors — auth 17, bootstrap 9 —, `cargo clippy --workspace --all-targets -- -D warnings` exit=0, `cargo test -p flora-tests --lib` 36 passed/0 failed, 9 testes DB de bootstrap com Postgres 18, `npm --prefix apps/frontend run check` 0 errors, `npm run lint` sem issues) e **[NV]** explícito para CI remota/deploy/E2E de produção até o deploy final; R7 release MINOR `0.6.0` → `0.7.0` com rollback documentado em `.specify/releases/097-first-seed-once/release.md`. Nenhuma promoção, tag, merge ou deploy executado por estes documentos.
