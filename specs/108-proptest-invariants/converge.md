# Converge: 108 — proptest

**Data**: 2026-11-22 · **Modo**: append-only · **Status: Converged**

- `flora-core/src/models/role.rs`: módulo `proptests` com round-trip `Permission` e não-concessão de strings arbitrárias (`prop_assume!`).
- `flora-core/src/models/mod.rs`: módulo `proptests` com limites de `Pagination` e independência do offset.
- `flora-core/src/config/bootstrap.rs`: módulo `proptests` cobrindo `is_wildcard_selector()` com `trim()`.
- Nenhum `#[ignore]`, nenhuma dependência nova, nenhum gate alterado.

Fronteira preservada; sem credenciais.
