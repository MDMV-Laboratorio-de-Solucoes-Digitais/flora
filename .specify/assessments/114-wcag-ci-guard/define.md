# Assess — define: 114

**Data**: 2026-11-22

## Requisitos funcionais

- **FR-001**: Extrair módulo TypeScript puro `apps/frontend/src/lib/utils/a11yAudit.ts` exportando `WCAG_22_AA_TAGS`, `formatWcagViolations` e `summarizeWcagAudit`, com cobertura de testes unitários TDD em `apps/frontend/tests/unit/a11yAudit.test.ts` (projeto Vitest `server`).
- **FR-002**: Corrigir acessibilidade WCAG 2.2 AA nos componentes de `/files` e dicionário pt-BR: adicionar `'file.selectInput': 'Selecionar arquivo para envio'` em `pt-BR.ts`; aplicar `aria-label={t['file.selectInput']}` e `text-red-700 dark:text-red-400` em `FileUpload.svelte`; aplicar `aria-hidden="true"` no `<svg>` decorativo e `text-blue-700 underline hover:text-blue-800 dark:text-blue-400` em `FilePreview.svelte`.
- **FR-003**: Atualizar `auditAccessibility` em `apps/frontend/tests/e2e/seed-flow.e2e.ts` para usar `WCAG_22_AA_TAGS` e `summarizeWcagAudit`, manter log/anotação e falhar com `expect(findings, summary).toEqual([])`, além de adicionar passo 5 auditando `/files` (reutilizando a sessão já rotacionada).
- **FR-004**: Atualizar documentação de estado (`STATE.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `OBJETIVO.md`, `ROADMAP.md`, `docs/governance/2026-11-22-114-wcag-ci-guard.md`) em modo append-only com recontagem determinística via `find`.
- **FR-005**: Manter 0 alterações em required checks, gate floors, backend/migrations ou supressões de regras axe; fechar R4 (`validate.md`), R7 (`release.md`), `test-roadmap.md`/`evidence.md` e Code Review agêntico independente.

## Requisitos não funcionais

- ESLint (`no-console: error`, `@typescript-eslint/no-explicit-any: error`, regras Unicorn/Svelte 5 Runes) e `svelte-check` com 0 erros e 0 avisos.
- Commits PT-BR granulares com paths explícitos; PR em inglês no padrão didático.
