# Release: 085 — First Seed Journey local one-time bootstrap

**Ciclo**: 085 · **Data**: 2026-11-20T03:17Z · **Branch**: `feature/085-local-first-seed` · **Baseline**: `38184fa50`
**Tipo**: nova superfície de API pública one-time, transação atômica de provisionamento, sessão já escopada à organização e ligação do botão "Sprout Workspace". **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `flora-api/src/routes/bootstrap.rs` (novo) — `create_bootstrap_router()` com `POST /bootstrap`; validação de `organization_name` (1–255 após trim) e `admin_email` (formato + lowercase) → 422; `pg_advisory_xact_lock`, contagem de organizações excluindo a placeholder `system` (`00000000-0000-0000-0000-000000000000`) e inserts de Organization, User (`Admin`), Role (`Admin`, `["*"]`, builtin), Membership, Workspace (`Main Workspace`) e Session na mesma transação → 201 com `{session_token, expires_in, user_id, organization_id, workspace_id}`; segundo uso/conflito → 409 sem escrita; auditoria `tracing::info!` só com `organization_id`/`user_id`.
- `flora-api/src/routes/mod.rs` — `pub mod bootstrap` e registro público `.nest("/api/v1", bootstrap::create_bootstrap_router())`, expondo `POST /api/v1/bootstrap` (corrigido após o verificador apontar prefixo duplicado).
- `flora-api/src/middleware/rate_limit.rs` — classe `Bootstrap` própria, 5 requisições/60 s por IP, chave `bootstrap:{ip}`.
- `flora-tests/src/contract/bootstrap.rs` (novo) + `flora-tests/src/contract/mod.rs` — 4 contratos DB-gated (`db_bootstrap_*`), sem `.unwrap()`/`.expect()`/`panic!`, com `anyhow::Result` + `?`, JWT decodificado, sessão ativa e teste de concorrência.
- `apps/frontend/src/routes/(auth)/+page.svelte` — `handleBootstrap()` ligado somente ao botão "Sprout Workspace": `fetch` a `/api/v1/bootstrap`, `authState.login(token, {id:user_id}, organization_id)` e `goto('/tasks')`; erro reutiliza `errorMessage`/`readErrorDetail`, preserva campos e inibe duplo clique; "Sign In"/OIDC intacto.
- `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts` (novo) — 4 testes: OIDC 503/429, bootstrap 201 e bootstrap 409.
- Verificação: gate **PASS**; `cargo fmt` e `clippy -D warnings` exit 0; 4 contratos `db_bootstrap` PASS; 4 Vitest PASS; `svelte-check` 0 erros/0 warnings; `lint` green; suíte completa `105 passed, 59 ignored`; E2E real `POST` **201**, segunda chamada **409**, `GET /api/v1/tasks` com token **200**. Verificador isolado `BootstrapCodeVerifier` encontrou 3 findings, todos fechados antes do R4 (registro em `validate.md` e `converge.md`).

## SemVer

**Proposta:** minor `0.3.5` → `0.4.0`. Motivo: nova rota pública e novo caminho funcional da UI (superfície de API ampliada, sem quebra das rotas existentes), com fronteira de segurança inédita — mesmo critério dos precedentes 079 (`0.2.9 → 0.3.0`, nova dependência/tooling) e 064/066 (minor `0.2.0` por nova fronteira de segurança). O ciclo 084 entregou `0.3.4 → 0.3.5`; `0.2.0` é marco histórico antigo, não compatível com a linhagem atual. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / SC / T)

