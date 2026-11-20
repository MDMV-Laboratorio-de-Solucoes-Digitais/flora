# Converge: 092 — npm lock sync (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20 — entrada 1

- **Decisão do ciclo**: regenerar `apps/frontend/package-lock.json` a partir do manifesto com o comando contratual `npm install --package-lock-only --ignore-scripts`, preservando `package.json` e o lockfile canónico da CI (`bun.lock`). Motivo: o lock npm estava congelado e sem `shadcn-svelte`, embora manifesto e `bun.lock` já o contivessem — drift estrutural com 25 alertas do Dependabot acumulados. Verdict `go` em `.specify/assessments/092-npm-lock-sync/decision.md`; fora de âmbito: CI, source, `validate`/`release`/`STATE`, PR/merge/produção e auditoria individual dos alertas.
- **Evidência determinística consolidada** (branch `feature/092-npm-lock-sync`, baseline `bf00caf5c`), atribuída ao handoff do main agent:
  - `apps/frontend/package-lock.json` → diff de **+116 linhas**; a raiz passa a conter `"shadcn-svelte": "1.8.1"` e as entradas transitivas necessárias (FR-001, SC-001);
  - `apps/frontend/package.json` e `apps/frontend/bun.lock` → **fora do diff**, intactos; `bun.lock` segue canónico para CI (FR-003, SC-002);
  - `npm install --package-lock-only --ignore-scripts` em `apps/frontend` → `exit=0`, sem lifecycle scripts e sem exposição de credenciais (FR-002, SC-003); reexecução idempotente, zero diffs adicionais (FR-005);
  - `git diff --stat` → somente `apps/frontend/package-lock.json` (1 arquivo) no diff de código (FR-004, SC-002);
  - gate autoritativo `python3 scripts/ci/check_gate_integrity.py` → **PASS**, `exit=0`; `npm run check`, `npm run lint` e `npm run build` em `apps/frontend` → sem falhas (evidência esperada/consolidada do handoff, não reexecutada neste agente);
  - inventário de entrada: 25 alertas do Dependabot no lockfile npm; **contagem viva pós-triagem `[NV]`**.
- **Analyze** (`specs/092-npm-lock-sync/analyze.md`, READ-ONLY, 2026-11-20): `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `sc=3/3`, `tasks=6/6`, `errors=[]` — nenhuma correção de fonte necessária.
- **Fronteira preservada**: `.github/workflows/`, código-fonte, testes, `.specify/memory/STATE.md`, ADRs, constituição, `validate/`, `release/` e worktrees alheios intocados. Fora de escopo: PR, merge, tag, deploy, produção e credenciais.
- **[NV]**: CI do PR/merge/produção, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo; contagem viva de alerts após a triagem não lida; gates, build, testes e formatters não reexecutados por este agente.
- **Sem novas tarefas**: nenhuma divergência spec/tasks a corrigir; T001..T006 permanecem [X] e o escopo segue limitado ao lockfile npm.

## Resultado

**Converged — a implementação satisfaz a spec e o analyzer.** Nenhuma tarefa nova, nenhuma correção de fonte e nenhuma reexecução de gates/formatters por este agente; registro criado apenas para rastreabilidade.

converge written: 1 file; gates/formatters skipped
