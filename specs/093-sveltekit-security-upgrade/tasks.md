# Tasks: 093 — Upgrade de segurança SvelteKit 3

**Input**: `specs/093-sveltekit-security-upgrade/spec.md`, `plan.md`,
`.specify/assessments/093-sveltekit-security-upgrade/decision.md`

**Date**: 2026-11-20

**Format**: `[ID] Description`

## Phase 1 — Evidência e baseline

- [X] T001 Registrar o baseline `b426e4f81`, o inventário `GHSA-3r8g` (high) + 3
  moderate em `@sveltejs/kit 2.70.3` que bloqueava o `dependency-review` da PR
  #318 (`dev` → `main`) e as versões alvo (FR-001, SC-001).
- [X] T002 Confirmar que `vite ^8.0.16`, `typescript ^6.0.3` e
  `@sveltejs/vite-plugin-svelte ^7.1.2` atendem aos peers de `kit 3.1.1` e
  `adapter-node 6.0.5`, sem bump adicional (FR-001, FR-002, SC-001).

## Phase 2 — Dependências e locks

- [X] T003 Atualizar `apps/frontend/package.json`: `@sveltejs/kit ^3.1.1`,
  `@sveltejs/adapter-node ^6.0.5` e `svelte ^5.57.1` (FR-001, SC-001).
- [X] T004 Regenerar `apps/frontend/bun.lock` (canónico da CI) e
  `apps/frontend/package-lock.json` com o mesmo conjunto de versões, sem trocar
  de gerenciador (FR-001, FR-003, SC-002).
- [X] T005 Verificar zero ocorrências de `2.70.3`, `5.5.7` e `5.56.10` nos dois
  locks e sincronia com o manifesto (FR-001, FR-003, SC-001, SC-002).

## Phase 3 — Fronteira e rastreabilidade

- [X] T006 Registrar nos artefatos do ciclo as quebras do SvelteKit 3 e estender
  o rastro de T001..T006 para T001..T009, preservando a fronteira: nenhum toque
  em `validate/`, `release/`, `STATE`, workflows, API, runtime, schema ou
  migration (FR-005, SC-003).

## Phase 4 — Migração SvelteKit 3

- [X] T007 Remover `apps/frontend/svelte.config.js` e mover `vitePreprocess()` e
  `adapter()` para dentro de `sveltekit({ ... })` em
  `apps/frontend/vite.config.ts` — configuração no plugin Vite, exigida pelo
  SvelteKit 3 (FR-002, SC-001).
- [X] T008 Preservar `$lib`: `resolve.alias` em `vite.config.ts` (via
  `fileURLToPath(new URL('src/lib', import.meta.url))`) e `paths` em
  `tsconfig.json`, que passa a estender `$app/tsconfig` com `include`/`exclude`
  explícitos (FR-003, SC-002).

## Phase 5 — Verificação e fecho

- [X] T009 Executar e registrar as verificações locais: `npm run check` (0 erros,
  0 warnings), `npm run lint` (OK), testes unitários do servidor (42/42) e
  `npm run build` (OK); reconfirmar a fronteira do diff; manter **[NV]** a CI
  remota e o `dependency-review`; fechar a rastreabilidade em
  decision/spec/plan/tasks (FR-004, FR-005, SC-002, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 (sequencial;
T007/T008 dependem do upgrade do kit; T009 consolida verificações, fronteira e
rastreabilidade).
