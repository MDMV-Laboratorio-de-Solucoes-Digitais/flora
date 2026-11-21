# Research: 098 — recuperação da senha do administrador bootstrap via env

**Data:** 2026-11-21 · **Evidência local determinística + relato de produção** · DB real/login ao vivo **[NV]** · branch `feature/098-password-recovery@76f936e245f70a7bd2735887e7ff8bc5e0c5c06f`

## Primária (local)

1. `flora-api/src/routes/bootstrap.rs:294-295` — o `POST /bootstrap` de 096 já
   exige `admin_password`, valida a complexidade e grava o hash na mesma
   transação atômica. A conta de produção, criada por esse caminho, **já nasce
   com `password_hash` não nulo** — o caso que a recuperação antiga excluía.
2. Diagnóstico do 401 (código anterior): `apply_startup_password()` retornava
   `Ok(false)` quando `user.password_hash.is_some()` e
   `flora-core/src/repositories/user_repository.rs:93-110` limitava a escrita a
   `WHERE password_hash IS NULL`. O chamador descartava o retorno e o skip não
   logava — daí o 401 e a ausência de sinal, com env correta.
3. `flora-api/src/routes/auth.rs:451-460` — o login normaliza o e-mail (`:451`),
   busca o usuário (`:452-453`), extrai `password_hash` (`:454-456`) e usa
   `verify_or_dummy()` (`:457`). Usuário inexistente, senha errada, conta sem
   hash e conta inativa retornam o mesmo `Error::InvalidCredentials` (`:458-461`)
   → **401 genérico**, sem distinção. Comparando a senha do env com o hash
   antigo, o 401 relatado é exatamente o que o código produzia.
4. `flora-api/src/middleware/rate_limit.rs:150,195,238-244` — classe
   `PasswordLogin` (5 requisições/60s por IP); a rota não está ausente nem
   bloqueada no caminho normal.
5. `flora-core/src/config/bootstrap.rs:1-7` — o módulo agora documenta as duas
   variáveis como **credenciais temporárias de recuperação**: quando ambas são
   definidas, elas sobrescrevem o hash do administrador **em CADA startup**,
   mesmo que já exista um hash; devem ser removidas após a recuperação. O
   segredo nunca é logado.
6. `flora-core/src/config/bootstrap.rs:21-42` — `BootstrapConfig` contém apenas
   `admin_email` e `admin_password`, com `#[serde(default, skip_serializing)]`
   na senha e `Debug` que imprime `[REDACTED]`. Não há chave adicional: as
   variáveis já configuradas em produção são a própria chave de recuperação.
7. `flora-core/src/config/bootstrap.rs:57-81` — `validate()` exige o par
   completo, e-mail válido e senha de 15..1024 caracteres, sem incluir valores
   nas mensagens de erro.
8. `flora-core/src/config/bootstrap.rs:96-134` — `apply_startup_password()`:
   valida (`:101`), retorna `Ok(false)` se falta env (`:102-107`), normaliza o
   e-mail (`:108`), avisa se não há usuário (`:111-115`), gera o hash (`:119`) e
   executa `UPDATE users SET password_hash = $2, password_updated_at = $3 WHERE
   id = $1` (`:120-128`) — **sem condição `IS NULL`**, portanto sobrescreve um
   hash existente. O comentário `:117-118` registra que o helper condicional
   silenciosamente ignoraria exatamente as contas que precisam de recuperação.
   Loga `info!` ao sobrescrever (`:131`), sem senha nem hash.
9. `flora-api/src/state/app_state.rs:96-99` — o retorno não é mais descartado:
   `let applied = ...` seguido de `tracing::info!(applied = applied, "Startup
   administrator password recovery")`. O resultado booleano é observável no
   startup.
10. `.env.example:32-39` — documenta `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
    `FLORA_BOOTSTRAP__ADMIN_PASSWORD` como temporárias, com instrução explícita
    de remover ambas após o login para que a senha não seja reaplicada no
    próximo restart; valores reais nunca comitados.
11. `flora-core/src/config/mod.rs:23,43-64` — `Config` contém
    `bootstrap: BootstrapConfig`; Figment com prefixo `FLORA_` e `split("__")`
    já mapeia as duas variáveis sem provider adicional.

## Leitura do relato de produção

- Login em `POST /api/v1/auth/password/login` com
  `luis.granja@mdmv.com.br` → **401** (conforme relato).
- `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `FLORA_BOOTSTRAP__ADMIN_PASSWORD` presentes e
  corretos no ambiente (conforme relato; valores secretos não lidos nem
  reproduzidos).
- Logs sem sinal da recuperação no estado anterior: filtro suprimia
  `flora_core`, o skip de "hash presente" não logava e o chamador descartava o
  retorno. O código atual passou a logar `applied` no startup
  (`app_state.rs:96-99`).
- Os três fatos são mutuamente consistentes com o código anterior: não é
  necessário invocar falha de rede, DNS, migração ausente ou env errado para
  explicar o 401.

## Diagnóstico

Cadeia fechada: conta com hash gravado no bootstrap → recuperação antiga recusava
sobrescrever (`password_hash IS NULL`) → login comparava com o hash antigo e
devolvia 401 genérico → nenhum sinal chegava ao chamador ou aos logs. A correção
implementada usa as **mesmas duas variáveis já configuradas** como credenciais
temporárias de recuperação: com ambas presentes e e-mail existente, o hash é
sobrescrito a cada startup, inclusive sobre uma credencial prévia; o resultado
é logado sem segredo. A natureza temporária é parte do contrato operacional:
remover as variáveis após a recuperação encerra a reaplicação.

## [NV]

- Estado real do DB de produção, valor efetivo das variáveis no serviço e o
  login ao vivo: **[NV]**. Nenhuma credencial foi lida e nenhum
  `POST /bootstrap` ou `/password/login` foi executado contra produção nesta
  investigação.
- Filtro de logging real, Deploy/Coolify, IdP externo e allowlist OIDC: **[NV]** —
  independentes deste ciclo e não observados.
- Correção não executada contra produção neste artefato; gates, build e testes
  são evidência consolidada do handoff. **[NV]**
- Cobertura idiomática: pt-BR nos artefatos; inglês em nomes de código, seguindo
  a convenção do repositório.
