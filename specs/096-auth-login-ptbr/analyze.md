# Analyze: 096 — Autenticação local (fallback de senha) e login pt-BR (READ-ONLY)

**Data**: 2026-11-20 · **Modo**: READ-ONLY · **Branch**: `feature/096-auth-login-ptbr` @ `1910d1c7d` · **Baseline**: `1910d1c7d`
**Artefatos revisados**: `.specify/feature.json` (= `specs/096-auth-login-ptbr`), `.specify/assessments/096-auth-login-ptbr/decision.md` (verdict `go`), `spec.md` (FR-001..FR-008, SC-001..SC-003), `plan.md`, `tasks.md` (T001..T014) e a cadeia de autenticação no repositório (`flora-core/src/config/oidc.rs`, `flora-api/src/routes/auth.rs`, `flora-api/src/routes/bootstrap.rs`, `flora-core/src/models/user.rs`, `apps/frontend/src/routes/(auth)/+page.svelte`).
**Escopo**: eliminar o bloqueio determinístico de acesso — a conta `luis.granja@mdmv.com.br` criada via bootstrap não consegue entrar porque o único mecanismo de login é o fluxo OIDC, cuja configuração cai nos defaults `http://localhost:8080` / `flora@localhost` com allowlist vazia e nenhum IdP disponível, e não há fallback de senha. A decisão adiciona autenticação local com Argon2id, recuperação condicional da credencial por variáveis de ambiente no startup (somente quando o hash está nulo) e tela de login em pt-BR, preservando o fluxo OIDC existente. Fora de escopo: required checks, gate, constituição/ADRs, schema além da migration de credencial, PR/merge/tag, deploy e observação ao vivo de produção.

## Diagnóstico determinístico (confirmado por leitura direta no baseline)

- `flora-core/src/config/oidc.rs:20-33` — `OidcConfig::default()` resolve `issuer_url` para `http://localhost:8080` e `client_id` para `flora@localhost` quando `ZITADEL_ISSUER_URL`/`ZITADEL_CLIENT_ID` não estão definidos, `client_secret` vazio e `allowed_redirect_uris: Vec::new()`; não há IdP configurado no alvo.
- `flora-api/src/routes/auth.rs:70-140` — `/auth/login` apenas inicia o fluxo OIDC com PKCE e valida `redirect_uri` contra a allowlist; não existe rota de login por senha.
- `flora-api/src/routes/bootstrap.rs` — `POST /bootstrap` cria o usuário com `User::new(&email, "Admin")` (`oidc_subject: None`) e devolve um token de sessão único; `flora-core/src/models/user.rs` não possui coluna de credencial, portanto a conta bootstrap não tem como autenticar depois.
- `apps/frontend/src/routes/(auth)/+page.svelte` — a tela exibe campos "Email"/"Password", mas `handleLogin()` ignora a senha e chama `GET /api/v1/auth/login` (OIDC); todos os rótulos e mensagens estão em inglês ("Welcome to Flora", "Sign In", "First Seed Journey").

## Checagem determinística

```
artifacts=11  critical=0  requirements_covered=8/8  fr=8/8  sc=3/3  tasks=14/14  go=True  errors=[]
FR match= True ['FR-001', 'FR-002', 'FR-003', 'FR-004', 'FR-005', 'FR-006', 'FR-007', 'FR-008']
SC match= True ['SC-001', 'SC-002', 'SC-003']
tasks unique= True ['T001', 'T002', 'T003', 'T004', 'T005', 'T006', 'T007', 'T008', 'T009', 'T010', 'T011', 'T012', 'T013', 'T014']
```

`requirements_covered=8/8` contabiliza os 8 requisitos funcionais (FR-001..FR-008); SC-001..SC-003 são rastreados nas mesmas linhas. `critical=0`: nenhuma divergência de contrato entre `decision.md`, `spec.md`, `plan.md`, `tasks.md` e a cadeia de autenticação revisada — o diagnóstico (OIDC sem IdP, allowlist vazia, sem fallback de senha) e as quatro frentes de correção (credencial Argon2id, bootstrap/recuperação, endpoint de login local com rate limit, UI pt-BR) são coerentes. `go=True` registrado em `decision.md`. `analyze` não alterou código, migration, config, workflow, gate, ADR, constituição, `STATE.md`, `validate/` nem `release/`, não reexecutou gates, build, testes, formatters nem comandos git de mutação.

## Rastreabilidade — 8/8 requisitos cobertos, 14/14 tarefas

