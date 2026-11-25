# Decision: 083 — Docs verified refresh

- **Slug**: 083-docs-verified-refresh
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md | decision.md (anterior, decidido em 2026-11-19T23:12Z) | `docs/PROGRESSO.md` e `docs/research/index.md` citados via research.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Últimas notas verificadas em `docs/PROGRESSO.md:159-164` e `docs/research/index.md:208-212` são 04:23:44Z (ciclo 072); contagens do snapshot 23:11Z divergem (`intake.md:9-16`). |
| Evidence strength | strong | `find` determinístico, gate PASS 4 checks no snapshot e citações caminho:linha nos dois arquivos (`:3`, `:12-22`, `:151`, `:186`, `:198-212`; `research.md:8-23`). |
| Value vs. inaction | adequate | Evita decisões ancoradas em números mortos; não altera comportamento de produto (`define.md:8,12`). |
| Feasibility / appetite | strong | Opção A: dois apêndices no EOF, sem dependência, DB, rede ou segredo (`concept.md:5-9`). |
| Strategic fit | strong | Disciplina append-only e 137/137 links (`docs/research/index.md:186`) preservadas; convenção dos ciclos 068/069/072 (`concept.md:9`). |
| Risk posture | strong | Nenhuma linha histórica tocada (FR-003/SC-003); remoto, CI, PR e produção marcados **[NV]** (`define.md:26`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; decisão original em 2026-11-19T23:12Z, branch `feature/083-docs-verified-refresh`, base `33d174095`, gate PASS (4 checks) e zero PRs no snapshot 23:11Z (estado ao vivo **[NV]**). Nenhum critério `weak`/`unknown`. 80/20 original integral: **E=(1×5)/(1×1)=5,0** — valor 1 (refresh documental que impede citação de números mortos), confiança 5 (contagens por `find`, gate colado, citações caminho:linha nos dois arquivos), esforço 1 (dois blocos no EOF + 5 artefatos), risco 1 (nenhum código/CI/gate/PR tocado; append-only puro). Único candidato autônomo do corte; alternativas (ADRs, migrações, dívidas remotas) exigem maintainer ou verificação ao vivo **[NV]**. As contagens do snapshot são specs 72, assessments 69, validations 68, releases 68, bugs 4, research 138 (137 sem `index.md`) e adr 6; o `+1` em specs encontrado na releitura viva é `specs/083-docs-verified-refresh/spec.md:1`, materializado às 23:11:17Z — ciclo em curso, não divergência (`research.md:23`). Opções B (não fazer) e C (reescrever/regenerar) rejeitadas em `concept.md:12-20`: a primeira preserva desinformação, a segunda destrói append-only e reabre dívida 047+ já fechada em `:186`.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; estado remoto/CI/PR/produção permanece **[NV]** e não bloqueia o apêndice).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Âncoras de "estado verificado" em `docs/PROGRESSO.md` e `docs/research/index.md` pararam no ciclo 072 (04:23:44Z) enquanto o snapshot 23:11Z mede contagens diferentes; leitores citam números mortos.
- **Chosen approach**: Opção A — apêndice datado `2026-11-19T23:12Z` (ciclo 083) somente no EOF dos dois arquivos (após `PROGRESSO.md:164` e `index.md:212`), com âncora local, contagens do snapshot, gate PASS com data/hora, escopo, preservação do histórico e **[NV]**; FR-001..FR-004 e SC-001..SC-004 de `define.md`.
- **In scope / out of scope**:
  - **In**: os dois apêndices e os 5 artefatos `.specify/assessments/083-docs-verified-refresh/`.
  - **Out**: `:3`, `:12-22`, `:186`, `:159-164`/`:208-212` e todo histórico, código, CI, gates, scripts, workflows, ADRs, constituição, specs numeradas, PRs, produção, DB/rede, segredos e worktrees alheios.
- **Success metrics**: `grep -n '2026-11-19T23:12Z'` → uma ocorrência nova em cada arquivo, no EOF; `git diff --name-only` só os dois arquivos + 5 artefatos 083; `git diff --numstat` com zero deleções nos dois arquivos; gate segue PASS (4 checks). Reexecução do gate e recontagem `find` **[NV]** nesta avaliação.
- **Carried-forward open questions**: `[NV]` execução dos comandos acima; `[NV]` CI ao vivo, Forgejo/PRs, merge, proteção de branch, produção e aprovações; toda afirmação remota permanece **[NV]**. Gates/formatters **skipped** nesta avaliação somente leitura; base `33d174095` preservada.
