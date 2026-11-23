# Converge: 114 — blindagem WCAG 2.2 AA no e2e-browser e rota /files

**Data**: 2026-11-22 · **Modo**: append-only · **Status: Converged**

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#373](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/373) (`apps/frontend/src/lib/utils/a11yAudit.ts`, `apps/frontend/tests/unit/a11yAudit.test.ts`)
- `T-002` → [#374](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/374) (`apps/frontend/src/lib/i18n/pt-BR.ts`, `FileUpload.svelte`, `FilePreview.svelte`)
- `T-003` → [#375](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/375) (`apps/frontend/tests/e2e/seed-flow.e2e.ts`)
- `T-004` → [#376](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/376) (`STATE.md`, `PROGRESSO.md`, `research/index.md`, `OBJETIVO.md`, `ROADMAP.md`, `docs/governance/2026-11-22-114-wcag-ci-guard.md`)
- `T-005` → [#377](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/377) (`test-roadmap.md`, `evidence.md`, `validate.md`, `release.md`, PR + ACR)

## Entrega e verificação local

- **FR-001 (`a11yAudit.ts` + TDD)**: teste RED falhou antes da criação do módulo (`Cannot find module '../../src/lib/utils/a11yAudit'`); após criação, 3/3 passaram; correção de 3 ocorrências `unicorn/prefer-string-raw` com `String.raw`.
- **FR-002 (`/files` + pt-BR)**: `'file.selectInput': 'Selecionar arquivo para envio'`; `FileUpload.svelte` com `aria-label` e `text-red-700 dark:text-red-400`; `FilePreview.svelte` com `aria-hidden="true"` no `<svg>` e `text-blue-700 underline hover:text-blue-800 dark:text-blue-400`.
- **FR-003 (`seed-flow.e2e.ts`)**: `auditAccessibility` afirma `expect(findings, summary).toEqual([])` e audita `/files` (Passo 5) após `/tasks` e `/channels`, reutilizando a sessão rotacionada; nenhuma regra axe desabilitada.
- **Portas locais**: `svelte-check` 0 erros/0 avisos; `prettier --check . && eslint .` exit 0; Vitest `server` **15 arquivos / 53 testes passed**; `vite build` exit 0; `python3 scripts/ci/check_gate_integrity.py` **PASS** (4 required checks, 10 legs / 77 pisos); `git diff --check` limpo.
- **[NV]**: execução remota do `e2e-browser` no Chromium, ACR independente, modo escuro, staging real e produção.
