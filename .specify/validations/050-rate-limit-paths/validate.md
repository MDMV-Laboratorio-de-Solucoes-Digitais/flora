# Validation: 050 — Rate limit paths

**Data:** 2026-11-17T16:20Z · **Baseline:** `github/dev@b3bfda505`
**Tipo:** correção de middleware Rust; sem mudança de limites, rotas ou API.

## Gates (executados)

| Gate | Resultado |
|---|---|
| `cargo test -p flora-api classifies` | exit 0 — 4 passed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 — 115 passed / 0 failed / 44 ignored |
| `python3 scripts/ci/check_gate_integrity.py` | PASS (4 required checks, pisos 43) |
| `bun run check` / `lint` / `test:unit -- --run --project server` / `build` | 0/0 · ok · 41/41 · ok |

## Evidência

Vermelho (antes): `/api/v1/auth/login`, `/api/v1/auth/refresh`, `/api/v1/users/invite`
classificados como `GENERAL`. Verde (depois): `Login`, `Refresh`, `Invite` e `/health`
→ `General`, provado por 4 testes puros.

Limites e janelas inalterados no diff: 5/60 login, 20/60 refresh, 10/3600 invite,
genérico = `state.default_limit/default_window`.

## SDD

`analyze` PASS sem conflitos; R1 `go (scoped)`; SDD nativo: completeness PASS,
task-ID por arquivo PASS, DAG PASS, converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; remoto [NV]).** R4 aprovado antes de R7.
Nenhum gate RED remanescente.

## NV

CI/PR/aprovações ao vivo: **[NV]**. Sem merge, auto-merge, tag ou deploy.
