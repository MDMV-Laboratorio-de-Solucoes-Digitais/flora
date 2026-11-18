# Feature Specification: DB env fail-closed 054

**Feature Branch**: `feature/054-db-env-fail-closed`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "V1 — DB legs podem passar vacuamente sem env"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Gate não mente sobre cobertura (Priority: P1)

Como mantenedor, quero que o job `db-tests` falhe se as variáveis de conexão
estiverem ausentes, para que required checks verdes signifiquem testes realmente executados.

**Why this priority**: 8 legs e 50 testes poderiam passar sem nenhuma asserção.

**Independent Test**: executar o `run:` do step sem env → exit != 0 antes do cargo.

**Acceptance Scenarios**:

1. **Given** o step `Run DB-backed contract tests`, **When** a env é removida,
   **Then** o step falha antes de iniciar o `cargo test`.
2. **Given** o guardião, **When** as linhas de env são removidas do workflow,
   **Then** ele fica RED.
3. **Given** um DB real isolado, **When** os 8 legs rodam, **Then** permanecem verdes.

### Edge Cases

- URL definida mas inválida: conexão falha → teste falha → leg RED (já correto).
- A expansão `:?` ocorre dentro de `run_leg`, antes do cargo, em todos os 8 legs.
- Nenhum required check é renomeado; proteção de branch inalterada.

## Requirements *(mandatory)*

- **FR-001**: `run_leg` DEVE usar `: "${VAR:?…}"` para as duas variáveis.
- **FR-002**: guardião DEVE exigir as duas chaves de env no bloco `db-tests`.
- **FR-003**: 8 pisos, 4 checks e nomes de jobs NÃO DEVEM mudar.
- **FR-004**: testes `#[ignore]` NÃO DEVEM ser editados.

## Success Criteria *(mandatory)*

- **SC-001**: sem env → exit != 0 antes do cargo.
- **SC-002**: com DB real → 8 legs verdes (50 testes).
- **SC-003**: mutação temporária (remover env do workflow em cópia) → guardião RED.
- **SC-004**: gates locais permanecem verdes.

## Assumptions

- `set -euo pipefail` permanece no topo do step (`rust.yml:89`).
- O job `db-tests` é inteiramente DB-backed; exigir env em todos os legs é correto.

## Open Questions

Nenhuma. Execução no runner: **[NV]**.
