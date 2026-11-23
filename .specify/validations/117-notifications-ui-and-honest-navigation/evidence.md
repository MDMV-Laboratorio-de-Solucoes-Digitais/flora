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
