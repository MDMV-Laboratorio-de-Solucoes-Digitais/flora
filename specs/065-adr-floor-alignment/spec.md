# Feature Specification: ADR floor alignment 065

**Feature Branch**: `feature/065-adr-floor-alignment`

**Created**: 2026-11-18

**Status**: Approved (SE authorization recorded)

**Input**: "Alinhar ADR-0003/0006 aos pisos reais do gate"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Governança coerente (Priority: P1)

Como maintainer, quero que os ADRs reflitam os pisos realmente exigidos pelo
guardião, para que a documentação de governança não contradiga a autoridade.

**Why this priority**: ADR desatualizado induz decisões erradas sobre o gate.

**Independent Test**: `grep` das citações + saída do guardião.

**Acceptance Scenarios**:

1. **Given** `docs/adr/0003:19`, **When** lido, **Then** traz
   `13/4/8/8/5/6/4/2 — total 51`.
2. **Given** `docs/adr/0006:10,52`, **When** lidos, **Then** trazem `piso 51`.
3. **Given** o guardião, **When** executado, **Then** imprime 51.

### Edge Cases

- SE citou "50"; valor real é 51 após o ciclo 064 — divergência registrada.
- Histórico de `STATE.md` não é reescrito.

## Requirements *(mandatory)*

- **FR-001/002**: cifras atualizadas nos 3 pontos.
- **FR-003**: sem código/gate/constituição/C5/C6.
- **FR-004**: autorização e divergência registradas.

## Success Criteria *(mandatory)*

- **SC-001**: 0 citações antigas de piso.
- **SC-002**: guardião = 51.
- **SC-003**: gates verdes.
- **SC-004**: diff = 2 ADRs + artefatos.

## Assumptions

- Autorização SE cobre exclusivamente documentação de governança.

## Open Questions

Nenhuma.
