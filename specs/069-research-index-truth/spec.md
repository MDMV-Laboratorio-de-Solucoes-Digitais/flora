# Feature Specification: Corrigir nota falsa de dívida no índice de research — 069

**Feature Branch**: `feature/069-research-index-truth`
**Created**: 2026-11-19
**Status**: Approved (`decision.md` go scoped)
**Input**: "Corrigir afirmação de que 047+ não estão indexados; cobertura real é 137/137"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Estado honesto do índice (Priority: P1)

Como agente ou maintainer que usa `docs/research/index.md` para descobrir pesquisas, quero que as notas de estado reflitam a cobertura real, para não procurar documentos que já estão linkados.

**Why this priority**: a nota atual afirma dívida inexistente e já induziu um candidato 80/20 inválido neste ciclo (backfill 047–067, eliminado por evidência).

**Independent Test**: executar a comparação de conjuntos (arquivos vs links) e conferir `missing=[]`.

**Acceptance Scenarios**:

1. **Given** o índice atual, **When** a comparação de conjuntos é executada, **Then** 137 arquivos e 137 links com `missing=[]`, `extra=[]`.
2. **Given** as notas antigas, **When** a nova nota é lida, **Then** elas permanecem intactas e a nova corrige o significado sem apagar histórico.

### User Story 2 — Rastreabilidade residual (Priority: P2)

Como leitor, quero saber que o resíduo é de atribuição de ciclo nos títulos, não de cobertura, para decidir se vale um ciclo futuro de renomeação.

**Independent Test**: a nota cita explicitamente a diferença entre cobertura e rotulagem.

## Requirements

- **FR-001** apêndice corretivo em `docs/research/index.md` com evidência e comando reprodutível.
- **FR-002** append-only: nenhuma nota antiga reescrita.
- **FR-003** rastreabilidade em `STATE.md`, artefatos SDD e um arquivo por commit.
- **SC-001** `missing=[]`, `extra=[]`, 137 links após a edição.
- **SC-002** gates do alvo verdes (local + `--remote`, Rust, frontend).
- **SC-003** diff restrito aos paths declarados; zero mudança em código/gates/workflows/ADRs.

## Scope / Out of scope

Dentro: 1 arquivo de docs + artefatos. Fora: renomeação de títulos, backfill, constituição, CI, deploy, produção.
