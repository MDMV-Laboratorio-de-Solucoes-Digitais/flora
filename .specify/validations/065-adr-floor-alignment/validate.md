# Validation: 065 — ADR floor alignment

**Data:** 2026-11-18T13:09Z · **Baseline:** `github/dev@aef13277a`
**Tipo:** documentação de governança autorizada; sem código/gate.

## Gates e verificações

| Gate | Resultado |
|---|---|
| `grep 43 docs/adr/0003* 0006*` | 0 ocorrências nas citações de piso |
| `python3 scripts/ci/check_gate_integrity.py` | `all 8 db contract floors intact (51 total)` |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0 / 0 |
| `bun run check/lint/test/build` | 0/0, ok, 42/42, ok |
| Verificador isolado | em execução — resultado colado antes do merge |

## Evidência

- Diff: 2 ADRs + artefatos do ciclo; nenhum código/gate/constituição alterado.
- Cifra alinhada ao `FLOORS` dinâmico do guardião (51), não ao número citado
  na autorização (50, pré-ciclo 064) — divergência registrada em `STATE.md`.
- ADR-0006 continua com a diretriz de RLS **canário, sem produção** (C6).

## SDD

Autorização SE em `STATE.md` antes do BLOCO 6; `analyze` PASS; R1 `go (scoped)`;
FR 4/4, SC 4/4, T001–T004 concluídas; `converge.md` com entrada 1.
Scripts SDD da factory (HEAD `c81f0f81`, em `/tmp`): completeness/DAG/task-id
RED por dívida legada — informativo; converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR [NV]).** R4 aprovado antes de R7.

## NV

Estado remoto/CI e aprovação para execução do canário C6: **[NV]**.
