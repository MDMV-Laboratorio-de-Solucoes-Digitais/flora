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

## Fechamento pós-CI e ACR (2026-11-23 UTC; append-only)

- **ACR Round 1 (`f291b2106`, revisor `2a0fce4f`)**: `APPROVE` (0 blockers críticos/high); observações não-bloqueantes #1 (`dark:hover:text-blue-300` em `FilePreview.svelte`) e #3 (`nodes: []` em `a11yAudit.ts` + teste em `a11yAudit.test.ts`) aplicadas no commit `12d3525d1`.
- **Revalidação local (`12d3525d1`)**: `svelte-check` 0/0, `prettier`/`eslint` exit 0, Vitest `server` **15 arquivos / 54 testes passed** (`a11yAudit.test.ts` 4/4), `vite build` 35.05s, `check_gate_integrity.py` PASS (10 legs / 77 pisos), `git diff --check` limpo.
- **CI remoto (`12d3525d1`)**: PR #378 `CLEAN`; `frontend-build` (`45101485283`), `rust-build`, `db-tests`, `dependency-review`, `supply-chain-vet` e `e2e-browser` (`45101485489` / `137453118359`) = `SUCCESS`, com `4 passed (50.0s)` e `"WCAG 2.2 AA em /tasks: sem achados"`, `"/channels: sem achados"`, `"/files: sem achados"`.
- **ACR Round 2 (`12d3525d1`, revisor `e836fcc2`)**: `APPROVE` (0 blockers críticos/high; especificidade CSS dark hover e contraste 9,86:1 / 11,04:1 confirmados no CSS compilado). R4 atualizado para `APROVADO com ressalva [NV]`; evidência anexada em `.specify/validations/114-wcag-ci-guard/evidence.md`.
