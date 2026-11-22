# Fix: D11

1. Migration `20261122000001_bootstrap_wildcard_one_shot.sql`: tabela singleton com `consumed_at`/`consumed_user_id` e linha inicial.
2. Serviço em `flora-core`: `find_wildcard_admin(pool)` (JOIN memberships/roles, `is_builtin`, `name IN ('Admin','Owner')`, deterministic `ORDER BY created_at, id`) e `try_consume_wildcard(tx, user_id)` com advisory lock + CAS.
3. `apply_startup_password`: usar `find_wildcard_admin` e não reaplicar hash se o marcador estiver consumido.
4. `password_login`: usar o mesmo helper; após `verify_or_dummy`, consumir o marcador antes de criar a sessão; falha → 401 genérico.
5. ADR `0007` com expiração em 2026-11-29 e quatro gates de desligamento/rotação.
6. Testes unitários puros + extensão de contratos existentes (sem novos `#[ignore]`).

**Não alterar**: gate, required checks, formato de sessão, rota de login, mensagens genéricas.
