# Plan: 111 — auditoria WCAG 2.2 AA automatizada

**Data**: 2026-11-22 · **Baseline**: `8c47dd637`

## Passos

1. `bun add -d @axe-core/playwright` em `apps/frontend`; sincronizar `package-lock.json` com `npm install --package-lock-only`.
2. Criar `apps/frontend/tests/e2e/accessibility.e2e.ts` com `AxeBuilder` nas rotas `/tasks` e `/channels`.
3. Executar `bun run check`, `bun run lint`, `bun run test:e2e` (com stack local já ativa na API :3000) e gate autoritativo.
4. Corrigir achados reais de acessibilidade encontrados; se não houver, registrar evidência.
5. R4 validate → R7 release → PR → CI → merge em `dev` → promoção escalada ao SE.

## Riscos e mitigação

Ver `shape.md`. Rollback: commit de reversão.
