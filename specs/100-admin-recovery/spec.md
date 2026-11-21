# Feature Specification: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

**Feature Branch**: `feature/100-admin-recovery`

**Created**: 2026-11-21

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/100-admin-recovery/decision.md` e diagnóstico determinístico do `applied = false` em produção (após 098)

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Recuperar o acesso sem saber o e-mail do administrador (Priority: P1)

Como operador de uma instalação já configurada, preciso recuperar a senha do
administrador real mesmo sem conhecer o e-mail cadastrado, porque
`GET /api/v1/bootstrap/status` informa `available = false` (organizações já
existem) e o e-mail configurado no env não corresponde a nenhum usuário.

**Why this priority**: é o bloqueio relatado. A recuperação de 098 faz busca
exata (`flora-core/src/config/bootstrap.rs:110-116`); com o e-mail do env
inexistente, o startup registra aviso, retorna `applied = false` e não escreve.
O `POST /bootstrap` está fechado porque já há organização
(`flora-tests/src/contract/bootstrap.rs:441-483`).

**Independent Test**: com organizações pré-existentes e um usuário ativo cujo
e-mail é desconhecido do operador, definir `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e
`FLORA_BOOTSTRAP__ADMIN_PASSWORD`, reiniciar e confirmar que o hash do primeiro
usuário ativo foi sobrescrito e que `POST /api/v1/auth/password/login` com o
e-mail real e a senha do env retorna 200 com sessão.

**Acceptance Scenarios**:

1. **Given** organizações pré-existentes, env presente com
   `ADMIN_EMAIL=*` e senha temporária válida, e ao menos um usuário ativo,
   **When** o startup roda, **Then** o primeiro usuário ativo (por `created_at
   ASC, id ASC`) tem o hash sobrescrito, `applied = true` é logado e o login com
   o e-mail real e a senha do env retorna 200 com `session_token`, `expires_in` e
   `next_step`.
2. **Given** o mesmo opt-in e a mesma senha, **When** o startup roda
   novamente, **Then** a sobrescrita é reaplicada de forma idempotente
   (`applied = true`), sem criar usuários, sessões ou efeitos colaterais, e o
   login continua funcionando.
3. **Given** as variáveis removidas após a recuperação, **When** o startup roda
   novamente, **Then** nenhuma sobrescrita acontece e a senha recuperada
   permanece válida.
4. **Given** env com `ADMIN_EMAIL` igual a um e-mail inexistente (sem `*`),
   **When** o startup roda, **Then** o aviso de e-mail inexistente é registrado,
   `applied = false` e nenhuma conta é alterada.

---

### User Story 2 — Ativar a recuperação apenas por opt-in explícito, sem fallback em typo (Priority: P1)

Como mantenedor, preciso que a seleção automática de conta só ocorra quando o
operador escrever literalmente `*`, para que um erro de digitação no e-mail não
troque a senha de um usuário qualquer.

**Why this priority**: o token `*` é inválido como e-mail, portanto é
impossível que surja por engano numa configuração normal. Sem isso, uma busca
que falhasse e "caísse" para o primeiro usuário ativo poderia sequestrar a
conta errada em produção.

**Independent Test**: Configurar e-mails com typo, `*` dentro de outro valor e
um e-mail válido inexistente; em todos os casos confirmar que a busca é exata,
que `applied = false` e que nenhuma conta é alterada. Somente `*` seleciona.

**Acceptance Scenarios**:

1. **Given** `ADMIN_EMAIL=luis.granj@mdmv.com.br` (typo) e senha válida,
   **When** o startup roda, **Then** `find_by_email` não encontra usuário, o
   aviso é registrado, `applied = false` e **nenhuma** conta é sobrescrita —
   não há fallback para o primeiro usuário ativo.
2. **Given** `ADMIN_EMAIL=*@example.com` ou `luis.granja@mdmv.com.br*`,
   **When** o startup roda, **Then** o valor é tratado como e-mail (inválido
   ou inexistente), sem ativar o opt-in; `validate()` falha para e-mail
   inválido ou a busca exata falha, sem escrita.
3. **Given** `ADMIN_EMAIL=*` e nenhum usuário ativo, **When** o startup roda,
   **Then** aviso específico é registrado e `apply_startup_password()` retorna
   `Ok(false)` sem escrita.
4. **Given** env incompleta (só e-mail ou só senha), **When** o startup roda,
   **Then** `BootstrapConfig::validate()` falha com erro de configuração e
   nenhuma escrita é feita, sem a senha na mensagem.

---

### User Story 3 — Observar o resultado sem expor credenciais e preservar a fronteira (Priority: P2)

Como mantenedor, preciso que o startup informe se a recuperação agiu e qual
conta foi selecionada, sem revelar a senha, e que o restante da superfície de
autenticação permaneça idêntico.

**Why this priority**: o log de `applied` (`flora-api/src/state/app_state.rs:93-99`)
é o único sinal operacional; e o 401 genérico, o JWT, o rate limit e o
`POST /bootstrap` são parte da superfície de segurança existente.

