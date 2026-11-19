# Define: 083 — Docs verified refresh

- **Slug**: 083-docs-verified-refresh · **Definido**: 2026-11-19T23:12Z
- **Entradas**: `intake.md`, `research.md`

## Problema

Leitores que tomam `docs/PROGRESSO.md` e `docs/research/index.md` como âncora de "estado verificado" encontram, como última nota datada, `2026-11-19T04:23:44Z` (ciclo 072) em `docs/PROGRESSO.md:159-164` e `docs/research/index.md:208-212`, enquanto o snapshot determinístico de 23:11Z registra contagens diferentes (specs 72, assessments 69, validations 68, releases 68, bugs 4, research 137 sem `index.md` / 138 com ele, adr 6). Cabeçalhos antigos (`docs/PROGRESSO.md:3`, `docs/research/index.md:3`) e tabelas `:12-22`/`:8-18` descrevem checkouts anteriores. Sem apêndice novo, decisões futuras citam números mortos.

## Afetados / metas / não-metas

Afetados: agentes de sessão e maintainers que usam os dois arquivos como âncora inicial; nenhum usuário final (defeito de governança/informação, não de runtime). Metas: registrar por apêndice datado o estado verificado do snapshot 23:11Z, com âncora local, contagens, gate PASS e data/hora, escopo estrito, preservação explícita do histórico e `[NV]`; manter 137/137 links intactos (`docs/research/index.md:186`). Não-metas: reescrever cabeçalhos/notas/tabelas, código, CI, gates, PRs, produção, ADRs, constituição, migrations, segredos e dívidas remotas.

## Requisitos funcionais

- **FR-001** — `docs/PROGRESSO.md` DEVE receber, somente no EOF (após `:164`), um apêndice datado `2026-11-19T23:12Z` (ciclo 083) com: âncora local, contagens do snapshot 23:11Z, gate PASS (4 checks) com data/hora do snapshot, delimitação de escopo e `[NV]` para estado remoto/CI/PR/produção. Nenhuma linha existente é alterada.
- **FR-002** — `docs/research/index.md` DEVE receber, somente no EOF (após `:212`), apêndice datado equivalente com as contagens atuais (137 arquivos sem `index.md`, 138 com ele; adr 6), reafirmando 137/137 links / `missing=[]` de `:186` sem reabrir a dívida 047+, e `[NV]` para remoto/CI.
- **FR-003** — todo conteúdo pré-existente DEVE permanecer byte-idêntico, em particular `docs/PROGRESSO.md:3` (âncora `94fcf19ca`), `:12-22`, `:151`, `:159-164` e `docs/research/index.md:3`, `:8-18`, `:186`, `:198-206`, `:208-212` (append-only). O apêndice NÃO PODE substituir, renomear ou "corrigir" nota histórica alguma.
- **FR-004** — o diff rastreado DEVE limitar-se aos dois arquivos de destino e aos 5 artefatos `.specify/assessments/083-docs-verified-refresh/`; nenhum código, config, gate, script, workflow, ADR, spec numerada, PR, DB, rede ou segredo é modificado. Toda afirmação remota/CI/PR/produção DEVE carregar `[NV]`; zero PRs abertos.

## Critérios de sucesso

- **SC-001** — ambos os arquivos contêm o apêndice `2026-11-19T23:12Z` no EOF, com contagens iguais ao snapshot 23:11Z e gate PASS com data/hora; `grep` da nova data retorna uma ocorrência em cada arquivo.
- **SC-002** — `git diff --name-only` lista somente `docs/PROGRESSO.md`, `docs/research/index.md` e os 5 artefatos 083; `git diff --numstat` não registra deleções (`0` na coluna de remoções) para os dois arquivos.
- **SC-003** — nenhuma linha histórica foi removida ou modificada: `docs/PROGRESSO.md:3`, `:12-22`, `:159-164` e `docs/research/index.md:3`, `:186`, `:208-212` permanecem idênticos; 137/137 links seguem válidos.
- **SC-004** — gate autoritativo segue PASS (4 checks) conforme snapshot; nenhum `[NV]` convertido em afirmação verificada; CI/produção/Forgejo/PRs **não verificados nesta avaliação** — reexecução e estado ao vivo **[NV]** (handoff ao main).

## Baseline

Branch `feature/083-docs-verified-refresh`, base `33d174095`; gate PASS (4 checks) e zero PRs no snapshot 23:11Z **[NV]** ao vivo. Últimas notas em `docs/PROGRESSO.md:159-164` e `docs/research/index.md:208-212`. Se nada for feito, leitores continuam ancorados em 04:23:44Z e podem citar contagens mortas.
