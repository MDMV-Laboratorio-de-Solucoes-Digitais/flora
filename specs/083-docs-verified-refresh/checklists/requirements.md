# Requirements Quality Checklist — 083

**Feature**: [spec.md](../spec.md) · **Slug**: 083-docs-verified-refresh
**Created**: 2026-11-19T23:14Z

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T23:14Z, contra `define.md`/`decision.md` (acessados 2026-11-19T23:14Z). A execução dos gates determinísticos, a reexecução do gate e a validação ao vivo de CI/Forgejo/PRs/merge/produção e os consumidores externos dos dois arquivos permanecem `[NV]`.

## Completude

- [x] FR-001 especifica que `docs/PROGRESSO.md` DEVE receber, somente no EOF (após `:164`), um apêndice datado `2026-11-19T23:12Z` (ciclo 083) contendo: âncora local `94fcf19ca` (de `:3`), as contagens do snapshot determinístico 23:11Z (`specs/` 72, `.specify/assessments/` 69, `.specify/validations/` 68, `.specify/releases/` 68, `.specify/bugs/` 4, `docs/research/` 138 com `index.md` / 137 sem ele, `docs/adr/` 6), gate PASS (4 checks) com data/hora do snapshot, delimitação explícita de escopo e `[NV]` para estado remoto/CI/PR/produção. Nenhuma linha pré-existente é alterada (append-only puro); a edição é estritamente aditiva.
- [x] FR-002 especifica que `docs/research/index.md` DEVE receber, somente no EOF (após `:212`), apêndice datado equivalente com as contagens atuais (137 arquivos sem `index.md`, 138 com ele; `docs/adr/` 6; `specs/` 72, assessments 69, validations 68, releases 68, bugs 4), reafirmando os 137/137 links e `missing=[]`/`extra=[]` registrados em `:186`/`:198-206` sem reabrir a dívida 047+, e com `[NV]` para remoto/CI. Nenhuma nota histórica é reescrita.
- [x] FR-003 exige byte-identicidade de todo o conteúdo pré-existente, em particular `docs/PROGRESSO.md:3` (âncora `github/dev@94fcf19ca`), `:12-22` (contagens 02:23Z do ciclo 056), `:151` e `:159-164` (correção append-only 04:23:44Z do ciclo 072), e `docs/research/index.md:3` (estado 01:58Z), `:8-18` (contagens 01:58Z do ciclo 068), `:186`, `:198-206` e `:208-212` (correções 02:35Z/04:22:55Z/04:23:44Z dos ciclos 069/072). O apêndice NÃO PODE substituir, renomear ou "corrigir" nota histórica alguma: contagens mortas permanecem como registro histórico; a única mudança é acréscimo no EOF.
- [x] FR-004 limita o diff rastreado aos dois arquivos de destino (`docs/PROGRESSO.md`, `docs/research/index.md`) e aos 5 artefatos `.specify/assessments/083-docs-verified-refresh/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`); nenhum código, config, gate, script, workflow, ADR, spec numerada, PR, DB, rede ou segredo é modificado, base `33d174095` preservada. Toda afirmação remota/CI/PR/produção DEVE carregar `[NV]`; zero PRs abertos. Os 6 documentos do ciclo em `specs/083-docs-verified-refresh/` são artefatos, não código.

## Clareza e consistência

- [x] SC-001 é verificável: `grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md docs/research/index.md` retorna exatamente uma ocorrência nova em cada arquivo, localizada no EOF; os apêndices trazem contagens iguais ao snapshot 23:11Z e gate PASS (4 checks) com data/hora. Antes da edição o `grep` retorna zero ocorrências (últimas notas são 04:23:44Z em `:159-164`/`:208-212`). `[NV]` até a edição.
- [x] SC-002 é verificável: `git diff --name-only` lista somente `docs/PROGRESSO.md`, `docs/research/index.md` e os 5 artefatos 083; `git diff --numstat` registra `0` na coluna de remoções para os dois arquivos (apenas acréscimos). Qualquer deleção ou path extra invalida o critério. `[NV]` até a implementação.
- [x] SC-003 é verificável: comparação byte-idêntica de `docs/PROGRESSO.md:3`, `:12-22`, `:159-164` e `docs/research/index.md:3`, `:186`, `:208-212` (e `:198-206`); nenhuma linha histórica removida ou modificada; os 137/137 links seguem válidos (`missing=[]`, `extra=[]`). `git diff -U0` nos dois arquivos mostra apenas linhas adicionadas.
- [x] SC-004 é verificável: gate autoritativo segue PASS (4 checks) conforme snapshot 23:11Z; nenhum `[NV]` convertido em afirmação verificada; CI/produção/Forgejo/PRs não verificados nesta avaliação — reexecução de `python3 scripts/ci/check_gate_integrity.py` e estado ao vivo ficam `[NV]` (handoff ao main).
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 — sem intervalos (R4/R7); a rastreabilidade por T001..T004 em `plan.md` e `tasks.md` usa cada ID nominalmente, com SC-001 em T001 e T002 (o `grep` da data é feito em cada arquivo) e SC-003 em T001, T002 e T003 (preservação histórica em cada EOF e na fronteira).
- [x] Evidência determinística (linhas coladas em `docs/PROGRESSO.md:3`, `:12-22`, `:151`, `:159-164` e `docs/research/index.md:3`, `:8-18`, `:186`, `:198-206`, `:208-212`; contagens do snapshot 23:11Z; gate PASS 4 checks; base `33d174095`; âncora `94fcf19ca`) distinguida de `[NV]` para os gates pós-edição, CI ao vivo, Forgejo/PRs, merge, proteção de branch, produção e aprovações.

