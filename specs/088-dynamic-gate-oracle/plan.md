# Implementation Plan: Oráculo dinâmico de contratos ignorados no gate de CI

**Branch**: `feature/088-dynamic-gate-oracle` @ `cbc885383` | **Date**: 2026-11-20 | **Spec**: [spec.md](./spec.md)

**Input**: `/specs/088-dynamic-gate-oracle/spec.md`

## Summary

Substituir a comparação cega entre `FLOORS` (no guarda) e `run_leg` (no
workflow) por uma verificação contra a lista real de testes `#[ignore]`
descoberta pelo cargo. Um contrato novo ou um piso defasado passa a falhar o
`rust-build` sem exigir novo ciclo de descoberta manual. Um arquivo de código.

## Technical Context

**Language/Version**: Python 3 (stdlib: `subprocess`, `re`) no guarda; Rust
1.98.1 edition 2024 no workspace `flora-tests`; YAML em `rust.yml` (somente
lido).

**Primary Dependencies**: `cargo test -p flora-tests -- --ignored --list` via
`subprocess.run(cwd=ROOT, capture_output=True, text=True, check=False)`;
regex de leg ancorada em linha; nomes de teste = linhas do stdout terminadas
em `: test`.

**Storage**: nenhum. Nenhuma rede, nenhum `--remote` alterado, nenhum segredo.

**Testing**: `python3 scripts/ci/check_gate_integrity.py` → PASS
`all 10 db contract floors intact (58 total)`; divergência controlada de piso
ou lista vazia → FAIL.

**Target Platform**: step `rust-build` de `.github/workflows/rust.yml` (job
obrigatório, ADR-0003).

**Constraints**: stdlib apenas; preservar checagens estáticas e `--remote`;
não tocar workflow, testes, `validate/`, `release/`, `STATE.md` nem outros
pisos/legs; diff limitado ao guarda + 4 artefatos 088.

**Scale/Scope**: 10 legs, 58 contratos descobertos, 1 arquivo de código.

## Constitution Check

*Gate*: **PASS**. Nenhum serviço/check obrigatório novo: o mesmo guarda
determinístico do ADR-0003 passa a consultar a fonte de verdade (os testes).
Fail-closed em cargo/list vazia/divergência. Tabela de complexidade vazia.

## Arquivos tocados

| Arquivo | Mudança | Evidência |
|---|---|---|
| `scripts/ci/check_gate_integrity.py` | oráculo dinâmico no lugar do loop estático de `FLOORS`: `LEG_RE`, subprocess `--ignored --list`, parser `: test`, validação por leg e fail-closed | diff sobre `cbc885383`; mensagem `all 10 db contract floors intact (58 total)` preservada |
| `.specify/assessments/088-dynamic-gate-oracle/decision.md` | novo — verdict go | 2026-11-20 |
| `specs/088-dynamic-gate-oracle/spec.md` | preenchido (FR-001..005, SC-001..003) | antes template |
| `specs/088-dynamic-gate-oracle/plan.md` | novo | 2026-11-20 |
| `specs/088-dynamic-gate-oracle/tasks.md` | novo — T001..T007 `[X]` | evidência local por tarefa |

**Fora de escopo**: `.github/workflows/rust.yml`, `flora-tests/src/contract/*.rs`,
`validate/`, `release/`, `.specify/memory/STATE.md`, ADRs, PRs e worktree
espelho.

## Structure Decision

Nenhuma estrutura nova: a descoberta entra como mais uma checagem dentro do
`main()` do guarda existente, alimentando a mesma lista `failures`/`checks`.
`FLOORS` permanece como declaração canônica dos pisos; a novidade é que cada
piso agora é conferido contra a contagem real de nomes ignorados que casam com
o filtro da leg.

## Next Phase

Implementação concluída e verificada localmente; fechamento documental do
ciclo 088. Nenhuma fase adicional.
