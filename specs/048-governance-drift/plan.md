# Implementation Plan: 048 — Governance drift

**Data:** 2026-11-17T14:34Z · **Baseline:** `github/dev@4ae58ccd2`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | divergência AGENTS.md vs Cargo.toml/workflows | — |
| doc-edit | correção factual em AGENTS.md | research |
| notes | notas datadas em OBJETIVO/ROADMAP | research |
| verification | grep + cargo metadata + gates | doc-edit, notes |

Ordem: research → doc-edit + notes → verification. Sem paralelismo de código (só docs).

## Abordagem

1. `AGENTS.md:4`: substituir a lista de 7 crates pelos 5 membros reais + raiz.
2. `AGENTS.md:7`: `1.98.0` → `1.98.1`.
3. Anexar nota de estado verificado em `OBJETIVO.md`/`ROADMAP.md` com contagens por `find`.
4. Reexecutar gates completos.

## Riscos

- Risco de governança: edição de instrução de agentes → revisão deliberada no PR.
- Mitigação: diff restrito a fatos verificáveis; nenhuma regra removida.

## Verificação

`grep` de ausência, `cargo metadata`, gates Rust/frontend e `check_gate_integrity.py`.

## Fora do plano

Código, workflows, constituição, ADRs, PR #262, merge/tag/deploy, remotos **[NV]**.
