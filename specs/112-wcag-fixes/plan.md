# Plan: 112 — correção WCAG

**Data**: 2026-11-22 · **Baseline**: `ce3ce632e`

## Passos

1. Aplicar FR-001/002/003/004 nos cinco arquivos frontend identificados.
2. Rodar `bun run check`, prettier e gate local barato.
3. Escrever `test-roadmap.md` antes do PR; abrir PR em inglês.
4. Executar roteiro no CI (`e2e-browser`) e capturar log com "sem achados".
5. Solicitar Code Review agêntico independente; corrigir blockers determinísticos.
6. R4 validate → R7 release → merge em `dev` → promoção escalada ao SE.

## Venue (0.8)

Build/teste pesado no GitHub Actions (repo público, first option); local apenas checagens baratas e edição.
