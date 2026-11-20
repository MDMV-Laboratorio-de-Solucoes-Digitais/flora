# Validation: 057 — Dynamic floor total

**Data:** 2026-11-18T02:48Z · **Baseline:** `github/dev@97838cd9c`
**Tipo:** correção de gate determinístico; sem autorização de CI/ADR/deploy.

## Gates (executados na worktree, 2026-11-18T02:46Z)

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | PASS — `all 8 db contract floors intact (50 total)` |
| Mutação: 9º piso + leg em cópia `/tmp` | `all 9 db contract floors intact (54 total)` |
| Verificador isolado | PASS 5/5 (diff, saída, consistência, risco, conteúdo) |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0 / 0 |
| `bun run check` / `lint` | 0 erros, 0 warnings / exit 0 |
| `bun run test:unit -- --run --project server` | 42/42 |
| `bun run build` | exit 0 |

## Evidência

- Diff de código: 1 arquivo, 1+/1−; regex de piso (`:117`), mensagem de falha
  (`:118`) e `FLOORS` (`:42-51`) inalterados.
- Verificador confirmou `len(FLOORS)=8`, `sum=50`, 8 invocações `run_leg` com
  pisos 13/4/8/8/5/6/4/2 — todos consistentes.
- Único risco teórico: consumidor que fizesse grep do literal antigo; nenhum
  encontrado no escopo verificado.

## SDD

Decisão 80/20 em `STATE.md` antes do BLOCO 6; `analyze` PASS; R1 `go (scoped)`;
FR 4/4, SC 4/4, T001–T005 concluídas; `converge.md` append-only com entrada 1.
Scripts SDD da factory (HEAD `38f68266`, em `/tmp`): completeness/DAG/task-id
RED por dívida legada — informativo, não gate do alvo; converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR ao vivo [NV]).** R4 aprovado
antes de R7. CI do PR observada ao vivo no Bloco 7 antes do merge (ADR-0003).

## NV

Estado remoto/CI/aprovações e correção dos ADRs: **[NV]**. Sem
merge/auto-merge/tag/deploy nesta fase.
