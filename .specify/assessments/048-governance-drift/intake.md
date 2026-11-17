# Intake: 048 — Governance drift

**Data:** 2026-11-17T14:33Z · **Âncora:** `github/dev@4ae58ccd2` · branch `feature/048-governance-drift`

## Sinal

Auditoria da realidade (BLOCO 2) confirmou divergência entre o arquivo que orienta
todos os agentes (`AGENTS.md`) e as fontes executáveis:

- `AGENTS.md:4` afirma 7 membros (`flora-tasks`, `flora-files` inclusive) e Rust `1.98`;
  `Cargo.toml:19` fixa `rust-version = "1.98.1"` e `:26-32` lista 5 membros + pacote raiz.
- `AGENTS.md:7` afirma toolchain pinned `1.98.0`; workflows e bug `rust-toolchain-pin-bump-1-98-1`
  registram `1.98.1` como correção de UB em vtable.

## Ranqueamento 80/20

`(I×C)/(E×R)` = (3×5)/(1×1) = **15.0**, à frente de C3 (10.0), N3 (4.0), C4 (3.75),
C5 (2.08) e C6 (0.80). Os 4 bug reports em `.specify/bugs/` já estão `applied`/`verified`.

## Teste vermelho (executado)

`grep -nE 'flora-tasks|flora-files|Toolchain pinned' AGENTS.md` → 2 linhas desatualizadas;
`grep -nE '^rust-version|^members' Cargo.toml` → `1.98.1` e 5 membros.

## Fronteiras

Escopo: `AGENTS.md` + nota datada em `OBJETIVO.md`/`ROADMAP.md`. Fora: código Rust,
workflows, constituição, ADRs, merge/tag/deploy. Decisão A (alvo) preservada.
