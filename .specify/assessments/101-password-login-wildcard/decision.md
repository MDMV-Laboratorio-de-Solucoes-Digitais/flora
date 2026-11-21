# Decision: 101 — login com `*` quando a opt-in de bootstrap está ativa

- **Slug**: 101-password-login-wildcard
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/101-password-login-wildcard` @ `5890471be35ec21f7c8eba9545bca3544edbd8d9`
- **Âncora**: `.specify/feature.json` → `specs/101-password-login-wildcard`
- **Evidência**: investigação local determinística + relato de produção (após 100)
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

Relato de produção após o ciclo 100: `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e
`FLORA_BOOTSTRAP__ADMIN_PASSWORD` estão presentes, o startup aplicou a senha ao
primeiro usuário ativo (`applied = true`, `flora-api/src/state/app_state.rs:93-99`),
e `POST /api/v1/auth/password/login` com `email="*"` ainda retorna 401. A cadeia
explica os fatos sem falha intermitente:

1. `flora-core/src/config/bootstrap.rs:56-60,129-143` — a opt-in `*` já seleciona
   o primeiro usuário ativo (`created_at ASC, id ASC`) e o hash da senha do env
   foi gravado nessa conta. A credencial está correta no banco.
2. `flora-api/src/routes/mod.rs:30` e `auth.rs:80-88` — a rota de login é
   `/api/v1/auth/password/login`, montada a partir de `create_auth_router()`.
3. `flora-api/src/routes/auth.rs:451-453` — o handler normaliza `req.email` e
   faz `find_by_email` **exata**; `*` não é um e-mail e não corresponde a nada.
4. `flora-api/src/routes/auth.rs:455-460` — sem usuário, `verify_or_dummy` gasta
   o trabalho dummy e devolve `false`; `Error::InvalidCredentials` produz o
   401 `invalid_credentials` idêntico ao de senha errada.
5. `flora-api/src/routes/auth.rs:463-491` — o sucesso emite sessão e JWT no
   formato OIDC; esse contrato não muda.
6. `flora-core/src/config/mod.rs:46-52` e `flora-api/src/state/app_state.rs:23-28`
   — `FLORA_BOOTSTRAP__ADMIN_EMAIL` já está em `config.bootstrap.admin_email` e
   é acessível ao handler via `AppState.config`. Nenhuma configuração nova é
   necessária.

O e-mail real da conta bootstrap é desconhecido de quem opera. A senha não
precisa ser reescrita; falta o alias que permita usá-la.

## Decisão

Registrar o ciclo 101 como **login wildcard condicionado à opt-in de bootstrap**,
com quatro compromissos rastreáveis (FR-001..FR-004), três critérios
(SC-001..SC-003) e oito tarefas (T001..T008, todas concluídas):

1. **Wildcard somente com opt-in (FR-001 / T002)**: o literal `*` é aceito como
   `email` somente quando `config.bootstrap.admin_email` também for exatamente
   `*` após `trim()` (`is_wildcard_selector()`, `bootstrap.rs:56-60`). Sem a
   opt-in, `*` segue a busca exata e o 401 genérico — nenhum caminho se abre;
   variantes como `*@x.com` ou `foo*` nunca ativam o alias.
2. **Seleção determinística (FR-002 / T003)**: com a opt-in, a rota resolve o
   primeiro usuário ativo pela mesma consulta de `bootstrap.rs:129-143`
   (`WHERE is_active = TRUE ORDER BY created_at ASC, id ASC LIMIT 1`); sem
   usuário ativo → 401 genérico, sem escrita e sem fallback. A seleção do login
   não pode divergir da do startup.
3. **401 genérico/sessão (FR-003 / T004–T005)**: `verify_or_dummy` e
   `Error::InvalidCredentials` (401 `invalid_credentials`) permanecem para senha
   errada, usuário sem hash, conta inativa, nenhum usuário ou alias sem opt-in.
   O sucesso devolve a resposta idêntica em formato OIDC (`session_token`,
   `expires_in`, `user_id`, `next_step`), com o mesmo JWT, TTL, classe
   `PasswordLogin` e fluxo OIDC. Nenhuma senha, hash ou e-mail real em log,
   resposta, erro ou mensagem de teste.
