# Feature Specification: proptest para invariantes de domínio

**Branch**: `feature/108-proptest-invariants` · **Created**: 2026-11-22 · **Status**: Specified — go

## User Scenarios

### US1 — Permissões (P1)

Para toda permissão canônica, `Permission::from_str(&p.to_string())` preserva o valor; strings arbitrárias que não sejam `*` nem alias não concedem acesso.

### US2 — Paginação (P1)

Para todo `limit: usize` e `offset: usize`, `resolved_limit()` está em `0..=100` e é independente do offset.

### US3 — Seletor wildcard (P2)

Para toda string de e-mail, `is_wildcard_selector()` equivale a `trim() == "*"`.

## Requirements

- **FR-001** proptests não-ignorados em `flora-core`.
- **FR-002** sem `unwrap`/`expect`/`panic` explícitos (usar `prop_assert`).
- **FR-003** nenhum piso do gate alterado.

## Success Criteria

- **SC-001** `cargo test -p flora-core` passa; **SC-002** `PROPTEST_CASES=500` passa; **SC-003** gate PASS (10 legs / 77 pisos).

## [NV]

Nenhuma mudança de produção; cobertura adicional é evidência local.
