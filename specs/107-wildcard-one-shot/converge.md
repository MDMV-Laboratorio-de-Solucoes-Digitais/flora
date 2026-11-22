# Converge: 107 — wildcard one-shot

**Data**: 2026-11-22 · **Modo**: append-only

## Entrada 1 — implementação

**Status: Converged.**

- Migration `20261122000001_bootstrap_wildcard_one_shot.sql`: tabela singleton com `consumed_at`/`consumed_user_id`.
- `flora-core/src/services/bootstrap_recovery.rs`: seleção por papel, leitura do marcador e CAS sob advisory lock.
- `apply_startup_password` e `password_login`: usam o helper; negação 401 genérica.
- Testes: helper `grant_admin` nos contratos existentes (sem novos `#[ignore]`); teste unitário da SQL.
- ADR `0007` com expiração em 2026-11-29.

### Correções durante a implementação

Imports do teste de bootstrap reconstruídos após edição parcial; `uuid::Uuid` → `Uuid`; resultados de `Membership`/advisory lock atribuídos a `_` para `-D unused-results`.

### Fronteira

Nenhum gate/required check/formato de sessão/credencial real alterado; nenhum branch de terceiros tocado.
