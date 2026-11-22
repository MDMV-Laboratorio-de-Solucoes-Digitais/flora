# Test Plan: D11

- **Unitário (não-ignored)**: predicado de papel e allowlist; regex/selector exato `*`; decisão consumido vs não-consumido.
- **DB-gated existentes**: estender `flora-tests/src/contract/bootstrap.rs` (wildcard targeting) e `contract/auth.rs` (login wildcard) com usuário sem papel administrativo → 401 e segundo login após consumo → 401, sem aumentar as contagens 13/23.
- **Determinístico**: `cargo test -p flora-core`, `cargo test -p flora-api`, `check_gate_integrity.py` (10 legs / 77 pisos), `git diff --check`.
- **[NV]**: staging real, credenciais reais e restart multi-réplica.