- `FR-001` único caminho utilizável em instalação sem organizações além da placeholder `system` (`POST /api/v1/bootstrap`); `FR-002` nome (1–255 após trim) e e-mail validados antes de qualquer gravação (422); `FR-003` Organization, User, Role, Membership, Workspace e Session criados em uma única transação com `pg_advisory_xact_lock`; `FR-004` sessão/JWT com `sub=user_id`, `jti`, `organization_id` e `email`, insert da sessão antes do commit; `FR-005` recusa 409 sem criar/modificar objetos quando já existe organização não-sistema; `FR-006` concorrência serializada pela trava transacional (no máximo uma criação); `FR-007` fluxo OIDC existente inalterado; `FR-008` bootstrap apenas no botão "Sprout Workspace", "Sign In" mantido; `FR-009` mensagem legível com campos preservados na recusa; `FR-010` `isSubmitting` inibe dupla submissão; `FR-011` auditoria com `organization_id`/`user_id`, sem token ou segredo; `FR-012` ausência de verificação de posse do e-mail explícita na spec/quickstart e registrada como ressalva.
- `SC-001` formulário válido em banco vazio termina em `/tasks` com a organização selecionada (E2E 201 + token aceito em 200); `SC-002` segunda tentativa e banco com dados retornam 409 com contagens inalteradas (contratos + E2E); `SC-003` nenhuma recusa grava objetos parciais — transação com rollback e unique violations mapeadas para 409; `SC-004` 4 contratos `db_bootstrap`, 4 Vitest, `cargo fmt`, `clippy -D warnings`, gate e suíte completa verdes; `SC-005` 4 testes frontend cobrem OIDC 503/429 e o login existente segue intacto.
- `T001`, `T002` branch/feature.json e DB descartável documentado em `quickstart.md`; `T003`, `T004`, `T005` classe de rate limit `Bootstrap`, rota registrada e módulo de contrato com `#[ignore = "requires Postgres"]`; `T006`, `T007`, `T008`, `T009`, `T010`, `T011`, `T012` contrato de criação + teste do clique, handler validado, transação atômica com a placeholder excluída, JWT/201, ligação à UI e auditoria sem segredos; `T013`, `T014`, `T015`, `T016` recusa da segunda chamada, recusa com organização existente, concorrência e mapeamento 409 com rollback; `T017`, `T018` casos de erro/submissão duplicada/preservação de campos e reuso de `errorMessage`/`isSubmitting`/`readErrorDetail`; `T019`, `T020`, `T022`, `T023` documentação de governança (FR-012), nota datada em `docs/PROGRESSO.md`, teste de regressão OIDC e diagrama/roteiro de API — registro documental do ciclo; `T021` gates e verificação real `201/409/200` executados com saídas coladas no R4. IDs únicos e sequenciais; verificador isolado `BootstrapCodeVerifier`: 3 findings fechados (rota, topologia de teste, contrato JWT) e correções de lint/compilação concluídas.

## Evidência

`.specify/validations/085-local-first-seed/validate.md` (R4 **APROVADO**, com saídas literais) · `.specify/assessments/085-local-first-seed/converge.md` · `specs/085-local-first-seed/spec.md`, `plan.md`, `tasks.md`, `contracts/bootstrap.md`, `quickstart.md` · `.specify/assessments/085-local-first-seed/decision.md`.

## Rollback

Reverter, em ordem inversa, os commits do ciclo 085 (código, testes e artefatos) com um novo commit de reversão — sem force-push e sem reescrita de histórico. Se parte das alterações ainda estiver não commitada, restaurar os 4 arquivos modificados (`flora-api/src/middleware/rate_limit.rs`, `flora-api/src/routes/mod.rs`, `flora-tests/src/contract/mod.rs`, `apps/frontend/src/routes/(auth)/+page.svelte`) e remover os arquivos novos (`flora-api/src/routes/bootstrap.rs`, `flora-tests/src/contract/bootstrap.rs`, `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts` e os diretórios de artefatos do ciclo).

A remoção funcional passa obrigatoriamente por retirar de `flora-api/src/routes/mod.rs` a linha `.nest("/api/v1", bootstrap::create_bootstrap_router())` e o `pub mod bootstrap;` — sem isso a rota `POST /api/v1/bootstrap` continua exposta — além da classe `Bootstrap` em `rate_limit.rs` e do registro do módulo em `flora-tests/src/contract/mod.rs`. Após a reversão, repetir os gates 1–5 de `plan.md`. Os bancos `flora_demo_085` e `flora_demo_085_e2e` são **descartáveis**, criados somente para contratos/E2E: podem ser drops sem consequência e **não são produção**; nenhum dado de produção é tocado, alterado ou removido. Gate, CI, workflows, ADRs alheias, constituição, STATE, specs de outros ciclos, PRs, rede, segredos e worktrees alheios permanecem intactos.

## [NV]

- **Merge/produção**: nenhum PR, tag, deploy ou merge realizado; status de CI ao vivo, Forgejo, aprovações, proteção de branch e ambiente de produção não verificados.
- **Posse do e-mail** (FR-012): limitação aceita somente para instalação nova sem IdP; não é mecanismo de autenticação para instalações estabelecidas.
- **UI em browser**: validação real foi HTTP (`201/409/200`) com token; sem captura de tela ou clique manual para conferir renderização da mensagem de erro.
- **Carga/concorrência em produção**: cobertas localmente por contrato e trava transacional; sem teste de carga em produção.
- **Reexecução nesta passagem**: nenhum gate/formatters/teste/git reexecutado ao escrever este release; evidência literal consolidada no R4.