## Edge cases

- [x] E se a data `2026-11-19T23:12Z` já estiver presente nos arquivos? Hoje a última nota datada é `04:23:44Z` (`:159-164`/`:208-212`); SC-001 exige zero ocorrências antes e uma depois em cada arquivo. Ocorrência prévia invalida o critério e exige investigação, não reescrita.
- [x] E se um operador "corrigir" as contagens antigas em `docs/PROGRESSO.md:12-22` ou `docs/research/index.md:8-18`? FR-003/SC-003 proíbem: são registros históricos de checkouts anteriores; o apêndice é que registra o estado novo. Qualquer linha removida ou alterada gera deleção no `git diff --numstat` e falha SC-002.
- [x] E se a dívida 047+ for reaberta por causa dos 137/137 links? FR-002 reafirma apenas o estado verificado de `:186`/`:198-206` (`missing=[]`, `extra=[]`) sem reabrir a correção do ciclo 069; qualquer alteração nos links ou no histórico de correções fica fora de escopo.
- [x] E se o apêndice for inserido em posição diversa do EOF? FR-001/FR-002 exigem estritamente após `:164` e `:212`; SC-001 inclui a conferência de localização no EOF (`grep` com a linha final). Posição diversa ou intercalação em nota histórica invalida o ciclo.
- [x] E se o gate mudar de PASS na reexecução? Nenhum gate é executado ou alterado nesta avaliação; SC-004 mantém PASS conforme o snapshot e marca a reexecução como `[NV]`. Divergência na reexecução pelo main é handoff, não correção neste ciclo (fora de fronteira).
- [x] E se houver remoto/PR/merge/produção envolvidos? Toda afirmação carrega `[NV]`; zero PRs abertos; nenhum push, merge, force-push ou reescrita de histórico. Apenas apêndices locais no EOF e os artefatos do ciclo.
- [x] E se a base não for `33d174095`? FR-004/SC-004 exigem base preservada em todo o resto; qualquer diff fora dos dois arquivos e dos 5 artefatos 083 invalida SC-002/SC-004.

## Portarias

- [x] R1 — `go (scoped)` registrado em `decision.md` (2026-11-19T23:12Z) antes da escrita dos artefatos, com E=(1×5)/(1×1)=5,0 e opção A (dois apêndices datados no EOF, append-only puro, sem dependência/DB/rede/segredo) eleita; nenhum critério `weak`/`unknown`.
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 082→083 documentada via `--dry-run`, sem bloqueio; o ponteiro observado é `{"feature_directory":"specs/083-docs-verified-refresh"}`); os `[NV]` não são perguntas bloqueantes.
- [x] R4 e R7 — rastreabilidade individual respeitada: FR-001..FR-004 e SC-001..SC-004 em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; T001..T004 rastreáveis, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo — `docs/PROGRESSO.md:3`, `:12-22`, `docs/research/index.md:186`, as notas `:159-164`/`:208-212` e todo o histórico, código, CI, gates, scripts, workflows, ADRs, constituição, specs numeradas, PRs, produção, DB/rede, segredos e worktrees alheios.
- [x] `[NV]` — execução dos gates determinísticos, CI ao vivo, Forgejo/PRs, merge, proteção de branch, produção e aprovações separados da evidência determinística das linhas coladas, do snapshot 23:11Z, do gate PASS (4 checks) e da base `33d174095`.
