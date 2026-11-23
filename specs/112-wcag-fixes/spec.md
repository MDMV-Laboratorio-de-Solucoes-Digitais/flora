# Feature Specification: Correção de violações WCAG 2.2 AA

**Feature ID**: 112 · **Status**: specified · **Data**: 2026-11-22

## User Scenario

Como usuário de leitor de tela, preciso que botões ícone tenham nome acessível, contraste suficiente e que o documento tenha título, para navegar pelo Flora sem barreiras.

## Acceptance Scenarios

1. **Given** a rota `(app)`, **when** o axe roda, **then** `button-name` não aparece para os botões do layout.
2. **Given** um botão desabilitado, **when** o axe mede contraste, **then** a razão é ≥4.5:1.
3. **Given** qualquer página, **when** o HTML é carregado, **then** `<title>` é não vazio.
4. **Given** o job `e2e-browser`, **when** a varredura termina, **then** o resumo "sem achados" aparece no log e nenhuma regra foi desativada.

## Requirements

- FR-001: `aria-label` nos botões ícone com chaves pt-BR.
- FR-002: contraste ≥4.5:1 em botões desabilitados e `--muted-foreground` ajustado.
- FR-003: `<title>` em `app.html`.
- FR-004: varredura informativa com log legível.

## [NV]

Modo escuro, staging real e auditoria humana WCAG.
