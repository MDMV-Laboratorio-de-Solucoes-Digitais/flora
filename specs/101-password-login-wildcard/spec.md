# Feature Specification: 101 — login com `*` quando a opt-in de bootstrap está ativa

**Feature Branch**: `feature/101-password-login-wildcard`

**Created**: 2026-11-21

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/101-password-login-wildcard/decision.md` e diagnóstico determinístico do 401 no login após a recuperação de 100

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Entrar com `*` e a senha temporária sem saber o e-mail (Priority: P1)

Como operador de uma instalação já recuperada, preciso fazer login com o mesmo
token `*` que selecionou a conta no startup, usando apenas a senha temporária
configurada, porque o e-mail real do administrador é desconhecido de quem opera.

**Why this priority**: é o bloqueio relatado. O ciclo 100 já gravou a senha no
primeiro usuário ativo (`applied = true`), mas `password_login` faz busca exata
(`flora-api/src/routes/auth.rs:451-453`); com `email="*"`, `find_by_email`
retorna `None`, `verify_or_dummy` devolve `false` e a rota responde 401
(`:455-460`). A credencial está correta — falta o alias para usá-la.

**Independent Test**: com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` ativa e a senha já
aplicada, enviar `POST /api/v1/auth/password/login` com `{"email":"*",
"password":<senha do env>}` e confirmar 200 com sessão em formato OIDC para o
primeiro usuário ativo, sem conhecer o e-mail real.

**Acceptance Scenarios**:

1. **Given** `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` ativa e a senha temporária já
   aplicada ao primeiro usuário ativo, **When** o login é chamado com
   `email="*"` e essa senha, **Then** a resposta é 200 com `session_token`,
   `expires_in`, `user_id` e `next_step: "select_or_create_organization"`, e o
   `user_id` corresponde ao primeiro usuário ativo por `created_at ASC, id ASC`.
2. **Given** a mesma opt-in e a mesma senha, **When** o login com `*` é repetido,
   **Then** cada chamada emite uma sessão válida com `jti` único, sem criar
   usuários, alterar hashes ou produzir efeitos colaterais.
3. **Given** as variáveis removidas e o aplicativo reiniciado, **When** o login
   é chamado com `email="*"`, **Then** a resposta volta a ser o 401 genérico;
   a senha configurada continua válida para o e-mail real da conta.
4. **Given** a opt-in ativa, **When** o login é chamado com o e-mail real da
   conta e a senha temporária, **Then** a resposta é 200 com a mesma sessão de
   sempre — o alias não substitui nem degrada o caminho normal.

---

### User Story 2 — Manter o alias fechado sem opt-in e sem fallback (Priority: P1)

Como mantenedor, preciso que `email="*"` só funcione quando
`FLORA_BOOTSTRAP__ADMIN_EMAIL` for literalmente `*`, para que um erro de digitação
na configuração ou no payload não abra um caminho de autenticação para conta
aleatória.

**Why this priority**: o `*` é inválido como e-mail e já é a opt-in explícita de
100. Sem a exigência de que os dois lados sejam exatamente `*`, uma busca que
falhasse e "caísse" para o primeiro usuário ativo poderia autenticar a conta
errada em produção.

**Independent Test**: configurar `FLORA_BOOTSTRAP__ADMIN_EMAIL` com e-mail
exato, com typo e com variantes de `*`; em todos os casos confirmar que
`email="*"` resulta no 401 genérico, sem seleção, sem escrita e sem distinção na
resposta.

**Acceptance Scenarios**:

1. **Given** `FLORA_BOOTSTRAP__ADMIN_EMAIL` com um e-mail válido qualquer (ou
   ausente), **When** o login é chamado com `email="*"`, **Then** a busca é
   exata, nenhum usuário é encontrado e a resposta é o 401 genérico atual —
   nenhum caminho se abre.
2. **Given** `FLORA_BOOTSTRAP__ADMIN_EMAIL` com typo ou variante como
   `*@example.com`, `foo*`, `'* '` ou `' *'`, **When** o login é chamado com
   `email="*"`, **Then** o alias não ativa; a comparação é exata após `trim()`
   somente para o token literal `*`.
