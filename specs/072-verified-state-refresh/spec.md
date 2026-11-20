# Feature Specification: Refresh verificado de estado — 072

**Feature Branch**: `feature/072-verified-state-refresh`
**Created**: 2026-11-19
**Status**: Approved (`decision.md` — go scoped)
**Input**: "Registrar por apêndice datado o estado verificado de 2026-11-19 nos documentos âncora do flora"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Âncora atual para agentes (Priority: P1)

Como agente que inicia uma sessão no flora, quero ler uma nota datada e recente em cada documento âncora, para não decidir com base no estado de 01:58Z (ciclo 068).

**Why this priority**: os ciclos 069 e 070 já estão fechados; sem nota nova, a leitura ingênua produz conclusões erradas.

**Independent Test**: conferir por `grep -n 'Estado verificado — 2026-11-19'` nos quatro arquivos e comparar as contagens com `find`/`ls`.

### User Story 2 — Histórico preservado (Priority: P1)

Como maintainer, quero que a correção seja append-only, para auditar a evolução das notas sem reescrever o passado.

**Acceptance Scenarios**: nenhuma linha pré-existente removida ou alterada; `git diff` mostra apenas acréscimos ao final.

### User Story 3 — Dívidas não reabertas (Priority: P2)

Como maintainer, quero que a leg RBAC do CI e a emenda constitucional continuem marcadas como escaladas/`[NV]`, para não virarem “concluídas” por inferência.

## Requirements

- **FR-001** Cada um de `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md` e `docs/research/index.md` DEVE receber apêndice datado com âncora local `cdf50eaff` (início da sessão) e contagens reais: 61 specs, 58 assessments, 57 validations, 57 releases, 4 bugs, 137 arquivos de research (138 com `index.md`).
- **FR-002** O apêndice DEVE cobrir explicitamente os ciclos 069 e 070 e o intervalo 001–070, preservando a correção 137/137 do ciclo 069.
- **FR-003** Nenhuma linha histórica dos quatro documentos PODE ser removida ou modificada; mudanças são apenas acréscimos no fim.
- **FR-004** Toda afirmação sobre CI ao vivo, merge, Forgejo, aprovações ou produção DEVE permanecer marcada `[NV]`.
- **FR-005** `.specify/memory/STATE.md` DEVE registrar decisão 80/20, execução e fechamento do ciclo 072; `index.md` da raiz DEVE receber nota curta quando apontar âncoras antigas.

## Success Criteria

- **SC-001** `grep`/`find` confirma apêndice datado nos quatro documentos com as contagens de FR-001.
- **SC-002** `git diff --name-only dev..HEAD` contém somente os documentos âncora, `index.md`, `STATE.md` e artefatos de `specs/072-*`/`.specify/{assessments,validations,releases}/072-*`.
- **SC-003** `git diff --numstat` não apresenta deleções nos quatro documentos âncora; comparação byte-a-byte do prefixo pré-existente confirma append-only.
- **SC-004** `python3 scripts/ci/check_gate_integrity.py` permanece PASS com 4 required checks e 8 pisos/51; nenhum arquivo de workflow, script ou piso é tocado.

## Scope / Out of scope

**Dentro**: documentação âncora + artefatos SDD + STATE.md.
**Fora**: código Rust/Svelte, testes, workflows, `scripts/ci/`, pisos, ADRs, constituição, migrations, worktree/branch 071, leg RBAC, emenda constitucional, produção.

## [NV]

Status remoto, CI ao vivo, proteção de branch, Forgejo e produção não são verificados nesta spec; PR será aberto sem merge.
