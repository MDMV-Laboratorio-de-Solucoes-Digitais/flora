# Feature Specification: Dead circuit breaker 061

**Feature Branch**: `feature/061-dead-circuit-breaker`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Remover placeholder de circuit-breaker nunca compilado"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Menos código morto (Priority: P1)

Como mantenedor, quero remover módulo nunca compilado e config órfã, para
reduzir superfície de leitura sem mudar comportamento.

**Why this priority**: Princípio I (debloat) da constituição do alvo.

**Independent Test**: `grep -rn circuit_breaker` → 0; gates verdes.

**Acceptance Scenarios**:

1. **Given** o repo, **When** varrido, **Then** não há referência a circuit_breaker.
2. **Given** `utils/mod.rs`, **When** lido, **Then** continua só `jwt`/`pkce`.
3. **Given** a suíte, **When** executada, **Then** permanece verde.

### Edge Cases

- Env `FLORA_MESSAGING__CIRCUIT_BREAKER_*` passa a ser ignorada sem erro
  (Figment sem `deny_unknown_fields`) — comportamento já inerte.
- Nenhum chamador de `Error::CircuitBreakerOpen` existe fora do arquivo morto.

## Requirements *(mandatory)*

- **FR-001**: arquivo removido.
- **FR-002**: campos/defaults removidos.
- **FR-003**: variante + braço de match removidos.
- **FR-004**: runtime/gate/ADRs inalterados.

## Success Criteria *(mandatory)*

- **SC-001**: 0 ocorrências de `circuit_breaker`.
- **SC-002**: fmt/clippy/deny/build/test exit 0.
- **SC-003**: diff = 3 arquivos de código.
- **SC-004**: gates do alvo verdes.

## Assumptions

- O arquivo nunca foi compilado (não declarado em `mod.rs`).
- Nenhum testo depende dos campos removidos.

## Open Questions

Nenhuma.
