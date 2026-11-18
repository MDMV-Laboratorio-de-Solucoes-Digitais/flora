# Feature Specification: DB tests fail locally 059

**Feature Branch**: `feature/059-db-tests-fail-local`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Eliminar passed vazio dos contratos DB sem env"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Teste sem DB não finge sucesso (Priority: P1)

Como desenvolvedor, quero que `cargo test --ignored` sem env falhe, para não
acreditar que os contratos DB passaram quando nada foi executado.

**Why this priority**: 50 testes reportam `passed` em 0.00s sem asserção.

**Independent Test**: rodar um leg --ignored sem env → exit != 0.

**Acceptance Scenarios**:

1. **Given** nenhuma env de DB, **When** um leg roda, **Then** falha com erro explícito.
2. **Given** DB real isolado, **When** os 50 testes rodam, **Then** passam.
3. **Given** `--list`, **When** executado, **Then** descobre os mesmos 50 testes.

### Edge Cases

- Os 9 helpers incluem 2 módulos em `auth.rs`; os outros 7 arquivos têm 1 cada.
- `else { return Ok(()) }` dos 50 chamadores fica inalcançável mas compila.
- CI já é fail-closed por `:?` em `run_leg` — nada muda nele.

## Requirements *(mandatory)*

- **FR-001**: 9 `db_state()` retornam `Err` sem env.
- **FR-002**: 50 chamadores intactos.
- **FR-003**: com DB real, testes passam.
- **FR-004**: CI/gate/ADRs intactos.

## Success Criteria *(mandatory)*

- **SC-001**: sem env → exit != 0.
- **SC-002**: `--list` = 50 testes.
- **SC-003**: com DB → 50 passed.
- **SC-004**: gates locais verdes.

## Assumptions

- `anyhow::bail!` disponível (dependência já declarada).
- Nenhum teste é executado sem `--ignored` na suíte padrão.

## Open Questions

Nenhuma. Runner: **[NV]**.
