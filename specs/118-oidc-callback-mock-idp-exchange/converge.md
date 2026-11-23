# Converge: 118 — servidor IdP OIDC HTTP mock e endurecimento do callback

**Data**: 2026-11-23 · **Modo**: append-only · **Status: Converged (pré-CI)**

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#399](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/399) (`flora-api/src/oidc.rs`, `flora-api/src/routes/auth.rs`)
- `T-002` → [#400](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/400) (`flora-tests/Cargo.toml`, `flora-tests/src/lib.rs`, `flora-tests/src/oidc_mock.rs`)
- `T-003` → [#401](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/401) (`flora-tests/src/integration/registration.rs`, `flora-tests/src/contract/auth.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`)
- `T-004` → [#402](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/402) (`docs/governance/2026-11-23-118-briefing-compreensao.md`, governança, docs append-only, R4/R7 `0.10.1`, `test-roadmap.md`/`evidence.md`, ACR)

## Entrega e verificação local (`task-1190`)

- **TDD RED → GREEN**: execução pré-implementação falhou em compilação (`exit 101`: `unresolved import exchange_code_and_verify_id_token`); após refatoração de `oidc.rs`, endurecimento em `auth.rs` e implementação de `MockOidcIdpServer` (`oidc_mock.rs`), toda a suíte passou sem ressalvas:
  - `cargo fmt --all -- --check`: exit `0`.
  - `cargo clippy --workspace --all-targets -- -D warnings`: `0` erros.
  - `cargo deny check bans licenses sources`: `bans ok, licenses ok, sources ok` (zero novas crates no `Cargo.lock`).
  - `cargo test --workspace`: `flora-api` `48 passed` (incl. `7/7` em `oidc::tests`), `flora-core` `35 + 21 passed`, `flora-tests` `36 passed; 0 failed; 82 ignored` (incl. `test_registration_flow_with_mock_oidc` e `test_registration_flow`).
  - `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` (PostgreSQL 18.7 `127.0.0.1:15433`): **`25 passed; 0 failed`** (`55.34s`), incluindo `db_oidc_callback_exchanges_code_registers_new_user_and_creates_session` e `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject`.
  - `python3 scripts/ci/check_gate_integrity.py`: `gate-integrity: PASS`, `all 10 db contract floors intact (82 total)` (`auth: 25`).
- **Briefing de Compreensão (`auth`)**: preenchido em `docs/governance/2026-11-23-118-briefing-compreensao.md`.
- **[NV]**: execução remota de `db-tests`, `rust-build`, `dependency-review`, ACR independente e troca HTTP contra provedor Zitadel externo real.

## Rodada 2 — Remediação ACR PR #403 (`task-1323`, append-only)

- **PR `#403` Rodada 1 (`4d21e79c5`)**: CI remoto (`11/11` concluídos + `1` skip push-only); ACR `72547119` (`REPROVADO`) apontou `email_verified` ausente, não persistência de `oidc_subject` no vínculo por e-mail de conta local (`NULL`), bypass de prefixo `http://localhost` / redirecionamentos HTTP / issuer no Discovery e vazamento de detalhes `reqwest`/`Url` em `Error::OidcProvider`.
- **Correções e re-verificação local (`task-1323`)**:
  - `flora-core/src/{traits/repository.rs,repositories/user_repository.rs}` (`UserRepository::set_oidc_subject_if_absent`), `flora-api/src/{oidc.rs,routes/auth.rs}`, `flora-tests/{Cargo.toml,src/oidc_mock.rs,src/integration/registration.rs,src/contract/auth.rs}`.
  - `cargo fmt --all -- --check`: exit `0`; `cargo clippy --workspace --all-targets -- -D warnings`: exit `0`; `cargo deny check bans licenses sources`: `bans ok, licenses ok, sources ok` (`Cargo.lock` removeu aresta `base64` não usada de `flora-tests`).
  - `cargo test --workspace`: `flora-api` `50 passed` (incl. `9/9` `oidc::tests`), `flora-core` `35 + 21 passed`, `flora-tests` `36 passed; 0 failed; 82 ignored`.
  - `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1`: **`25 passed; 0 failed`** (`77.95s`); `python3 scripts/ci/check_gate_integrity.py`: **`gate-integrity: PASS`** (`82` total).
- **[NV]**: ACR Rodada 2 + CI remoto no commit remediado e troca HTTP contra provedor Zitadel externo real.
