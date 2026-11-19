# Implementation Plan: 071 — remover stub morto AuthService

**Entrada:** `spec.md`, `clarify.md`, `research.md`, `decision.md` (go).

## Passos

1. **T001** `git rm flora-core/src/services/auth_service.rs`. *(FR-001, SC-003)*
2. **T002** `flora-core/src/services/mod.rs` — remover `mod` e `pub use`. *(FR-001, SC-001)*
3. **T003** `flora-api/src/state/app_state.rs` — remover import, campo, init e struct literal. *(FR-002, SC-001)*
4. **T004** `flora-api/src/extractors/auth.rs` + 6 contratos — remover imports e inicializadores. *(FR-003, SC-001)*
5. **T005** verificar `grep` zero, `cargo check/test` e preservação de `ServiceUnavailable`/`AuthProvider`; corrigir até verde. *(FR-004, SC-001, SC-002)*
6. **T006** `STATE.md`, `analyze.md`, `converge.md`. *(FR-005)*
7. **T007** `validate.md` (R4), `release.md` (R7), gates, PR, merge, higiene. *(SC-002, SC-003)*

## Gates

```
grep -rn 'AuthService\|auth_service' --include='*.rs' .   # -> 0
python3 scripts/ci/check_gate_integrity.py               # local + --remote
cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources; cargo build --workspace; cargo test --workspace
frontend: check/lint/test:unit/build
git diff --name-only dev..HEAD
```

## Rollback

Reverter os commits do PR em `dev/` com novo commit de reversão (sem force-push e sem reescrita). Nenhum dado, schema, workflow ou produção a restaurar.
