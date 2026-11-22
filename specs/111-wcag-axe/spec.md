# Feature Specification: Auditoria WCAG 2.2 AA automatizada

**Feature ID**: 111 · **Status**: specified · **Data**: 2026-11-22

## User Scenario

Como mantenedor do Flora, quero uma varredura automatizada de acessibilidade nos fluxos já cobertos pelo e2e, para detectar regressões WCAG 2.2 AA antes do merge sem depender apenas de revisão humana.

## Acceptance Scenarios

1. **Given** a suíte e2e com sessão seed, **when** o teste de acessibilidade roda em `/tasks`, **then** `AxeBuilder` retorna zero violações nas tags WCAG 2.0/2.1/2.2 A e AA.
2. **Given** o mesmo teste em `/channels`, **when** há violação, **then** a mensagem de falha lista `id`, `impact` e alvo em pt-BR.
3. **Given** os lockfiles, **when** `bun install --frozen-lockfile` e `npm ci` rodam, **then** ambos resolvem a nova dependência sem divergência.

## Requirements

- FR-001: dependência `@axe-core/playwright` em devDependencies.
- FR-002: teste e2e `accessibility.e2e.ts` cobrindo `/tasks` e `/channels`.
- FR-003: nenhum required check, gate floor, migration ou credencial alterado.

## [NV]

Execução em staging real, CI remoto após merge e aprovação humana de acessibilidade.