3. **Given** a opt-in ativa e nenhum usuário ativo no banco, **When** o login é
   chamado com `email="*"`, **Then** a resposta é o mesmo 401 genérico, sem
   escrita e sem fallback para outra conta.
4. **Given** a opt-in ativa, **When** o login é chamado com `email="*"` e senha
   errada, **Then** a resposta é idêntica à de um usuário desconhecido — sem
   indicar que o alias foi reconhecido.

---

### User Story 3 — Preservar 401/sessão e não expor credenciais (Priority: P2)

Como mantenedor, preciso que o alias não mude a superfície de autenticação nem
revele segredos: o 401 continua genérico, a sessão continua em formato OIDC e
nenhuma senha, hash ou e-mail real aparece em log, resposta, erro ou teste.

**Why this priority**: o formato do 401 e da sessão são parte do contrato de
segurança existente, e a janela da opt-in é temporária — um vazamento de
credencial ou uma resposta diferenciada seria uma regressão pior que o bloqueio
original.

**Independent Test**: executar os estados relevantes e conferir que senha
errada, usuário sem hash, conta inativa, alias sem opt-in e usuário desconhecido
devolvem exatamente o mesmo corpo 401; que o sucesso devolve a sessão OIDC
usual; que o rate limit `PasswordLogin` continua ativo; e que nenhum artefato
contém segredo.

**Acceptance Scenarios**:

1. **Given** senha errada, usuário sem hash, conta inativa ou alias sem opt-in,
   **When** o login local é chamado, **Then** a resposta é o mesmo 401
   `invalid_credentials` de hoje (`auth.rs:455-460`), sem distinção de motivo.
2. **Given** autenticação bem-sucedida via alias, **When** a resposta é
   inspecionada, **Then** ela contém somente `session_token`, `expires_in`,
   `user_id` e `next_step`, com o mesmo JWT, TTL e formato do fluxo OIDC; a
   classe de rate limit `PasswordLogin` permanece a mesma.
3. **Given** o `POST /api/v1/bootstrap` e o fluxo OIDC, **When** executados,
   **Then** permanecem 201/409 sob o mesmo lock e com a mesma emissão de sessão
   — nenhum contrato é alterado.
4. **Given** o diff do ciclo, **When** os gates do alvo são consolidados,
   **Then** permanecem PASS e nenhum arquivo de `validate/`, `release/`,
   `STATE.md`, `PROGRESSO*` ou `AGENTS.md` é alterado.

---

### Edge Cases

- Sem nenhuma variável de bootstrap, `config.bootstrap.admin_email` é `None` e o
  alias não existe: `email="*"` segue a busca exata e resulta em 401 genérico.
- Opt-in incompleta (só e-mail `*` sem senha): `BootstrapConfig::validate()`
  (`bootstrap.rs:69-95`) falha no startup; o alias não é habilitado e nenhum
  segredo aparece na mensagem.
- O token é comparado de forma exata após `trim()` nos dois lados (`:56-60`):
  `*@x.com`, `foo*`, `' *'` e `'* '` não ativam o alias.
- O e-mail comum continua normalizado com `trim().to_lowercase()`
  (`auth.rs:451`); o alias não altera esse comportamento.
- Múltiplos usuários ativos: o alias resolve o mais antigo por `created_at ASC,
  id ASC` — o mesmo critério de `bootstrap.rs:129-143`; critério determinístico
  e documentado.
- Nenhum usuário ativo: 401 genérico, sem escrita e sem fallback.
- Conta inativa ou sem hash: 401 genérico, inalterado em relação ao fluxo atual.
- Senha errada com alias: 401 idêntico ao de usuário desconhecido.
- Variáveis mantidas no ambiente: o alias permanece disponível enquanto a
  opt-in estiver ativa; `.env.example:36-45` orienta removê-las após o login.
- Nenhuma credencial, hash, token ou e-mail real da conta aparece em log,
  resposta de erro, mensagem de teste ou artefato do ciclo.
