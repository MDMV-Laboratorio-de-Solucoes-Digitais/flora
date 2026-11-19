# Implementation Plan: 083 — Docs verified refresh

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/083-docs-verified-refresh/{intake,research,define,concept,decision}.md` (go (scoped), opção A, E=5,0).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`. Convenção: Conventional Commits em pt-BR; cada apêndice vai em commit próprio e os documentos do ciclo podem ser commitados em bloco à parte.
**Base do diff**: `33d174095` (HEAD da branch `feature/083-docs-verified-refresh` no início do ciclo).

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-002 |
| T002 | FR-002 | SC-001, SC-003 |
| T003 | FR-003 | SC-002, SC-003 |
| T004 | FR-004 | SC-002, SC-004 |

Cada célula referencia identificadores individuais — não intervalos. Cobertura: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 aparecem ao menos uma vez. SC-001 aparece em T001 e T002 porque o `grep` da data só satisfaz a métrica quando o apêndice existe nos dois arquivos; SC-002 aparece em T001 (acréscimo sem deleção em `PROGRESSO.md`), T003 (byte-identicidade do histórico) e T004 (auditoria de escopo); SC-003 aparece em T002 (137/137 preservados) e T003 (linhas históricas idênticas); SC-004 aparece em T004 (gate, `[NV]` e fronteira de paths).

## Passos

1. **T001** Em `docs/PROGRESSO.md`, acrescentar somente após a última linha (`:164`, newline final `0a`) o apêndice `## Estado verificado — 2026-11-19T23:12Z (ciclo 083; append-only)` com: âncora local `dev@33d174095` (base do ciclo; remoto `[NV]`), contagens do snapshot `2026-11-19T23:11Z` (specs 72, assessments 69, validations 68, releases 68, bugs 4, research 137 sem `index.md` / 138 com ele, adr 6), gate PASS (4 checks) com data/hora do snapshot, delimitação de escopo e `[NV]` para remoto/CI/PR/produção. Conferir que `:1-164` permanece byte-idêntico, inclusive `:3` (`github/dev@94fcf19ca`), `:12-22`, `:151` e `:159-164`; `grep -n '2026-11-19T23:12Z'` retorna uma ocorrência no EOF. Commitar somente `docs/PROGRESSO.md` (`git add docs/PROGRESSO.md`; nunca `git add -A`/`.`). *(FR-001, SC-001, SC-002)*
2. **T002** Em `docs/research/index.md`, acrescentar somente após `:212` apêndice datado equivalente com 137 arquivos sem `index.md` (138 com ele) e adr 6, reafirmando 137 arquivos / 137 links / `missing=[]` de `:186` sem reabrir a dívida 047+ e sem regenerar o índice; `[NV]` para remoto/CI. Conferir que `:1-212` permanece byte-idêntico, inclusive `:3-6`, `:8-18`, `:186`, `:198-206` e `:208-212`, e que `grep -n '2026-11-19T23:12Z'` retorna uma ocorrência no EOF. Commitar somente `docs/research/index.md`. *(FR-002, SC-001, SC-003)*
3. **T003** Verificar, sem editar, que nada além dos dois apêndices mudou: linhas históricas de `docs/PROGRESSO.md` (`:3`, `:12-22`, `:151`, `:159-164`) e de `docs/research/index.md` (`:3`, `:8-18`, `:186`, `:198-206`, `:208-212`) byte-idênticas ao base; 137/137 links válidos; newline final preservado. Não substituir, renomear ou "corrigir" nota antiga; não reabrir 047+; não converter `[NV]` em fato; não tocar specs numeradas, worktrees alheios, código, CI, gate, workflows, ADRs, constituição, DB/rede ou segredos. Qualquer diff fora dos apêndices invalida o passo. *(FR-003, SC-002, SC-003)*
4. **T004** Auditar o escopo com `git diff --name-only` e `git diff --numstat`: somente `docs/PROGRESSO.md`, `docs/research/index.md`, os 5 artefatos de assessment `.specify/assessments/083-docs-verified-refresh/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`) e os 6 documentos do próprio ciclo `specs/083-docs-verified-refresh/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`). Exigir `0` na coluna de remoções dos dois arquivos e uma ocorrência da data em cada um. Rodar os gates determinísticos, exigindo gate PASS (4 checks) e registrando como `[NV]` a reexecução e a recontagem `find` até o handoff ao main. Preparar o rollback abaixo. *(FR-004, SC-002, SC-004)*

## Gates determinísticos

```
# 0. base de comparação (raiz do repo)
BASE=33d174095

# 1. escopo — esperado: somente os 2 arquivos de destino, os 5 artefatos 083
#    e os 6 documentos do próprio ciclo; nenhum outro path
git diff --name-only "$BASE"..HEAD

# 2. append-only nos dois arquivos — esperado: 0 na coluna de remoções
git diff --numstat -- docs/PROGRESSO.md docs/research/index.md

# 3. data nova no EOF — esperado: 1 ocorrência em cada arquivo
grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md docs/research/index.md

# 4. integridade de gate (reexecução pós-edição) — esperado: PASS, 4 checks
python3 scripts/ci/check_gate_integrity.py

# 5. recontagem find por categoria (referência: snapshot 23:11Z —
#    72/69/68/68/4, research 137 sem index.md / 138 com ele, adr 6) — [NV]
find specs -mindepth 1 -maxdepth 1 -type d | wc -l
find .specify/assessments -mindepth 1 -maxdepth 1 -type d | wc -l
find .specify/validations -mindepth 1 -maxdepth 1 -type d | wc -l
find .specify/releases -mindepth 1 -maxdepth 1 -type d | wc -l
find .specify/bugs -mindepth 1 -maxdepth 1 -type d | wc -l
find docs/research -maxdepth 1 -type f -name '*.md' ! -name index.md | wc -l
find docs/adr -maxdepth 1 -type f -name '*.md' | wc -l
```

O gate 1 verifica SC-002 (escopo); o gate 2 verifica SC-002/SC-003 (zero deleções); o gate 3 verifica SC-001; o gate 4 verifica SC-004; o gate 5 é `[NV]` até o handoff. A auditoria de escopo espera apenas os dois arquivos e os artefatos/documentos do ciclo 083; qualquer outro path invalida SC-004. Como os diretórios 083 podem estar ainda não rastreados no início do ciclo, auditorias com `$BASE..HEAD` pressupõem que os artefatos foram adicionados com paths explícitos (nunca `git add -A`/`.`). Todos os comandos são determinísticos e não dependem de DB, segredos ou rede; sua execução real fica com o main agent e é `[NV]` nesta spec (marcadores em SC-002 e SC-003 de `spec.md`).

## Rollback

Reverter os commits que introduziram T001/T002 ou restaurar diretamente os dois paths ao base:

```
git checkout 33d174095 -- docs/PROGRESSO.md docs/research/index.md
```

Isso remove os apêndices e devolve ambos os arquivos byte-idênticos ao base, com as notas `04:23:44Z` em `:159-164`/`:208-212` e os cabeçalhos/tabelas históricos intactos. Nenhum outro path é tocado — código, config, gate/script, workflow, ADR, constituição, STATE, specs alheias, CI, PR, produção, DB, rede ou segredos permanecem intactos. Os artefatos do ciclo 083 (5 de assessment e 6 documentos) são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico; após a reversão, repetir os gates 1, 2, 3 e 4 para confirmar o retorno ao estado base.
