# Release: 112 — correção WCAG 2.2 AA

**Ciclo**: 112 · **Data**: 2026-11-22 · **Baseline**: `ce3ce632e`

## Entregue

Correção de BUG-111-01/02/03: nomes acessíveis nos botões ícone, contraste ≥4.5:1 em botões desabilitados e `<title>` no documento. Resumo axe legível no log do CI; nenhuma regra desativada.

## SemVer

PATCH `0.7.13` → `0.7.14`: correção de acessibilidade sem mudança de API/contrato/migration.

## Changelog

FR-001/BUG-111-01 · FR-002/BUG-111-02 · FR-003/BUG-111-03 · FR-004 (log). R5 `bug.assess → bug.fix → bug.test`.

## Evidência

R4 APROVADO PARA CI · prettier/check locais OK · gate PASS (10 legs / 77 pisos). CI e ACR pendentes — **[NV]**.

## Rollback

Novo commit de reversão dos arquivos alterados; sem force-push.

## [NV]

Modo escuro, staging real, deploy.
