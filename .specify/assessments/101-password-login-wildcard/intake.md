# Intake: 101 — login com `*` quando a opt-in de bootstrap está ativa

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/101-password-login-wildcard@5890471be35ec21f7c8eba9545bca3544edbd8d9` · `.specify/feature.json` → `specs/101-password-login-wildcard`

## Sinal

Após o ciclo 100, o relato de produção mudou de natureza: `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` estão presentes, o hash do primeiro usuário ativo foi sobrescrito com sucesso (`applied = true`, `flora-api/src/state/app_state.rs:93-99`), e ainda assim `POST /api/v1/auth/password/login` retorna 401. A causa é determinística: o e-mail real da conta bootstrap é desconhecido de quem opera, e o handler de login faz busca **exata** por `req.email` (`flora-api/src/routes/auth.rs:451-453`). Enviar `email="*"` — o mesmo token que selecionou a conta no startup — não encontra usuário (`find_by_email` → `None`), `verify_or_dummy` gasta o trabalho dummy e a rota devolve `Error::InvalidCredentials` (`:457-460`).

A senha já está correta no banco; falta uma forma de usá-la sem conhecer o e-mail. A solução não é escrever mais uma vez no startup, nem adivinhar o endereço: é aceitar o próprio token `*` como alias de login **somente** enquanto a opt-in `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` estiver ativa, resolvendo para o mesmo primeiro usuário ativo (`created_at ASC, id ASC`) usado pela recuperação de 100. Fora da opt-in, `*` continua sendo um e-mail inválido e o 401 genérico permanece — nenhum caminho novo se abre.

## Evidência

```
ESTADO OBSERVÁVEL (relato de produção, após 100):
FLORA_BOOTSTRAP__ADMIN_EMAIL     = '*'  (opt-in explícita, já ativa)
FLORA_BOOTSTRAP__ADMIN_PASSWORD   presente (valor secreto não lido nem reproduzido)
startup                          applied = true  (hash gravado no 1º usuário ativo)
POST /api/v1/auth/password/login {"email":"*","password":...} -> 401 invalid_credentials
e-mail real da conta bootstrap   desconhecido de quem opera

CÓDIGO QUE PRODUZ O 401:
flora-api/src/routes/mod.rs:30             nest("/api/v1/auth", auth::create_auth_router())
flora-api/src/routes/auth.rs:80-88         rota /password/login -> password_login
flora-api/src/routes/auth.rs:447-492       handler password_login
flora-api/src/routes/auth.rs:451           email = req.email.trim().to_lowercase()
flora-api/src/routes/auth.rs:452-453       find_by_email(&email) — exata, sem token especial
flora-api/src/routes/auth.rs:455-457       stored_hash + verify_or_dummy (dummy quando None)
flora-api/src/routes/auth.rs:458-460       !password_ok || !is_active -> Error::InvalidCredentials (401)
flora-api/src/routes/auth.rs:463-491       sessão + JWT + {session_token, expires_in, user_id, next_step}
flora-api/src/middleware/rate_limit.rs:136,151,195,241-245  classe PasswordLogin (inalterada)

OPT-IN JÁ EXISTENTE (100):
flora-core/src/config/bootstrap.rs:26-32   BootstrapConfig { admin_email, admin_password (skip_serializing) }
flora-core/src/config/bootstrap.rs:35-46   Debug com [REDACTED] para a senha
flora-core/src/config/bootstrap.rs:56-60   is_wildcard_selector(): admin_email.trim() == "*"
flora-core/src/config/bootstrap.rs:69-95   validate(): aceita e-mail válido ou exatamente '*'
flora-core/src/config/bootstrap.rs:129-143 seleção: is_active = TRUE ORDER BY created_at ASC, id ASC LIMIT 1
flora-core/src/config/mod.rs:46-52,59-67   Figment FLORA_ + split("__"); Config::validate chama bootstrap.validate()
flora-api/src/state/app_state.rs:23-28     AppState.config: Arc<Config> acessível na rota
flora-tests/src/contract/auth.rs:1039-1183 suíte db_password_login (200, 401 genérico, sem hash, inativo)
flora-tests/src/contract/bootstrap.rs:410,568-598  seleção wildcard e nenhum usuário ativo
.env.example:36-45                         documenta a opt-in '*' e a remoção das variáveis
```

Nenhuma credencial real foi lida, registrada ou transmitida nesta investigação. O estado do env e o `applied = true` são informação do relato; os valores secretos não são reproduzidos neste artefato.

## Autorização

Modo autônomo. Decisão `go` registrada em `.specify/assessments/101-password-login-wildcard/decision.md` antes da implementação. Escopo: aceitar `email='*'` no `POST /api/v1/auth/password/login` **somente** quando `FLORA_BOOTSTRAP__ADMIN_EMAIL` for também `*` (a opt-in temporária de 100, já ativa em produção), resolvendo para o primeiro usuário ativo por `created_at ASC, id ASC` — a mesma seleção determinística da recuperação. Sem a opt-in, `*` não ativa nada: busca exata, 401 genérico, nenhuma conta alterada. O 401, a emissão de sessão em formato OIDC, o JWT, o rate limit `PasswordLogin`, o `POST /bootstrap` e o fluxo OIDC permanecem idênticos. Nenhuma senha, hash ou e-mail real em log/resposta; testes e `.env.example` cobrem o contrato. As variáveis continuam temporárias e devem ser removidas após o login. DB real e login ao vivo permanecem **[NV]**.

## Fronteira

Backend Rust (`flora-api/src/routes/auth.rs` — alias `*` sob opt-in; eventual helper compartilhado em `flora-core` para a seleção determinística), contratos em `flora-tests`, documentação de env (`.env.example`) e artefatos do ciclo. Sem toques em `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`. Nenhuma mudança de gate, workflow, ADR ou constituição. Nenhuma senha real em log, resposta, teste ou artefato.
