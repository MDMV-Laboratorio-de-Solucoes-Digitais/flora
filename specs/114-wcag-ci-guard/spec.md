# Feature Specification: Blindagem WCAG 2.2 AA no e2e-browser e rota /files

**Feature ID**: 114 · **Status**: specified · **Data**: 2026-11-22

## User Scenario

Como usuário que depende de leitores de tela, navegação por teclado e contraste adequado — e como mantenedor que confia na esteira automatizada para impedir regressões — quero que o teste `e2e-browser` falhe diante de qualquer violação WCAG 2.2 AA e que a rota `/files` atenda aos mesmos critérios já verificados em `/tasks` e `/channels`.

## Acceptance Scenarios

1. **SC-001 (Utilitário puro e TDD)**: **Given** resultados vazios ou com múltiplas violações do axe-core, **when** `summarizeWcagAudit` e `formatWcagViolations` são executados nos testes unitários `a11yAudit.test.ts`, **then** retornam `passed: true` com `"sem achados"` para lista vazia e `passed: false` com cada nó formatado (`id [impact] ["selector"]`, fallback `'indefinido'`) para violações.
2. **SC-002 (Correções em `/files`)**: **Given** os componentes `FileUpload.svelte` e `FilePreview.svelte` e o dicionário `pt-BR.ts`, **when** renderizados em `/files`, **then** o `<input type="file">` possui `aria-label` pt-BR (`'Selecionar arquivo para envio'`), o ícone decorativo `<svg>` possui `aria-hidden="true"`, o erro usa `text-red-700 dark:text-red-400` (≥6,1:1) e o link de download usa `text-blue-700 underline hover:text-blue-800 dark:text-blue-400` (≥6,4:1 + distinção não apenas por cor).
3. **SC-003 (Asserção e cobertura de rota no E2E)**: **Given** o fluxo smoke em `apps/frontend/tests/e2e/seed-flow.e2e.ts`, **when** `e2e-browser` executa em `/tasks`, `/channels` e `/files` reutilizando a sessão rotacionada após `select-org`, **then** `auditAccessibility` falha caso `findings.length > 0` (`expect(findings, summary).toEqual([])`) e emite `"WCAG 2.2 AA em /tasks: sem achados"`, `"WCAG 2.2 AA em /channels: sem achados"` e `"WCAG 2.2 AA em /files: sem achados"`.
4. **SC-004 (Documentação append-only)**: **Given** o ciclo 114 materializado no repo, **when** as contagens via `find` são registradas, **then** `STATE.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `OBJETIVO.md`, `ROADMAP.md` e `docs/governance/2026-11-22-114-wcag-ci-guard.md` preservam todo o histórico anterior.
5. **SC-005 (Governança e portas intactas)**: **Given** o PR do ciclo 114, **when** `check_gate_integrity.py` e os checks do GitHub Actions executam, **then** 0 required checks, 0 pisos DB e 0 regras axe foram relaxados, R4/R7 e Code Review agêntico estão concluídos.

## Requirements

- **FR-001**: Criar `apps/frontend/src/lib/utils/a11yAudit.ts` exportando `WCAG_22_AA_TAGS`, `formatWcagViolations` e `summarizeWcagAudit`, coberto por `apps/frontend/tests/unit/a11yAudit.test.ts`.
- **FR-002**: Adicionar chave `'file.selectInput': 'Selecionar arquivo para envio'` em `apps/frontend/src/lib/i18n/pt-BR.ts` e corrigir `FileUpload.svelte` (`aria-label`, `text-red-700 dark:text-red-400`) e `FilePreview.svelte` (`aria-hidden="true"`, `text-blue-700 underline hover:text-blue-800 dark:text-blue-400`).
- **FR-003**: Refatorar `auditAccessibility` em `apps/frontend/tests/e2e/seed-flow.e2e.ts` para usar `a11yAudit.ts`, afirmar `expect(findings, summary).toEqual([])` e adicionar o Passo 5 auditando `/files`.
- **FR-004**: Atualizar `STATE.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `OBJETIVO.md`, `ROADMAP.md` e criar `docs/governance/2026-11-22-114-wcag-ci-guard.md` com contagens reais via `find`.
- **FR-005**: Executar `test-roadmap.md` → CI → ACR independente → `evidence.md` → R4 `validate.md` → R7 `release.md`, sem alterar required checks, migrations ou regras axe.

## [NV]

Modo escuro em navegador real, staging real com credenciais externas e deploy em produção permanecem `[NV]`.
