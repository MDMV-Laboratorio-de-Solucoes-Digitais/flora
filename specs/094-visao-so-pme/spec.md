# Feature Specification: Visão SO-PME 094

**Feature Branch**: `feature/094-visao-so-pme`

**Created**: 2026-11-20

**Status**: Approved (decision `go` — registro documental NÃO-normativo)

**Input**: "Flora como Sistema Operacional da PME/MDMV — nota de governança, sem mudança de gate"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Visão registrada como nota não-normativa (Priority: P1)

Como SE/dono da MDMV, quero registrar a visão de Flora como Sistema
Operacional da PME/MDMV em nota de governança datada, para orientar direção
sem criar obrigação nem autorizar mudança de autoridade.

**Why this priority**: é o propósito do ciclo — capturar a visão sem efeito normativo.

**Independent Test**: ler `decision.md` e `spec.md` e confirmar natureza NÃO-normativa e data.

**Acceptance Scenarios**:

1. **Given** o ciclo 094, **When** lidos os artefatos, **Then** a visão aparece como nota NÃO-normativa datada 2026-11-20.
2. **Given** a nota, **When** verificada, **Then** nenhuma emenda/gate/workflow/migração/código é introduzido.

### User Story 2 — Mapa de integrações, pessoas e agents (Priority: P2)

Como futuro contribuidor, quero um mapa único das integrações, pessoas e
agents MDMV citados na visão, para entender o escopo sem inferir dependências.

**Why this priority**: torna a visão concreta e verificável sem implementação.

**Independent Test**: conferir cada item do mapa neste documento.

**Acceptance Scenarios**:

1. **Given** o mapa, **When** lido, **Then** constam Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma.
2. **Given** o mapa, **When** lido, **Then** constam sócios/funcionários/clientes e Autonoma, dark-factory, marketing, monetization, seller.

### User Story 3 — Status de produção e append-only (Priority: P3)

Como sessão futura, quero o status de produção com evidência e registro
append-only, para reconsultar fatos sem revalidar trabalho alheio.

**Why this priority**: preserva evidência datada sem alterar histórico.

**Independent Test**: comparar evidências registradas com o `git diff` do ciclo.

**Acceptance Scenarios**:

1. **Given** o status, **When** lido, **Then** `/health` 200, deploy 21:23:59Z/`cee7b0cda`, refs Forgejo sincronizados, advisories 0.
2. **Given** os artefatos, **When** comparados, **Then** nenhuma linha anterior é alterada (append-only).

### Edge Cases

- Evidência de produção datada pode ficar desatualizada — registrada como fatos da sessão, não revalidada aqui; **[NV]** para estado remoto.
- Nenhuma linha/artefato anterior é modificado; nenhum toque em `STATE.md` / `validate.md` / `release.md`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: registrar nota de visão do SE — Flora como Sistema Operacional da PME/MDMV, ponto único de trabalho e integração entre sistemas, pessoas e agents MDMV, com dogfooding na MDMV; natureza NÃO-normativa.
- **FR-002**: incluir mapa de integrações — sistemas (Coolify, Forgejo, Grafana+VictoriaMetrics, Uptime Kuma), pessoas (sócios/funcionários/clientes) e agents MDMV (Autonoma, dark-factory, marketing, monetization, seller).
- **FR-003**: registrar status de produção com evidência — `/health` 200, deploy `finished` 21:23:59Z commit `cee7b0cda`, refs Forgejo sincronizados, advisories de runtime 0.
- **FR-004**: manter registro append-only e datado (2026-11-20), sem alterar linhas/artefatos anteriores.
- **FR-005**: garantir zero mudança de gate — nenhuma emenda constitucional, gate, workflow, migração ou código; gate permanece `check_gate_integrity.py` sob ADR-0003.

### Key Entities

- **Nota de visão (NÃO-normativa)**: registro datado da direção SE, sem força de obrigação.
- **Mapa de integrações**: sistemas, pessoas e agents listados na visão.
- **Status de produção**: evidência datada de `/health`, deploy, refs e advisories.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: nota datada 2026-11-20 contém FR-001..FR-005 integralmente.
- **SC-002**: status de produção registra as 4 evidências (health, deploy+commit, refs, advisories) com valores verificáveis.
- **SC-003**: alterações do ciclo confinadas aos 4 artefatos centrais, à nota de governança datada e aos apêndices append-only em `STATE.md`/`PROGRESSO.md`; nenhuma modificação em gate/código/constituição/ADRs nem em registros de fase preexistentes (`validate.md`/`release.md`); `check_gate_integrity.py` PASS.

## Assumptions

- Evidência de produção fornecida pelo contexto da sessão, não revalidada aqui (**[NV]** para estado remoto).
- `check_gate_integrity.py` sob ADR-0003 continua sendo o gate autoritativo.

## Open Questions

Nenhuma; aprovações e estado remoto: **[NV]**.
