# Tasks: 083 — refresh documental verificado (apêndices datados no EOF)

- **T001** Em `docs/PROGRESSO.md`, acrescentar **somente no EOF (após `:164`)** um apêndice datado `2026-11-19T23:12Z` (ciclo 083) contendo: âncora local `github/dev@94fcf19ca` (conferida em `:3`), as contagens do snapshot determinístico 23:11Z (`specs/` 72, `.specify/assessments/` 69, `.specify/validations/` 68, `.specify/releases/` 68, `.specify/bugs/` 4, `docs/research/` 138 com `index.md` / 137 sem ele, `docs/adr/` 6), o gate PASS (4 checks) com data/hora do snapshot, a delimitação explícita de escopo (apêndices no EOF + 5 artefatos 083) e `[NV]` para remoto/CI/PR/produção. Nenhuma linha pré-existente é tocada: `:3`, `:12-22`, `:151` e `:159-164` seguem byte-idênticos (a nota 04:23:44Z do ciclo 072 permanece como registro histórico). Ao fim, `grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md` retorna exatamente uma ocorrência, na última linha. *(FR-001, SC-001, SC-003)*
- **T002** Em `docs/research/index.md`, acrescentar **somente no EOF (após `:212`)** apêndice datado `2026-11-19T23:12Z` equivalente, com as contagens atuais (137 arquivos sem `index.md`, 138 com ele; `docs/adr/` 6; `specs/` 72, assessments 69, validations 68, releases 68, bugs 4), reafirmando os 137/137 links e `missing=[]`/`extra=[]` registrados em `:186`/`:198-206` **sem reabrir a dívida 047+** e com `[NV]` para remoto/CI. Nenhuma nota histórica é reescrita: `:3`, `:8-18`, `:186`, `:198-206` e `:208-212` seguem byte-idênticos. Ao fim, `grep -n '2026-11-19T23:12Z' docs/research/index.md` retorna exatamente uma ocorrência, na última linha — completando SC-001 em conjunto com T001. *(FR-002, SC-001, SC-003)*
- **T003** Preservar a fronteira append-only: Manter byte-idêntico todo o conteúdo pré-existente dos dois arquivos, em especial `docs/PROGRESSO.md:3` (âncora `94fcf19ca`), `:12-22` (contagens 02:23Z do ciclo 056), `:151` e `:159-164`, e `docs/research/index.md:3` (estado 01:58Z), `:8-18` (contagens 01:58Z do ciclo 068), `:186`, `:198-206` e `:208-212` (correções 02:35Z/04:22:55Z/04:23:44Z dos ciclos 069/072). O apêndice NÃO substitui, renomeia ou "corrige" nota histórica alguma: contagens mortas permanecem como registro, e 137/137 links seguem válidos. Nenhum `[NV]` é convertido em afirmação verificada; SC-004 mantém PASS conforme o snapshot e delega a reexecução ao main. *(FR-003, SC-003, SC-004)*
- **T004** Auditar o diff e preparar o rollback: `git diff --name-only` deve listar somente `docs/PROGRESSO.md`, `docs/research/index.md` e os 5 artefatos de assessment (`.specify/assessments/083-docs-verified-refresh/`: `intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`); `git diff --numstat` deve registrar `0` na coluna de remoções para os dois arquivos (apenas acréscimos) e `git diff -U0` mostrar só linhas adicionadas. Zero mutação de código, config, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, specs numeradas, PRs, produção, DB/rede, segredos e worktrees alheios; base `33d174095` preservada; zero PRs abertos. Registrar `[NV]` para a reexecução do gate, CI ao vivo, Forgejo/PRs, merge, proteção de branch, produção e aprovações (handoff ao main). *(FR-004, SC-002, SC-004)*

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-003 |
| T002 | FR-002 | SC-001, SC-003 |
| T003 | FR-003 | SC-003, SC-004 |
| T004 | FR-004 | SC-002, SC-004 |

SC-001 aparece em T001 e T002 porque o `grep` da nova data é conferido em cada arquivo separadamente (uma ocorrência no EOF de cada um). SC-003 aparece em T001, T002 e T003: a preservação é verificada em cada EOF e, de forma consolidada, na fronteira append-only. SC-004 aparece em T003 e T004: os `[NV]` não são convertidos em afirmação verificada e a auditoria de escopo limita o diff. IDs sempre individuais, sem intervalos (R4/R7).

## Gates determinísticos (locais/offline; `[NV]` até a implementação)

```
# 1. nova data — esperado: exatamente uma ocorrência em cada arquivo, no EOF
grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md docs/research/index.md

# 2. escopo — esperado: somente os dois arquivos + 5 artefatos 083
git diff --name-only

# 3. append-only — esperado: 0 na coluna de remoções para os dois arquivos
git diff --numstat -- docs/PROGRESSO.md docs/research/index.md
git diff -U0 -- docs/PROGRESSO.md docs/research/index.md

# 4. histórico intacto — esperado: linhas coladas idênticas; 137/137 links
sed -n '3p;12,22p;151p;159,164p' docs/PROGRESSO.md
sed -n '3p;8,18p;186p;198,206p;208,212p' docs/research/index.md

# 5. integridade de gate (reexecução) — esperado: PASS, 4 checks
python3 scripts/ci/check_gate_integrity.py
```

Os comandos 1–4 são determinísticos e executados pelo main agent após a edição; o comando 5 e a recontagem `find` são **[NV]** nesta avaliação (sem gates/formatters/git aqui), conforme `decision.md`. A auditoria espera somente `docs/PROGRESSO.md`, `docs/research/index.md` e os 5 artefatos 083; qualquer outro path ou qualquer deleção invalida SC-002/SC-004. Os 6 documentos do ciclo em `specs/083-docs-verified-refresh/` (`spec.md`, `clarify.md`, `plan.md`, `tasks.md`, `checklist.md`, `checklists/requirements.md`) são artefatos do próprio ciclo, não código.

Rollback: remover os apêndices `2026-11-19T23:12Z` do EOF dos dois arquivos, restaurando cada um ao estado do base `33d174095` (`docs/PROGRESSO.md` com 164 linhas e `docs/research/index.md` com 212 linhas); nenhuma linha histórica é reescrita — `:3`, `:12-22`, `:151`, `:159-164`, `:3`, `:8-18`, `:186`, `:198-206` e `:208-212` permanecem intactos. Os artefatos 083 (de assessment e de ciclo) podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.

Convenção: um arquivo por commit; Conventional Commits em pt-BR; paths explícitos nos comandos; nunca `git add -A`/`.`. Sem `/speckit.validate` nem `/speckit.release`.
