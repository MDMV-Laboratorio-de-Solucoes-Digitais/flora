# Clarify: 070 — contrato RBAC real

**Data:** 2026-11-19 · **R3:** máx. 3 perguntas, uma por vez.

## Esclarecimentos necessários

**Nenhum.** O comportamento esperado está nos handlers existentes e em `STATE.md` (decisão SE do ciclo 066: `RoleWrite` dedicada, `OrgAdmin` habilitada, built-ins imutáveis).

## Decisões derivadas sem perguntar

1. **Não mudar os handlers**: se o contrato revelar defeito, ele será registrado como bug/dívida, não corrigido neste ciclo (escopo go scoped).
2. **Nomes `db_rbac_*`**: necessários para casar com o filtro `contract::rbac::db_` do leg `db-tests`.
3. **DB descartável próprio**: `flora_070_rbac_verify`, criado e a ser removido por esta sessão; bancos `flora`, `flora_022_test`, `flora_023_test` e `floraaudit012*` permanecem intactos.
4. **Fail-closed**: sem env, `bail!` — nunca `Ok(())` (correção do ciclo 059).
