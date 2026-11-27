# Evidence — consolidação de checkout (2026-11-27)

**Branch:** `fix/consolidate-checkout-20261127` @ `c755c1c74a` · **Base:** `github/dev=b44b53cdea`.

| Passo | Comando/ação | Saída observada | Resultado |
|---|---|---|---|
| 1 | `git status --short` na worktree consolidada | Somente `?? .omp/plans/` e `?? libws_lint_probe3.rmeta`; todo trabalho útil está em 7 commits | PASS |
| 2 | `git diff --check` | RC 0 | PASS |
| 3 | `grep -Ei 'manual-only\|no deploy on push\|10 legs\|six DB-gated' AGENTS.md README.md` | Nenhum match | PASS |
| 4 | `grep -nE '^## ' AGENTS.md` | 8 headings na ordem exigida (linhas 3, 9, 27, 41, 78, 89, 105, 112) | PASS |
| 5 | `git diff --numstat` | `STATE.md` +96/−0, `PROGRESSO.md` +48/−0: append-only puro; demais arquivos com edições explícitas | PASS |
| 6 | `find .specify/queue/consolidation-archive-20261127 -type f \| wc -l` | 18 arquivos (MANIFEST + 17 fontes); hashes conferidos pelo agente responsável | PASS |
| 7 | `git ls-files .specify/validations \| cut -d/ -f3 \| sort -u \| wc -l` | 119 diretórios; nenhum copiado porque já estavam presentes e idênticos à origem | PASS |
| 8 | `git diff --name-only github/dev..HEAD -- .github/workflows/ scripts/ci/check_gate_integrity.py flora-core/ flora-api/ flora-tests/ apps/frontend/ Cargo.toml` | Nenhum arquivo alterado — branch é somente documentação/config template | PASS |
| 9 | `python3 scripts/ci/check_gate_integrity.py` (3 tentativas locais, inclusive com `CARGO_TARGET_DIR` reaproveitado) | Timeout por rebuild local completo (900 s); **não é RED de política**. Registro autoritativo em 26/11 no head exato: `gate-integrity: PASS`, 11 legs / 89 pisos, RC 0 | `[NV]` local / `[EV]` histórico no head |
| 10 | `python3 scripts/ci/check_deploy_workflows.py` | PASS nas execuções anteriores no mesmo head; reexecução local não concluída pelo mesmo timeout | `[NV]` local / `[EV]` histórico |

**Decisão de venue (0.8):** GitHub Actions é a autoridade; o trabalho é documental e o branch não altera nenhum arquivo que o gate avalia. Não insisti no gate local para não consumir o notebook compartilhado (memória disponível <2 GiB nas tentativas). O PR dependerá dos checks remotos no head exato, que serão observados ao vivo antes de qualquer merge.

**Preservação:** variante manual-only dos quatro workflows e do validador, upgrade Spec Kit 1.4.3 e gate antigo ficam no archive local (não aplicados). Política vigente (`github/dev`, PR #440) e gate 11/89 permanecem intactos. Runbook, INBOX de 21/11, `.omp/plans/` e `libws_lint_probe3.rmeta` foram trazidos para a worktree consolidada; os dois últimos permanecem untracked por serem scratch/artifact.

**[NV]:** CI remoto no head `c755c1c74a`, ACR, aprovação/promoção e efeitos de deploy.
