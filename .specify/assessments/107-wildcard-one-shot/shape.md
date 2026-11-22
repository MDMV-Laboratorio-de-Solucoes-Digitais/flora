# Shape: 107 — wildcard one-shot

- Migration singleton `bootstrap_wildcard_state` com `consumed_at`/`consumed_user_id`.
- Helper SQL único em `flora-core` para selecionar administrador built-in e consumir o marcador sob `pg_advisory_xact_lock`.
- Startup e login usam o mesmo helper; nenhuma duplicação de consulta.
- Consumo na mesma transação da emissão da sessão; falha → rollback → 401 genérico.
- ADR `0007` com expiração em 2026-11-29 e quatro gates de reentrada/rotação.
- Testes: unitários puros + extensão de contratos existentes, sem novos `#[ignore]`.
