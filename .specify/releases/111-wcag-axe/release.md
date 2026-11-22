# Release: 111 — auditoria WCAG 2.2 AA automatizada

**Ciclo**: 111 · **Data**: 2026-11-22 · **Baseline**: `8c47dd637`

## Entregue

Dependência dev `@axe-core/playwright` (lockfiles sincronizados) e teste e2e de acessibilidade em `/tasks` e `/channels` com tags WCAG 2.0/2.1/2.2 A+AA. Nenhum required check, gate, migration ou credencial alterado.

## SemVer

PATCH `0.7.12` → `0.7.13`: inclusão de teste informativo e dependência de desenvolvimento; sem mudança de API ou comportamento de produção.

## Changelog

FR-001 dependência e lockfiles · FR-002 teste axe-core · FR-003 nenhum required check. Decisão F9/FR-06/FR-08 do assessment raiz. Sem bug report em `.specify/bugs/`.

## Evidência

R4 APROVADO PARA CI · `bun run check` 0 erros · prettier OK · gate PASS (10 legs / 77 pisos). E2E local [NV] por exaustão de RAM; CI advisory decide.

## Rollback

Novo commit de reversão dos quatro arquivos; sem force-push.

## [NV]

Resultado do job `e2e-browser` no CI e qualquer execução em staging real.
