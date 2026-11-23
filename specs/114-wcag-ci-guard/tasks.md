# Tasks: 114 — blindagem WCAG 2.2 AA no e2e-browser e rota /files

**Data**: 2026-11-22 · **Spec**: `specs/114-wcag-ci-guard/spec.md`

- T-001: Implement pure WCAG 2.2 AA audit utility (`apps/frontend/src/lib/utils/a11yAudit.ts`) and TDD Vitest unit tests (`apps/frontend/tests/unit/a11yAudit.test.ts`) [FR-001 / SC-001]
- T-002: Fix WCAG 2.2 AA accessibility defects in `/files` components and pt-BR dictionary (`FileUpload.svelte`, `FilePreview.svelte`, `pt-BR.ts`) [FR-002 / SC-002]
- T-003: Enforce fail-on-violation WCAG assertion and add `/files` route audit in `apps/frontend/tests/e2e/seed-flow.e2e.ts` [FR-003 / SC-003]
- T-004: Append verified-state documentation recounts and governance record (`STATE.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `OBJETIVO.md`, `ROADMAP.md`, `docs/governance/2026-11-22-114-wcag-ci-guard.md`) [FR-004 / SC-004]
- T-005: Author test roadmap, collect live CI evidence, complete independent ACR, and close R4 validate / R7 release [FR-005 / SC-005]
