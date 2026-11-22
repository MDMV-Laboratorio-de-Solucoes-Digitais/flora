# Idea Intake: proptest para invariantes de domínio

- **Slug**: `108-proptest-invariants`
- **Created**: 2026-11-22
- **Source**: decisão F7 do assessment `avaliacao-arquitetura-multiagente-flora`
- **Type**: improvement

## Idea (as captured)

> "proptest para invariantes de domínio (tenant, sessão, RBAC) — ADOPT; dependência já presente, zero serviço novo, sem alterar pisos do gate."

## Restated

Transformar propriedades já implícitas em testes determinísticos com geração de casos, cobrindo permissões, paginação e seletor de recuperação.

## Origin & Context

- **Raised by**: SE, via assessment multiagente.
- **Trigger**: INBOX concluído; próximo 80/20 da fila (F7, score máximo).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: quais invariantes são puros e não exigem DB?]
- [NEEDS CLARIFICATION: proptest! dispara lints `panic`?]
