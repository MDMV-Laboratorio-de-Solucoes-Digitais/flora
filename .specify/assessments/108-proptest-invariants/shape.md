# Shape: 108 — proptest

- `proptest!` em módulos `#[cfg(test)]` já existentes (role) e novos (models/mod.rs, config/bootstrap.rs).
- Estratégias: `prop::sample::select` para enum de permissões; `any::<String>()` filtrada para não-grants; `prop_oneof!` para limites extremos; `prop::option::of` para e-mails.
- Sem novos `#[ignore]`; validação por `cargo test -p flora-core`, `PROPTEST_CASES=500` e gate.
