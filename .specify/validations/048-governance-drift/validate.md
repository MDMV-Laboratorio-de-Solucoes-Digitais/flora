# Validation: 048 — Governance drift

**Data:** 2026-11-17T14:38Z · **Baseline:** `github/dev@4ae58ccd2`
**Tipo:** correção de documentação governante; sem código, tag, release ou deploy.

## Gates (executados)

| Gate | Resultado |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 (`bans ok, licenses ok, sources ok`) |
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 — 111 passed / 0 failed / 44 ignored |
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 required checks, pisos 43 |
| `bun run check` | 0 erros, 0 warnings |
| `bun run lint` | Prettier + ESLint limpos |
| `bun run test:unit -- --run --project server` | 39/39 |
| `bun run build` | exit 0 |

## Evidência da correção

```
grep -nE 'flora-tasks|flora-files|Toolchain pinned `1.98.0`' AGENTS.md
→ linha 4 contém apenas a NEGAÇÃO ("There are no `flora-tasks`/`flora-files` crates");
  nenhuma afirmação de 7 crates permanece.
→ linha 7: "Toolchain pinned `1.98.1` (per Cargo.toml:19 and the
  rust-toolchain-pin-bump-1-98-1 bug fix)".

cargo metadata --no-deps (flora*):
['flora-core','flora-api','flora-messaging','flora-search','flora-tests','flora']
```

SDD nativo: completeness PASS (38 specs), task-ID por arquivo PASS (440), DAG PASS
(0 arestas), converge append-only PASS (0 artefatos nesta base — vacuo).

## R4 — Veredito

**Approved (evidência local determinística; remoto [NV]).** R1 `go (scoped)` presente;
R4 aprovado antes de R7. Nenhum gate RED.

## NV / não feito

CI/PR/branch protection ao vivo, merges e tag: **[NV]**. PR #262 permanece em aberto
em branch distinto e não foi tocado. Os 4 bug reports em `.specify/bugs/` já estão
`applied`/`verified` — nenhum bug em aberto para fluxo paralelo.
