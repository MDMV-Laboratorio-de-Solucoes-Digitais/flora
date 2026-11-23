# Analyze: 118 (READ-ONLY)

**Data**: 2026-11-23 · **Baseline**: `github/dev@2ef8fd97e`

## Checagem cruzada (`spec.md` ↔ `plan.md` ↔ `tasks.md` ↔ GitHub `#399`..`#402`)

- **Rastreabilidade FR → Tasks → Issues R6**:
  - `FR-001` / `FR-002` / `SC-001` / `SC-002` → `T-001` → [#399](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/399) (`flora-api/src/oidc.rs`, `flora-api/src/routes/auth.rs`)
  - `FR-003` / `SC-003` → `T-002` → [#400](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/400) (`flora-tests/Cargo.toml`, `flora-tests/src/lib.rs`, `flora-tests/src/oidc_mock.rs`)
  - `FR-004` / `SC-003` / `SC-004` → `T-003` → [#401](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/401) (`flora-tests/src/integration/registration.rs`, `flora-tests/src/contract/auth.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`)
  - `FR-005` / `SC-005` → `T-004` → [#402](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/402) (Briefing `auth`, governança, docs append-only, R4/R7 `0.10.1`, `test-roadmap.md`/`evidence.md`, ACR)
- **Lockstep & Dependências**:
  - `FLOORS["auth"]` e `run_leg auth contract::auth::db_ ...` sobem em par `23 -> 25` (total `80 -> 82`), preservando os 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`).
  - Zero novas crates no `Cargo.lock` (`base64`, `jsonwebtoken`, `serde_urlencoded` já estão no `[workspace.dependencies]`).
- **Órfãos / Conflitos**: 0 requisitos órfãos; 0 tarefas órfãs; 0 conflitos com Constituição v1.4.0 ou ADR-0003.
- **Ordem normativa**: `tasks` concluído e sincronizado com `#399`..`#402` antes de `analyze`; `analyze` somente leitura antes de `12 implement`.
- **Resumo**: `fr=5/5 sc=5/5 tasks=4/4 critical=0 go=True`.

## [NV]

Execução remota de `db-tests` (Postgres 18), `rust-build`, `dependency-review` e fluxo OIDC contra Zitadel externo real.
