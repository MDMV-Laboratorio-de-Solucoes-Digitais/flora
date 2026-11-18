# Feature Specification: DB gate uncovered legs 053

**Feature Branch**: `feature/053-db-gate-uncovered-legs`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Cobrir os 6 testes DB ignorados sem leg no gate (C25b)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Gate cobre todos os contratos DB (Priority: P1)

Como mantenedor, quero que todo teste DB `#[ignore]` tenha um leg no job
`db-tests`, para que nenhum contrato fique fora da decisão de merge.

**Why this priority**: 6 de 50 contratos (12%) nunca executam na CI.

**Independent Test**: `cargo test --list` nos dois filtros → 4 e 2;
`check_gate_integrity.py` → PASS com 8 pisos.

**Acceptance Scenarios**:

1. **Given** o workflow `rust.yml`, **When** inspecionado, **Then** contém os legs
   `tasks` e `workspace` com filtros `db::db_` e pisos 4 e 2.
2. **Given** o guardião, **When** executado, **Then** valida oito pisos (49 total).
3. **Given** os required checks, **When** o merge for decidido, **Then** permanecem os mesmos 4.

### Edge Cases

- Filtros curtos casam zero testes — o leg deve usar o caminho completo.
- Nenhum required check é renomeado; proteção de branch permanece válida.
- O fail-open de `db_state()` permanece como dívida V1 (fora de escopo, **[NV]**).

## Requirements *(mandatory)*

- **FR-001**: leg `tasks` → `contract::tasks::db::db_`, piso 4.
- **FR-002**: leg `workspace` → `contract::workspace::db::db_`, piso 2.
- **FR-003**: `FLOORS` espelha 8 entradas; regex aceita `db::db_` sem afrouxar as antigas.
- **FR-004**: required checks, ADR-0003 e constituição inalterados.

## Success Criteria *(mandatory)*

- **SC-001**: dois novos legs presentes com filtros completos.
- **SC-002**: `--list` confirma 4 e 2 testes.
- **SC-003**: guardião PASS com mensagem 8 pisos / 49 total.
- **SC-004**: gates locais permanecem verdes.

## Assumptions

- Os seis testes passam no Postgres 18 do CI (verificação ao vivo **[NV]**).
- `run_leg` continua contando apenas `N passed`, sem mudança de semântica.

## Open Questions

Nenhuma. Execução remota dos novos legs: **[NV]**.
