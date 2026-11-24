# Converge: 119 — UI CRUD de administração de papéis RBAC em `/settings`

**Data**: 2026-11-24 · **Modo**: append-only · **Status: Converged (pré-CI)**

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#405](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/405) (`apps/frontend/src/lib/types/models.ts`, `src/lib/utils/roles.ts`, `src/lib/api/roles.ts`, `src/lib/state/RoleStore.svelte.ts`, `tests/unit/roles.test.ts`)
- `T-002` → [#406](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/406) (`apps/frontend/src/lib/components/rbac/RoleManager.svelte`, `src/routes/(app)/settings/+page.svelte`, `src/routes/layout.css`, `src/lib/i18n/pt-BR.ts`)
- `T-003` → [#407](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/407) (`apps/frontend/tests/e2e/seed-flow.e2e.ts`)
- `T-004` → [#408](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/408) (`docs/governance/2026-11-23-119-briefing-compreensao.md`, governança, docs append-only, R4/R7 `0.11.0`, `test-roadmap.md`/`evidence.md`, ACR)

## Entrega e verificação local (`task-1679`, `task-1692`)

- **TDD RED → GREEN**: execução pré-implementação falhou em import ausente de `src/lib/utils/roles.ts` e `src/lib/state/RoleStore.svelte.ts`; após implementação, todos os portões locais passaram integralmente sobre `github/dev@42c38bf52`:
  - `bun run check` (`svelte-check --tsconfig ./tsconfig.json`): `0 errors and 0 warnings` (`task-1679`).
  - `bun run lint` (`prettier --check . && eslint .`): exit `0` (`task-1679`).
  - `bun run test:unit -- --run --project server`: `Test Files 19 passed (19)`, `Tests 80 passed (80)`, incluindo `tests/unit/roles.test.ts (9 tests)` (`task-1679`).
  - `VITE_API_URL=http://127.0.0.1:3000 bun run build`: `✓ built in 15.92s` (`adapter-node`, exit `0`, `task-1679`).
  - `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test`: `gate-integrity: PASS` (`82` pisos DB intactos), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS` (`task-1679`).
  - **Correção WCAG 2.2 AA no tema claro (`apps/frontend/src/routes/layout.css`)**: `--primary` e `--ring` de `:root` ajustados de `152 76% 39%` para `152 76% 25%` (mantendo `.dark` inalterado), elevando o contraste de botões primários de `2.72:1` para `5.85:1` com `--primary-foreground`.
  - `bun run test:e2e` (Chromium real contra `flora-api` + `vite preview` + Postgres 18.7 `127.0.0.1:15433`, `task-1692`): **`4 passed (59.9s)`** e **`14/14` auditorias WCAG 2.2 AA (`AxeBuilder`): `sem achados`**, incluindo `/settings (roles dark mode)` e `/settings (roles light mode)`, exercitando listagem de papéis nativos somente leitura, criação (`POST 201`), persistência pós-`page.reload()`, edição (`PATCH 200`) e exclusão (`DELETE 204`).
- **Capturas publicadas em Gist público**:
  - Índice: <https://gist.github.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468>
  - Dark mode: <https://gist.githubusercontent.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468/raw/f5256db122df364ece5af955ff73ab5240d0cd4e/18-settings-roles-dark.png>
  - Light mode: <https://gist.githubusercontent.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468/raw/f5256db122df364ece5af955ff73ab5240d0cd4e/19-settings-roles-light.png>
- **Briefing de Compreensão (`auth` / `rbac`)**: `docs/governance/2026-11-23-119-briefing-compreensao.md`.
- **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), PR de promoção `dev -> main` e deploy manual em staging/produção no Coolify (`workflow_dispatch`).
