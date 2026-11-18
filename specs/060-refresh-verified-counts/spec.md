# Feature Specification: Refresh verified counts 060

**Feature Branch**: `feature/060-refresh-verified-counts`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Contagens verificadas desatualizadas após ciclos 057–059"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Estado verificado atual (Priority: P1)

Como leitor de `PROGRESSO.md`, quero uma nota datada com contagens atuais,
para não usar números de uma snapshot anterior como se fossem o presente.

**Why this priority**: `index.md` defere a contagem a este arquivo.

**Independent Test**: comparar cada número da nova tabela com `find`.

**Acceptance Scenarios**:

1. **Given** `PROGRESSO.md`, **When** lido ao final, **Then** há nota de 03:50Z com contagens atuais.
2. **Given** a nota anterior (02:23Z), **When** comparada, **Then** está integralmente preservada.
3. **Given** a âncora, **When** lida, **Then** é `262ac042f` com **[NV]** remoto.

### Edge Cases

- `docs/research/` tem 136 arquivos incluindo `index.md`; a contagem do corpus é 135.
- Nenhum número de ADR/gate muda (6 ADRs, 8 pisos/50).

## Requirements *(mandatory)*

- **FR-001**: nova seção datada com contagens `find`.
- **FR-002**: nota anterior preservada.
- **FR-003**: âncora + **[NV]**.
- **FR-004**: sem código/migration/ADR/gate.

## Success Criteria *(mandatory)*

- **SC-001**: números conferem com `find`.
- **SC-002**: diff só acréscimo ao final.
- **SC-003**: verificador isolado PASS.
- **SC-004**: gates verdes.

## Assumptions

- `find` é a fonte autoritativa de contagens (convenção do alvo).
- Nenhuma afirmação remota sem **[NV]**.

## Open Questions

Nenhuma.
