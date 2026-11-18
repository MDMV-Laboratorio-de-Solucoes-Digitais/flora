# Feature Specification: Root doc index 058

**Feature Branch**: `feature/058-root-doc-index`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Criar index.md na raiz (dívida registrada no Bloco 4)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Entrada única de documentação (Priority: P1)

Como mantenedor ou agente, quero um `index.md` na raiz que aponte para visão,
roadmap, estado verificado, índice de research e ADRs.

**Why this priority**: Bloco 4 exige o arquivo; ele é o único ponto de entrada ausente.

**Independent Test**: validar todos os links relativos do índice.

**Acceptance Scenarios**:

1. **Given** a raiz, **When** `index.md` é aberto, **Then** lista os pontos de entrada.
2. **Given** cada link, **When** resolvido, **Then** o arquivo/diretório existe.
3. **Given** o cabeçalho, **When** lido, **Then** traz data, âncora e **[NV]**.

### Edge Cases

- `guide-to-strict-rust.md` é gitignored — não deve ser linkado.
- `AGENTS.md` é convenção, não estado verificado — agrupado em "Referência".
- Nenhuma contagem duplicada de `PROGRESSO.md`.

## Requirements *(mandatory)*

- **FR-001**: links relativos válidos para os 7 alvos nomeados.
- **FR-002**: data, âncora e `[NV]` no cabeçalho/rodapé.
- **FR-003**: sem duplicação de conteúdo.
- **FR-004**: nenhum código/workflow/ADR alterado.

## Success Criteria *(mandatory)*

- **SC-001**: 100% dos links resolvem.
- **SC-002**: ≤ 40 linhas.
- **SC-003**: verificador isolado PASS.
- **SC-004**: gates verdes.

## Assumptions

- Os documentos linkados permanecem no repo (todos versionados).
- Nenhuma afirmação remota sem **[NV]**.

## Open Questions

Nenhuma. Estado remoto: **[NV]**.
