# Implementation Plan: 070 — contrato RBAC real

**Entrada:** `spec.md`, `clarify.md`, `research.md`, `decision.md` (go).
**Ordem:** um arquivo por commit, paths explícitos.

## Passos

1. **T001** escrever `flora-tests/src/contract/rbac.rs` com `mod db` + `db_state()` fail-closed + helpers + 3 testes `#[ignore]`. *(FR-001, FR-002, FR-003, FR-004, SC-001)*
2. **T002** executar os 3 contratos no DB descartável `flora_070_rbac_verify`; corrigir a fonte até PASS; executar sem env para confirmar fail-closed. *(SC-001)*
3. **T003** `STATE.md` — apêndice de execução. *(FR-005)*
4. **T004** `analyze.md` (READ-ONLY) e `converge.md` (append-only). *(FR-005)*
5. **T005** `validate.md` (R4) e `release.md` (R7) após gates. *(SC-002)*
6. **T006** gates locais, PR, merge após CI verde, higiene. *(SC-002, SC-003)*

## Gates

```
FLORA_DATABASE__POSTGRES_URL=postgresql://postgres:postgres@127.0.0.1:5432/flora_070_rbac_verify \
  cargo test -p flora-tests contract::rbac::db_ -- --ignored --test-threads=1
cargo test -p flora-tests contract::rbac::db_ -- --ignored --test-threads=1   # sem env -> falha
python3 scripts/ci/check_gate_integrity.py            # local + --remote
cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources; cargo build --workspace; cargo test --workspace
frontend: check/lint/test:unit/build
```

## Rollback

Reverter os commits do PR em `dev/` com novo commit de reversão; nenhuma mudança de schema, dados ou produção. Remover o DB descartável `flora_070_rbac_verify` ao final.
