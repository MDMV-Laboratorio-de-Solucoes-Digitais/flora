# Test roadmap — consolidação de checkout (2026-11-27)

Escopo: consolidação autorizada pelo SWE das 16 entradas sujas e das 5 worktrees em uma única linha de trabalho, sem perder trabalho. IDs: FR-01 manter invariantes do `AGENTS.md`; FR-02 alinhar política de deploy ao `github/dev`; FR-03 preservar apêndices append-only; FR-04 preservar variante manual-only e upgrade Spec Kit em archive; FR-05 restaurar validações 123–126.

Pré-condições: worktree `consolidated-20261127` criada de `github/dev=b44b53cdea`; branch `fix/consolidate-checkout-20261127`; commits focados por arquivo; nenhum `git add -A`/`.`, force-push ou deploy.

| Passo | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `git -C <wt> status --short` | Somente `.omp/plans/` e `libws_lint_probe3.rmeta` untracked; todo trabalho útil commitado | CLI | sim |
| 2 | `git -C <wt> diff --check` | RC 0 | CLI | sim |
| 3 | `grep -Ei 'manual-only\|no deploy on push\|10 legs\|six DB-gated' AGENTS.md README.md` | Nenhum match | CLI | sim |
| 4 | `grep -nE '^## ' AGENTS.md` | 8 headings na ordem exigida | CLI | sim |
| 5 | Comparar `STATE.md`/`PROGRESSO.md` com as três fontes (base, checkout compartilhado, lightsout-126) | Prefixo base byte-idêntico; todas as seções ausentes anexadas; 0 deleções no diff | CLI | sim |
| 6 | `find .specify/queue/consolidation-archive-20261127 -type f \| wc -l` | 18 arquivos (MANIFEST + 17 fontes), hashes conferidos | CLI | sim |
| 7 | `git -C <wt> ls-files .specify/validations \| cut -d/ -f3 \| sort -u \| wc -l` | 119 diretórios de validação presentes | CLI | sim |
| 8 | `python3 scripts/ci/check_gate_integrity.py` na worktree consolidada | `gate-integrity: PASS`, 11 legs / 89 pisos, RC 0 | CLI | sim |
| 9 | `python3 scripts/ci/check_deploy_workflows.py` | `deploy-workflows: PASS`, RC 0 | CLI | sim |
| 10 | Abrir PR para `github/dev` com corpo didático e ACR independente | PR reviewable; achados advisory registrados | GitHub/CLI | parcial (revisão humana/agente) |

Rollback: reverter os commits da branch (novo commit de reversão), sem force-push; a política automática do `github/dev` e o gate 11/89 permanecem intactos; o archive local preserva as variantes não aplicadas até decisão do SWE.