4. **Testes e env (FR-004 / T006–T008)**: contratos em
   `flora-tests/src/contract/auth.rs:1039-1183` cobrem opt-in `*` → 200/sessão,
   sem opt-in → 401, variantes → 401, senha errada → 401 idêntico a desconhecido,
   nenhum usuário ativo → 401, formato de sessão preservado e nenhum segredo;
   `.env.example:36-45` documenta o alias e a remoção das variáveis após o
   login. Gates do alvo PASS (handoff), fronteira preservada.

## Evidência de entrada

- Baseline `5890471be35ec21f7c8eba9545bca3544edbd8d9`, branch
  `feature/101-password-login-wildcard`.
- Relato de produção: opt-in `*` ativa, `applied = true`, login com `*` → 401,
  e-mail real desconhecido.
- `flora-api/src/routes/auth.rs:447-492` — `password_login`: normalização,
  `find_by_email`, `verify_or_dummy`, 401 e emissão de sessão/JWT.
- `flora-core/src/config/bootstrap.rs:56-60,69-95,129-143` — token `*` e
  seleção determinística já definidos e testados em 100.
- `flora-core/src/config/mod.rs:46-52` e `flora-api/src/state/app_state.rs:23-28`
  — a opt-in já chega ao handler sem nova variável.
- `flora-api/src/middleware/rate_limit.rs:136,151,195,241-245` — classe
  `PasswordLogin` inalterada.
- `flora-tests/src/contract/auth.rs:1039-1183` e
  `flora-tests/src/contract/bootstrap.rs:410,568-598` — suítes existentes a
  estender, não a substituir.
- `.env.example:36-45` — documentação das variáveis temporárias.

## Racional

**go.** O bloqueio é consequência direta de quatro pontos verificáveis: a opt-in
`*` já gravou a senha no primeiro usuário ativo, o login faz busca exata e não
conhece o e-mail real, o 401 é genérico e o resultado é observável. A extensão é
isolada: o mesmo token que já é opt-in explícita passa a ser um alias de `email`
apenas enquanto a opt-in estiver definida, resolvendo a mesma conta do startup e
reaproveitando integralmente verificação, sessão, JWT e rate limit. Nenhum gate,
contrato de login, emissão de sessão, lock ou infraestrutura é alterado. Todos os
FR/SC são verificáveis por contratos e inspeção locais; o diff é reversível e a
fronteira é preservada. O custo aceito é a existência temporária de um alias
válido apenas sob opt-in explícita — mitigado pela exigência de
`ADMIN_EMAIL=*`, pela seleção determinística, pela documentação de remoção das
variáveis após o login e pela preservação do 401 genérico.

## [NV]

- Estado real do DB de produção (e-mail do administrador, `is_active`,
  `created_at`, valor efetivo das variáveis) e login ao vivo: **[NV]**. Nenhuma
  credencial foi lida e nenhuma requisição de login/bootstrap foi executada
  contra produção nesta investigação.
- Filtro de logging real, Deploy/Coolify, IdP externo e allowlist OIDC: **[NV]** —
  independentes deste ciclo; o veredito autoriza apenas a mudança local.
- Correção não executada contra produção neste artefato; o gate do alvo está
  registrado como PASS no handoff consolidado e este documento não reexecutou
  gates, build, testes ou formatters.

## Fora de âmbito

Mudanças em `analyze/`, `converge/`, `validate/`, `release/`,
`.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`; alteração de required
checks, workflows, ADRs ou constituição; mudança no contrato do `POST /bootstrap`,
no advisory lock, no formato do 401, no JWT, no rate limit ou na emissão de
sessão; IDP externo, allowlist OIDC, nova variável de configuração, nova rota ou
API alternativa de login. Nenhuma senha real é fornecida ou registrada neste
ciclo.

## Handoff

`/speckit.specify` — especificar o login wildcard como requisitos verificáveis
(FR-001..FR-004 / SC-001..SC-003) e tarefas T001..T008, com o `*` aceito somente
sob a opt-in `FLORA_BOOTSTRAP__ADMIN_EMAIL=*`, seleção determinística do primeiro
usuário ativo, 401 genérico e sessão OIDC inalterados, nenhum segredo em
log/resposta, testes e documentação de env e fronteira preservada.
