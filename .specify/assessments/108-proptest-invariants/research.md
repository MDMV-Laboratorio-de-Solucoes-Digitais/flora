# Research: 108 — proptest

- `proptest = "1.5"` em `Cargo.toml:208`, dev-dependency em `flora-core/Cargo.toml:59`, resolvido 1.11.0 no lock; zero `proptest!` no checkout.
- `check_gate_integrity.py:42-53,126-159` compara contagens exatas de testes `#[ignore]` (10 legs / 77); proptests não-ignorados em `flora-core` não alteram pisos.
- Invariantes puros: `Role::has_permission` (`models/role.rs:66-75`), `Pagination::resolved_limit` (`models/mod.rs:58-63`), `is_wildcard_selector` (`config/bootstrap.rs:59-63`).
- Lints: `unwrap_used`/`expect_used`/`panic` em deny — usar `prop_assert!`.
