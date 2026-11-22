# Assess — intake: 113 — fechamento de bugs documentais stale

**Data**: 2026-11-22

## Problema

Três registros em `.specify/bugs/` descrevem status como proposto/planejado embora o código e os testes já estejam mesclados: D15 (HITL receipt port), OIDC redirect fallback e WCAG-111. Isso infla a dívida aparente e prejudica o próximo 80/20.

## Pedido

Registrar status verificado com evidência determinística, preservando histórico (append-only) e marcando **[NV]** o que depende de ambiente remoto.

## Fora de escopo

Mudança de código, deploy, credenciais, required checks.
