# Evidência de Validação — ciclo 122 (`122-org-members-rbac-management`)

- **Data**: 2026-11-24 UTC
- **Baseline**: `github/dev@8f984adb7`
- **Issues R6**: [#420](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/420)–[#423](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/423) (`feature/122-org-members-rbac-management` → `dev`)
- **Veredito R4**: **APROVADO com ressalva `[NV]`** *(Passos 1–5 verificados localmente nas tasks `task-2207`, `task-2234`, `task-2283`, `task-2315` e `task-2376`; Passo 6 CI remoto/ACR pendente após abertura do PR; Passo 7 staging/produção `[NV]`)*

## Tabela do Roteiro de Testes (`test-roadmap.md`, 7 passos)

| # | Passo | Resultado observado | Evidência / Referência |
|---|---|---|---|
| 1 | Backend TDD RED (`task-2207`: `contract::rbac::db::db_rbac_members_`) | `FAIL` esperado (`400 vs 401` em `/members`; `204 vs 403` auto-rebaixamento; `0 passed; 2 failed`) | PG 18.7 `127.0.0.1:15433` (`task-2207`) |
| 2 | Rust `fmt` + `clippy --all-targets -- -D warnings` + `cargo deny` + `cargo test --workspace` (`task-2315`) | `PASS` (`bans ok, licenses ok, sources ok`; `flora-api` `55/55`, `flora-core` `36+21`, `flora-tests` `36` + `87 ignored`) | Execução local `task-2315` |
| 3 | Contratos DB `contract::rbac::db::db_` + `check_gate_integrity.py` + `check_deploy_workflows.py` (`task-2315`) | `PASS` (`contract::rbac::db::db_` **`5 passed; 0 failed`** em `10.96s`; `gate-integrity: PASS` **`87` pisos**; `deploy-workflows: PASS`) | Execução local `task-2315` |
| 4 | Frontend TDD RED (`task-2234`) → `check`, `lint`, Vitest `server`, `vite build` (`task-2283`, `task-2376`) | `RED`: `Cannot find module '$lib/state/MemberStore.svelte'` → `GREEN`: `svelte-check` `0/0`, `prettier`/`eslint` exit `0`, **`21/21` arquivos / `89/89` testes**, `vite build` `✓ built in 1m 20s` | Execução local `task-2234`, `task-2283`, `task-2376` |
| 5 | Playwright E2E Chromium + `flora-api` recompilado + PG 18.7 (`task-2376`) | `PASS` (**`4 passed (37.5s)`**, **`15/15` auditorias WCAG 2.2 AA: `sem achados`** incl. `/settings (members light mode)`; atribuição + `reload()` + restauração de `Bob Builder` e remoção + readição de `Carol Viewer`; capturas inspecionadas via `read_file`) | Chromium real + PG 18.7 (`task-2376`) + [members PNG](https://gist.githubusercontent.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda/raw/b1adc000faca61a36bd4433e9033f8549456a229/20-settings-members-light.png) |
| 6 | `gh pr checks <PR> --watch` + inspeção ao vivo + ACR independente (`pro`) | Pendente abertura do PR para `dev` (`[NV]`) | Atualizado após criação/validação do PR |
| 7 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Ambiente externo |

## Evidências Visuais (GitHub Gist público — HTTP 200 verificado)

- Índice Gist: <https://gist.github.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda>
- `/settings` — Gestão de membros (`Membros e atribuição de papéis`): <https://gist.githubusercontent.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda/raw/b1adc000faca61a36bd4433e9033f8549456a229/20-settings-members-light.png>
- `/settings` — Tema escuro (`.dark`): <https://gist.githubusercontent.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda/raw/b1adc000faca61a36bd4433e9033f8549456a229/18-settings-roles-dark.png>
- `/settings` — Tema claro: <https://gist.githubusercontent.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda/raw/b1adc000faca61a36bd4433e9033f8549456a229/19-settings-roles-light.png>
