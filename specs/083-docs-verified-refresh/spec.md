# Feature Specification: refresh documental verificado — 083

**Feature Branch**: `feature/083-docs-verified-refresh`
**Created**: 2026-11-19T23:14Z
**Status**: Approved (`decision.md` — go (scoped), opção A, E=5,0)
**Input**: "atualizar os apêndices datados de estado verificado no EOF de `docs/PROGRESSO.md` e `docs/research/index.md`" — decidido em `.specify/assessments/083-docs-verified-refresh/decision.md`; tratado como refresh documental append-only, sem mudança de comportamento, código ou contrato.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Âncora verificada atualizada em `docs/PROGRESSO.md` (Priority: P1)

Como maintainer ou agente de sessão que toma `docs/PROGRESSO.md` como âncora inicial de "estado verificado", quero encontrar, no fim do arquivo, um apêndice datado `2026-11-19T23:12Z` (ciclo 083) com âncora local, contagens do snapshot 23:11Z, gate PASS com data/hora, delimitação de escopo e `[NV]`, para que decisões futuras não citem as contagens mortas da nota `04:23:44Z`.

**Why this priority**: é o cerne do ciclo. A última nota datada está em `docs/PROGRESSO.md:159-164` (`2026-11-19T04:23:44Z`, ciclo 072); o cabeçalho em `:3` ainda traz `github/dev@94fcf19ca` (ciclo 055) e a tabela em `:12-22` descreve o checkout de `2026-11-18T02:23Z`. O snapshot determinístico `2026-11-19T23:11Z` mediu specs 72, assessments 69, validations 68, releases 68, bugs 4, research 137 sem `index.md` (138 com ele) e adr 6 — números diferentes dos registrados. Nenhuma linha histórica é reescrita: o apêndice é acréscimo puro após `:164`.

**Independent Test**: `grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md` retorna exatamente uma ocorrência, no EOF; `git diff --numstat -- docs/PROGRESSO.md` registra `0` na coluna de remoções; a leitura do apêndice confirma âncora local, as contagens do snapshot, gate PASS (4 checks) com data/hora, escopo restrito e `[NV]` para remoto/CI/PR/produção.

**Acceptance Scenarios**:

1. **Given** a última nota datada em `docs/PROGRESSO.md:159-164` (`04:23:44Z`) e o EOF em `:164` com newline final, **When** o apêndice `2026-11-19T23:12Z` é acrescentado logo após `:164`, **Then** `grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md` retorna uma ocorrência no EOF e as linhas `:1-164` permanecem byte-idênticas ao base `33d174095`.
2. **Given** o apêndice acrescentado, **When** um leitor o consulta, **Then** encontra âncora local `dev@33d174095` (base do ciclo; estado remoto `[NV]`), contagens 72/69/68/68/4, research 137 sem `index.md`/138 com ele e adr 6, gate PASS (4 checks) com data/hora do snapshot `2026-11-19T23:11Z`, delimitação de escopo e `[NV]` para remoto/CI/PR/produção.

---

### User Story 2 — Âncora verificada atualizada em `docs/research/index.md` (Priority: P1)

Como maintainer ou agente que usa `docs/research/index.md` como inventário de research, quero um apêndice datado equivalente no EOF, com as contagens atuais e a reafirmação de que os links seguem 137/137, para que a correção do ciclo 069 não seja lida como pendente e a dívida 047+ não seja reaberta.

**Why this priority**: a última nota datada está em `docs/research/index.md:208-212` (`04:23:44Z`, ciclo 072); o cabeçalho em `:3-6` registra `2026-11-19T01:58Z` (ciclo 068) e a tabela em `:8-18` descreve contagens antigas. A correção append-only em `:186` (ciclo 069) já provou **137 arquivos / 137 links / `missing=[]` / `extra=[]`** e fechou a dívida 047+ por evidência — reabri-la seria retrocesso. O snapshot 23:11Z mediu 137 arquivos sem `index.md` (138 com ele) e 6 ADRs.

**Independent Test**: `grep -n '2026-11-19T23:12Z' docs/research/index.md` retorna exatamente uma ocorrência, no EOF; `git diff --numstat -- docs/research/index.md` registra `0` na coluna de remoções; o apêndice reafirma 137/137 e `missing=[]` sem alterar `:186` nem regenerar o índice.

**Acceptance Scenarios**:

1. **Given** a última nota em `docs/research/index.md:208-212` e o EOF em `:212`, **When** o apêndice `2026-11-19T23:12Z` é acrescentado após `:212`, **Then** `grep -n '2026-11-19T23:12Z' docs/research/index.md` retorna uma ocorrência no EOF e `:1-212` permanece byte-idêntico ao base.
2. **Given** a correção 137/137 em `:186`, **When** o apêndice é lido, **Then** ele reafirma 137 arquivos / 137 links / `missing=[]`, registra 137 sem `index.md` (138 com ele) e adr 6, e não modifica, renomeia ou "corrige" a nota histórica nem reabre a dívida 047+.

