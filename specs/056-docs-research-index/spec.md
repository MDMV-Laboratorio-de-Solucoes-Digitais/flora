# Feature Specification: Docs research index 056

**Feature Branch**: `feature/056-docs-research-index`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Fechar dívida de documentação: PROGRESSO.md + índice de research"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Entrada verificada para o corpus (Priority: P1)

Como mantenedor ou agente, quero um índice completo dos documentos de research
e uma nota de progresso com estado verificado, para localizar evidências sem
varredura ad hoc.

**Why this priority**: 135 documentos sem índice e nenhum `PROGRESSO.md`.

**Independent Test**: contar entradas do índice e comparar com `find`; validar links.

**Acceptance Scenarios**:

1. **Given** `docs/research/`, **When** o índice é gerado, **Then** contém 135 entradas.
2. **Given** cada entrada, **When** resolvida, **Then** o arquivo existe.
3. **Given** `docs/PROGRESSO.md`, **When** lido, **Then** traz data, âncora,
   contagens, gates, dívidas e **[NV]**.

### Edge Cases

- Arquivos sem cabeçalho `# ` usam o nome do arquivo como título.
- Agrupamento por data extraído do prefixo `YYYY-MM-DD`.
- Nenhum documento de ADR/agents entra no índice (escopo apenas research).

## Requirements *(mandatory)*

- **FR-001**: 135 entradas com links relativos válidos.
- **FR-002**: geração determinística a partir dos arquivos atuais.
- **FR-003**: `PROGRESSO.md` com nota datada de estado verificado.
- **FR-004**: nenhum código/workflow/ADR alterado.

## Success Criteria *(mandatory)*

- **SC-001**: entradas = 135 = `find | wc -l`.
- **SC-002**: 100% dos links resolvem.
- **SC-003**: verificador isolado confirma cobertura.
- **SC-004**: gates do alvo permanecem verdes.

## Assumptions

- O corpus não muda durante a geração (verificado por contagem antes/depois).
- Nenhuma afirmação remota é feita sem **[NV]**.

## Open Questions

Nenhuma. Estado remoto: **[NV]**.
