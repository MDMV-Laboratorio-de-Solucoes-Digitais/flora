# Decision: 096 — Login local pt-BR para conta bootstrap

- **Slug**: 096-auth-login-ptbr
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (original 2026-11-21) | specs/096-auth-login-ptbr/{spec,plan,tasks,analyze,converge}.md | .specify/bugs/oidc-redirect-uri-not-allowed/bug.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | A conta bootstrap `luis.granja@mdmv.com.br` não entra: defaults OIDC sem IdP (`oidc.rs:27-31`), allowlist vazia (`:38`), redirect do frontend fora do callback (`(auth)/+page.svelte:43-45` vs `auth.rs:101-116`) e conta criada sem senha (`bootstrap.rs:32-37,147-158`; `user.rs:13-33`) (`intake.md:5-19`; `research.md:1-12`). |
| Evidence strength | adequate | Cadeia fechada por leitura direta com `arquivo:linha`, relato registrado no bug e contratos locais; gates/build/testes são evidência consolidada do handoff, não reexecutados; IdP externo (Zitadel), segredo, URIs e produção **[NV]** (`research.md:25-31`; `define.md:SC-003`). |
| Value vs. inaction | strong | Sem fallback local, um deploy sem IdP é um deploy sem login: a conta existe mas nenhum mecanismo a autentica (`define.md:5`; `analyze.md:Escopo`). |
| Feasibility / appetite | adequate | Fallback isolado com duas colunas nullable, Argon2id `m=19456,t=2,p=1` em PHC, uma rota de login com rate limit próprio e UI pt-BR; sem mudança de gate, contrato OIDC ou infraestrutura externa (`shape.md:5-24`; `define.md:FR-001..FR-008`). |
| Strategic fit | strong | Desbloqueia o Seed local-first sem depender de provedor externo, preservando integralmente o fluxo OIDC e as contas com `oidc_subject` (`shape.md:1-4,20`; `analyze.md:R4`). |
| Risk posture | adequate | Dummy hash + 401 idêntico, recuperação idempotente e sem segredo em log/resposta mitigam enumeração e vazamento; crates de hash precisam entrar com versão pinada e lockfile atualizado, e o IdP permanece **[NV]** (`shape.md:26-33`; `define.md:SC-002`). |

## Verdict & Rationale

Veredito histórico **`go`** de 2026-11-21 preservado: branch `feature/096-auth-login-ptbr` @ `1910d1c7d`, âncora `.specify/feature.json` → `specs/096-auth-login-ptbr`, bug `.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`. A decisão original registrou oito compromissos FR-001..FR-008, três critérios SC-001..SC-003 e quatorze tarefas T001..T014 (todas concluídas), com analyze `critical=0`, `requirements_covered=8/8`, `fr=8/8`, `sc=3/3`, `tasks=14/14`, `errors=[]` (`specs/096-auth-login-ptbr/analyze.md`). A cadeia era determinística: `flora-core/src/config/oidc.rs:27-31` com `http://localhost:8080`/`flora@localhost`/secret vazio e `.env.example:66-68` comentado; `allowed_redirect_uris` nascia vazio (`:38`); `flora-api/src/routes/auth.rs:101-116` só aceitava o callback exato `{base_url}/api/v1/auth/callback` e rejeitava `${location.origin}/callback` com `redirect_uri is not allowed`; `bootstrap.rs:32-37,147-158` e `flora-core/src/models/user.rs:13-33` criavam a conta sem `password_hash` e não havia serviço de hash (`flora-core/src/traits/auth_provider.rs:9-12` citava o fallback só como ideia; `grep argon2/password_hash` → apenas `validation.rs:59-105`). A forma escolhida foi um fallback local coexistente com o OIDC — migration com `password_hash`/`password_updated_at` nullable, `PasswordService` Argon2id com dummy hash e 401 genérico, `admin_password` no bootstrap, `POST /api/v1/auth/password/login` com a classe de rate limit do padrão em `flora-api/src/middleware/rate_limit.rs:138-193`, recuperação por `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` e tela de login pt-BR com `lang="pt-BR"` em `app.html:2`. A semântica original da recuperação — somente com `password_hash IS NULL`, sem sobrescrever hash definido — foi registrada como tal e depois revisitada pelos ciclos 098, 100 e 101; isso é evolução documentada, não contradição deste `go`. IdP externo, URIs registradas, segredo, rede, ambiente remoto/produção e valor real de `FLORA_APP__BASE_URL` permanecem **[NV]** (`research.md:25-30`; `decision.md` original:`[NV]`). Gate do alvo registrado como PASS no handoff consolidado; este artefato não reexecutou gates, build, testes ou formatters.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as incógnitas abaixo ficam em aberto para o handoff).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Conta bootstrap existente sem credencial e sem rota de login utilizável, porque o único fluxo (OIDC) cai em defaults sem IdP e rejeita o `redirect_uri` enviado pelo frontend (`intake.md:7-19`).
- **Chosen approach**: Fallback local de autenticação por senha, isolado e coexistente com o OIDC — Argon2id PHC `m=19456,t=2,p=1`, dummy hash/401 genérico, `admin_password` no bootstrap, recuperação por env no startup e interface pt-BR; configurar/substituir o IdP externo ficou fora de escopo (`shape.md:3-24,37`).
- **In scope / out of scope**:
  - **In (FR-001..FR-008 / SC-001..SC-003 / T001..T014)**: migration versionada em `flora-core/src/migrations/sql/` com `users.password_hash` e `users.password_updated_at` nullable preservando contas OIDC; `PasswordService` com verificação dummy; senha gravada na mesma transação atômica do bootstrap; `POST /api/v1/auth/password/login` com token/sessão equivalentes ao callback OIDC; recuperação idempotente por env (semântica original: somente com hash nulo); rate limit próprio; `POST /api/v1/auth/password/login` e tela "Entrar" em pt-BR com `lang="pt-BR"`; dependências/lockfile e gates do alvo (`define.md:FR-001..FR-008`; `shape.md:5-24`).
  - **Out/deferred/NV**: IdP externo e allowlist OIDC, novos métodos de login, `analyze/`, `converge/`, `validate/`, `release/`, `.specify/memory/STATE.md` e `PROGRESSO*`, required checks, workflows, ADRs e constituição (`define.md:Fora de escopo`; `decision.md` original:`Fora de âmbito`).
- **Success metrics**: login da conta bootstrap com `session_token` escopado e sessão aceita em rotas autenticadas (baseline: conta sem senha e 401/`redirect_uri is not allowed`); e-mail inexistente, senha errada e sem hash devolvem o mesmo 401 sem enumeração (baseline: rota inexistente); migration aplica em banco vazio e populado sem perda, recuperação idempotente e gates PASS (baseline: gate do alvo registrado como PASS no handoff; não reexecutado); analyze `critical=0`, `8/8`, `3/3`, `14/14` (`define.md:SC-001..SC-003`; `analyze.md:9-18,46`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: existe IdP externo (Zitadel) configurado, com quais URIs registradas e segredo, e qual é a disponibilidade de rede (**[NV]**)?]; [NEEDS CLARIFICATION: qual é o valor real de `FLORA_APP__BASE_URL` e o comportamento do ambiente remoto/produção (**[NV]**)?]; [NEEDS CLARIFICATION: a conta de produção já possui `password_hash` e qual caminho a criou — a recuperação original exigia hash nulo, revisitada em 098/100/101 (**[NV]**)?] (`research.md:25-30`; `define.md:SC-001`; `problem.md` equivalente em `define.md:5`).
