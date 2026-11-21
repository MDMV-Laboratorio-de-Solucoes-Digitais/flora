# Intake: 100 — recuperação do administrador real via opt-in explícito (wildcard `*`)

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/100-admin-recovery@9cf6775a7bd46395648ca3491536535373e0de6b` · `.specify/feature.json` → `specs/100-admin-recovery`

## Sinal

Após o ciclo 098, o relato de produção permanece bloqueado, mas com uma causa
nova e determinística: `GET /api/v1/bootstrap/status` reporta
`available = false` (já existem organizações, logo o `POST /bootstrap` não pode
recriar o administrador), `FLORA_BOOTSTRAP__ADMIN_EMAIL` e
`FLORA_BOOTSTRAP__ADMIN_PASSWORD` estão presentes e corretos no ambiente, e o
startup loga `applied = false`. O aviso emitido em
`flora-core/src/config/bootstrap.rs:112-114` — "FLORA_BOOTSTRAP__ADMIN_EMAIL
does not match an existing user; startup password skipped" — fecha a cadeia: o
e-mail configurado **não existe** na tabela `users`. O e-mail real do
administrador existente é diferente e não é conhecido de quem opera a
recuperação.

O diagnóstico não é intermitente: a recuperação de 098 faz uma busca exata por
e-mail (`repo.find_by_email(&email)`, `bootstrap.rs:111`) e, quando não há
correspondência, registra o aviso e retorna `Ok(false)` sem escrita
(`:112-116`). Com o env "correto" apontando para um e-mail inexistente, esse é
exatamente o resultado esperado do código. É necessária uma via de recuperação
que não exija saber o e-mail do administrador real — e que só seja ativada por
um opt-in explícito, para que um erro de digitação no e-mail nunca troque a
senha de uma conta aleatória.

## Evidência

```
ESTADO OBSERVÁVEL (relato de produção):
GET /api/v1/bootstrap/status     available = false  -> organizações já existem
FLORA_BOOTSTRAP__ADMIN_EMAIL     presente e correto no ambiente (valor não lido)
FLORA_BOOTSTRAP__ADMIN_PASSWORD  presente e correto no ambiente (valor não lido)
startup                          applied = false  (app_state.rs:99)
                                 aviso "ADMIN_EMAIL does not match an existing user"
                                 (bootstrap.rs:112-114)

CÓDIGO QUE PRODUZ O applied = false:
flora-core/src/config/bootstrap.rs:102-107   exige AMBAS as variáveis (Ok(false) se faltar)
flora-core/src/config/bootstrap.rs:108       e-mail normalizado com trim().to_lowercase()
flora-core/src/config/bootstrap.rs:110-116   find_by_email -> None => warn + Ok(false), sem UPDATE
flora-core/src/config/bootstrap.rs:119-128   hash Argon2id + UPDATE users SET password_hash ...
flora-core/src/config/bootstrap.rs:57-81     validate(): (None,None) ok; par exige e-mail válido
                                              (com '@' e domínio) e senha 15..1024 — hoje REJEITA '*'
flora-core/src/traits/repository.rs:16-45    UserRepository: find_by_email/find_by_id/create/update...
                                              (nenhum método "primeiro usuário ativo")
flora-api/src/state/app_state.rs:93-99       let applied = ...; tracing::info!(applied, ...)
flora-api/src/routes/auth.rs:451-460         verify_or_dummy -> 401 InvalidCredentials genérico
flora-tests/src/contract/bootstrap.rs:441-483
                                             status unavailable com organização pré-existente
flora-tests/src/contract/bootstrap.rs:340-407
                                             db_startup_password_overwrites_existing_hash
flora-tests/src/contract/auth.rs:1104-1173   login 200 / 401 genérico / sem hash / inativo
.env.example:32-39                           documenta as duas variáveis como temporárias
```

Nenhuma credencial real foi lida, registrada ou transmitida nesta investigação.
A correção das variáveis e o valor do e-mail configurado são informações do
relato; os valores secretos não são reproduzidos neste artefato.

## Autorização

Modo autônomo. Decisão `go` registrada em
`.specify/assessments/100-admin-recovery/decision.md` antes da implementação.
Escopo: um **opt-in explícito** — `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` (wildcard
literal `*`), combinado com `FLORA_BOOTSTRAP__ADMIN_PASSWORD` — passa a selecionar
o **primeiro usuário ativo** (`is_active = true`, ordenado por `created_at ASC,
id ASC`) e a sobrescrever o hash dessa conta de forma **idempotente**. Qualquer
outro valor continua sendo interpretado como e-mail exato: se não existir,
`applied = false` e nenhuma conta é alterada — **sem fallback** em caso de typo.
O resultado e a seleção são registrados sem segredo; testes e `.env.example`
cobrem o novo contrato. As variáveis continuam temporárias e devem ser removidas
após a recuperação. Rota de login, contrato do `POST /bootstrap`, rate limit e
fluxo OIDC permanecem inalterados. O DB real de produção e o login ao vivo
permanecem **[NV]**.

## Fronteira

Backend Rust (`flora-core/src/config/bootstrap.rs` — validação do token `*` e
seleção do primeiro usuário ativo; `flora-api/src/state/app_state.rs` —
observabilidade), contratos em `flora-tests`, documentação de env
(`.env.example`) e artefatos do ciclo. Sem toques em `validate/`, `release/`,
`.specify/memory/STATE.md` ou `PROGRESSO*`. Nenhuma mudança de gate, workflow,
ADR ou constituição. Nenhuma senha real em log, resposta, teste ou artefato.
