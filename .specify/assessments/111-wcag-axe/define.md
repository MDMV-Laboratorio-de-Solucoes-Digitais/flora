# Assess — define: 111

**Data**: 2026-11-22

## Requisitos funcionais

- **FR-001**: instalar `@axe-core/playwright` como dependência de dev do frontend canônico, sincronizando `bun.lock` e `package-lock.json`.
- **FR-002**: adicionar teste e2e que execute `AxeBuilder` com tags `wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa`, `wcag22aa` em páginas autenticadas já usadas pelo smoke (`/tasks`, `/channels`).
- **FR-003**: manter o job `e2e-browser` informativo; nenhum required check, gate floor ou migration alterado.

## Requisitos não funcionais

- Nenhuma credencial no repo ou logs.
- Teste determinístico: sem dependência de dados temporais; reutiliza `alice.json` do setup existente.
- Rollback: novo commit de reversão, sem force-push.
