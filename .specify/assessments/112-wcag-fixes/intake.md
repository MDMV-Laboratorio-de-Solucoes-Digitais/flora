# Assess — intake: 112 — correção de violações WCAG

**Data**: 2026-11-22 · **Origem**: `.specify/bugs/wcag-axe-2026-11-22/assessment.md` (R5)

## Problema

O job `e2e-browser` do ciclo 111 encontrou três violações reais: `button-name` (critical), `color-contrast` (serious) e `document-title` (critical). A varredura ficou report-only; os botões ícone sem nome, o contraste insuficiente em botões desabilitados e a ausência de `<title>` permanecem.

## Pedido

Corrigir BUG-111-01/02/03 sem desativar regras axe nem alterar required checks, mantendo a varredura informativa e registrando evidência.

## Fora de escopo

Required checks, migration, backend, deploy, credenciais, auditoria humana completa WCAG.
