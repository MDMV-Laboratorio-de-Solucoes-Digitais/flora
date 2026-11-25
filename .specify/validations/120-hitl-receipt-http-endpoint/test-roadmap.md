# Test roadmap — ciclo 120

**Data**: 2026-11-24 · **Baseline**: `github/dev@e47983874` · **Superfície**: CLI / Rust HTTP router (`POST /api/v1/hitl/receipts`) + PostgreSQL 18 (zero alterações em `apps/frontend`)

## Escopo

FR-001..FR-004 · SC-001..SC-003 · Domínio crítico `auth / rbac / hitl` (`#411`–`#413`), trava lockstep `82 -> 85` (`hitl: 3`).

## Pré-condições

- `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target`.
- Postgres 18 ativo (`FLORA_DATABASE__POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:15433/flora'`).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | Pré-implementação (`TDD RED`): `cargo test -p flora-tests contract::hitl::` antes de criar `hitl.rs` ou após registrar contrato | Falha esperada (`unresolved module` / `404` antes de montar rota) | CLI / Cargo | sim |
| 2 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` | Formatação limpa e `0` warnings sob `pedantic` / lints estritos | CLI / Rust | sim |
| 3 | `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok` (zero dependências novas) | CLI / Supply-chain | sim |
| 4 | `cargo test --workspace` | Todos os unitários e integração passam (incl. teste `503` em `flora-core` e `3` testes unitários em `flora-api::routes::hitl`) | CLI / Unitários | sim |
| 5 | `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` | `gate-integrity: PASS` (`11` legs / `85` pisos com `hitl: 3`), `deploy-workflows: PASS`, `--self-test` PASS | CLI / Gate | sim |
| 6 | `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` | **`3 passed; 0 failed`** contra Postgres 18 (`201` HMAC + `operator_id` servidor, `401`/`403`/`422` spoof, `422` decisão inválida e `503` sanitizado) | HTTP (`oneshot`) + PG 18 | sim |
| 7 | `gh pr checks <PR> --watch` + inspeção de logs (`rust-build`, `db-tests` `85/85`, `frontend-build`, `e2e-browser`) + ACR independente (`research`, `pro`) | Todos required checks GREEN e ACR `APROVADO` | CI / GitHub | sim |
| 8 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Produção | não (`[NV]`) |

## Dispensa de capturas de UI (Bloco 0.3)

Ciclo puramente backend Rust/HTTP (`flora-core`, `flora-api`, `flora-tests`, CI floors); nenhuma alteração em `apps/frontend`, dispensando capturas visuais de tela e baseando a evidência nos logs CLI/HTTP/DB e runs do GitHub Actions.

## Rollback

Novo commit de reversão removendo `flora-api/src/routes/hitl.rs` e `flora-tests/src/contract/hitl.rs`, restaurando `flora-core/src/error.rs`, `flora-api/src/routes/mod.rs`, `flora-tests/src/contract/mod.rs`, e retornando em lockstep `hitl: 3` (`85 -> 82`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