**Independent Test**: executar os estados relevantes e conferir `applied` e os
motivos de skip/sobrescrita em log sem senha, hash ou e-mail real; executar os
contratos de bootstrap e login e conferir que o 401 permanece idêntico para
senha errada, usuário sem hash e conta inativa, que o `POST /bootstrap` continua
201/409 sob advisory lock e que o diff não toca em `validate/`, `release/`,
`STATE.md` ou `PROGRESSO*`.

**Acceptance Scenarios**:

1. **Given** opt-in com sobrescrita bem-sucedida, **When** o startup roda,
   **Then** `flora-core/src/config/bootstrap.rs:130` registra a sobrescrita com
   `user_id`, sem senha, hash ou e-mail real, e `applied = true` aparece em
   `app_state.rs:99`.
2. **Given** uma senha errada, um usuário sem hash ou uma conta inativa,
   **When** o login local é chamado, **Then** a resposta é o mesmo 401 genérico
   de hoje (`flora-api/src/routes/auth.rs:451-460`).
3. **Given** o `POST /api/v1/bootstrap`, **When** ele é chamado em banco vazio ou
   já configurado, **Then** retorna 201/409 sob o mesmo lock e transação.
4. **Given** o diff do ciclo, **When** os gates do alvo são consolidados,
   **Then** eles permanecem PASS e nenhum arquivo proibido é alterado.

---

### Edge Cases

- Sem nenhuma variável de bootstrap: `apply_startup_password()` retorna `false`
  e não escreve no banco.
- Env incompleta: `validate()` falha; nenhuma escrita parcial e nenhum valor de
  senha na mensagem de erro.
- `ADMIN_EMAIL=*`: token de opt-in, aceito somente com senha presente; o e-mail
  é comparado após `trim()`, de forma exata (`:67`).
- E-mail normalizado com `trim().to_lowercase()` (`bootstrap.rs:108` e
  `auth.rs:451`) — a busca exata é a mesma nos dois caminhos.
- Múltiplos usuários ativos: o opt-in escolhe o mais antigo por `created_at ASC,
  id ASC`; critério determinístico e documentado.
- Nenhum usuário ativo: aviso e `Ok(false)` sem escrita.
- Conta com hash pré-existente: sobrescrita incondicional
  (`bootstrap.rs:120-128`) — é exatamente o caso do 401 anterior.
- Variáveis mantidas no ambiente: a senha é reaplicada em cada restart, por
  design; `.env.example:32-39` orienta removê-las após a recuperação.
- Nenhuma credencial, hash, token ou e-mail real da conta aparece em log,
  resposta de erro, mensagem de teste ou artefato do ciclo.
- DB real de produção, login ao vivo, Deploy/Coolify, IdP externo e allowlist
  OIDC são independentes e permanecem **[NV]**.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001 — wildcard `*` explícito**: `BootstrapConfig::validate()`
  (`flora-core/src/config/bootstrap.rs:57-81`) DEVE aceitar o token literal `*`
  como `admin_email` quando `admin_password` estiver presente, sem afrouxar a
  validação para qualquer outro valor. A comparação é exata após `trim()`
  (`:67`): somente `*` ativa a seleção; `*@x.com`, `foo*` ou um e-mail com typo
  NÃO ativam. `is_configured()` (`:44-49`) continua válido para o par e nenhuma
  chave de configuração nova é introduzida.

- **FR-002 — seleção primeiro usuário ativo**: com o opt-in,
  `apply_startup_password()` (`bootstrap.rs:96-132`) DEVE selecionar o primeiro
  usuário ativo com `SELECT id FROM users WHERE is_active = true ORDER BY
  created_at ASC, id ASC LIMIT 1` e usar esse `id` na sobrescrita; sem usuário
  ativo, aviso e `Ok(false)`. Fora do opt-in, a busca permanece exata via
  `repo.find_by_email()` (`:111`); sem correspondência, aviso e `Ok(false)`
  (`:112-116`). NUNCA há fallback da busca por e-mail para a seleção por
  wildcard.

- **FR-003 — sobrescrita idempotente**: a escrita DEVE ser
  `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1`
  (`bootstrap.rs:120-128`), incondicional quanto a `password_hash`, com senha
  validada e hasheada via `PasswordService` (Argon2id) antes do UPDATE
  (`:78,119`). Reaplicar o mesmo opt-in/senha em startups sucessivos DEVE
  produzir o mesmo resultado funcional e `applied = true`, sem criar usuários,
  sessões ou efeitos colaterais. `set_password_hash_if_absent`
  (`user_repository.rs:93-110`) permanece fora da via de recuperação.

- **FR-004 — log sem segredo**: `flora-api/src/state/app_state.rs:93-99` DEVE
  manter `let applied = ...` seguido de `tracing::info!(applied = applied,
  "Startup administrator password recovery")`. `bootstrap.rs` DEVE registrar o
  motivo do skip (e-mail inexistente ou nenhum usuário ativo, `:112-116`) e a
  conta sobrescrita via `user_id` (`:130`), sem incluir senha, hash ou o e-mail
  real em nenhuma mensagem. `Debug` redactado (`:31-42`) e `skip_serializing`
  (`:27`) permanecem.

