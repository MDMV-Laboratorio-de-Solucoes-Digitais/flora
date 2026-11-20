# Feature Specification: Release manifest lockstep 047

**Feature Branch**: `feature/047-release-manifest-lockstep`

**Created**: 2026-11-17

**Status**: Approved (decision `go (scoped)`)

**Input**: "Corrige manifesto de release para 6 crates"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Tag de release não falha por contagem falsa (Priority: P1)

Como maintainer, quero que a etapa de verificação do workflow de release aceite
os 6 manifests realmente presentes no workspace, para que uma tag não seja
rejeitada por uma asserção obsoleta.

**Why this priority**: é a única causa determinística identificada para falha de
qualquer release neste checkout.

**Independent Test**: executar localmente o mesmo `jq -e` do workflow contra
`cargo metadata --no-deps`; o resultado deve ser exit 0 quando as versões são iguais.

**Acceptance Scenarios**:

1. **Given** o workspace com 6 pacotes `flora*`, **When** a asserção do workflow é executada, **Then** a cláusula de contagem é satisfeita.
2. **Given** um pacote `flora*` com versão diferente da tag, **When** a asserção é executada, **Then** ela ainda falha (lockstep preservado).

### Edge Cases

- Pacote raiz `flora` conta como manifest (está incluído em `cargo metadata`).
- `jq` ausente na CI: fora de escopo; o workflow já depende de `jq`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O passo "Assert lockstep version" DEVE usar `length == 6`.
- **FR-002**: A cláusula `all versions == $RELEASE_VERSION` DEVE permanecer.
- **FR-003**: A correção DEVE ser limitada a `.github/workflows/release.yml`.
- **FR-004**: Nenhum required check DEVE ser removido, esquivado ou renomeado.

### Key Entities

- **Manifest `flora*`**: pacote retornado por `cargo metadata --no-deps` cujo nome é `flora` ou começa com `flora-`.

## Success Criteria *(mandatory)*

- **SC-001**: O comando de asserção retorna exit 0 com as versões alinhadas (verde observado, colado em `validate.md`).
- **SC-002**: `python3 scripts/ci/check_gate_integrity.py` permanece PASS.
- **SC-003**: Diff revisável ≤ 2 linhas no workflow.

## Assumptions

- O workspace atual (5 membros + pacote raiz) é a fonte autoritativa da contagem.
- Nenhuma crates `flora-tasks`/`flora-files` será reintroduzida neste ciclo.
- Não há execução de release/tag/deploy nesta sessão.

## Open Questions

Nenhuma. Itens remotos permanecem **[NV]**.
