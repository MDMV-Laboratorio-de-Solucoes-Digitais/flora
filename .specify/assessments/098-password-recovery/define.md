# Define: 098 — recuperação da senha do administrador bootstrap via env

**Data:** 2026-11-21 · **Branch**: `feature/098-password-recovery@76f936e245f70a7bd2735887e7ff8bc5e0c5c06f`

## Problema

Em produção (096), o login local com `luis.granja@mdmv.com.br` retorna 401 mesmo
com `FLORA_BOOTSTRAP__ADMIN_EMAIL` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` presentes
e corretos, e sem sinal da recuperação nos logs. A conta já nasceu com
`password_hash` não nulo (`flora-api/src/routes/bootstrap.rs:294-295`) e a
recuperação anterior recusava sobrescrever esse hash
(`flora-core/src/config/bootstrap.rs` e
`repositories/user_repository.rs:93-110`, `WHERE password_hash IS NULL`); o login
comparava a senha enviada com o hash antigo e devolvia 401 genérico
(`flora-api/src/routes/auth.rs:451-460`); o chamador descartava o retorno e o
skip não logava (`flora-api/src/state/app_state.rs`). A correção usa as próprias
variáveis já configuradas como credenciais **temporárias de recuperação**: com
ambas presentes, o hash é sobrescrito a cada startup, inclusive sobre credencial
existente, e o resultado é logado sem segredo. Como efeito operacional, as
variáveis devem ser removidas após a recuperação para que a senha temporária não
seja reaplicada no próximo restart. O contrato de login, o bootstrap, o rate
limit e os gates não mudam.

## Requisitos funcionais

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

## Critérios de sucesso

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

## Fora de escopo

Mudar o contrato do `POST /bootstrap`, o advisory lock ou a emissão de sessão;
alterar o formato do 401 genérico, o JWT, a classe de rate limit ou o fluxo OIDC;
adicionar nova variável de configuração, nova interface de usuário, fluxo de
"esqueci minha senha" por e-mail ou API pública de troca de senha; configurar
IdP externo; mudar gates, workflows, ADRs ou constituição; tocar em `validate/`,
`release/` ou estado de governança. Nenhuma senha real é fornecida ou lida neste
ciclo.
