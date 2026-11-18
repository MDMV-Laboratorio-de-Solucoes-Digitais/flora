# Validation: 063 — Validate OSV run

**Data:** 2026-11-18T04:50Z · **Baseline:** `github/dev@e1ff815bf`
**Tipo:** validação de CI advisory; sem código/gate/autorização alterados.

## Gates e verificações (2026-11-18T04:47–04:49Z)

| Gate | Resultado |
|---|---|
| `gh workflow run osv-scan.yml --ref dev` | exit 0 — run `44239662140` |
| `gh run view 44239662140` | `status=completed`, `conclusion=success`, 04:46:43Z → 04:47:13Z |
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 checks, 8 pisos / 50 total |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0 / 0 |
| `bun run check` / `lint` | 0 erros, 0 warnings / exit 0 |
| `bun run test:unit -- --run --project server` | 42/42 |
| `bun run build` | exit 0 |
| Verificador isolado | PASS 6/6 |
| `git diff dev --numstat -- STATE.md` | `27  0` (append-only) |

## Evidência

- Run executado no ref `dev`, que contém o pin `@v2.7.1` — o workflow
  reutilizável resolveu e completou todas as etapas, incluindo upload SARIF.
- `success` é conclusão do scan, não ausência de vulnerabilidades
  (`fail-on-vuln: false`) — ressalva registrada.
- Nota anterior `[NV]` preservada; correção por apêndice datado.
- Nenhum segredo lido/copiado; nenhuma credencial no diff.

## SDD

Decisão 80/20 em `STATE.md` antes do BLOCO 6; `analyze` PASS; R1 `go (scoped)`;
FR 4/4, SC 4/4, T001–T004 concluídas; `converge.md` com entrada 1.
Scripts SDD da factory (HEAD `b5bc7265`, em `/tmp`): completeness/DAG/task-id
RED por dívida legada — informativo; converge append-only PASS.

## R4 — Veredito

**Approved (evidência local + execução real observada; CI/PR do ciclo [NV]).**
R4 aprovado antes de R7. CI do PR observada ao vivo no Bloco 7 antes do merge.

## NV

Estado final do PR/CI e impacto futuro da migração `ubuntu-latest` → Ubuntu 26:
**[NV]**. Sem merge/auto-merge/tag/deploy nesta fase.
