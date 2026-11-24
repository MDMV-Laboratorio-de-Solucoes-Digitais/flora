# Converge: 121 — Seletor interativo de tema e contraste WCAG em `/settings`

**Data**: 2026-11-24 · **Modo**: append-only · **Status: Converged (pré-CI)** · **Fecha**: `BF-04`

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#415](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/415) (`apps/frontend/src/lib/utils/theme.ts`, `src/lib/state/ThemeStore.svelte.ts`, `tests/unit/theme.test.ts`)
- `T-002` → [#416](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/416) (`apps/frontend/src/lib/i18n/pt-BR.ts`, `src/lib/components/settings/ThemeSelector.svelte`, `src/routes/(app)/settings/+page.svelte`, `src/routes/layout.css`, `src/lib/components/rbac/RoleManager.svelte`)
- `T-003` → [#417](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/417) (`apps/frontend/tests/e2e/seed-flow.e2e.ts`)
- `T-004` → [#418](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/418) (`docs/governance/2026-11-24-121-settings-theme-selector.md`, R1/R4/R7 `0.13.0`, docs append-only fechando `BF-04`, ACR, promoção)

## Entrega e verificação local (`task-2015`, `task-2049`, `task-2063`)

- **TDD RED → GREEN**: execução pré-implementação (`task-2015`) falhou por import ausente de `$lib/state/ThemeStore.svelte`; após implementação, toda a suíte local passou sobre `github/dev@2ff91e229`:
  - `bun run check` (`svelte-check --tsconfig ./tsconfig.json`): `0 errors and 0 warnings` (`task-2049`).
  - `bun run lint` (`prettier --check . && eslint .`): exit `0` (`task-2049`).
  - `bun run test:unit -- --run --project server`: `Test Files 20 passed (20)`, `Tests 84 passed (84)`, incluindo `tests/unit/theme.test.ts (4 tests)` (`task-2049`).
  - `VITE_API_URL=http://127.0.0.1:3000 bun run build`: `✓ built in 1m 2s` (`adapter-node`, exit `0`, `task-2049`).
  - `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test`: `gate-integrity: PASS` (`85` pisos DB intactos), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS` (`task-2049`).
  - `bun run test:e2e` (Chromium real contra `flora-api` + `vite preview` + PG 18.7, `task-2063`): **`4 passed (1.4m)`** e **`14/14` auditorias WCAG 2.2 AA: `sem achados`**, exercitando clique real nos botões `Claro` e `Escuro` em `/settings`, persistência em `localStorage['flora.theme']` após `page.reload()` e contraste `:root --destructive: 0 84.2% 40%`.
- **Capturas publicadas em Gist público (`HTTP 200`)**:
  - Índice: <https://gist.github.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf>
  - Dark mode: <https://gist.githubusercontent.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf/raw/df2807eb29bc09c73ea86001dd88f38db123078d/18-settings-roles-dark.png>
  - Light mode: <https://gist.githubusercontent.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf/raw/df2807eb29bc09c73ea86001dd88f38db123078d/19-settings-roles-light.png>
- **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), promoção `dev -> main` e deploy manual no Coolify (`workflow_dispatch`).
