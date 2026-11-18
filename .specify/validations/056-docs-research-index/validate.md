# Validation: 056 — Docs research index

**Data:** 2026-11-18T02:29Z · **Baseline:** `github/dev@94fcf19ca`
**Tipo:** documentação verificável; sem código, CI ou autorização alterados.

## Gates e verificações (2026-11-18T02:21–02:28Z)

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 required checks, 8 pisos / 50 total |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0 / 0 |
| `bun run check` / `lint` | 0 erros, 0 warnings / exit 0 |
| `bun run test:unit -- --run --project server` | 42/42 |
| `bun run build` | exit 0 |
| `find docs/research -name '*.md' ! -name index.md \| wc -l` | 135 |
| Entradas no índice / links válidos / duplicatas / ausentes | 135 / 135 / 0 / 0 |
| Duas gerações consecutivas | mesmo MD5 (`f772e67c…`) |
| Verificador isolado (reverificação) | PASS em 3/3 itens |

## Evidência

- Índice: header com metodologia e comando de regeneração; seções por data
  extraídas do prefixo `YYYY-MM-DD`; títulos do primeiro `# ` (fallback: nome).
- `PROGRESSO.md`: contagens conferidas por `find` já com os artefatos do ciclo
  (`specs` 47 dirs/202 arquivos, assessments 44/175, validations 42/43,
  releases 42/42, bugs 4/12, research 135, adr 6).
- Correções pós-verificação: contagens, redação da fonte (exclui `index.md`),
  `[NV]` inline para PR #273/remote, `feature.json` local/gitignored e ADR-0006.
- Nenhuma credencial copiada; nenhuma instrução incorporada dos documentos.

## SDD

Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6. `analyze` PASS sem
conflitos; R1 `go (scoped)`; FR 4/4, SC 4/4, T001–T005 concluídas; `converge.md`
append-only com entrada 1. Scripts SDD da factory (HEAD `046e1f2a`, em `/tmp`):
completeness/DAG/task-id RED por dívida legada — informativo, não gate do alvo;
converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR ao vivo [NV]).** R4 aprovado
antes de R7. CI do PR será observada ao vivo no Bloco 7 antes do merge (ADR-0003).

## NV

Estado remoto/CI/proteção, valor atual de `feature.json` e aprovação para
corrigir ADRs: **[NV]**. Sem merge/auto-merge/tag/deploy nesta fase.
