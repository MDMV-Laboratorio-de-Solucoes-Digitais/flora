# Converge: 096 — Autenticação local (fallback de senha) e login pt-BR (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20 — entrada 1

- **Decisão do ciclo**: eliminar o bloqueio de acesso determinístico — a conta `luis.granja@mdmv.com.br` criada via bootstrap não entra porque o único mecanismo de login é o OIDC, cuja configuração cai nos defaults `http://localhost:8080` / `flora@localhost` com allowlist vazia e nenhum IdP disponível, e não há fallback de senha. A correção adiciona credencial local Argon2id (`m=19456, t=2, p=1`, PHC, dummy hash, `401` genérico), exige `admin_password` no bootstrap e grava o hash na mesma transação, expõe `POST /api/v1/auth/password/login` com rate limiting, recupera a credencial no startup por `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` somente quando o hash é nulo e traduz a tela de login para pt-BR (`lang="pt-BR"`). O fluxo OIDC existente permanece; verdict `go` e cadeia de evidência em `.specify/assessments/096-auth-login-ptbr/decision.md`.
- **Evidência determinística consolidada** (branch `feature/096-auth-login-ptbr`, baseline `1910d1c7d`), atribuída ao handoff do main agent e à leitura direta do baseline:
  - `flora-core/src/config/oidc.rs` cai em `http://localhost:8080`/`flora@localhost` com `client_secret` vazio e `allowed_redirect_uris: Vec::new()`; `flora-api/src/routes/auth.rs` só inicia o fluxo OIDC com PKCE — não existe rota de senha (FR-002, FR-004, SC-001; T004, T008);
  - `flora-api/src/routes/bootstrap.rs` cria o usuário com `User::new` (`oidc_subject: None`) e devolve um token único, e `flora-core/src/models/user.rs` não tem coluna de credencial — por isso a conta bootstrap não consegue reautenticar (FR-001, FR-003, SC-003; T002, T003, T006);
  - a migration de `users.password_hash`/`users.password_updated_at` preserva as contas OIDC existentes, é reversível e não altera required checks/gates (FR-001, SC-003; T002, T003, T014);
  - `PasswordService` com Argon2id `m=19456, t=2, p=1` em PHC, verificação com dummy hash e resposta `401` uniforme para senha errada e conta inexistente; `argon2`/`password-hash` pinados e lockfile/supply-chain conferidos (FR-002, SC-002; T001, T004, T005, T014);
  - `POST /api/v1/auth/password/login` emite token/sessão no mesmo formato do OIDC (`FloraClaims`/`encode_token`), com rate limit e `429` sem enumeração de conta (FR-004, FR-006, SC-001, SC-002; T008, T009);
  - recuperação no startup por `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` restrita a `password_hash IS NULL`, sem sobrescrever credencial existente e sem registrar a senha em log (FR-005, SC-001, SC-003; T010, T011);
  - frontend: "Sign In" passa a enviar e-mail/senha ao endpoint local com o mesmo tratamento de `429` e a mesma transição para `/tasks`; `app.html` com `lang="pt-BR"` e todas as strings da tela, inclusive a falha genérica, traduzidas (FR-007, FR-008, SC-002, SC-003; T012, T013);
  - verificação esperada do handoff: `cargo test`, `npm run check`, build e `python3 scripts/ci/check_gate_integrity.py` → **PASS** (`exit=0`); não reexecutados por este agente (FR-001..FR-008, SC-001..SC-003; T005, T007, T009, T011, T013, T014);
  - fronteira: `validate/` e `release/` são escritos apenas como registros documentais das fases R4/R7 deste ciclo; a implementação não tocou gate, required checks, constituição, ADRs, PR/merge/tag, schema além da migration de credencial, ambientes de produção nem `STATE.md` (SC-003; T014).
- **Analyze** (`specs/096-auth-login-ptbr/analyze.md`, READ-ONLY, 2026-11-20): `critical=0`, `requirements_covered=8/8`, `fr=8/8`, `sc=3/3`, `tasks=14/14`, `errors=[]` — nenhuma correção de fonte necessária.
- **Fronteira preservada**: `.github/workflows/ci.yml`, `rust.yml`, gate, required checks, constituição, ADRs, fluxo OIDC existente, PR/merge/tag e worktrees alheiros intocados pela implementação. Fora de escopo: deploy, configuração ao vivo do ambiente e a credencial real da conta bootstrap.
- **[NV]**: login da conta bootstrap em produção, configuração das variáveis de recuperação no ambiente de destino, `cargo test`/`npm run check`/build/gate (evidência esperada do handoff) e CI ao vivo não observados nesta passagem.
- **Sem novas tarefas**: nenhuma divergência spec/tasks a corrigir; T001..T014 permanecem no escopo de dependências, migration, modelo/repositório, PasswordService, bootstrap, endpoint de senha com rate limit, recuperação por env, frontend pt-BR, testes e fecho de lockfile/supply-chain/gates.

## Resultado

**Converged — a implementação satisfaz a spec e o analyzer.** Nenhuma tarefa nova, nenhuma correção de fonte e nenhuma reexecução de gates/formatters por este agente; registro criado apenas para rastreabilidade.

converge written: 1 file; gates/formatters skipped
