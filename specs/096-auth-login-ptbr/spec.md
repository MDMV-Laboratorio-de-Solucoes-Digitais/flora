# Feature Specification: 096 — login local pt-BR para conta bootstrap

**Feature Branch**: `feature/096-auth-login-ptbr`

**Created**: 2026-11-21

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/096-auth-login-ptbr/decision.md` e bug `.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Entrar com a conta bootstrap (Priority: P1)

Como administrador da instalação, preciso entrar com o e-mail e a senha da conta
criada no bootstrap, sem depender de um provedor OIDC, para poder usar o Flora
em produção.

**Why this priority**: é o defeito relatado — `luis.granja@mdmv.com.br` existe,
mas os defaults de OIDC apontam para `localhost:8080`, a allowlist de redirects
está vazia e o frontend envia um redirect rejeitado; não há nenhuma credencial
de senha na conta.

**Independent Test**: com o banco já inicializado pelo bootstrap, chamar
`POST /api/v1/auth/password/login` com e-mail e senha corretos e confirmar
`session_token` aceito em rota autenticada; repetir com senha errada e conferir
`401` genérico.

**Acceptance Scenarios**:

1. **Given** uma conta bootstrap com `password_hash` válido, **When** o usuário
   envia e-mail e senha corretos a `/api/v1/auth/password/login`, **Then** a API
   responde `session_token` e `expires_in` com o mesmo escopo organizacional do
   fluxo OIDC.
2. **Given** a mesma conta, **When** a senha está errada, **Then** a API
   responde o mesmo `401` genérico usado para e-mail inexistente, sem revelar
   qual dos dois falhou.
3. **Given** um ambiente sem `ZITADEL_*`, **When** o usuário entra pela tela de
   login local, **Then** a sessão é estabelecida sem contatar IdP externo
   **[NV] para o comportamento remoto**.

---

### User Story 2 — Bootstrap e recuperação seguros da credencial (Priority: P1)

Como mantenedor, preciso que a senha do administrador seja obrigatória no
bootstrap e possa ser recuperada por variáveis de ambiente no startup quando a
conta ainda não tiver hash, sem sobrescrever uma senha já definida.

**Why this priority**: sem credencial gravada na mesma transação do bootstrap,
ou sem um caminho de recuperação para instalações já existentes, a conta
criada continua inacessível — e uma recuperação que sobrescreve senha válida
seria uma regressão de segurança.

**Independent Test**: executar bootstrap sem `admin_password` e conferir
rejeição; executar com senha válida e confirmar o hash na transação; reiniciar
com `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` com hash já presente e
confirmar que nada mudou.

**Acceptance Scenarios**:

1. **Given** um payload de bootstrap sem `admin_password`, **When** a requisição
   é enviada, **Then** é rejeitada e nenhuma organização/usuário é criado.
2. **Given** `admin_password` válido, **When** o bootstrap executa, **Then** o
   hash Argon2id é gravado com a conta dentro da mesma transação (commit
   atômico).
3. **Given** uma conta com `password_hash IS NULL` e as variáveis de
   recuperação definidas, **When** a aplicação inicia, **Then** o hash é
   preenchido uma vez; reinícios posteriores não o alteram.

---

### User Story 3 — Tela de login em português (Priority: P2)

Como usuário brasileiro, preciso de um formulário "Sign In" em pt-BR na tela de
login, com e-mail e senha, mantendo a opção SSO, e de uma página com
`lang="pt-BR"`.

**Why this priority**: a interface atual só oferece o fluxo OIDC e exibe
mensagens em inglês; sem o formulário local, o fallback implementado na API
não é utilizável pelo usuário final.

**Independent Test**: abrir a tela de login e conferir `lang="pt-BR"` no HTML,
rótulos e mensagens em pt-BR, envio do formulário para a rota local e
preservação do botão SSO.

**Acceptance Scenarios**:

1. **Given** a tela de login, **When** o usuário preenche e-mail e senha e
   envia, **Then** o frontend chama `POST /api/v1/auth/password/login` e, em
   caso de sucesso, estabelece a sessão e navega para `/tasks`.
2. **Given** uma falha de autenticação, **When** o erro é exibido, **Then** a
   mensagem é genérica e está em pt-BR, sem indicar se o e-mail existe.

---

### Edge Cases

- E-mail inexistente, senha errada e conta sem hash devem produzir o mesmo
  `401` — a verificação usa dummy hash para evitar enumeração e diferença de
  tempo.
- `admin_password` inválido deve abortar o bootstrap antes de qualquer
  INSERT; a validação reaproveita `flora-core/src/utils/validation.rs:59-105`.
- `FLORA_BOOTSTRAP__ADMIN_EMAIL` sem senha, ou senha sem e-mail, não dispara
  recuperação; e-mail de conta inexistente também não cria usuário.
- A recuperação nunca sobrescreve `password_hash` já definido; `password_hash`
  nulo mantém o login OIDC funcionando como antes.
