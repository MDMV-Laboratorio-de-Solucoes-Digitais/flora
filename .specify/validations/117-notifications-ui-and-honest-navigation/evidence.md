# Evidência de Validação — ciclo 117 (`117-notifications-ui-and-honest-navigation`)

- **Data**: 2026-11-23 UTC
- **PR**: [#396](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396) (`feature/117-notifications-ui-and-honest-navigation` → `dev`)
- **HEAD verificado para código & CI**: `189edd4feee9232b8c0e35c7ec9b419871652603`
- **Veredito R4**: **APROVADO com ressalva `[NV]`** *(tema escuro em navegador real e staging/produção externos fora do escopo do smoke em CI)*

## Tabela do Roteiro de Testes (`test-roadmap.md`, 11 passos)

| # | Passo | Resultado observado | Evidência / Referência |
|---|---|---|---|
| 1 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` | `PASS` (exit `0`, `0` warnings) | Local `task-331` + CI [`rust-build`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768424/job/137693063118) |
| 2 | `cargo test --workspace` | `PASS` (`flora-api` `46`, `flora-core` `35+21`, `flora-tests` `36 passed / 80 ignored`) | Local `task-331` + CI `rust-build` |
| 3 | `python3 scripts/ci/check_gate_integrity.py` | `PASS` (`gate-integrity: PASS`, 4 required checks, 10 legs / 80 pisos, `notifications: 6`) | Local `task-331` + CI `rust-build` |
| 4 | `bun run check && bun run lint` em `apps/frontend` | `PASS` (`svelte-check`: `0 errors and 0 warnings`; `prettier` + `eslint` exit `0` após fix ACR `189edd4fe`) | Local `task-496` + CI [`frontend-build`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768337/job/137693058969) |
| 5 | `bun run test:unit -- --run --project server` em `apps/frontend` | `PASS` (`Test Files 18 passed (18)`, `Tests 71 passed (71)` — inclui TDD RED `task-199` → GREEN `task-222` e hardening `task-496`) | Local `task-496` + CI `frontend-build` |
| 6 | `bun run build` em `apps/frontend` | `PASS` (`vite build` + `@sveltejs/adapter-node ✔ done`, `25.33s`) | Local `task-496` + CI `frontend-build` |
| 7 | `gh pr checks 396 --watch` no head `189edd4fe` | `PASS` (`0 cancelled, 0 failing, 11 successful, 1 skipped` — `dependency-review` skip apenas em evento `push`, `success` em `pull_request`) | [`Rust CI`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768424) · [`CI`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768337) |
| 8 | Logs ao vivo de `db-tests` (Postgres 18) | `PASS` — 10/10 pernas / `80/80` contratos (`notifications: passed=6 floor=6`, incl. `db_notifications_org_scoped_target_url_and_read_all ... ok`) | [`db-tests` log](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768424/job/137693062993) |
| 9 | Logs ao vivo de `e2e-browser` (Chromium) | `PASS` — `4 passed (42.1s)`, Passos 6 (`/notifications` `GET 200` + empty state + sem `/dms`/IA na UI) e 7 (`/dms` honesto), `WCAG 2.2 AA`: `sem achados` em `/tasks`, `/channels`, `/files`, `/notifications` e `/dms` | [`e2e-browser` log](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768424/job/137693063173) |
| 10 | Code Review Agêntico Independente (`research`, modelo `pro`) | `APROVADO` (rodada 1 `052fd698` apontou 2 `MEDIUM` de corrida no frontend + `LOW-3`; corrigidos em `189edd4fe` e revalidados na rodada 2 `dfa9e102` com `0` blockers) | [PR #396 comment #1](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396#issuecomment-7101161293) · [PR #396 comment #2](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396#issuecomment-7101299896) |
| 11 | Tema escuro real em navegador e staging/produção externos | `[NV]` | Smoke Chromium executa em tema claro contra `dev-seed.sql`; deploy Coolify fora do gate de merge (`dev`) |

## Evidência Visual Local (Playwright Chromium + PostgreSQL 18.7) e Correção WCAG 2.5.3

- **Execução local do contrato DB (`task-618`, PostgreSQL 18.7 em `127.0.0.1:15433`)**: `cargo test -p flora-tests contract::notifications::db_notifications:: -- --ignored --test-threads=1` → **`test result: ok. 6 passed; 0 failed`** (`finished in 3.05s`), incluindo `db_notifications_org_scoped_target_url_and_read_all ... ok`.
- **Achado e correção em tela real (`task-650` → `task-694`)**: ao popular 2 notificações não lidas na UI e auditar `/notifications (unread)` com `@axe-core/playwright`, o roteiro revelou `WCAG 2.2 AA: label-content-name-mismatch [serious] ["a[href$=\"notifications\"]"]` (`aria-label="Notificações (2 não lida(s))"` conflitava com texto visível `"Notificações 2"`). Corrigido em `apps/frontend/src/routes/(app)/+layout.svelte` (`notificationsCollapsedAriaLabel` apenas quando `!isSidebarOpen`; quando expandida, `<Badge>` + `<span class="sr-only">` compõem o nome acessível naturalmente) e guardado permanentemente pelo Passo 6b em `apps/frontend/tests/e2e/seed-flow.e2e.ts`.
- **Reexecução Playwright local (`task-694`)**: `bun run check` (`0` erros/avisos), `bun run lint` (`exit 0`), `bun run test:unit -- --run --project server` (`exit 0`), `npm run build` (`exit 0`) e `bun run test:e2e` → **`4 passed (35.1s)`**, `WCAG 2.2 AA: sem achados` em `/tasks`, `/channels`, `/files`, `/notifications`, `/notifications (unread)` e `/dms`.
- **Galeria de capturas (8 PNGs em [Gist `962e84502fb4cf5689266e10f2224967`](https://gist.github.com/lfgranja/962e84502fb4cf5689266e10f2224967))**:
  1. [`01-select-org.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/01-select-org.png) — `/select-org` (`Acme Corporation`)
  2. [`02-tasks-board.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/02-tasks-board.png) — `/tasks` (tarefa criada + sidebar honesta sem `/dms` e sem botão IA)
  3. [`03-channels-chat.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/03-channels-chat.png) — `/channels` (`# general`, autores reais `Alice Admin` / `Bob Builder`)
  4. [`04-files-workspace.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/04-files-workspace.png) — `/files` (seletor de workspace + estado vazio)
  5. [`05-notifications-empty.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/05-notifications-empty.png) — `/notifications` (estado vazio escopado à organização)
  6. [`06-notifications-unread-and-badge.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/06-notifications-unread-and-badge.png) — `/notifications` populado (`Menção`, `Atribuição`, badge `2` na sidebar, WCAG `sem achados`)
  7. [`07-notifications-after-read-all.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/07-notifications-after-read-all.png) — `/notifications` após `Marcar todas como lidas` (`POST /read-all` `204`, itens `Lida`, badge zerado)
  8. [`08-dms-honest-unavailable.png`](https://gist.githubusercontent.com/lfgranja/962e84502fb4cf5689266e10f2224967/raw/369fb4fcbf114d87365a81fa5069303990aaf53f/08-dms-honest-unavailable.png) — `/dms` (aviso honesto pt-BR + `Ir para Canais`)