---

### User Story 3 — Histórico preservado, diff estreito e reversível (Priority: P2)

Como maintainer, quero que nenhuma linha pré-existente seja alterada e que o diff rastreado fique restrito aos dois arquivos de destino e aos artefatos do ciclo 083, com gate determinístico e rollback de um único path, para que o refresh seja auditável e reversível sem tocar código, CI, gate, PR ou produção.

**Why this priority**: FR-003/SC-003 são a cerca de compatibilidade e FR-004/SC-004 são a fronteira da mudança. Qualquer edição em `docs/PROGRESSO.md:3`, `:12-22`, `:151`, `:159-164` ou `docs/research/index.md:3`, `:8-18`, `:186`, `:198-206`, `:208-212` transformaria um apêndice em reescrita histórica; qualquer efeito colateral em código, config, gate, script, workflow, ADR, spec numerada, DB/rede ou segredo transformaria o ciclo em mudança de contrato que exige reavaliação. Zero PRs abertos no snapshot; estado ao vivo `[NV]`.

**Independent Test**: `git diff --name-only` lista somente os dois arquivos e os artefatos do ciclo; `git diff --numstat` não registra deleções nos dois arquivos; `python3 scripts/ci/check_gate_integrity.py` segue PASS (4 checks) e `git checkout 33d174095 -- docs/PROGRESSO.md docs/research/index.md` devolve ambos byte-idênticos ao base sem force-push.

**Acceptance Scenarios**:

1. **Given** os apêndices aplicados, **When** o diff é auditado contra `33d174095`, **Then** nenhum outro path é listado além de `docs/PROGRESSO.md`, `docs/research/index.md` e dos artefatos do ciclo 083, e `git diff --numstat` mostra `0` remoções nos dois arquivos.
2. **Given** os gates determinísticos, **When** executados sem DB, rede ou segredos, **Then** o gate autoritativo permanece PASS (4 checks), nenhum `[NV]` é convertido em afirmação verificada e a restauração dos dois paths ao base reverte o ciclo por completo na parte documental.

---

### Edge Cases

- **Apêndice já presente**: FR-001/FR-002 são idempotentes — nada a acrescentar e SC-001 já satisfeito; nenhuma linha é duplicada.
- **`find` ao vivo conta specs 73**: o diretório `specs/083-docs-verified-refresh/` foi materializado às `23:11:17Z`; o `+1` em relação ao snapshot (72) é o próprio ciclo em curso, não uma divergência a corrigir. O apêndice registra 72 (snapshot 23:11Z), não a recontagem pós-materialização.
- **Contagens mudam após a edição**: o apêndice é registro do snapshot 23:11Z; recontagem e reexecução do gate após a edição ficam com o main agent e são `[NV]` nesta spec.
- **Números históricos "mortos"**: `:3`, `:12-22`, `:8-18`, `:159-164` e `:208-212` permanecem com os valores da época, intencionalmente — o apêndice não os substitui nem os "corrige" (FR-003/SC-003).
- **Dívida 047+**: não reabrir. A verificação 137/137 em `:186` (ciclo 069) permanece válida; não regenerar `index.md` por `find` nem inventar fonte `.generated.md`.
- **`[NV]`**: nunca promover a afirmação verificada. Reexecução do gate, CI ao vivo, Forgejo/PRs, merge, proteção de branch, produção e aprovações permanecem marcados.
- **Newline final**: ambos os arquivos terminam com `0a` em `:164`/`:212`; o apêndice é acréscimo após a última linha, mantendo newline final.
- **Worktrees alheios e specs numeradas**: fora de escopo; nenhum histórico de outro ciclo é tocado.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `docs/PROGRESSO.md` DEVE receber, somente no EOF (após `:164`), um apêndice datado `2026-11-19T23:12Z` (ciclo 083) com: âncora local `dev@33d174095` (base do ciclo; remoto `[NV]`), contagens do snapshot `2026-11-19T23:11Z` (specs 72, assessments 69, validations 68, releases 68, bugs 4, research 137 sem `index.md` / 138 com ele, adr 6), gate PASS (4 checks) com data/hora do snapshot, delimitação de escopo e `[NV]` para estado remoto/CI/PR/produção. Nenhuma linha existente é alterada.
- **FR-002**: `docs/research/index.md` DEVE receber, somente no EOF (após `:212`), apêndice datado equivalente com as contagens atuais do snapshot (137 arquivos sem `index.md`, 138 com ele; adr 6), reafirmando 137/137 links / `missing=[]` de `:186` sem reabrir a dívida 047+, e `[NV]` para remoto/CI. Nenhuma linha existente é alterada.
- **FR-003**: todo conteúdo pré-existente DEVE permanecer byte-idêntico, em particular `docs/PROGRESSO.md:3` (âncora `github/dev@94fcf19ca`), `:12-22`, `:151`, `:159-164` e `docs/research/index.md:3`, `:8-18`, `:186`, `:198-206`, `:208-212` (append-only). O apêndice NÃO PODE substituir, renomear ou "corrigir" nota histórica alguma, nem regenerar o índice.
- **FR-004**: o diff rastreado DEVE limitar-se aos dois arquivos de destino e aos 5 artefatos `.specify/assessments/083-docs-verified-refresh/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`), acompanhados dos documentos do próprio ciclo em `specs/083-docs-verified-refresh/`; nenhum código, config, gate, script, workflow, ADR, spec numerada, PR, DB, rede ou segredo é modificado. Toda afirmação remota/CI/PR/produção DEVE carregar `[NV]`; zero PRs abertos.

