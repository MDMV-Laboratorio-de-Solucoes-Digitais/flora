# Feature Specification: 092 — npm lock sync

**Feature Branch**: `feature/092-npm-lock-sync`

**Created**: 2026-11-20

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/092-npm-lock-sync/decision.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Lockfile npm coerente com a fonte de verdade (Priority: P1)

Como mantenedor de `apps/frontend`, preciso que `package-lock.json` reflita o
manifesto e o `bun.lock` canónico, para que o lockfile npm não seja fonte de
informação falsa (entrada `shadcn-svelte` ausente, 25 alertas acumulados).

**Why this priority**: único objetivo do ciclo; sem ele, o lockfile npm permanece
congelado e divergente, enquanto a CI usa `bun.lock`.

**Independent Test**: comparar `package-lock.json` com `package.json`/`bun.lock`
e reexecutar o comando contratual, verificando que não produz novas alterações.

**Acceptance Scenarios**:

1. **Given** `package-lock.json` sem `shadcn-svelte` e `bun.lock` canónico com a
   dependência, **When** a sincronização é executada, **Then**
   `package-lock.json` passa a conter a entrada e a refletir o manifesto.
2. **Given** o lockfile já sincronizado, **When** o comando contratual é
   reexecutado, **Then** nenhuma alteração adicional é produzida.

### Edge Cases

- Se `npm` não estiver disponível ou o registry falhar, a tarefa falha
  explicitamente; não há fallback que altere `bun.lock` ou o manifesto.
- `--ignore-scripts` impede lifecycle scripts; `node_modules` não é alvo.
- A operação não audita nem resolve individualmente os 25 alertas do
  Dependabot; elimina apenas o drift estrutural conhecido.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE manter `apps/frontend/package-lock.json` em
  conformidade com `apps/frontend/package.json` e com o lockfile canónico
  `apps/frontend/bun.lock`, incluindo a dependência `shadcn-svelte`.
- **FR-002**: A sincronização DEVE ser executada em `apps/frontend` com o
  comando contratual `npm install --package-lock-only --ignore-scripts`.
- **FR-003**: A sincronização NÃO DEVE modificar `apps/frontend/package.json`
  nem `apps/frontend/bun.lock`.
- **FR-004**: A sincronização NÃO DEVE executar scripts de ciclo de vida de
  dependências nem alterar código-fonte, configuração de CI, `validate`,
  `release` ou `STATE`; o escopo é exclusivamente o lockfile npm.
- **FR-005**: A sincronização DEVE ser idempotente e DEVE registrar a evidência
  do resultado nos artefatos da feature (decisão, spec, plano e tarefas).

### Key Entities

- **package-lock.json**: lockfile npm em `apps/frontend`, alvo da sincronização.
- **bun.lock**: lockfile canónico para CI; fonte de verdade, intacto.
- **package.json**: manifesto de dependências; fonte de verdade, intacto.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Após a sincronização, `package-lock.json` contém `shadcn-svelte`
  e não apresenta drift de primeira ordem face ao manifesto/`bun.lock`; a
  reexecução do comando contratual produz zero alterações adicionais.
- **SC-002**: O diff da mudança lista exclusivamente
  `apps/frontend/package-lock.json` — `package.json`, `bun.lock`, source e CI
  permanecem intactos.
- **SC-003**: O comando contratual termina com código de saída zero, sem
  execução de lifecycle scripts e sem exposição de credenciais.

## Assumptions

- `npm` e o registry estão disponíveis no ambiente de execução.
- O caminho canónico da CI continua sendo `bun.lock`; este ciclo não troca o
  gerenciador de lockfile da CI.
- Os alertas do Dependabot são contexto de evidência, não escopo de remediação.
