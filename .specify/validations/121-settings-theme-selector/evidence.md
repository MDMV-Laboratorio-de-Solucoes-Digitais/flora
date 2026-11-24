# Evidência de Validação — ciclo 121 (`121-settings-theme-selector`)

- **Data**: 2026-11-24 UTC
- **Baseline**: `github/dev@2ff91e229`
- **Issues R6**: [#415](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/415)–[#418](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/418) (`feature/121-settings-theme-selector` → `dev`)
- **Veredito R4**: **APROVADO com ressalva `[NV]`** *(Passos 1–7 verificados localmente em Vitest + build + Chromium real/PG 18.7 nas tasks `task-2015`, `task-2049` e `task-2063`; Passo 8 CI remoto/ACR pendente após abertura do PR; Passo 9 staging/produção `[NV]`)*

## Tabela do Roteiro de Testes (`test-roadmap.md`, 9 passos)

| # | Passo | Resultado observado | Evidência / Referência |
|---|---|---|---|
| 1 | TDD RED pré-implementação (`task-2015`) | `FAIL` esperado (`Error: Cannot find module '$lib/state/ThemeStore.svelte'`, exit `1`) | Execução local `task-2015` |
| 2 | `bun run check && bun run lint` em `apps/frontend` (`task-2049`) | `PASS` (`svelte-check found 0 errors and 0 warnings`; `All matched files use Prettier code style!`; `eslint .` exit `0`) | Execução local `task-2049` |
| 3 | `bun run test:unit -- --run --project server` (`task-2049`) | `PASS` (`Test Files 20 passed (20)`, `Tests 84 passed (84)`, incl. `tests/unit/theme.test.ts (4 tests)`) | Execução local `task-2049` |
| 4 | `VITE_API_URL=http://127.0.0.1:3000 bun run build` (`task-2049`) | `PASS` (`✓ built in 1m 2s`, adapter-node `✔ done`, exit `0`) | Execução local `task-2049` |
| 5 | `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` (`task-2049`) | `PASS` (`gate-integrity: PASS`, `85` pisos intactos; `deploy-workflows: PASS`; `deploy-workflow-self-test: PASS`) | Execução local `task-2049` |
| 6 | `bun run test:e2e` — Passo 9 `/settings` `.dark` inicial (`task-2063`) | `PASS` (`WCAG 2.2 AA em /settings (roles dark mode): sem achados`; captura `18-settings-roles-dark.png` inspecionada via `read_file`) | Chromium real + PG 18.7 (`task-2063`) + [dark PNG](https://gist.githubusercontent.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf/raw/df2807eb29bc09c73ea86001dd88f38db123078d/18-settings-roles-dark.png) |
| 7 | `bun run test:e2e` — Passo 9 `/settings` clique em `Claro`, `page.reload()`, ida-e-volta `Escuro`/`Claro` e `--destructive: 0 84.2% 40%` (`task-2063`) | `PASS` (**`4 passed (1.4m)`**, **`14/14` auditorias WCAG 2.2 AA: `sem achados`** incl. `/settings (roles light mode)`; captura `19-settings-roles-light.png` inspecionada via `read_file`) | Chromium real + PG 18.7 (`task-2063`) + [light PNG](https://gist.githubusercontent.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf/raw/df2807eb29bc09c73ea86001dd88f38db123078d/19-settings-roles-light.png) |
| 8 | `gh pr checks <PR> --watch` + inspeção ao vivo + ACR independente (`pro`) | Pendente abertura do PR para `dev` (`[NV]`) | Atualizado após criação/validação do PR |
| 9 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Ambiente externo |

## Evidências Visuais (GitHub Gist público — HTTP 200 verificado)

- Índice Gist: <https://gist.github.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf>
- `/settings` em tema escuro (`.dark` com `Aparência -> Escuro` ativo): <https://gist.githubusercontent.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf/raw/df2807eb29bc09c73ea86001dd88f38db123078d/18-settings-roles-dark.png>
- `/settings` em tema claro após clique em `Claro` e `page.reload()`: <https://gist.githubusercontent.com/lfgranja/62253fe7a30a00b665e8277ae7b719cf/raw/df2807eb29bc09c73ea86001dd88f38db123078d/19-settings-roles-light.png>
