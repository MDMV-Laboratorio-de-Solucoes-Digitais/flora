# Define: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

**Data:** 2026-11-21 · **Branch**: `feature/100-admin-recovery@9cf6775a7bd46395648ca3491536535373e0de6b`

## Problema

Em produção, `GET /api/v1/bootstrap/status` informa `available = false`
(organizações já existem), as variáveis `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
`FLORA_BOOTSTRAP__ADMIN_PASSWORD` estão presentes e corretas, mas o startup loga
`applied = false` com o aviso de que o e-mail não corresponde a usuário
(`flora-core/src/config/bootstrap.rs:110-116`). A recuperação de 098 faz busca
**exata** por e-mail; o e-mail configurado não existe em `users` e o e-mail real
do administrador é desconhecido de quem opera. Não há como reaplicar a senha
pelo e-mail configurado, e o `POST /bootstrap` está fechado.

A solução é um **opt-in explícito**: o valor literal `*` em
`FLORA_BOOTSTRAP__ADMIN_EMAIL`, combinado com `FLORA_BOOTSTRAP__ADMIN_PASSWORD`,
seleciona o **primeiro usuário ativo** (`is_active = true`, `ORDER BY created_at
ASC, id ASC LIMIT 1`) e sobrescreve o hash dessa conta de forma **idempotente**.
O token `*` é inválido como e-mail, portanto não pode aparecer por engano numa
configuração normal: qualquer outro valor continua sendo tratado como e-mail
exato e, se não existir, resulta em `applied = false` sem escrita — **sem
fallback** em caso de typo. O resultado e a conta selecionada são registrados sem
segredo; testes e `.env.example` documentam o contrato. As variáveis seguem
temporárias e devem ser removidas após a recuperação. Login, bootstrap, rate
limit, OIDC e gates não mudam.

## Requisitos funcionais

- **FR-001 — wildcard `*` explícito**: `BootstrapConfig::validate()`
  (`flora-core/src/config/bootstrap.rs:57-81`) DEVE aceitar o token literal `*`
  como valor de `admin_email` quando `admin_password` também estiver presente,
  sem afrouxar a validação de e-mail para nenhum outro valor. O token DEVE ser
  comparado de forma exata após `trim()` (`:67`): somente `*` ativa a seleção;
  strings como `luis.granja@mdmv.com.br*`, `*@x.com` ou um e-mail com typo NÃO
  ativam. `is_configured()` (`:44-49`) permanece válido para o par. Nenhuma
  chave de configuração nova é introduzida.

- **FR-002 — seleção primeiro usuário ativo**: com o opt-in ativo,
  `apply_startup_password()` (`bootstrap.rs:96-132`) DEVE selecionar o primeiro
  usuário ativo com consulta determinística — `SELECT id FROM users WHERE
  is_active = true ORDER BY created_at ASC, id ASC LIMIT 1` — e usar esse `id`
  na sobrescrita. Sem usuário ativo, DEVE registrar aviso e retornar
  `Ok(false)` sem escrita. Fora do opt-in, a busca permanece exata via
  `repo.find_by_email()` (`:111`); se não houver correspondência, aviso e
  `Ok(false)` (`:112-116`). NUNCA deve haver fallback da busca por e-mail para a
  seleção por wildcard.

- **FR-003 — sobrescrita idempotente**: a escrita DEVE continuar sendo
  `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE id = $1`
  (`bootstrap.rs:120-128`), incondicional quanto a `password_hash`, com a senha
  validada e hasheada via `PasswordService` (Argon2id) antes do UPDATE
  (`:78,119`). Reaplicar o mesmo opt-in/senha em startups sucessivos DEVE
  produzir o mesmo resultado funcional (login com a senha do env) e retornar
  `applied = true`, sem criar usuários, sessões ou efeitos colaterais. O helper
  `set_password_hash_if_absent` (`user_repository.rs:93-110`) permanece fora da
  via de recuperação.

- **FR-004 — log sem segredo**: `flora-api/src/state/app_state.rs:93-99` DEVE
  manter `let applied = ...` seguido de `tracing::info!(applied = applied,
  "Startup administrator password recovery")`. `bootstrap.rs` DEVE registrar o
  motivo do skip (e-mail inexistente ou nenhum usuário ativo, `:112-116`) e a
  conta sobrescrita via `user_id` (`:130`), sem incluir senha, hash ou o valor
  do e-mail real em nenhuma mensagem. `Debug` redactado (`:31-42`) e
  `skip_serializing` (`:27`) permanecem.

- **FR-005 — testes e `.env.example`**: contratos em
  `flora-tests/src/contract/bootstrap.rs` (`db_startup_password_overwrites_existing_hash`,
  `:340-407`) e testes unitários de `BootstrapConfig` (`bootstrap.rs:135-169`)
  DEVEM cobrir: opt-in `*` com seleção e sobrescrita; e-mail exato inexistente →
  aviso e `applied = false`; typo → sem fallback; nenhum usuário ativo →
  `Ok(false)`; senha inválida → erro de configuração; e ausência de segredo em
  log/resposta. `.env.example:32-39` DEVE documentar `*` como opt-in explícito,
  o critério "primeiro usuário ativo" e a remoção das variáveis após o login,
  sem valores reais. Gates do alvo DEVEM permanecer PASS e o diff NÃO DEVE tocar
  em `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`; OIDC, `POST /bootstrap`,
  advisory lock e rate limit `PasswordLogin` seguem inalterados.

## Critérios de sucesso

- **SC-001 — login em produção**: com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e a senha
  temporária no env, um restart seleciona o administrador real (primeiro usuário
  ativo), sobrescreve o hash e `POST /api/v1/auth/password/login` com o e-mail
  real da conta e a senha do env retorna 200 com sessão no formato OIDC. A
  recuperação em si não exigiu conhecer o e-mail; a remoção das variáveis e um
  novo restart não desfazem a senha recuperada.

- **SC-002 — opt-in explícito e sem fallback em typo**: somente o valor literal
  `*` ativa a seleção por usuário ativo. Um e-mail configurado com erro de
  digitação continua sendo buscado exatamente; sem correspondência, o startup
  registra o aviso, mantém `applied = false` e **nenhuma** conta é alterada.
  Nenhum cenário de typo resulta em sobrescrita.

- **SC-003 — nenhuma credencial em log/resposta**: nenhuma senha ou hash aparece
  em log, resposta, erro, teste ou artefato do ciclo. `applied`, o motivo do
  skip e o `user_id` selecionado são observáveis sem expor a credencial ou o
  e-mail real da conta.

## Fora de escopo

Mudar o contrato do `POST /bootstrap` (201/409), o `pg_advisory_xact_lock`, a
emissão de sessão, o formato do 401 genérico, o JWT, a classe de rate limit ou o
fluxo OIDC; adicionar nova variável de configuração, UI, fluxo de "esqueci minha
senha" por e-mail ou API pública de troca de senha; configurar IdP externo;
alterar gates, workflows, ADRs ou constituição; tocar em `validate/`, `release/`
ou estado de governança. Nenhuma senha real é fornecida ou lida neste ciclo.
