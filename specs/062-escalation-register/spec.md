# Feature Specification: Escalation register 062

**Feature Branch**: `feature/062-escalation-register`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Consolidar escalonamentos e estado OSV verificado"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Fila bloqueada transparente (Priority: P1)

Como sessão autônoma futura, quero uma tabela única em `STATE.md` com o que
está bloqueado, por quê e o que é necessário, para não reavaliar ou executar
trabalho sem autorização.

**Why this priority**: evita sequestro de autoridade e gasto repetido de análise.

**Independent Test**: ler a tabela e comparar cada linha com as fontes citadas.

**Acceptance Scenarios**:

1. **Given** `STATE.md`, **When** lido ao final, **Then** há tabela com 4 itens.
2. **Given** cada item, **When** verificado, **Then** bloqueio e ação necessária explícitos.
3. **Given** o OSV, **When** comparado com `gh api`, **Then** estado e contagem batem.

### Edge Cases

- `workflow_dispatch` existe mas não foi disparado — nenhum efeito remoto.
- Nenhuma linha anterior de `STATE.md` é alterada.

## Requirements *(mandatory)*

- **FR-001**: tabela com 4 itens escalados.
- **FR-002**: estado OSV verificado ao vivo.
- **FR-003**: nenhum item bloqueado executado.
- **FR-004**: sem código/ADR/migração/gate.

## Success Criteria *(mandatory)*

- **SC-001**: 4 linhas completas.
- **SC-002**: OSV confere com `gh api`.
- **SC-003**: verificador PASS.
- **SC-004**: gates verdes.

## Assumptions

- `gh api` é leitura autorizada de metadados públicos do repo.
- Nenhuma aprovação é simulada.

## Open Questions

Nenhuma; aprovações: **[NV]**.
