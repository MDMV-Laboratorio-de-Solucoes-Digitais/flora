# Feature Specification: C6 canário RLS (design) 067

**Feature Branch**: `feature/067-c6-canary-rls-design`

**Created**: 2026-11-18

**Status**: Approved (SE authorization recorded — document-only scope)

**Input**: "Desenhar canário RLS com rollback, sem habilitação em produção"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Desenho auditável (Priority: P1)

Como maintainer/SE, quero um desenho de canário RLS com pré-condições, fases,
critérios de saída e rollback, para decidir a promoção com evidência em vez de
intuição.

**Why this priority**: ADR-0005 já provou que ligar FORCE hoje quebraria a app;
sem desenho, a decisão de promoção fica indefinida e C6 permanece aberto.

**Independent Test**: leitura do documento + checagem dos itens (a)–(d) do ADR-0006.

**Acceptance Scenarios**:

1. **Given** a migração init, **When** o documento é lido, **Then** distingue
   `ENABLE ROW LEVEL SECURITY` de enforcement ativo e cita `file:line`.
2. **Given** o documento, **When** as fases são lidas, **Then** há critérios de
   entrada/saída e nenhuma fase toca produção sem autorização do SE.
3. **Given** uma falha no canário, **When** o rollback é aplicado, **Then** as
   queries voltam a retornar linhas sem exclusão/atualização de dados.

### User Story 2 — Documentação coerente (Priority: P2)

Como agente/maintainer, quero contagens verificadas e índices atualizados, para
que o estado do repo não contradiga os gates.

**Why this priority**: `PROGRESSO.md` e `research/index.md` estavam defasados
(ciclo 060 / sem seção 2026-11-18), induzindo decisões erradas.

**Independent Test**: `find`/scripts comparados com as notas datadas.

**Acceptance Scenarios**:

1. **Given** `docs/PROGRESSO.md`, **When** lido, **Then** traz nota datada com
   contagens reais e 8 pisos / 51 total.
2. **Given** `docs/research/index.md`, **When** lido, **Then** inclui a entrada da
   pesquisa ES (PR #285) e a deste ciclo.

### Edge Cases

- `users`/`organizations` não entram no canário (lookup de login precede o GUC).
- GUC ausente/inválido ⇒ `NULL` ⇒ zero linhas; nunca fallback permissivo.
- `set_config(...,true)` sem `BEGIN` não persiste — pré-condição explícita.
- Nenhuma flag do canário pode condicionar required check.

## Requirements *(mandatory)*

FR-001 a FR-007 e SC-001 a SC-004 conforme
`.specify/assessments/067-c6-canary-rls-design/define.md`.

## Non-functional

- Nenhuma credencial, nenhum acesso a produção, nenhuma action de deploy.
- Histórico append-only preservado em `STATE.md`, `PROGRESSO.md`, `index.md`,
  `converge.md`.

## Out of scope

Implementação do wrapper, policies em runtime, alteração de required checks,
ADRs, constituição, código, workflows e migrações.
