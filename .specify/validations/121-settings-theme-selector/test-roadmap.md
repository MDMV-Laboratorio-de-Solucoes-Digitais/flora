# Test roadmap — ciclo 121 (`121-settings-theme-selector`)

**Data**: 2026-11-24 · **Baseline**: `github/dev@2ff91e229` · **Superfície**: UI real Svelte 5 (`/settings` — `ThemeSelector.svelte`, `RoleManager.svelte`), `ThemeStore` + utilitários `utils/theme.ts` (Vitest `server`), tokens `:root --destructive` (`layout.css`) e E2E Playwright Chromium + Axe WCAG 2.2 AA

## Escopo

FR-001/002/003/004 · SC-001..SC-004 · Fechamento de `BF-04` (sem alterações Rust, migrations SQL ou pisos DB `85`).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `bun run test:unit -- --run --project server tests/unit/theme.test.ts` (pré-implementação TDD RED) | Falha esperada por módulo ausente antes da criação de `utils/theme.ts` e `ThemeStore.svelte.ts` | CLI / Vitest | sim |
| 2 | `bun run check && bun run lint` em `apps/frontend` | `svelte-check` 0 erros/0 avisos; `prettier` + `eslint` exit 0 | CLI / Frontend | sim |
| 3 | `bun run test:unit -- --run --project server` em `apps/frontend` | `theme.test.ts` (4/4) e toda a suíte unitária (`20` arquivos / `84` testes) passam | CLI / Vitest | sim |
| 4 | `VITE_API_URL=http://127.0.0.1:3000 bun run build` em `apps/frontend` | Build SvelteKit `adapter-node` exit 0 | CLI / Frontend | sim |
| 5 | `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` | `gate-integrity: PASS` (`85` pisos intactos), `deploy-workflows: PASS` | CLI / Gate | sim |
| 6 | `bun run test:e2e` (Chromium real + PG 18.7) — Passo 9 `/settings` `.dark` inicial | Heading `Aparência`, botão `Escuro` com `aria-pressed="true"`, `Claro` `aria-pressed="false"`, WCAG 2.2 AA `sem achados`; captura `18-settings-roles-dark.png` | Browser / UI | sim |
| 7 | Passo 9 E2E — Clique real em `Claro`, `page.reload()`, contraste `--destructive`, ida-e-volta `Escuro`/`Claro` | Tema claro persiste em `localStorage['flora.theme']` e ausência de `.dark` após `reload()`, WCAG 2.2 AA `sem achados` (`--destructive: 0 84.2% 40%`), captura `19-settings-roles-light.png` | Browser / UI | sim |
| 8 | `gh pr checks <PR> --watch` + inspeção ao vivo de `frontend-build`, `rust-build`, `db-tests` (`85/85`), `e2e-browser` + ACR independente (`pro`) | Todos os required checks GREEN, `e2e-browser` PASS (`14/14` WCAG 2.2 AA `sem achados`) e ACR `APROVADO` | CI / GitHub | sim |
| 9 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Produção | não (`[NV]`) |
