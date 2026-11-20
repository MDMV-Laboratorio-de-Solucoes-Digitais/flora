# Research: fechamento pós-merge do ciclo 072

- **Coletado**: 2026-11-19T04:51Z · **Entradas**: `intake.md` + comandos locais/gh.

## Evidência determinística

| Comando | Saída |
|---|---|
| `gh pr view 294 --json state,mergedAt,mergeCommit` | `MERGED`, `04:44:19Z`, `407d26ddce414ad88e3eb6daf0e6bf104dfdf029` |
| `gh pr view 295 --json state,mergedAt,mergeCommit` | `MERGED`, `04:50:23Z`, `c16be5c303e2209710c3e539fbc394329392e572` |
| `gh pr checks 295 --watch` (04:45:36Z→04:50:13Z) | todos `pass`: frontend-build, rust-build, db-tests, dependency-review, supply-chain-vet, e2e-browser; `mergeStateStatus=CLEAN` |
| `git switch dev && git merge --ff-only github/dev` | `dev@c16be5c30`; `git status --short` vazio |
| `python3 scripts/ci/check_gate_integrity.py` (04:50:34Z) | `gate-integrity: PASS`, 4 checks, 8 pisos/51, exit 0 |
| `gh pr list --state open` | `[]` |
| `git worktree list` | worktree alheia `feature/073-state-closure` preservada |
| `find` | specs 63, assessments 60, validations 59, releases 59, bugs 4, research 137 (138 com índice) |

## Prior art

- Ciclo 070 e 071 registraram fechamento pós-merge no STATE com PR, data, checks e merge commit — mesmo padrão a seguir.
- Ciclo 072 registrou “Entrega e fechamento” **antes** do merge; a correção deve ser nova nota append-only, não reescrita.

## Restrições

- Waiver do SE para aprovador distinto foi registrado em `STATE.md:547-553` e nas mensagens dos merge commits; não é autorização para novos merges sem registro.
- `[NV]` permanece para CI futuro, Forgejo, produção e aprovações remotas.
- Nenhum bug em `.specify/bugs/` foi acionado (4 existentes aplicados/verificados).
