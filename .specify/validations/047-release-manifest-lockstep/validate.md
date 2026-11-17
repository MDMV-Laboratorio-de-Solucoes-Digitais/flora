# Validation: 047 — Release manifest lockstep

**Data:** 2026-11-17T13:52Z · **Baseline:** `feature/047-release-manifest-lockstep@04bf70bdd`
**Tipo:** correção determinística de workflow (sem release/deploy)

## Gates (executados, saídas coladas)

| Gate | Resultado |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 (`bans ok, licenses ok, sources ok`; warnings de licença não-encontrada) |
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 — 101 passed / 0 failed (50 DB-gated `ignored`) |
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 required checks, pisos 43 intactos |
| frontend `bun run check` | exit 0 — 0 erros, 0 warnings |
| frontend `bun run lint` | exit 0 — Prettier + ESLint limpos |
| frontend `bun run test:unit -- --run --project server` | exit 0 — 11 arquivos, 40/40 |
| frontend `bun run build` | exit 0 |

## Evidência determinística do defect

```
export RELEASE_VERSION=0.0.0
RED_EXIT=1      # asserção pré-correção (length == 8)
GREEN_EXIT=0    # asserção corrigida (length == 6)
WORKFLOW_STEP_EXIT=0   # comando extraído do próprio release.yml
```

Metadata local: 6 pacotes `flora*` (`flora`, `flora-api`, `flora-core`,
`flora-messaging`, `flora-search`, `flora-tests`), todos `0.1.0` no checkout.
O teste verde usa uma cópia com versões alinhadas a `0.0.0` para simular a tag.

## SDD (substituição A — gates nativos do alvo)

```
sdd_completeness: PASS (37 specs)
task_id_uniqueness(final): PASS (434 definições top-level, 47 subtasks, 37 files)
work_order_dag: PASS (no edges declared)
converge_append_only: PASS (no converge artifacts; vacuous)
```
Reexecução restrita a 047: artefatos presentes, FR 4/4, SC 3/3, T001–T006 únicos.

## R4 — Veredito

**Approved (evidência local determinística; remoto [NV]).** R1 `go (scoped)` presente;
R4 aprovado antes de R7. Nenhum gate RED; nenhum ciclo em voo foi interrompido.

## NV / não feito

Execução do workflow em GitHub Actions, branch protection, tag/release e merges: **[NV]**.
Nenhum merge, auto-merge, tag, deploy, push de tag ou alteração de required check.
Dívidas de fase 043–046 e ausência de `plan.md` em 025–042 permanecem registradas,
fora deste ciclo.
