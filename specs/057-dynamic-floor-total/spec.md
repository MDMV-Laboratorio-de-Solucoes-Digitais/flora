# Feature Specification: Dynamic floor total 057

**Feature Branch**: `feature/057-dynamic-floor-total`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Eliminar duplicação de verdade no total de pisos do guardião"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Saída do gate acompanha FLOORS (Priority: P1)

Como mantenedor, quero que a mensagem de sucesso do guardião seja derivada de
`FLOORS`, para que alterar um piso não exija editar também um número escrito à mão.

**Why this priority**: o literal já causou drift em quatro ciclos consecutivos.

**Independent Test**: mutação em cópia `/tmp` adicionando um 9º piso → texto muda.

**Acceptance Scenarios**:

1. **Given** o guardião atual, **When** executado, **Then** imprime
   `"all eight db contract floors intact (50 total)"`.
2. **Given** uma cópia com 9 pisos, **When** executada, **Then** imprime o
   total 54 sem edição da mensagem.
3. **Given** a regex de piso, **When** um piso diverge, **Then** a falha é a mesma de antes.

### Edge Cases

- Texto "eight" também é derivado de `len(FLOORS)` (pode virar "nine").
- A soma usa `sum(FLOORS.values())`; nenhum outro uso de `FLOORS` muda.
- Nenhum required check é renomeado.

## Requirements *(mandatory)*

- **FR-001**: mensagem deriva `len(FLOORS)` e `sum(FLOORS.values())`.
- **FR-002**: regex e mensagens de falha inalteradas.
- **FR-003**: saída atual preservada.
- **FR-004**: 1 linha, 1 arquivo.

## Success Criteria *(mandatory)*

- **SC-001**: guardião PASS com saída idêntica.
- **SC-002**: mutação com 9º piso imprime 54.
- **SC-003**: gates locais verdes.
- **SC-004**: diff = 1 linha.

## Assumptions

- `FLOORS` permanece a fonte canônica dos pisos (já usada na validação).
- Nenhuma outra ferramenta depende do literal exato além da saída humana.

## Open Questions

Nenhuma. Estado remoto: **[NV]**.

## Correção pós-implementação (2026-11-18T02:47Z)

SC-001 passa a exigir a saída `all 8 db contract floors intact (50 total)`
(numeral derivado de `len(FLOORS)`, antes a palavra `eight`). A mutação com
9º piso + leg correspondente imprime `all 9 db contract floors intact (54 total)`.
