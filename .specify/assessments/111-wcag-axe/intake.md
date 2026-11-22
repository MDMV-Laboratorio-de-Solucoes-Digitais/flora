# Assess — intake: 111 — auditoria WCAG 2.2 AA automatizada

**Data**: 2026-11-22 · **Fonte**: `avaliacao-arquitetura-multiagente-flora` (F9, FR-06, FR-08)

## Problema

O frontend canônico (Svelte 5, `apps/frontend`) não tem verificação automatizada de acessibilidade. A conformidade WCAG 2.2 AA hoje depende apenas de revisão humana (FR-07), o que é lento, não reproduzível e não detecta regressões.

## Pedido

Adicionar varredura automatizada com `axe-core` sobre fluxos já cobertos pelo e2e, em job informativo, sem criar required check novo.

## Fora de escopo

Auditoria humana, correção de todos os achados, mudança de required checks, deploy, credenciais.
