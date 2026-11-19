# Implementation Plan: 074 — fechamento pós-merge do ciclo 072

**Entrada**: `spec.md`, `clarify.md`, `research.md`, `decision.md` (go).
**Ordem**: um arquivo por commit, paths explícitos.

## Passos

1. **T001** Capturar `sha256`/bytes do prefixo de `.specify/memory/STATE.md` antes da edição. *(FR-003, SC-003)*
2. **T002** Anexar nota datada com merge de #295 (`c16be5c3`, `04:50:23Z`) e checks observados. *(FR-001, FR-004, SC-001)*
3. **T003** Anexar citação a #294 (`407d26dd`, `04:44:19Z`) e ao waiver do SE. *(FR-002, SC-001)*
4. **T004** Verificar prefixo idêntico e `git diff --name-only`. *(FR-003, SC-002, SC-003)*
5. **T005** `analyze.md` READ-ONLY e `converge.md` append-only. *(FR-005)*
6. **T006** Gates locais, `validate.md` (R4), `release.md` (R7), PR via `gh` sem merge. *(FR-004, SC-004)*

## Gates

```
python3 scripts/ci/check_gate_integrity.py
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
cargo build --workspace
cargo test --workspace
frontend: bun run check / lint / test:unit -- --run --project server / build
sha256(prefixo STATE) antes/depois
```

## Rollback

Reverter os commits deste PR em `dev/` com novo commit de reversão; nada de código, schema, required check ou produção é alterado.
