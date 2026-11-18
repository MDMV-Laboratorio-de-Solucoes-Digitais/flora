# Validation: 055 — Auth floor thirteen

**Data:** 2026-11-18T01:49Z · **Baseline:** `github/dev@d5d7ef5d`
**Tipo:** elevação de CI (fortalecimento de piso); sem autorização, ADR ou deploy.

## Gates (executados na worktree, 2026-11-18T01:36–01:48Z)

| Gate | Resultado |
|---|---|
| `cargo test -q -p flora-tests 'contract::auth::db_' -- --list` | 13 testes |
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 8 pisos / 50 total |
| Mutação lockstep (só workflow volta a 12) | **FAIL** — `db floor for auth (13) missing or changed` |
| Leg auth com Postgres 18.6 isolado | exit 0 — **13 passed** / 0 failed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0 / 0 |
| `bun run check` / `lint` | 0 erros, 0 warnings / exit 0 |
| `bun run test:unit -- --run --project server` | 42/42 (12 arquivos) |
| `bun run build` | exit 0 |

## Evidência

- Diff de código: 2 arquivos, 3+/3−; nenhum teste modificado.
- Guardião e workflow alterados em lockstep; a mutação prova que o guardião
  rejeita divergência.
- Verificador isolado: 13 `#[ignore]` em `auth.rs`; `--test-threads=1`, UUIDv7
  e janelas temporais robustas; nenhum `cfg` condicional além de `#[cfg(test)]`.
- DB `flora` existente não tocada; verificação usou `flora_055_verify`, removida.
- `docs/adr/0003:19` e registros históricos de bugs mantêm números antigos —
  dívida documental, fora de escopo.

## SDD

Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6 (10.0). `analyze` PASS
sem conflitos; R1 `go (scoped)`; FR 4/4, SC 4/4, T001–T005 concluídas;
`converge.md` append-only com entrada 1. Scripts SDD da factory executados em
`/tmp` (não materializados): completeness/task-id/DAG RED por dívida legada —
informativo, não gate do alvo; converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR ao vivo [NV]).** R4 aprovado
antes de R7. CI do PR será observada ao vivo no Bloco 7 antes do merge (ADR-0003).

## NV

Resultado do leg no runner do GitHub e estágio final de checks/PR: **[NV]**.
Sem merge/auto-merge/tag/deploy nesta fase.
