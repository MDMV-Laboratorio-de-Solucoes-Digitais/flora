# Plan: 118 — servidor IdP OIDC HTTP mock e endurecimento do callback

**Data**: 2026-11-23 · **Baseline**: `github/dev@2ef8fd97e`

## Mapeamento FR/SC → Arquivos e Verificação

| FR / SC | Arquivos alvo | Verificação |
|---|---|---|
| FR-001 / FR-002 / SC-001 / SC-002 | `flora-api/src/oidc.rs`, `flora-api/src/routes/auth.rs` | `cargo test -p flora-api oidc::` (`8+` unitários) + `cargo clippy --workspace --all-targets -- -D warnings` |
| FR-003 / SC-003 | `flora-tests/Cargo.toml`, `flora-tests/src/lib.rs`, `flora-tests/src/oidc_mock.rs` | `cargo deny check bans licenses sources && cargo test -p flora-tests` |
| FR-004 / SC-003 / SC-004 | `flora-tests/src/integration/registration.rs`, `flora-tests/src/contract/auth.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml` | `python3 scripts/ci/check_gate_integrity.py` (`82` total) + `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` (`25/25`) |
| FR-005 / SC-005 | `2026-11-23-118-briefing-compreensao.md`, `2026-11-23-118-oidc-callback-mock-idp-exchange.md`, `converge.md`, `.specify/validations/118-*/*`, `.specify/releases/118-*/release.md`, `STATE.md`, `PROGRESSO.md`, `OBJETIVO.md`, `ROADMAP.md`, `research/index.md` | `git diff --check`, recontagem `find`, ACR independente e CI remoto verde |

## Ordem de Execução

1. Abrir 4 issues R6 (`T-001`..`T-004`) no GitHub e finalizar `analyze.md` (somente leitura) antes de editar código.
2. Em TDD (`RED → GREEN`), implementar `MockOidcIdpServer` (`oidc_mock.rs`) + testes reais de falha/`registration.rs` + `2` contratos DB (`contract/auth.rs`), verificar falha pré-implementação (`RED`) onde aplicável (`inactive`/`conflicting sub`/`EdDSA` refactor), concluir `oidc.rs` e endurecimento em `auth.rs` (`GREEN`).
3. Atualizar trava lockstep `auth: 23 -> 25` em `check_gate_integrity.py` e `.github/workflows/rust.yml`.
4. Executar todos os gates locais + Postgres 18 (`auth 25/25`) + preencher Briefing `auth`, R4, R7 (`0.10.1`) e docs append-only; abrir PR para `dev`, concluir ACR + CI remoto, mesclar em `dev` e atualizar PR `#371` (`release/112-promotion` → `main`).