- DB real de produção, login ao vivo, Deploy/Coolify, IdP externo e allowlist
  OIDC são independentes e permanecem **[NV]**.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001 — wildcard somente com opt-in**: `POST /api/v1/auth/password/login`
  DEVE aceitar `email` literal `*` somente quando `FLORA_BOOTSTRAP__ADMIN_EMAIL`
  (`config.bootstrap.admin_email`, disponível em `AppState.config`) também for
  exatamente `*` após `trim()`. A comparação é exata nos dois lados: `*@x.com`,
  `foo*`, valores com whitespace ou qualquer e-mail (inclusive com typo) NÃO
  ativam o alias. Sem a opt-in, `email="*"` segue a busca exata existente
  (`auth.rs:451-453`) e termina em 401 genérico — nenhum caminho se abre.
  Nenhuma chave de configuração nova é introduzida; reutiliza-se a mesma
  definição de token (`is_wildcard_selector()`, `bootstrap.rs:56-60`).

- **FR-002 — seleção determinística**: com a opt-in ativa e `email="*"`, o
  handler DEVE resolver o usuário com a mesma consulta determinística da
  recuperação — `SELECT ... FROM users WHERE is_active = TRUE ORDER BY created_at
  ASC, id ASC LIMIT 1` (`flora-core/src/config/bootstrap.rs:129-143`) — e usar
  esse registro no restante do fluxo. Sem usuário ativo, DEVE retornar o mesmo
  401 genérico, sem escrita e sem fallback. Fora do alias, a busca permanece
  exata via `repo.find_by_email()` (`auth.rs:453`). A seleção do login NUNCA deve
  divergir da seleção do startup.

- **FR-003 — 401 genérico/sessão**: o caminho wildcard DEVE preservar
  `PasswordService::verify_or_dummy()` (`password_service.rs:75-86`) e
  `Error::InvalidCredentials` (401 `invalid_credentials`, `auth.rs:458-460`)
  para senha errada, usuário sem hash, conta inativa ou nenhum usuário — sem
  distinção entre "alias sem opt-in", "senha errada" e "usuário desconhecido".
  No sucesso, DEVE devolver a resposta idêntica em formato OIDC:
  `session_token`, `expires_in`, `user_id`,
  `next_step: "select_or_create_organization"` (`auth.rs:463-491`), com o mesmo
  JWT, TTL e classe de rate limit `PasswordLogin`
  (`flora-api/src/middleware/rate_limit.rs:136,151,195`). O fluxo OIDC, o
  `POST /bootstrap`, o JWT e o rate limit permanecem inalterados: o alias é um
  nome alternativo para a conta, não um novo método de autenticação. Nenhuma
  senha, hash ou e-mail real aparece em log ou resposta.

- **FR-004 — testes e `.env.example`**: contratos em
  `flora-tests/src/contract/auth.rs` (`mod db_password_login`, `:1039-1183`)
  DEVEM cobrir: opt-in `*` com senha configurada → 200 e sessão do primeiro
  usuário ativo; sem opt-in → 401 genérico; variantes (`*@x.com`, `foo*`,
  whitespace) → 401; senha errada com `*` → 401 idêntico ao de usuário
  desconhecido; nenhum usuário ativo → 401; conta inativa e usuário sem hash →
  401 inalterados; formato de sessão preservado; nenhum segredo em log/resposta.
  `.env.example:36-45` DEVE documentar que, enquanto `ADMIN_EMAIL=*` estiver
  ativa, o login pode usar `email="*"` com a senha temporária, e que ambas as
  variáveis devem ser removidas com restart após o login, sem valores reais.
  Gates do alvo DEVEM permanecer PASS e o diff NÃO DEVE tocar em `validate/`,
  `release/`, `STATE.md`, `PROGRESSO*` ou `AGENTS.md`; OIDC, `POST /bootstrap`,
  advisory lock e rate limit seguem inalterados.

### Key Entities

- **BootstrapConfig / opt-in `*`**: config de startup com `admin_email` e
  `admin_password` (redactado, `skip_serializing`); `admin_email = *` é o token
  de opt-in de 100, não uma chave nova. O mesmo token é a condição do alias no
  login.
