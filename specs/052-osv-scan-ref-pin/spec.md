# Feature Specification: OSV scan ref pin 052

**Feature Branch**: `feature/052-osv-scan-ref-pin`

**Created**: 2026-11-17

**Status**: Approved (decision `go (scoped)`)

**Input**: "Validar tag osv-scan@v2 (próximo 80/20 do ciclo 051)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Varredura advisory semanal executável (Priority: P1)

Como mantenedor, quero que o scan semanal de dependências referencie um ref
existente, para receber o sinal advisory de vulnerabilidades sem depender de um
tag fantasma.

**Why this priority**: é o único sinal de supply-chain agendado do repositório.

**Independent Test**: `git ls-remote` confirma o tag; `grep` no workflow confirma o pin.

**Acceptance Scenarios**:

1. **Given** o workflow `osv-scan.yml`, **When** inspecionado, **Then** usa `@v2.7.1`.
2. **Given** o action upstream, **When** consultado, **Then** `v2.7.1` existe e
   expõe os inputs usados.
3. **Given** os required checks, **When** o gate rodar, **Then** permanecem os mesmos 4.

### Edge Cases

- O job continua não-obrigatório e com `fail-on-vuln: false`.
- Nenhuma credencial ou segredo é lido pelo job.

## Requirements *(mandatory)*

- **FR-001**: o workflow DEVE referenciar um tag existente e documentado.
- **FR-002**: o pin DEVE ser `v2.7.1`.
- **FR-003**: os inputs `scan-args`/`fail-on-vuln` NÃO DEVEM mudar.
- **FR-004**: required checks, ADR-0003 e constituição NÃO DEVEM mudar.

## Success Criteria *(mandatory)*

- **SC-001**: `grep -n 'reusable.yml@'` → `v2.7.1`.
- **SC-002**: `check_gate_integrity.py` → PASS com os 4 checks e pisos 43.
- **SC-003**: diff de código = 1 linha em 1 arquivo.

## Assumptions

- O ref `@v2` não é resolvido pelo GitHub Actions (sem branch/tag `v2` upstream).
- Execução real do job após o pin permanece **[NV]** até o próximo agendamento.

## Open Questions

Nenhuma. Estado remoto de CI **[NV]**.