### Key Entities

N/A — o ciclo é um refresh documental append-only em dois arquivos Markdown; nenhuma entidade de dados, schema, migração ou contrato de leitura é introduzido. As únicas "entidades" são os dois blocos de apêndice datado e os artefatos de especificação do ciclo.

## Success Criteria *(mandatory)*

- **SC-001**: ambos os arquivos contêm o apêndice `2026-11-19T23:12Z` no EOF, com contagens iguais ao snapshot 23:11Z (72/69/68/68/4, research 137 sem `index.md` / 138 com ele, adr 6) e gate PASS (4 checks) com data/hora; `grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md docs/research/index.md` retorna uma ocorrência em cada arquivo.
- **SC-002**: `git diff --name-only` lista somente `docs/PROGRESSO.md`, `docs/research/index.md`, os 5 artefatos 083 e os documentos do próprio ciclo; `git diff --numstat -- docs/PROGRESSO.md docs/research/index.md` não registra deleções (`0` na coluna de remoções) para os dois arquivos — apenas acréscimos. `[NV]` até a auditoria pós-edição.
- **SC-003**: nenhuma linha histórica foi removida ou modificada: `docs/PROGRESSO.md:3`, `:12-22`, `:159-164` e `docs/research/index.md:3`, `:186`, `:208-212` permanecem idênticos ao base `33d174095`; 137/137 links seguem válidos (`missing=[]`). `[NV]` até a auditoria pós-edição.
- **SC-004**: gate autoritativo segue PASS (4 checks) conforme o snapshot; nenhum `[NV]` convertido em afirmação verificada; CI/produção/Forgejo/PRs **não verificados nesta avaliação** — reexecução do gate, recontagem `find` e estado ao vivo `[NV]` (handoff ao main agent).

## Scope / Out of scope

**Dentro**: apêndice datado somente no EOF de `docs/PROGRESSO.md` (após `:164`) e de `docs/research/index.md` (após `:212`); os 6 documentos do ciclo em `specs/083-docs-verified-refresh/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`); e os 5 artefatos de assessment em `.specify/assessments/083-docs-verified-refresh/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`).

**Fora**: `docs/PROGRESSO.md:3`, `:12-22`, `:151`, `:159-164`, `docs/research/index.md:3`, `:8-18`, `:186`, `:198-206`, `:208-212` e todo o histórico; código, CI, gates, scripts, workflows, ADRs, constituição, specs numeradas, PRs, produção, DB/rede, segredos e worktrees alheios. A lista de escopo do `decision.md` foi escrita às 23:12Z, antes da materialização dos 6 documentos do ciclo; estes acompanham o diff como artefatos de especificação, não como código.

## Assumptions

- O snapshot determinístico `2026-11-19T23:11Z` é a fonte da verdade para as contagens do apêndice; o `+1` em `find` ao vivo (specs 73) é a materialização de `specs/083-docs-verified-refresh/` às `23:11:17Z`, não uma divergência a corrigir.
- `docs/PROGRESSO.md:164` e `docs/research/index.md:212` são os EFIs atuais, ambos com newline final (`0a`): o apêndice é acréscimo puro.
- Gate autoritativo `python3 scripts/ci/check_gate_integrity.py` → PASS, 4 checks no snapshot 23:11Z; reexecução pós-edição `[NV]`.
- A correção 137/137 em `docs/research/index.md:186` (ciclo 069) é válida e fecha a dívida 047+; não há fonte regenerável verificável para o índice, que não deve ser regenerado.
- Base de comparação do diff: `33d174095` (HEAD da branch `feature/083-docs-verified-refresh` no início do ciclo).
- `.specify/feature.json` é estado local ignorado, reescrito a cada troca de feature; hoje registra `specs/083-docs-verified-refresh` (a divergência 082→083 está resolvida em `clarify.md`).

## [NV]

- Execução dos comandos determinísticos pós-edição (`git diff --name-only`, `git diff --numstat`, `grep -n '2026-11-19T23:12Z'`, reexecução de `python3 scripts/ci/check_gate_integrity.py` e recontagem `find`) — fica com o main agent; não executada nesta avaliação (marcadores em SC-002 e SC-003).
- Gate ao vivo, CI, Forgejo/PRs, merge, proteção de branch, produção e aprovações — nenhuma inferência além da evidência local e de `decision.md`.
- Zero PRs abertos no snapshot; estado remoto ao vivo não verificado.
