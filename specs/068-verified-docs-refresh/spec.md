# Feature Specification: Refresh verificado de documentação — 068

**Feature Branch**: `feature/068-verified-docs-refresh`
**Created**: 2026-11-19
**Status**: Approved (go scoped em `.specify/assessments/068-verified-docs-refresh/decision.md`)
**Input**: "Corrigir contagens e intervalos defasados em docs, preservando append-only"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Contagens confiáveis (Priority: P1)

Como leitor ou agente que consulta o estado do repositório, quero que as contagens publicadas sejam iguais às de `find`, para basear decisões em evidência em vez de números copiados de ciclos antigos.

**Why this priority**: a tabela de `docs/research/index.md` erra seis colunas e `docs/PROGRESSO.md` erra duas; são os dois primeiros arquivos que um agente lê.

**Independent Test**: executar os comandos `find` de `research.md` e comparar com as cifras publicadas.

**Acceptance Scenarios**:

1. **Given** `docs/research/index.md`, **When** a tabela é lida, **Then** apresenta `137 / 6 / 58 dirs (280 arquivos) / 55 dirs / 54 / 54 / 4` e o cabeçalho diz `137`.
2. **Given** `docs/PROGRESSO.md`, **When** a nota de 2026-11-19 é lida, **Then** as contagens são 58/55/54/54/4/137/6 e as notas anteriores permanecem intactas.

### User Story 2 — Intervalo verificado honesto (Priority: P2)

Como maintainer, quero que `OBJETIVO.md` e `ROADMAP.md` declarem explicitamente que o intervalo observado chega ao ciclo 067, para não induzir leitura de "estado atual" parado em 033/046.

**Independent Test**: `tail` dos dois arquivos mostra nota datada de 2026-11-19 com o intervalo 001–067.

### User Story 3 — Comentário coerente com o código (Priority: P2)

Como revisor de `flora-api`, quero que o cabeçalho de `rbac.rs` descreva os handlers como reais, para não reabrir uma decisão já tomada no ciclo 066.

**Independent Test**: `sed -n '1,8p' flora-api/src/routes/rbac.rs` não contém `placeholders`.

### User Story 4 — Rastreabilidade do ciclo (Priority: P3)

Como auditor, quero os artefatos SDD e o `STATE.md` com a decisão 80/20 e o fechamento datados, para reconstruir por que a mudança foi feita.

## Requirements

- **FR-001** atualizar contagens em `docs/research/index.md` e manter a dívida 047+ registrada.
- **FR-002** anexar nota datada em `docs/PROGRESSO.md` com contagens, gate, âncora e **[NV]**.
- **FR-003** anexar notas datadas em `OBJETIVO.md` e `ROADMAP.md` com intervalo até 067.
- **FR-004** corrigir comentário em `flora-api/src/routes/rbac.rs:4-5`.
- **FR-005** registrar decisão/execução em `STATE.md` e artefatos do ciclo; um arquivo por commit.
- **SC-001** contagens publicadas = `find` atual.
- **SC-002** gates locais e remoto verdes.
- **SC-003** diff limitado aos paths declarados; zero mudança em workflows/migrations/ADRs/constituição/produção.

## Scope / Out of scope

Dentro: os 5 arquivos acima + artefatos do ciclo. Fora: indexação retroativa 047–067, emenda constitucional, código de negócio, contratos de teste, CI, deploy, produção.
