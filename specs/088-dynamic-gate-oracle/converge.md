# Converge: 088 — Oráculo dinâmico de contratos ignorados no gate de CI (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20 — entrada 1

- **Decisão R1** (`.specify/assessments/088-dynamic-gate-oracle/decision.md`, verdict **go**, 2026-11-20): substituir o espelho manual `FLOORS` ↔ `run_leg` por um oráculo dinâmico dentro do próprio `scripts/ci/check_gate_integrity.py`, sem tocar workflow, ADRs, constituição, PR ou produção. Motivo: 086 (3 contratos RBAC) e 087 (4 bootstrap) foram esquecimentos descobertos só por humano; um terceiro passaria pelo espelho.
- **Evidência determinística consolidada** (branch `feature/088-dynamic-gate-oracle`, baseline `cbc885383`), atribuída ao handoff do main agent:
  - `python3 scripts/ci/check_gate_integrity.py` → **gate-integrity PASS**, `all 10 db contract floors intact (58 total)`, `exit=0`;
  - `python3 -m py_compile scripts/ci/check_gate_integrity.py` → `exit=0`;
  - `git diff` → **1 arquivo**, `scripts/ci/check_gate_integrity.py`, `1 file changed, 59 insertions(+), 5 deletions(-)` (1 arquivo, +59/-5);
  - `LEG_RE` extrai as 10 `run_leg`; subprocess `--ignored --list` fornece os 58 nomes; contagem exata por `startswith`, com fail-closed em cargo falho/lista vazia/leg duplicada/divergência.
- **Analyze** (`specs/088-dynamic-gate-oracle/analyze.md`, READ-ONLY, 2026-11-20): `artifacts=4`, `critical=0`, `requirements_covered=7/7`, `fr=5/5`, `sc=3/3`, `tasks=7/7`, `go=True`, `errors=[]` — nenhuma correção de fonte necessária.
- **Fronteira preservada**: checagens estáticas (`REQUIRED`, jobs, `continue-on-error`, path filters, `cargo build/test --workspace`) e `--remote` intactas; stdlib apenas. Fora de escopo e intocados: `.github/workflows/rust.yml`, contratos em `flora-tests/`, `validate/`, `release/`, `.specify/memory/STATE.md`, ADRs e worktrees alheios.
- **[NV]**: CI ao vivo, PR/merge/produção, aprovações e proteção de branch não verificados neste ciclo; gates, build, testes e formatters não reexecutados por este agente; dívidas de governança preexistentes seguem como dívida, não editadas aqui.
- **Sem novas tarefas**: nenhuma divergência spec/plan/tasks a corrigir; `spec.md`, `plan.md` e `tasks.md` permanecem inalterados.

## Resultado

**Converged — a implementação satisfaz spec, plan, tasks e analyzer.** Nenhuma tarefa nova, nenhuma correção de fonte e nenhuma reexecução de gates/formatters por este agente; registro criado apenas para rastreabilidade.

converge written: 1 file; gates/formatters skipped