- **password_login**: `POST /api/v1/auth/password/login`
  (`flora-api/src/routes/auth.rs:447-492`). Com a opt-in e `email="*"`, resolve
  o primeiro usuário ativo; fora disso, mantém a busca exata. Verificação e
  emissão de sessão permanecem as mesmas.
- **Seleção do primeiro usuário ativo**: `WHERE is_active = TRUE ORDER BY
  created_at ASC, id ASC LIMIT 1` (`bootstrap.rs:129-143`) — determinística,
  compartilhada com a recuperação de 100, usada apenas sob opt-in.
- **401 genérico**: `Error::InvalidCredentials` → `invalid_credentials`, o mesmo
  para todos os motivos de falha; o alias não cria resposta diferenciada.
- **Sessão OIDC**: `session_token`, `expires_in`, `user_id`, `next_step`, JWT e
  TTL emitidos como hoje (`auth.rs:463-491`); inalterada no caminho wildcard.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001 — login em produção com '*' e senha configurada**: com
  `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e a senha temporária já aplicada ao primeiro
  usuário ativo, `POST /api/v1/auth/password/login` com `email="*"` e a senha do
  env retorna 200 com sessão em formato OIDC — sem conhecer o e-mail real da
  conta. Após remover as variáveis e reiniciar, o alias deixa de funcionar, mas
  a senha configurada continua válida para o e-mail real.

- **SC-002 — sem opt-in não abre caminho**: quando
  `FLORA_BOOTSTRAP__ADMIN_EMAIL` não é `*`, `email="*"` é tratado como endereço
  comum: a busca exata não encontra nada e a resposta é o 401 genérico atual.
  Nenhuma seleção por usuário ativo acontece, nenhum fallback é aplicado e
  nenhuma conta é alterada. Um typo no e-mail configurado ou no `email` enviado
  nunca ativa o alias.

- **SC-003 — sem segredo em log/resposta**: nenhuma senha, hash ou e-mail real
  aparece em log, resposta, erro, teste ou artefato do ciclo. O 401 do caminho
  wildcard é idêntico ao genérico existente e a sessão bem-sucedida contém
  apenas os campos usuais. Contratos e specs passam, o gate do alvo é PASS
  (registrado no handoff) e o diff não toca em `validate/`, `release/`,
  `STATE.md`, `PROGRESSO*` ou `AGENTS.md`; OIDC, `POST /bootstrap`, advisory lock
  e rate limit seguem inalterados. O estado real do DB de produção e o login ao
  vivo permanecem **[NV]**.

## Assumptions

- O diagnóstico é o da investigação determinística: a opt-in de 100 já aplicou a
  senha ao primeiro usuário ativo, o login faz busca exata e o e-mail real da
  conta bootstrap é desconhecido de quem opera.
- O estado do env (`ADMIN_EMAIL=*`, `applied = true`) é informação do relato;
  nenhuma credencial real foi lida, registrada ou reproduzida neste ciclo.
- `ADMIN_EMAIL=*` é um opt-in operacional temporário: o operador deve remover
  ambas as variáveis e reiniciar após o login, pois enquanto estiver definido o
  alias continua disponível e a senha é reaplicada a cada startup.
- Quando houver mais de um usuário ativo, o critério "primeiro" é o `created_at`
  mais antigo, desempatado por `id`; é o mesmo registro escolhido por 100 na
  gravação da senha.
- O alias não é um novo método de autenticação: ele apenas nomeia a conta cuja
  senha já foi verificada pelo `verify_or_dummy`, com o mesmo 401 e a mesma
  sessão do fluxo local.
- O estado real do DB de produção, o login ao vivo, o filtro de logging,
  Deploy/Coolify, IdP externo e allowlist OIDC são **[NV]** e pertencem a outros
  ciclos; esta especificação cobre apenas o contrato local observável.
- As verificações e o estado do gate são os registrados no handoff consolidado;
  este artefato não reexecutou gates, build, testes ou formatters.