- A migração precisa ser segura em banco vazio e em banco com usuários OIDC:
  colunas nullable, sem default que crie credencial.
- Limites de rate limit da rota local não podem afetar as classes já existentes
  de bootstrap/OIDC.
- Nenhuma senha, hash ou segredo pode aparecer em log, resposta de erro ou
  artefato do ciclo.
- IdP externo, configuração real de `FLORA_APP__BASE_URL` e deploy remoto
  permanecem **[NV]** — a especificação cobre o contrato local observável.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE adicionar, por migration versionada em
  `flora-core/src/migrations/sql/`, as colunas `users.password_hash` (TEXT,
  nullable) e `users.password_updated_at` (TIMESTAMPTZ, nullable),
  preservando as contas OIDC existentes com `password_hash IS NULL`.
- **FR-002**: O sistema DEVE implementar `PasswordService` com Argon2id nos
  parâmetros `m=19456`, `t=2`, `p=1` e serialização PHC; DEVE verificar um dummy
  hash quando o usuário ou o hash não existe e DEVE responder `401` genérico
  idêntico em toda falha, limpando segredos em memória.
- **FR-003**: `POST /api/v1/bootstrap` DEVE exigir `admin_password`, validar a
  complexidade e gravar o hash Argon2id na mesma transação que cria
  organização, usuário, papel, associação, workspace e sessão, sem escrita
  parcial em caso de erro.
- **FR-004**: O sistema DEVE expor `POST /api/v1/auth/password/login` com e-mail
  e senha, verificando o hash via FR-002 e emitindo token JWT e sessão com o
  mesmo escopo organizacional do fluxo OIDC.
- **FR-005**: No startup, o sistema DEVE recuperar a credencial por
  `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `FLORA_BOOTSTRAP__ADMIN_PASSWORD` somente
  quando a conta existir com `password_hash IS NULL`; a operação DEVE ser
  idempotente, NÃO DEVE sobrescrever hash existente e NÃO DEVE registrar a
  senha em log.
- **FR-006**: A rota de login local DEVE ter rate limiting próprio, seguindo o
  padrão de classes de `flora-api/src/middleware/rate_limit.rs`, sem alterar as
  classes de bootstrap ou OIDC já vigentes.
- **FR-007**: O frontend DEVE oferecer um formulário "Sign In" local em pt-BR
  (e-mail + senha) que chama FR-004, preservando a opção SSO existente e
  exibindo mensagem genérica em caso de falha.
- **FR-008**: `apps/frontend/src/app.html` DEVE declarar `lang="pt-BR"` e as
  strings da tela de login, incluindo erros e estados de envio, DEVEM estar em
  pt-BR.

### Key Entities

- **User / password_hash**: conta existente (global, com `oidc_subject`
  opcional) passa a ter credencial local opcional; `NULL` significa "sem senha",
  compatível com contas somente OIDC.
- **PasswordService**: único lugar de hash e verificação Argon2id/PHC; dai vem
  o dummy hash que iguala as respostas de falha.
- **Bootstrap**: transação única que cria a instalação e agora também a
  credencial do administrador.
- **Recuperação por env**: par
  `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD`, lido no
  startup via Figment (`FLORA_` + `__`), aplicado só a hash nulo.
- **Sessão local**: `session_token` + `expires_in` equivalentes ao callback
  OIDC, com escopo de organização.
- **Tela de login**: formulário local pt-BR convivendo com o botão SSO.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A conta bootstrap (ex.: `luis.granja@mdmv.com.br`) entra com a
  senha definida no bootstrap ou pela recuperação por env; a resposta contém
  `session_token` aceito em rota autenticada com o escopo organizacional
  correto, sem contatar IdP externo.
- **SC-002**: E-mail inexistente, senha errada e conta sem hash retornam o mesmo
  `401` genérico, sem enumeração nem vazamento de hash; nenhuma credencial
  aparece em log, resposta ou artefato do ciclo.
- **SC-003**: A migration aplica em banco vazio e em banco populado sem perda de
  dados; a recuperação por env é idempotente; os gates do alvo passam (PASS
  registrado no handoff) e o diff não toca em `analyze/`, `converge/`,
  `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Assumptions

- O diagnóstico é o de `.specify/bugs/oidc-redirect-uri-not-allowed/bug.md`: o
  bloqueio decorre de defaults OIDC + allowlist vazia + conta sem senha, não de
  falha intermitente de rede.
- A existência de um IdP externo e sua configuração são **[NV]**; o fallback
  local não exige nenhum provedor externo.
- A validação de complexidade de
  `flora-core/src/utils/validation.rs:59-105` é reaproveitada, sem duplicar
  regras.
- O contrato de sessão do callback OIDC
  (`flora-api/src/routes/auth.rs:247-291`) é o modelo para o login local.
- As verificações e o estado do gate são os registrados no handoff
  consolidado; este artefato não reexecutou gates, build, testes ou
  formatters.
