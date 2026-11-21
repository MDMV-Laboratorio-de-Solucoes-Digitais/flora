# Feature Specification: 098 — recuperação da senha do administrador bootstrap via env

**Feature Branch**: `feature/098-password-recovery`

**Created**: 2026-11-21

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/098-password-recovery/decision.md` e diagnóstico determinístico do 401 em produção (096)

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Recuperar o acesso da conta bootstrap pré-existente (Priority: P1)

Como administrador de uma instalação já configurada, preciso que as variáveis
já definidas no ambiente reapliquem a senha à conta `luis.granja@mdmv.com.br`,
para entrar no login local sem recriar a instalação nem conhecer a senha antiga.

**Why this priority**: é o bloqueio relatado. A conta nasce com `password_hash`
não nulo (`flora-api/src/routes/bootstrap.rs:294-295`) e a recuperação anterior
recusava sobrescrever esse hash; o login devolvia 401 genérico
(`flora-api/src/routes/auth.rs:451-460`).

**Independent Test**: com uma conta que já tenha hash e as duas variáveis de env
configuradas, reiniciar e confirmar que `POST /api/v1/auth/password/login` com a
senha do env retorna 200 com sessão; após remover as variáveis e reiniciar,
confirmar que não há nova sobrescrita e que a senha recuperada continua válida.

**Acceptance Scenarios**:

1. **Given** a conta `luis.granja@mdmv.com.br` com hash não nulo e
   `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `FLORA_BOOTSTRAP__ADMIN_PASSWORD` corretos,
   **When** o startup roda, **Then** o hash é sobrescrito pelo da senha do env e
   o login local com essa senha retorna 200 com `session_token`, `expires_in` e
   `next_step`.
2. **Given** as variáveis removidas após a recuperação, **When** o startup roda
   novamente, **Then** nenhuma sobrescrita acontece e a senha recuperada
   permanece válida.
3. **Given** env incompleta (só e-mail ou só senha), **When** o startup roda,
   **Then** `BootstrapConfig::validate()` falha com erro de configuração e
   nenhuma escrita é feita, sem a senha na mensagem.
4. **Given** um e-mail de env que não corresponde a usuário, **When** o startup
   roda, **Then** a recuperação é ignorada com aviso (`bootstrap.rs:111-115`) e
   nenhuma conta é alterada.

---

### User Story 2 — Saber se a recuperação agiu, sem expor a senha (Priority: P1)

Como mantenedor, preciso que o startup informe se a recuperação foi aplicada,
sem revelar a senha, para distinguir "não configurado", "e-mail inexistente" e
"hash sobrescrito" nos logs.

**Why this priority**: o chamador descartava o retorno (`let _applied`) e o skip
anterior não logava — o que explicava a ausência de sinal relatada, mesmo com o
env correto.

**Independent Test**: executar o startup nos estados relevantes e conferir que
`applied` e os motivos de skip/sobrescrita aparecem em log sem senha, hash ou
dados extras.

**Acceptance Scenarios**:

1. **Given** env ausente, **When** o startup roda, **Then** `applied = false` é
   registrado em `app_state.rs:96-99` e nenhuma escrita ou aviso de senha é
   emitido.
2. **Given** env com e-mail inexistente, **When** o startup roda, **Then** um
   aviso é registrado (`bootstrap.rs:112-114`) e `applied = false`.
3. **Given** env com sobrescrita bem-sucedida, **When** o startup roda, **Then**
   `bootstrap.rs:130-132` registra a sobrescrita com `user_id`, sem senha nem
   hash, e `applied = true`.

---

### User Story 3 — Preservar o contrato de autenticação e a fronteira (Priority: P2)

Como mantenedor, preciso que a recuperação não altere o contrato do login, do
bootstrap nem os gates, para que a correção seja reversível e auditável.

