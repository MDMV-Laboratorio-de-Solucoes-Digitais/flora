# Release: 114 — blindagem WCAG 2.2 AA no e2e-browser e rota /files

**Ciclo**: 114 · **Data**: 2026-11-22 · **Baseline**: `github/dev@b32f7cafe`

## Entregue

- `FR-001` / `T-001` (`#373`): módulo puro `apps/frontend/src/lib/utils/a11yAudit.ts` com suíte TDD `apps/frontend/tests/unit/a11yAudit.test.ts` (3/3 passed).
- `FR-002` / `T-002` (`#374`): correções de acessibilidade WCAG 2.2 AA em `/files` (`FileUpload.svelte` `aria-label` + contraste de erro `text-red-700 dark:text-red-400`; `FilePreview.svelte` `aria-hidden="true"` no `<svg>` + contraste/sublinhado `text-blue-700 underline hover:text-blue-800 dark:text-blue-400`; `pt-BR.ts` `'file.selectInput'`).
- `FR-003` / `T-003` (`#375`): `auditAccessibility` em `apps/frontend/tests/e2e/seed-flow.e2e.ts` reforçada para falhar em violações (`expect(findings, summary).toEqual([])`) e estendida com auditoria da rota `/files` (Passo 5).
- `FR-004` / `T-004` (`#376`): notas datadas append-only com recontagem `find` e registro de governança `docs/governance/2026-11-22-114-wcag-ci-guard.md`.
- `FR-005` / `T-005` (`#377`): roteiro de testes, validação R4, evidência pós-CI e ACR independente sem alteração de required checks ou supressão de regras axe.

## SemVer

PATCH `0.7.15` → `0.7.16`: melhoria de acessibilidade WCAG 2.2 AA e blindagem de teste E2E frontend sem alteração de API, contrato, schema ou required checks.

## Evidência

Portas locais verdes (`svelte-check` 0/0, `lint` exit 0, Vitest `server` 15/53, `vite build` exit 0, `check_gate_integrity.py` PASS 10 legs / 77 pisos). CI remoto e ACR registrados após abertura do PR em `evidence.md`.

## Rollback

Novo commit de reversão dos arquivos do ciclo 114; sem force-push.

## [NV]

Modo escuro em navegador real, staging real, Forgejo ao vivo e deploy em produção.
