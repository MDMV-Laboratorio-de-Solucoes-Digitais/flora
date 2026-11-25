# Converge: 122 — Gestão de membros e papéis RBAC em `/settings`

**Data**: 2026-11-24 · **Modo**: append-only · **Status: Converged (pré-CI)** · **Fecha**: `BF-02 -> fechado`

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#420](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/420) (`flora-core/src/traits/repository.rs`, `repositories/membership_repository.rs`, `flora-api/src/routes/rbac.rs`, `flora-tests/src/contract/rbac.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`)
- `T-002` → [#421](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/421) (`apps/frontend/src/lib/types/models.ts`, `src/lib/utils/members.ts`, `src/lib/api/roles.ts`, `src/lib/state/MemberStore.svelte.ts`, `tests/unit/members.test.ts`)
- `T-003` → [#422](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/422) (`apps/frontend/src/lib/i18n/pt-BR.ts`, `src/lib/components/rbac/MemberManager.svelte`, `src/routes/(app)/settings/+page.svelte`, `tests/e2e/seed-flow.e2e.ts`)
- `T-004` → [#423](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/423) (`docs/governance/2026-11-24-122-briefing-compreensao.md`, governança, R1/SDD/R4/R7 `0.14.0`, docs append-only fechando `BF-02 -> fechado`, ACR, promoção)

## Entrega e verificação local (`task-2207`, `task-2234`, `task-2283`, `task-2315`, `task-2376`)

- **Backend TDD RED → GREEN**: `contract::rbac::db::db_rbac_members_*` falhou pré-implementação (`task-2207`: `0 passed; 2 failed`); após implementação (`task-2315`): `fmt`, `clippy --all-targets -- -D warnings`, `cargo deny check` (`bans ok, licenses ok, sources ok`), `cargo test --workspace` (`flora-api 55`, `flora-core 36+21`, `flora-tests 36/87`), `contract::rbac::db::db_` **`5 passed; 0 failed`** (`10.96s`), `cargo build -p flora-api`, `check_gate_integrity.py` (`PASS`, **`87` pisos**), `check_deploy_workflows.py` + `--self-test` (`PASS`).
- **Frontend TDD RED → GREEN & E2E**: `members.test.ts` falhou pré-implementação (`task-2234`); após implementação (`task-2283`, `task-2376`): `svelte-check` `0/0`, `prettier`/`eslint` `exit 0`, Vitest `server` **`21/21` arquivos / `89/89` testes**, `vite build` `✓ built in 1m 20s`, Playwright E2E Chromium real + `flora-api` + PG 18.7 (**`4 passed (37.5s)`**, **`15/15` auditorias WCAG 2.2 AA: `sem achados`**).
- **Capturas publicadas em Gist público (`HTTP 200`)**:
  - Índice: <https://gist.github.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda>
  - Membros em `/settings`: <https://gist.githubusercontent.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda/raw/b1adc000faca61a36bd4433e9033f8549456a229/20-settings-members-light.png>
  - Dark mode: <https://gist.githubusercontent.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda/raw/b1adc000faca61a36bd4433e9033f8549456a229/18-settings-roles-dark.png>
  - Light mode: <https://gist.githubusercontent.com/lfgranja/2458075a6526da72fd23bd4433e9033f8549456a229/19-settings-roles-light.png>
- **Briefing de Compreensão (`auth` / `rbac`)**: `docs/governance/2026-11-24-122-briefing-compreensao.md`.
- **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), promoção `dev -> main` e deploy manual no Coolify (`workflow_dispatch`).
