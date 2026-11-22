# Feature Specification: Briefing de Compreensão

**Branch**: `feature/109-comprehension-briefing` · **Created**: 2026-11-22 · **Status**: Specified — go

## User Scenarios

### US1 — Registrar invariantes (P1)

Todo ciclo que altera auth/sessão, RBAC, tenant, bootstrap/recovery ou HITL produz um briefing com intenção, invariantes antes/depois, decisões implícitas, comando de verificação e pergunta-teste ao humano.

### US2 — Revisão deliberada (P2)

O briefing acompanha o PR e permite ao SE validar compreensão sem reler todo o diff.

## Requirements

- **FR-001** template versionado em `.specify/templates/`.
- **FR-002** nota de governança com escopo e obrigatoriedade documental.
- **FR-003** nenhum gate/required check/migration/código alterado.

## Success Criteria

- **SC-001** template presente e referenciado; **SC-002** gate PASS; **SC-003** nenhuma credencial ou segredo no template.

## [NV]

Adesão humana nos próximos ciclos.