| # | Requisito (spec) | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — migration adiciona `users.password_hash` e `users.password_updated_at`, preservando contas OIDC existentes e a idempotência (SC-003) | T002, T003, T014 | a migration é a base da credencial local; T003 adapta model/repository sem alterar `oidc_subject`, e T014 verifica lockfile/supply-chain/gates; nenhuma coluna existente é removida e a migration é reversível |
| R2 | FR-002 — hash Argon2id com `m=19456, t=2, p=1` em formato PHC, verificação com dummy hash e resposta `401` genérica, sem vazamento de existência de conta (SC-002) | T001, T004, T005 | T001 pinã `argon2`/`password-hash`; T004 implementa `PasswordService` com parâmetros e PHC; T005 cobre hash/verify, dummy hash e o `401` uniforme; deps e lockfile conferidos em T014 |
| R3 | FR-003 — `POST /bootstrap` passa a exigir `admin_password` e grava o hash na mesma transação atômica de organização/usuário/roles/sessão (SC-001, SC-003) | T006, T007 | o bootstrap atual já insere tudo em uma transação com advisory lock; T006 acrescenta o campo e o hash dentro dela, T007 testa o caminho feliz, a rejeição de payload sem senha e a ausência de senha em log/resposta |
| R4 | FR-004 — `POST /api/v1/auth/password/login` autentica e emite token/sessão no mesmo formato do fluxo OIDC (SC-001) | T008, T009 | T008 registra o endpoint ao lado de `/login`, `/callback`, `/refresh` em `create_auth_router()` e reusa `FloraClaims`/`encode_token` e a criação de sessão; T009 cobre sucesso, senha errada e conta inexistente com o mesmo corpo de erro |
| R5 | FR-005 — recuperação no startup por `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD`, aplicada apenas quando `password_hash IS NULL`, sem sobrescrever credencial existente (SC-001, SC-003) | T010, T011 | T010 acrescenta a leitura condicional na inicialização/estado da app, usando o mesmo `PasswordService`; T011 cobre os quatro estados (sem env, com env e hash nulo, com env e hash presente, env incompleto) e a ausência da senha em log |
| R6 | FR-006 — rate limiting no login local, devolvendo `429` sem revelar se a conta existe (SC-002) | T008, T009, T014 | T008 aplica o limitador ao endpoint de senha com a mesma semântica de `429` já usada pela UI; T009 verifica teto e janela; T014 confirma que nenhum required check/gate foi alterado |
| R7 | FR-007 — frontend "Sign In" local em pt-BR envia e-mail/senha ao endpoint de senha e usa a mesma transição para `/tasks` do bootstrap (SC-001) | T012, T013 | `handleLogin()` hoje ignora a senha e chama o OIDC; T012 liga os campos ao novo endpoint, mantém o tratamento de `429` e o `authState.login`, e T013 cobre o fluxo com testes frontend |
| R8 | FR-008 — `app.html` com `lang="pt-BR"` e todas as strings da tela de login traduzidas, inclusive a falha genérica e o texto de primeira instalação (SC-003) | T013, T014 | T013 troca "Welcome to Flora"/"Sign In"/"First Seed Journey"/mensagens de erro por equivalentes pt-BR e ajusta o atributo de idioma; T014 consolida testes/lockfile/gates e confirma que nenhuma credencial aparece em log ou resposta |

SC-001 (a conta bootstrap consegue entrar em produção pelo login local) é coberta por R3/R4/R5/R7; SC-002 (falha genérica sem enumeração de contas e sem vazamento de senha) por R2/R6/R7; SC-003 (migration idempotente/reversível e nenhuma credencial em log ou resposta) por R1/R3/R5/R8. T001 fixa as dependências; T002/T003/T014 a migration e o modelo; T004/T005 o serviço de senha e seus testes; T006/T007 o bootstrap e seus testes; T008/T009 o endpoint, o rate limit e seus testes; T010/T011 a recuperação por env e seus testes; T012/T013 a tela pt-BR e os testes frontend; T014 o fecho de lockfile/supply-chain/gates. O detalhe literal está consolidado em `.specify/validations/096-auth-login-ptbr/validate.md`.

## [NV]

- **Produção e conta real**: o login da conta `luis.granja@mdmv.com.br` em produção não foi executado nesta passagem; a recuperação por env depende de `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` configurados no ambiente de destino e permanece **[NV]** até o deploy final.
- **Verificações esperadas**: `cargo test`, `npm run check`, build e `gate-integrity: PASS` (`exit=0`) são evidências esperadas do handoff — não reexecutadas por este agente, conforme o modo READ-ONLY.
- **CI/PR/produção**: nenhum PR, merge, tag, required check ou promoção foi executado ou inspecionado ao vivo; gate local esperado não substitui CI remota.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram executados por este agente; a análise é somente leitura e a evidência vem do handoff do main agent e da leitura direta do baseline.

## Resultado

**PASS — 8/8 requisitos cobertos, 3/3 critérios de sucesso, 14/14 tarefas, 0 críticos.** FR-001..FR-008, SC-001..SC-003 e T001..T014 (únicos e sequenciais) estão alinhados ao `go` de `decision.md` e ao escopo determinístico de adicionar autenticação local com Argon2id, recuperação condicional por env e tela de login pt-BR sem remover o fluxo OIDC. Nenhuma tarefa nova, nenhuma divergência de contrato e nenhuma edição de fonte nesta fase; `[NV]` preservado nos artefatos do ciclo. Analyze somente leitura — gates, build, testes e formatters não foram reexecutados aqui.
