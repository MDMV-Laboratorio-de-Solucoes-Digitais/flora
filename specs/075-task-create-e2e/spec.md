# Feature Specification: E2E dedicado de criação de tarefa — 075

**Feature Branch**: `feature/075-task-create-e2e` · **Created**: 2026-11-19
**Status**: Approved (`decision.md` — go scoped)

## User Scenarios

### User Story 1 — Guard da UI (P1)
Como maintainer, quero prova de que `Create task` fica desabilitado sem workspace/título, para detectar regressão de validação.

### User Story 2 — Criação com descrição (P1)
Como maintainer, quero criar tarefa com título e descrição únicos e ver a confirmação, para cobrer o contrato da Fase 3.

### User Story 3 — Coluna Todo (P2)
Como maintainer, quero ver o card na coluna Todo, para validar o mapeamento de status.

## Requirements

- **FR-001** Reforçar `apps/frontend/tests/e2e/seed-flow.e2e.ts` (único consumidor da sessão one-shot); arquivo adicional exigiria re-mint por teste e quebraria a suíte — divergência da issue #193 registrada.
- **FR-002** Seleção de org Acme com escopo por linha, como em `seed-flow.e2e.ts:16-20`.
- **FR-003** Asserção do botão desabilitado antes de workspace/título.
- **FR-004** Criação com `crypto.randomUUID().slice(0,8)`, descrição e checagem `Task created: <title>`.
- **FR-005** Card e descrição visíveis na região `To Do column` (`aria-label` real) com `getByText(..., { exact: true })`.
- **FR-006** Nenhuma mudança em código de produção, CI, worktree 073 ou issues.

## Success Criteria

- **SC-001** `playwright test --list` descobre o teste.
- **SC-002** `bun run check` + lint + Prettier sem erros no arquivo.
- **SC-003** diff = 1 arquivo de teste + artefatos 075.
- **SC-004** gates do alvo PASS; nenhum DB compartilhado mutado.

## Scope

Dentro: 1 arquivo E2E + artefatos. Fora: código, API, CI, ADRs, constituição, issues, worktree 073. **[NV]** execução completa se ambiente isolado indisponível.
