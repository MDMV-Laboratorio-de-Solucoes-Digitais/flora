# Define: 096 — login local pt-BR para conta bootstrap

**Data:** 2026-11-21 · **Branch**: `feature/096-auth-login-ptbr@1910d1c7d`

## Problema

A instalação criada via bootstrap tem uma conta administradora, mas nenhum meio
de autenticá-la: o único fluxo existente depende de um IdP cujos defaults são
`localhost:8080`, a allowlist de redirects está vazia e o frontend envia um
redirect que é rejeitado. Sem fallback de senha, um deploy sem IdP é um deploy
sem login. A correção precisa ser segura por construção (hash forte, falha
genérica, sem vazamento de credencial) e utilizável em português.

## Requisitos funcionais

- **FR-001**: criar migration SQL versionada em `flora-core/src/migrations/sql/`
  adicionando `users.password_hash` (TEXT, nullable) e
  `users.password_updated_at` (TIMESTAMPTZ, nullable), preservando contas OIDC
  existentes com `password_hash IS NULL`.
- **FR-002**: implementar `PasswordService` com Argon2id `m=19456`, `t=2`,
  `p=1`, serialização PHC, verificação contra dummy hash quando o usuário ou o
  hash não existe e resposta `401` genérica idêntica em todos os casos de
  falha; segredos em memória devem ser limpos.
- **FR-003**: `POST /api/v1/bootstrap` passa a exigir `admin_password`, validar
  complexidade e gravar o hash Argon2id na mesma transação que cria
  organização, usuário, papel, associação, workspace e sessão — sem escrita
  parcial em caso de erro.
- **FR-004**: expor `POST /api/v1/auth/password/login` com e-mail e senha,
  verificando o hash via FR-002 e emitindo token JWT e sessão com o mesmo
  escopo organizacional do fluxo OIDC.
- **FR-005**: recuperar credencial no startup por
  `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `FLORA_BOOTSTRAP__ADMIN_PASSWORD`, apenas
  quando a conta existir com `password_hash IS NULL`; idempotente, sem
  sobrescrever hash já definido e sem registrar a senha em log.
- **FR-006**: aplicar rate limiting à rota de login local, seguindo o padrão de
  classes existente em `flora-api/src/middleware/rate_limit.rs`, sem alterar as
  classes de bootstrap/OIDC já vigentes.
- **FR-007**: implementar na tela de login do frontend um formulário "Sign In"
  local em pt-BR (e-mail + senha) que chama FR-004, mantendo a opção SSO
  existente e exibindo mensagem genérica em caso de falha.
- **FR-008**: trocar `lang="en"` por `lang="pt-BR"` em
  `apps/frontend/src/app.html` e traduzir para pt-BR as strings da tela de login,
  inclusive erros e estados de envio.

## Critérios de sucesso

- **SC-001**: a conta bootstrap (ex.: `luis.granja@mdmv.com.br`) consegue entrar
  em produção com a senha definida no bootstrap ou via recuperação por env; o
  login retorna `session_token` com escopo organizacional correto e a sessão é
  aceita nas rotas autenticadas.
- **SC-002**: e-mail inexistente, senha errada e usuário sem hash retornam o
  mesmo `401` genérico — sem enumeração de conta, sem hash, sem diferença
  mensurável de resposta; nenhuma credencial aparece em log, resposta ou
  artefato do ciclo.
- **SC-003**: a migration aplica em banco vazio e em banco já populado sem
  perda de dados; a recuperação por env é idempotente; gates do alvo passam e o
  diff não toca em `analyze/`, `converge/`, `validate/`, `release/`,
  `STATE.md` ou `PROGRESSO*`.

## Fora de escopo

Configurar ou substituir o IdP externo; alterar o contrato do fluxo OIDC
existente; mudar gates, workflows, ADRs ou a constituição; autorizar login por
outros meios (magic link, passkey); tocar em `validate/`, `release/` ou estado
de governança. IdP externo permanece **[NV]**.
