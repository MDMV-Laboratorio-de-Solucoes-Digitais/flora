# Evidência de Validação — ciclo 119 (`119-rbac-roles-admin-ui`)

- **Data**: 2026-11-24 UTC
- **Baseline**: `github/dev@42c38bf52`
- **Issues R6**: [#405](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/405)–[#408](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/408) (`feature/119-rbac-roles-admin-ui` → `dev`)
- **Veredito R4**: **APROVADO com ressalva `[NV]`** *(Passos 1–7 verificados localmente em Chromium real + Postgres 18.7 nas tasks `task-1679` e `task-1692`; Passo 8 CI remoto/ACR aguardando etapa de PR; Passo 9 staging/produção `[NV]`)*

## Tabela do Roteiro de Testes (`test-roadmap.md`, 9 passos)

| # | Passo | Resultado observado | Evidência / Referência |
|---|---|---|---|
| 1 | `bun run check && bun run lint` em `apps/frontend` (`task-1679`) | `PASS` (`svelte-check found 0 errors and 0 warnings`; `All matched files use Prettier code style!`; `eslint .` exit `0`) | Execução local `task-1679` |
| 2 | `bun run test:unit -- --run --project server` em `apps/frontend` (`task-1679`) | `PASS` (`Test Files 19 passed (19)`, `Tests 80 passed (80)`, incl. `tests/unit/roles.test.ts (9 tests)`) | Execução local `task-1679` |
| 3 | `VITE_API_URL=http://127.0.0.1:3000 bun run build` em `apps/frontend` (`task-1679`) | `PASS` (`✓ built in 15.92s`, adapter-node `✔ done`, exit `0`) | Execução local `task-1679` |
| 4 | `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` (`task-1679`) | `PASS` (`gate-integrity: PASS`, 4 required checks, `all 10 db contract floors intact (82 total)`, `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS`) | Execução local `task-1679` |
| 5 | `bun run test:e2e` — Passo 9 `/settings` `.dark` inicial (`task-1692`) | `PASS` (`Admin` com badge `Nativo` visível sem ações destrutivas; `WCAG 2.2 AA em /settings (roles dark mode): sem achados`; captura visual `18-settings-roles-dark.png` inspecionada via `read_file`) | Chromium real + PG 18.7 (`task-1692`) + [dark PNG](https://gist.githubusercontent.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468/raw/f5256db122df364ece5af955ff73ab5240d0cd4e/18-settings-roles-dark.png) |
| 6 | Passo 9 E2E — Criar papel (`POST /api/v1/roles` → `201`) + `page.reload()` (`task-1692`) | `PASS` (papel personalizado criado via UI, persistido em PostgreSQL 18.7 e recarregado após `page.reload()`) | Chromium real + `flora-api` + PG 18.7 (`task-1692`) |
| 7 | Passo 9 E2E — Editar (`PATCH` → `200`), tema claro (`19-settings-roles-light.png`) e excluir (`DELETE` → `204`) (`task-1692`) | `PASS` (`4 passed (59.9s)`, **`14/14` auditorias WCAG 2.2 AA: `sem achados`** incl. `WCAG 2.2 AA em /settings (roles light mode): sem achados` após `:root --primary: 152 76% 25%`; captura `19-settings-roles-light.png` inspecionada via `read_file`) | Chromium real + PG 18.7 (`task-1692`) + [light PNG](https://gist.githubusercontent.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468/raw/f5256db122df364ece5af955ff73ab5240d0cd4e/19-settings-roles-light.png) |
| 8 | `gh pr checks <PR> --watch` + inspeção ao vivo de `frontend-build`, `rust-build`, `db-tests` (`82/82`), `e2e-browser` + ACR independente (`pro`) | Pendente abertura do PR para `dev` (`[NV]`) | Atualizado após criação/validação do PR |
| 9 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Ambiente externo |

## Evidências Visuais (GitHub Gist público — HTTP 200 verificado)

- Índice Gist: <https://gist.github.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468>
- `/settings` RBAC em tema escuro (`.dark`): <https://gist.githubusercontent.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468/raw/f5256db122df364ece5af955ff73ab5240d0cd4e/18-settings-roles-dark.png>
- `/settings` RBAC em tema claro (`:root --primary: 152 76% 25%`, contraste `5.85:1`): <https://gist.githubusercontent.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468/raw/f5256db122df364ece5af955ff73ab5240d0cd4e/19-settings-roles-light.png>
