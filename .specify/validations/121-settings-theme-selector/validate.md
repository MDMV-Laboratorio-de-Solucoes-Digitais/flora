# Validação R4 — ciclo 121 (`121-settings-theme-selector`)

**Data**: 2026-11-24 UTC · **Baseline**: `github/dev@2ff91e229` · **Veredito: APROVADO com ressalva `[NV]`** *(Passos 1–7 verificados localmente nas tasks `task-2015`, `task-2049` e `task-2063`; Passo 8 CI remoto/ACR após abertura do PR para `dev`; Passo 9 staging/produção `[NV]`)*

| Comando | Resultado observado |
|---|---|
| `bun run test:unit -- --run --project server tests/unit/theme.test.ts` pré-implementação (`task-2015`) | FAIL esperado (`Cannot find module '$lib/state/ThemeStore.svelte'`, exit `1`) |
| `bun run check && bun run lint` em `apps/frontend` (`task-2049`) | `svelte-check found 0 errors and 0 warnings`; `All matched files use Prettier code style!` + `eslint .` exit `0` |
| `bun run test:unit -- --run --project server` (`task-2049`) | `Test Files 20 passed (20)`, `Tests 84 passed (84)` (incl. `4/4` em `tests/unit/theme.test.ts`) |
| `VITE_API_URL=http://127.0.0.1:3000 bun run build` (`task-2049`) | `✓ built in 1m 2s`, adapter-node `✔ done` (exit `0`) |
| `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` (`task-2049`) | `gate-integrity: PASS` (`85` pisos intactos), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS` |
| `bun run test:e2e` (Chromium real + `flora-api` + `vite preview` + Postgres 18.7, `task-2063`) | **`4 passed (1.4m)`**; **`14/14` auditorias WCAG 2.2 AA: `sem achados`** (incl. clique real em `Claro`/`Escuro`, persistência pós-`page.reload()` em `localStorage['flora.theme']` e `/settings` claro com `--destructive: 0 84.2% 40%`) |

Roteiro de testes: `.specify/validations/121-settings-theme-selector/test-roadmap.md` · Evidência detalhada e capturas Gist: `.specify/validations/121-settings-theme-selector/evidence.md`.
Governança: `docs/governance/2026-11-24-121-settings-theme-selector.md`.
Issues R6 vinculadas: `#415`–`#418`.

**Conclusão**: todas as verificações frontend (`svelte-check`, `prettier`, `eslint`, `84` testes unitários, `vite build`), integridade do gate CI (`85` pisos DB + validadores de deploy) e fluxo E2E real no Chromium com Axe WCAG 2.2 AA (`14/14` sem achados, alternância interativa de tema + `page.reload()`) estão verdes; permanecem **`[NV]`** até a etapa de PR o CI remoto/ACR (Passo 8) e, fora do gate de PR, o deploy manual em staging/produção via Coolify (Passo 9).