- **FR-005 — testes e `.env.example`**: contratos em
  `flora-tests/src/contract/bootstrap.rs` (`db_startup_password_overwrites_existing_hash`,
  `:340-407`) e testes unitários de `BootstrapConfig` (`bootstrap.rs:135-169`)
  DEVEM cobrir: opt-in `*` com seleção e sobrescrita; e-mail exato inexistente →
  aviso e `applied = false`; typo → sem fallback; nenhum usuário ativo →
  `Ok(false)`; senha inválida → erro de configuração; ausência de segredo em
  log/resposta; e login 200 com a senha recuperada
  (`flora-tests/src/contract/auth.rs:1104-1173`). `.env.example:32-39` DEVE
  documentar `*` como opt-in explícito, o critério de seleção e a remoção das
  variáveis após o login, sem valores reais. Gates do alvo DEVEM permanecer PASS
  e o diff NÃO DEVE tocar em `validate/`, `release/`, `STATE.md` ou
  `PROGRESSO*`; OIDC, `POST /bootstrap`, advisory lock e rate limit
  `PasswordLogin` seguem inalterados.

### Key Entities

- **BootstrapConfig**: config de startup com `admin_email` e `admin_password`
  (redactado, `skip_serializing`); valida formato/complexidade sem revelar o
  segredo. `admin_email = *` é o token de opt-in, não uma chave nova.
- **apply_startup_password**: rotina de recuperação. Com `*`, seleciona o
  primeiro usuário ativo; sem `*`, mantém a busca exata de 098. Retorna `bool`
  observável.
- **Seleção do primeiro usuário ativo**: `SELECT id FROM users WHERE is_active =
  true ORDER BY created_at ASC, id ASC LIMIT 1` — determinística e usada apenas
  sob opt-in.
- **UPDATE de recuperação**: `UPDATE users SET password_hash = $2,
  password_updated_at = $3 WHERE id = $1` (`bootstrap.rs:120-128`) —
  incondicional, idempotente quanto ao resultado funcional.
- **Password login**: `POST /api/v1/auth/password/login`, inalterado; verifica o
  hash armazenado e devolve 401 genérico ou sessão no formato OIDC.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001 — login em produção**: com `ADMIN_EMAIL=*` e a senha temporária no
  env, um restart seleciona o administrador real (primeiro usuário ativo),
  sobrescreve o hash e `POST /api/v1/auth/password/login` com o e-mail real e a
  senha do env retorna 200 com sessão. A recuperação não exigiu conhecer o
  e-mail; remover as variáveis e reiniciar não desfaz a senha recuperada.

- **SC-002 — opt-in explícito e sem fallback em typo**: somente o valor literal
  `*` ativa a seleção por usuário ativo. Um e-mail com erro de digitação
  continua sendo buscado exatamente; sem correspondência, o startup registra o
  aviso, mantém `applied = false` e nenhuma conta é alterada. Nenhum cenário de
  typo resulta em sobrescrita.

- **SC-003 — nenhuma credencial em log/resposta**: nenhuma senha ou hash aparece
  em log, resposta, erro, teste ou artefato do ciclo. `applied`, o motivo do
  skip e o `user_id` selecionado são observáveis sem expor a credencial ou o
  e-mail real da conta. Contratos e specs passam, o gate do alvo é PASS
  (registrado no handoff) e o diff não toca em `validate/`, `release/`,
  `STATE.md` ou `PROGRESSO*`; OIDC, `POST /bootstrap`, advisory lock e rate
  limit seguem inalterados. O estado real do DB de produção e o login ao vivo
  permanecem **[NV]**.

## Assumptions

- O diagnóstico é o da investigação determinística: organizações existem, o env
  aponta para um e-mail inexistente e a recuperação de 098 faz busca exata,
  explicando o `applied = false` e o aviso no startup.
- A correção das variáveis é informação do relato; nenhuma credencial real foi
  lida, registrada ou reproduzida neste ciclo.
- `ADMIN_EMAIL=*` é um opt-in operacional: o operador deve removê-lo (junto com
  a senha) após a recuperação, pois enquanto estiver definido a senha é
  reaplicada a cada startup.
- Quando houver mais de um usuário ativo, o critério "primeiro" é o `created_at`
  mais antigo, desempatado por `id`; o administrador da instalação original é o
  esperado na produção relatada.
- O estado real do DB de produção, o login ao vivo, o filtro de logging,
  Deploy/Coolify, IdP externo e allowlist OIDC são **[NV]** e pertencem a outros
  ciclos; esta especificação cobre apenas o contrato local observável.
- As verificações e o estado do gate são os registrados no handoff consolidado;
  este artefato não reexecutou gates, build, testes ou formatters.
