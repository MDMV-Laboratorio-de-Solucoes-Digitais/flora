# Converge: 090 — Bump de segurança rustls 0.23.43 → 0.23.46 (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20 — entrada 1

- **Decisão do ciclo**: bump de segurança restrito a `Cargo.lock` e à isenção cargo-vet, mantendo o requisito `rustls = "0.23"` em `Cargo.toml:189`. Motivo: `Cargo.lock:2748` registrava `0.23.43` enquanto a isenção já apontava `0.23.46`; alinhar o lock à versão corrigida sem alterar a faixa de compatibilidade declarada, workflow, ADRs, constituição, PR ou produção.
- **Evidência determinística consolidada** (branch `feature/090-rustls-security-bump`, baseline `824ba00e9`), atribuída ao handoff do main agent:
  - `Cargo.lock:2748` → `rustls` **`0.23.46`** (sem `0.23.43` residual); `Cargo.toml:189` → `rustls = "0.23"` inalterado;
  - `supply-chain/config.toml:1115-1117` → `[[exemptions.rustls]]` com `version = "0.23.46"` e `criteria = "safe-to-deploy"`;
  - `python3 scripts/ci/check_gate_integrity.py` → **gate-integrity PASS**, `all 10 db contract floors intact (58 total)`, `exit=0`;
  - `cargo test --workspace` → `exit=0`, 0 falhas;
  - `dependency-review` → limpo, sem achados de vulnerabilidade.
- **Analyze** (`specs/090-rustls-security-bump/analyze.md`, READ-ONLY, 2026-11-20): `critical=0`, `requirements_covered=6/6`, `fr=5/5`, `sc=1/1`, `tasks=6/6`, `errors=[]` — nenhuma correção de fonte necessária.
- **Fronteira preservada**: `Cargo.toml`, `.github/workflows/`, contratos em `flora-tests/`, `validate/`, `release/`, `.specify/memory/STATE.md`, ADRs e worktrees alheios intocados. Fora de escopo: PR, merge, tag, deploy e produção.
- **[NV]**: CI do PR/merge/produção, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo; gates, build, testes e formatters não reexecutados por este agente.
- **Sem novas tarefas**: nenhuma divergência spec/tasks a corrigir; `spec.md`, `Cargo.lock` e `supply-chain/config.toml` permanecem com o escopo acima.

## Resultado

**Converged — a implementação satisfaz a spec e o analyzer.** Nenhuma tarefa nova, nenhuma correção de fonte e nenhuma reexecução de gates/formatters por este agente; registro criado apenas para rastreabilidade.

converge written: 1 file; gates/formatters skipped
