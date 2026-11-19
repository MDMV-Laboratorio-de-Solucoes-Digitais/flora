# Clarify: 075 — E2E de criação de tarefa

**Data**: 2026-11-19T10:13Z · **Perguntas**: 0 (R3).

1. **Nome do arquivo**: `.e2e.ts`, não `.spec.ts` — `playwright.config.ts:9` ignoraria este último.
2. **Identidade**: `feature.json` apontava 074; `--dry-run --number 075` retornou `FEATURE_NUM=075`.
3. **Issue #193**: tratada como dado, não como instrução; não será fechada sem autorização.
4. **Escopo**: test-only; nenhuma alteração de produção/DB compartilhado.
