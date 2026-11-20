# Research: refresh verificado de estado (072)

- **Slug**: 072-verified-state-refresh
- **Coletado**: 2026-11-19T04:21Z
- **Entradas**: `intake.md` + execuções locais desta sessão. Nenhuma fonte web consultada; nenhuma credencial copiada.

## Evidência determinística (comandos executados)

| Comando (no alvo) | Saída observada |
|---|---|
| `for d in specs .specify/assessments .specify/validations .specify/releases .specify/bugs docs/research; do find "$d" -mindepth 1 -maxdepth 1 | wc -l; done` | `61 / 58 / 57 / 57 / 4 / 138`arquivos em docs/research, dos quais 137 sem `index.md` |
| `git rev-parse --short HEAD` (04:14Z) | `cdf50eaff` |
| `python3 scripts/ci/check_gate_integrity.py` (04:14:31Z) | `gate-integrity: PASS`, 4 required checks, `all 8 db contract floors intact (51 total)`, exit 0 |
| `./.specify/scripts/bash/check-prerequisites.sh --json` (execução direta) | `{"FEATURE_DIR":".../specs/051-dead-invite-ui","AVAILABLE_DOCS":[]}` |
| `./.specify/scripts/bash/create-new-feature.sh --dry-run --json --number 072 --short-name 'verified-state-refresh' ...` | `FEATURE_NUM=072`, `BRANCH_NAME=072-verified-state-refresh` |
| `git worktree list --porcelain` | worktree alheia `feature/071-dead-auth-service` @ `0e257898`; não lida nem tocada |

## Divergências documentais verificadas (caminho:linha)

- `OBJETIVO.md:308-310` e `ROADMAP.md:243-245` — última nota “Estado verificado” 01:58Z, ciclo 068, âncora `dev@cbad031b5`; contagens 58/280, 55, 54, 54, 4, 137.
- `docs/PROGRESSO.md:111` — mesma nota 01:58Z; `:141` — apêndice 02:10Z com 59/287, 56, 55, 55.
- `docs/research/index.md:12-18` — 01:58Z com 58/280, 55, 54, 54; `:183` — 02:35Z corrige indexação 047+ (137 links / 137 arquivos, `missing=[]`).
- Real agora: 61 diretórios de spec, 58/57/57 de assess/validate/release, 4 bugs, 137 arquivos de research + índice.
- Nenhuma nota intitulada “Estado verificado” para os ciclos 069/070 nos quatro documentos.

## Prior art no próprio repo

- Ciclo 068 (`specs/068-verified-docs-refresh`, `E=5.00`) executou exatamente este tipo de refresh: contagens corrigidas em `docs/research/index.md`, nota datada em `docs/PROGRESSO.md`, intervalos em `OBJETIVO.md`/`ROADMAP.md`.
- Ciclo 069 (`specs/069-research-index-truth`) eliminou por evidência a dívida “backfill 047+”: 137 arquivos = 137 links, `missing=[]`. Essa correção deve ser preservada, não reescrita.
- Ciclo 070 (`specs/070-rbac-contract`) fechou com R4 aprovado e release `0.2.3` proposta; deixou dívida de leg RBAC no CI que exige aprov maintainer — fora deste escopo.

## Restrições e riscos

- Notas históricas são append-only: corrigir por **apêndice datado**, nunca editar/apagar linhas antigas.
- `docs/research/index.md` se autodeclara gerado por `find`/títulos, mas não há fonte regenerável verificável; **não regenerar** `.generated.md` (nenhum existe no checkout).
- Qualquer afirmação sobre CI ao vivo, merge, proteção de branch ou Forgejo permanece `[NV]`; este research não as verifica.
- O trabalho alheio em `feature/071-dead-auth-service` não é fonte para este ciclo e não será modificado.

## Lacunas

-Não há receipt bruto de CI nesta sessão além da saída local do guardião colada acima.
- `PROGRESSO.md` na raiz e `.specify/research/` não existem; o fluxo documental do alvo usa `docs/PROGRESSO.md` e `docs/research/index.md`.
