# Feature Specification: Local logger 049

**Feature Branch**: `feature/049-local-logger`

**Created**: 2026-11-17

**Status**: Approved (decision `go (scoped)`)

**Input**: "Remove transporte remoto morto do logger frontend"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Sem requisições 404 no carregamento (Priority: P1)

Como usuário, quero que o frontend não tente enviar logs para um endpoint inexistente,
para que não haja falhas silenciosas nem ruído de rede em cada sessão.

**Why this priority**: o logger é acionado na conexão WebSocket normal, não só em erros.

**Independent Test**: `rg '/api/v1/telemetry/logs' apps/frontend/src` → 0; teste unitário
com `fetch` espião prova que nenhuma chamada é feita.

**Acceptance Scenarios**:

1. **Given** o logger importado, **When** `debug/info/warn/error` são chamados, **Then** nenhuma requisição é emitida.
2. **Given** mais de 200 entradas, **When** `snapshot()` é lido, **Then** retorna no máximo 200 entradas (as mais recentes).

### Edge Cases

- Ambiente node (sem `window`): logger deve funcionar sem timers/listeners.
- Regra `no-console`: nenhuma saída via `console.*`.

## Requirements *(mandatory)*

- **FR-001**: zero requisições de rede no logger.
- **FR-002**: assinaturas públicas inalteradas.
- **FR-003**: buffer limitado a 200 entradas.
- **FR-004**: `snapshot(): readonly LogEntry[]` com cópia defensiva.
- **FR-005**: contrato 002 atualizado.

## Success Criteria *(mandatory)*

- **SC-001**: 0 ocorrências do caminho `/api/v1/telemetry/logs` no frontend.
- **SC-002**: teste `logger.spec.ts` passa no projeto `server`.
- **SC-003**: gates frontend/Rust e `check_gate_integrity.py` verdes.

## Assumptions

- Nenhum consumidor externo de `flush()`.
- Nenhum backend de telemetria será adicionado neste ciclo.

## Open Questions

Nenhumas; remoto **[NV]**.
