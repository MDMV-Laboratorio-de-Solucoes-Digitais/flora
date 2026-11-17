# Validation: 052 — OSV scan ref pin

**Data:** 2026-11-17T23:44Z · **Baseline:** `github/dev@01d822205`
**Tipo:** correção de ref em workflow advisory; sem required checks, autorização ou deploy.

## Gates (executados na worktree, 2026-11-17T23:42–23:43Z)

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 required checks, pisos 43 |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 — bans ok, licenses ok, sources ok |
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 — 105 passed / 0 failed / 51 ignored (soma dos alvos) |
| `bun run check` | 0 erros, 0 warnings |
| `bun run lint` | exit 0 (Prettier + ESLint) |
| `bun run test:unit -- --run --project server` | 42/42 (12 arquivos) |
| `bun run build` | exit 0 |
| `grep -n 'reusable.yml@' .github/workflows/osv-scan.yml` | `14:…@v2.7.1` |

## Evidência

- `git ls-remote … 'refs/heads/v2' 'refs/tags/v2'` → vazio; `v2.7.1` →
  `32d3cd482bdadfb38b186c0ad1b031902d55d932` (2026-11-17T23:32Z).
- Docs oficiais Google (23:33Z) recomendam `@v2.7.1`; workflow no tag expõe
  `scan-args` e `fail-on-vuln`, preservados.
- Diff de código: 1 arquivo, 1 linha. Nenhum required check, ADR ou constituição alterado.

## SDD

`analyze` PASS após correção na fonte; R1 `go (scoped)`; FR 4/4, SC 3/3,
T001–T004 concluídas; `converge.md` append-only com entrada 1.
Scripts SDD da factory executados em `/tmp` (não materializados) com baselines
neutros: completeness/task-id/DAG RED por divergência de layout — **informativo,
não gate do alvo**; converge append-only PASS com baseline derivado do alvo.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR ao vivo [NV]).** R4 aprovado
antes de R7. CI do PR será observada ao vivo no Bloco 7 antes de qualquer merge.

## NV

Execução real do job OSV após o pin, estado final de checks/PR/aprovações: **[NV]**.
Sem merge/auto-merge/tag/deploy nesta fase.
