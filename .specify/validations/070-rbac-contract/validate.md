# Validation: 070 — contrato RBAC real

**Data:** 2026-11-19T03:09Z · **Baseline:** `github/dev@0c7890ef4`
**Tipo:** testes DB-gated; nenhuma mudança de comportamento em produção.
**Veredito R4:** **APROVADO**.

## Gates e verificações (saídas coladas)

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`; 4 required checks; `all 8 db contract floors intact (51 total)`; exit 0 |
| `check_gate_integrity.py --remote` | `dev`/`main`: live required contexts match; exit 0; `[NV]` runner/Forgejo |
| `cargo fmt --all -- --check` | RED antes do `cargo fmt --all`; **exit 0** após aplicação e commit `style(rbac)` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished dev profile`; exit 0 |
| `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok`; exit 0 |
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | `passed=105 failed=0 ignored=55`; exit 0 |
| 3 contratos RBAC com DB descartável | `3 passed; 0 failed; 85 filtered out` — `FLORA_DATABASE__POSTGRES_URL=.../flora_070_rbac_verify` |
| 8 legs DB existentes (auth…workspace) | todos `ok`, pisos 13/4/8/8/5/6/4/3 preservados |
| RBAC sem env (fail-closed) | `FAILED ... FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests`; exit 101 |
| Teste de mutação | inverter a asserção cross-org → `Error: CROSS-ORG ROLE LEAK: [...]`; fonte restaurada byte-a-byte; `3 passed` novamente |
| Frontend check/lint/test/build | 0/0; Prettier/ESLint ok; 42/42; exit 0 |
| `analyze.md` | 1 conflito (FR-002/003 em notação de intervalo) corrigido na fonte → `go=True`, `errors=[]` |
| Verificador isolado | `PASS-WITH-NITS` — 4 pontos OK + 1 nit de redação; nit corrigido por notas append-only em `converge.md`/`STATE.md` |

## Fronteira verificada

- `git diff --name-only dev..HEAD` → apenas `flora-tests/src/contract/rbac.rs`, `.specify/memory/STATE.md`, `specs/070-rbac-contract/**`, `.specify/assessments/070-rbac-contract/**` (mais `validations`/`releases` deste ciclo).
- Zero alteração em `flora-api/`, `flora-core/`, migrations, `.github/workflows/`, `scripts/ci/`, ADRs ou constituição; nenhum piso/job alterado.
- Nenhum `unwrap`/`expect`/`panic` novo; lints do alvo respeitados (2 `unused_results` corrigidos com `let _membership`/`let _session`).
- Banco usado: `flora_070_rbac_verify` (criado por esta sessão); bancos de outras sessões intactos.

## Dívida / [NV]

- **O CI não executa estes 3 contratos**: não há leg `contract::rbac::db::db_` em `rust.yml`, e eles são `#[ignore]`. Adicioná-lo exige alterar `rust.yml` + `FLOORS` do guardião + ADRs (governança, aprov maintainer) — ciclo separado. **[NV]**.
- CI futura, Forgejo, produção, aprovações e execução do runner: **[NV]** — CI ao vivo observada no BLOCO 7 antes do merge.

## SDD

R1 `go (scoped)`; R3 zero perguntas; R4 este documento aprovado antes de `release.md`; R5 sem bug reportado; R7 release com SemVer/changelog/rollback. Nenhuma invocação de `/speckit.validate` ou `/speckit.release`.
