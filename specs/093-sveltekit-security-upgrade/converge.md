# Converge: 093 — Upgrade de segurança SvelteKit 2.70.3 → 3.1.1 (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20 — entrada 1

- **Decisão do ciclo**: promover o frontend para `@sveltejs/kit 3.1.1`, `@sveltejs/adapter-node 6.0.5` e `svelte 5.57.1`, mantendo `vite ^8.0.16`, `typescript ^6.0.3` e `@sveltejs/vite-plugin-svelte ^7.1.2` dentro dos peers exigidos pelo kit corrigido, e regenerar os locks sem nenhuma ocorrência de `2.70.3`. Motivo: o `dependency-review` da PR #318 (`dev` → `main`) bloqueava o merge por `GHSA-3r8g` (high) + 3 moderate em `@sveltejs/kit 2.70.3`; a versão `3.1.1` corrige o conjunto. Verdict `go` em `.specify/assessments/093-sveltekit-security-upgrade/decision.md`; fora de âmbito: API de aplicação, runtime, schema, migration, contrato de produção, workflow, required check, PR/merge/deploy e produção.
- **Evidência determinística consolidada** (branch `feature/093-sveltekit-security-upgrade`, baseline `b426e4f81`), atribuída ao handoff do main agent:
  - `apps/frontend/package.json` → `@sveltejs/kit 3.1.1`, `@sveltejs/adapter-node 6.0.5` (peer `@sveltejs/kit ^3`) e `svelte 5.57.1`; `vite ^8.0.16` (≥ `^8.0.12`), `typescript ^6.0.3` (≥ `^6.0.0`) e `@sveltejs/vite-plugin-svelte ^7.1.2` (≥ `^7.0.0`) já compatíveis (FR-001, FR-002, SC-001);
  - `apps/frontend/bun.lock` e `apps/frontend/package-lock.json` → versões corrigidas em ambos, **`grep 2.70.3` com zero ocorrências**; `bun.lock` permanece canónico da CI e nenhum gerenciador de lock foi trocado (FR-003, SC-002);
  - `npm run check`, `npm run lint`, `npm run build` e `npm run test` em `apps/frontend` → sem falhas, `exit=0` nas quatro verificações (evidência esperada/consolidada do handoff, não reexecutada neste agente) (FR-004, SC-002);
  - gate autoritativo `python3 scripts/ci/check_gate_integrity.py` → **PASS**, `exit=0` (10 legs / 58 pisos intactos — evidência esperada/consolidada do handoff);
  - `dependency-review` na PR #318 (`dev` → `main`) → **verde**: `GHSA-3r8g` (high) e os 3 moderate deixam de ser achados com o kit `3.1.1` (FR-004, SC-003; resultado remoto `[NV]` nesta passagem);
  - diff de código limitado a `apps/frontend/package.json`, `apps/frontend/bun.lock` e `apps/frontend/package-lock.json`, além dos artefatos documentais do ciclo 093 (FR-005, SC-003).
- **Analyze** (`specs/093-sveltekit-security-upgrade/analyze.md`, READ-ONLY, 2026-11-20): `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `sc=3/3`, `tasks=6/6`, `errors=[]` — nenhuma correção de fonte necessária.
- **Fronteira preservada**: `.github/workflows/`, required checks, código-fonte, testes, `.specify/memory/STATE.md`, ADRs, constituição, `validate/`, `release/` e worktrees alheios intocados. Fora de escopo: PR, merge, tag, deploy, produção e credenciais.
- **[NV]**: CI ao vivo, merge, produção, aprovações de maintainer e proteção de branch não verificados neste ciclo; contagem viva de advisories após o upgrade não lida; gates, build, testes e formatters não reexecutados por este agente.
- **Sem novas tarefas**: nenhuma divergência spec/tasks a corrigir; T001..T006 permanecem no escopo do upgrade de segurança e da sincronização dos locks.

## Resultado

**Converged — a implementação satisfaz a spec e o analyzer.** Nenhuma tarefa nova, nenhuma correção de fonte e nenhuma reexecução de gates/formatters por este agente; registro criado apenas para rastreabilidade.

converge written: 1 file; gates/formatters skipped
