# Research: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

**Data:** 2026-11-21 · **Evidência local determinística + relato de produção** · DB real/login ao vivo **[NV]** · branch `feature/100-admin-recovery@9cf6775a7bd46395648ca3491536535373e0de6b`

## Primária (local)

1. `flora-core/src/config/bootstrap.rs:96-107` — `apply_startup_password()`
   valida a config (`:101`) e exige **ambas** as variáveis; se alguma falta,
   retorna `Ok(false)` sem escrita. As variáveis estão presentes em produção,
   logo este ramo não explica o `applied = false` relatado.
2. `flora-core/src/config/bootstrap.rs:108` — o e-mail do env é normalizado com
   `trim().to_lowercase()`, o mesmo critério do login (`auth.rs:451`). A busca é
   exata: `repo.find_by_email(&email)` (`:111`) não faz correspondência parcial
   nem por domínio.
3. `flora-core/src/config/bootstrap.rs:110-116` — quando `find_by_email` retorna
   `None`, a rotina emite `tracing::warn!` ("does not match an existing user;
   startup password skipped") e retorna `Ok(false)`. **Este é o ramo observado em
   produção**: o e-mail configurado não existe em `users`; nenhuma linha é
   alterada.
4. `flora-core/src/config/bootstrap.rs:119-128` — com e-mail existente, a senha é
   hasheada via `PasswordService` (Argon2id) e o hash é gravado com `UPDATE users
   SET password_hash = $2, password_updated_at = $3 WHERE id = $1`. `applied` é
   `rows_affected() > 0` (`:128`) e a sobrescrita é logada com `user_id` (`:130`),
   sem senha nem hash. A escrita permanece incondicional — é a base da
   idempotência da sobrescrita.
5. `flora-core/src/config/bootstrap.rs:57-81` — `validate()`: `(None, None)` é
   válido; o par exige e-mail no formato usual (`@`, local/domínio não vazios e
   domínio com `.`) e senha de 15..1024 caracteres
   (`PasswordService::validate_password`, `:78`). Um `*` literal **hoje é
   rejeitado** (`:73-77`): para o opt-in precisamos aceitar exclusivamente o
   token `*` como valor especial de `admin_email`, mantendo a validação de
   e-mail para todo o resto.
6. `flora-core/src/config/bootstrap.rs:44-49,21-29` — `is_configured()` só exige
   `Some` em ambos os campos, então `*` + senha continua habilitando a
   recuperação; `Debug` imprime `[REDACTED]` para a senha (`:31-42`) e o campo
   tem `skip_serializing` (`:27`). Nenhuma chave de configuração nova é
   necessária: o próprio `ADMIN_EMAIL` passa a aceitar o token de opt-in.
7. `flora-core/src/traits/repository.rs:16-45` — `UserRepository` oferece
   `find_by_email`, `find_by_id`, `create`, `update`, `deactivate` e
   `set_password_hash_if_absent`, mas **nenhum** método "primeiro usuário
   ativo". A seleção do opt-in será uma consulta pontual no próprio caminho de
   recuperação: `SELECT id FROM users WHERE is_active = true ORDER BY created_at
   ASC, id ASC LIMIT 1`. A ordem determinística (o mais antigo primeiro) evita
   ambiguidade e é coerente com o fato de o administrador real ter sido criado
   na instalação original.
8. `flora-api/src/state/app_state.rs:93-99` — o retorno já é observável:
   `let applied = ...` seguido de `tracing::info!(applied = applied, "Startup
   administrator password recovery")`. O log do resultado permanece; apenas a
   semântica de `applied = true` passa a incluir o caso do wildcard.
9. `flora-api/src/routes/auth.rs:451-460` — `verify_or_dummy()` devolve o mesmo
   `Error::InvalidCredentials` (401 genérico) para senha errada, usuário sem
   hash e conta inativa. A recuperação não altera esse contrato: ela apenas
   faz o hash armazenado passar a corresponder à senha do env.
10. `flora-tests/src/contract/bootstrap.rs:340-407` —
    `db_startup_password_overwrites_existing_hash` já cobre a busca exata e a
    sobrescrita; `:441-483` cobre `bootstrap/status` indisponível com
    organização pré-existente — é o estado de produção relatado e a razão pela
    qual `POST /bootstrap` não é uma saída.
11. `.env.example:32-39` — documenta as duas variáveis como temporárias e
    orienta removê-las após o login. A documentação passa a explicar o token
    `*` como opt-in explícito, o critério de seleção e o risco de um typo
    (que **não** ativa a seleção).

## Leitura do relato de produção

- `GET /api/v1/bootstrap/status` → `available = false`: organizações existem;
  o caminho de bootstrap está fechado (contrato em
  `flora-tests/src/contract/bootstrap.rs:441-483`).
- Env `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `FLORA_BOOTSTRAP__ADMIN_PASSWORD`
  presentes e corretos (conforme relato; valores secretos não lidos nem
  reproduzidos).
- Startup com `applied = false` e aviso de e-mail inexistente
  (`bootstrap.rs:112-114`): consequência direta da busca exata por um e-mail
  que não está em `users`.
- Os três fatos são mutuamente consistentes com o código: não é necessário
  invocar falha de rede, migração ausente ou valor de env incorreto.
- O e-mail real do administrador é desconhecido de quem opera — daí a necessidade de
  um opt-in que não dependa dele. O valor `*` é escolhido como token explícito
  justamente por ser **inválido como e-mail** e, portanto, impossível de
  surgir por engano numa configuração de e-mail normal.

## Diagnóstico

Cadeia fechada: organizações existem → `POST /bootstrap` indisponível → env com
e-mail inexistente → `find_by_email` retorna `None` → aviso e `applied = false`,
sem escrita. A recuperação de 098 resolveu o caso "e-mail existe, mas já tem
hash"; não resolve "e-mail configurado não existe". A extensão de 100 é
deliberadamente estreita: o token literal `*` em `ADMIN_EMAIL` (com
`ADMIN_PASSWORD` presente) é um **opt-in explícito** que seleciona o primeiro
usuário ativo por ordem determinística e sobrescreve o hash de forma
idempotente. Qualquer outro valor — inclusive um typo — continua sendo
interpretado como e-mail exato; sem correspondência, `applied = false` e
nenhuma conta é alterada. Assim o mecanismo não pode ser ativado por acidente.

## [NV]

- Estado real do DB de produção (e-mail real do administrador, `is_active`,
  `created_at`, valor efetivo das variáveis) e o login ao vivo: **[NV]**.
  Nenhuma credencial foi lida e nenhum `POST /bootstrap` ou `/password/login`
  foi executado contra produção nesta investigação.
- Filtro de logging real, Deploy/Coolify, IdP externo e allowlist OIDC: **[NV]** —
  independentes deste ciclo e não observados.
- Correção não executada contra produção neste artefato; gates, build e testes
  são evidência consolidada do handoff. **[NV]**
- Cobertura idiomática: pt-BR nos artefatos; inglês em nomes de código e nas
  mensagens de log existentes, seguindo a convenção do repositório.