**Why this priority**: o 401 genérico, o JWT, a classe de rate limit
(`flora-api/src/middleware/rate_limit.rs:150,195`) e o `POST /bootstrap` são parte
da superfície de segurança existente e não podem mudar.

**Independent Test**: executar os contratos de bootstrap e login e conferir que
o 401 permanece idêntico para senha errada, usuário sem hash e conta inativa,
que o `POST /bootstrap` continua 201/409 sob advisory lock e que o diff não toca
em `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

**Acceptance Scenarios**:

1. **Given** uma senha errada, um usuário sem hash ou uma conta inativa, **When**
   o login local é chamado, **Then** a resposta é o mesmo 401 genérico de hoje.
2. **Given** o `POST /api/v1/bootstrap`, **When** ele é chamado em banco vazio ou
   já configurado, **Then** retorna 201/409 sob o mesmo lock e transação.
3. **Given** o diff do ciclo, **When** os gates do alvo são consolidados, **Then**
   eles permanecem PASS e nenhum arquivo proibido é alterado.

---

### Edge Cases

- Sem nenhuma variável de bootstrap: `apply_startup_password()` retorna `false`
  e não escreve no banco.
- Env incompleta: `validate()` falha; nenhuma escrita parcial e nenhum valor de
  senha na mensagem de erro.
- E-mail normalizado com `trim().to_lowercase()` (`bootstrap.rs:108` e
  `auth.rs:451`) — a busca é a mesma nos dois caminhos.
- Conta com hash pré-existente: sobrescrita incondicional
  (`bootstrap.rs:120-128`) — é exatamente o caso que produzia o 401.
- Variáveis mantidas no ambiente: a senha é reaplicada em cada restart, por
  design; `.env.example:32-39` orienta removê-las após a recuperação.
- Nenhuma credencial, hash, token ou dado pessoal extra aparece em log, resposta
  de erro, mensagem de teste ou artefato do ciclo.
- DB real de produção, login ao vivo, Deploy/Coolify, IdP externo e allowlist
  OIDC são independentes e permanecem **[NV]**.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE registrar o resultado da recuperação no startup sem
  expor segredo: `flora-api/src/state/app_state.rs:96-99` DEVE manter
  `let applied = ...` seguido de `tracing::info!(applied = applied, "Startup
  administrator password recovery")`. `apply_startup_password()` DEVE avisar
  quando o e-mail do env não corresponde a usuário
  (`bootstrap.rs:111-115`) e registrar a sobrescrita (`:130-132`), sem incluir
  senha, hash ou dados pessoais extras em nenhuma mensagem.

- **FR-002**: A recuperação DEVE ser acionada exclusivamente por
  `FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` — variáveis
  já configuradas em produção — sem nenhuma chave adicional. `BootstrapConfig`
  (`flora-core/src/config/bootstrap.rs:21-42`) DEVE manter `admin_email` e
  `admin_password` com `#[serde(default, skip_serializing)]` e `Debug`
  redactado; `validate()` (`:57-81`) continua validando presença, e-mail e senha
  de 15..1024 caracteres sem revelar valores. Ambas as variáveis são
  **temporárias** e devem ser removidas após a recuperação.

- **FR-003**: O sistema DEVE sobrescrever o hash sempre que ambas as variáveis
  estiverem presentes e o e-mail existir, **inclusive quando `password_hash` já
  for não nulo**: `UPDATE users SET password_hash = $2, password_updated_at = $3
  WHERE id = $1` (`bootstrap.rs:120-128`), sem condição `IS NULL`. O helper
  `set_password_hash_if_absent` (`user_repository.rs:93-110`) permanece, mas NÃO
  é a via de recuperação (comentário `bootstrap.rs:117-118`). A senha DEVE ser
  validada e hasheada via `PasswordService` (Argon2id) antes de qualquer escrita.

- **FR-004**: Os contratos DEVEM cobrir: (a) sem env → `applied = false`, nenhuma
  escrita; (b) env com e-mail inexistente → aviso e `applied = false`; (c) conta
  com hash pré-existente → hash substituído (o caso do 401) e
  `POST /api/v1/auth/password/login` com a senha do env retorna 200 com sessão;
  (d) conta sem hash → hash gravado; (e) 401 genérico inalterado para senha
  errada, usuário sem hash e conta inativa. NENHUM contrato DEVE conter a senha
  em claro em logs, respostas ou nomes de teste. O contrato anterior de
  idempotência deve ser ajustado à nova semântica de sobrescrita.

- **FR-005**: `.env.example:32-39` DEVE documentar `FLORA_BOOTSTRAP__ADMIN_EMAIL`
  e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` como credenciais temporárias, com a
  instrução de remover ambas após a recuperação e reiniciar, sem valores reais.
  Os gates do alvo DEVEM permanecer PASS e o diff NÃO DEVE tocar em `validate/`,
  `release/`, `STATE.md` ou `PROGRESSO*`. O fluxo OIDC, o contrato do
  `POST /bootstrap` (201/409), o `pg_advisory_xact_lock` e a classe de rate limit
  `PasswordLogin` permanecem inalterados.

### Key Entities

- **BootstrapConfig**: config de startup com `admin_email` e `admin_password`
  (redactado, `skip_serializing`); valida formato e complexidade sem revelar o
  segredo. As duas variáveis são a chave de recuperação, não uma chave nova.
- **apply_startup_password**: rotina de recuperação que sobrescreve o hash a cada
  startup enquanto as variáveis estiverem definidas; retorna `bool` observável.
- **UPDATE de recuperação**: `UPDATE users SET password_hash = $2,
  password_updated_at = $3 WHERE id = $1` (`bootstrap.rs:120-128`), sem
  condição `IS NULL`; substitui o helper condicional nesse caminho.
- **set_password_hash_if_absent**: helper de repositório mantido para outros
  usos, mas fora da via de recuperação.
- **Password login**: `POST /api/v1/auth/password/login`, inalterado; verifica o
  hash armazenado e devolve 401 genérico ou sessão no formato OIDC.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Com a conta `luis.granja@mdmv.com.br` pré-existente (hash não
  nulo) e as duas variáveis já configuradas, um restart sobrescreve o hash e
  `POST /api/v1/auth/password/login` com a senha do env retorna 200 com
  `session_token`; após remover as variáveis e reiniciar, não há nova
  sobrescrita e a senha recuperada continua válida.

- **SC-002**: Nenhuma credencial aparece em log, resposta, erro, teste ou
  artefato do ciclo. O resultado `applied` e o motivo do skip são observáveis no
  startup (`app_state.rs:96-99`, `bootstrap.rs:111-115,130-132`) sem expor senha.

- **SC-003**: Contratos e specs passam, o gate do alvo é PASS (registrado no
  handoff) e o diff não toca em `validate/`, `release/`, `STATE.md` ou
  `PROGRESSO*`; OIDC, `POST /bootstrap`, advisory lock e rate limit seguem
  inalterados. O estado real do DB de produção e o login ao vivo permanecem
  **[NV]**.

## Assumptions

- O diagnóstico é o da investigação determinística: a conta de produção já tem
  hash gravado pelo bootstrap de 096 e a recuperação anterior recusava
  sobrescrever, explicando o 401 e a ausência de sinal nos logs.
- A correção do env é informação do relato; nenhuma credencial real foi lida,
  registrada ou reproduzida neste ciclo.
- `FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` são as
  credenciais temporárias de recuperação; sua remoção após o uso é parte do
  contrato operacional documentado.
- O estado real do DB de produção, o login ao vivo, o filtro de logging,
  Deploy/Coolify, IdP externo e allowlist OIDC são **[NV]** e pertencem a outros
  ciclos; esta especificação cobre apenas o contrato local observável.
- As verificações e o estado do gate são os registrados no handoff consolidado;
  este artefato não reexecutou gates, build, testes ou formatters.
