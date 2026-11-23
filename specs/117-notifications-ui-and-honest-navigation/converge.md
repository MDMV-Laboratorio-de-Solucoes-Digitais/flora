# Converge: 117 — central de notificações escopada por organização e navegação honesta

**Data**: 2026-11-23 · **Modo**: append-only · **Status: Converged (pré-CI)**

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#391](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/391) (`NotificationRepository`, `PgNotificationRepository`, `flora-api/src/routes/notifications.rs`, `flora-tests/src/contract/notifications.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`)
- `T-002` → [#392](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/392) (`models.ts`, `mappers.ts`, `pt-BR.ts`, `utils/notifications.ts`, `NotificationStore.svelte.ts`, `tests/unit/notifications.test.ts`, `tests/unit/mappers.test.ts`)
- `T-003` → [#393](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/393) (`(app)/notifications/+page.svelte`, `(app)/+layout.svelte`, `(app)/dms/+page.svelte`)
- `T-004` → [#394](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/394) (`apps/frontend/tests/e2e/seed-flow.e2e.ts`)
- `T-005` → [#395](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/395) (Briefing de Compreensão `tenant`, governança, docs append-only, R4/R7 `0.10.0`, `test-roadmap.md`/`evidence.md`, ACR)

## Entrega e verificação local

- **TDD frontend RED → GREEN**: `apps/frontend/tests/unit/notifications.test.ts` falhou em `RED` (`task-199`: `Cannot find module '../../src/lib/state/NotificationStore.svelte'`, exit `1`) antes da criação do módulo; após implementação (`task-222`), `mappers.test.ts` `4/4` e `notifications.test.ts` `4/4` (`8 passed`).
- **Rust e gate (`task-331`)**:
  - `cargo fmt --all -- --check`: exit `0`.
  - `cargo clippy --workspace --all-targets -- -D warnings`: `0` erros.
  - `cargo test --workspace`: `flora-api` `46 passed` (incl. `notifications::tests` `2/2`), `flora-core` `35 + 21 passed`, `flora-tests` `36 passed; 0 failed; 80 ignored`.
  - `python3 scripts/ci/check_gate_integrity.py`: `gate-integrity: PASS`, `all 10 db contract floors intact (80 total)` (`notifications: 6`).
- **Frontend (`task-295`)**:
  - `bun run check` (`svelte-check`): `svelte-check found 0 errors and 0 warnings`.
  - `bun run lint` (`prettier --check . && eslint .`): `All matched files use Prettier code style!` / `0` erros ESLint, exit `0`.
  - `bun run test:unit -- --run --project server`: `Test Files  18 passed (18)`, `Tests  69 passed (69)`.
  - `bun run build`: `✓ built in 3m 10s` (`@sveltejs/adapter-node` `✔ done`).
- **Briefing de Compreensão (`tenant`)**: preenchido em `docs/governance/2026-11-23-117-briefing-compreensao.md`.
- **[NV]**: execução remota de `db-tests` (Postgres 18), `e2e-browser` (Passos 6 e 7 + WCAG 2.2 AA), ACR independente, tema escuro real e staging/produção.

## Pós-CI e ACR independente (append-only, 2026-11-23)

- **Hardening pós-ACR (`189edd4fe`, `task-496`)**: após ressalvas `MEDIUM-1` / `MEDIUM-2` / `LOW-3` do revisor `052fd698`, centralizou-se deduplicação em voo e `syncContext`/`loadSequence` em `NotificationStore.svelte.ts`, extraindo `buildNotificationContextKey` e `apps/frontend/src/lib/api/notifications.ts`. Revalidação local (`task-496`): `svelte-check` 0/0, `prettier`/`eslint` exit 0, Vitest `18 files / 71 tests passed`, `vite build` `25.33s`.
- **ACR final (`dfa9e102`, modelo `pro`)**: **`APROVADO`** sobre `189edd4fe` ([PR #396 comment #2](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396#issuecomment-7101299896)).
- **CI remoto (`189edd4fe`)**: [`Rust CI 45174768424`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768424) (`rust-build`, `dependency-review`, `supply-chain-vet`, `db-tests` `80/80` incl. `notifications: passed=6 floor=6`, `e2e-browser` `4 passed (42.1s)` e WCAG 2.2 AA `sem achados` em `/tasks`, `/channels`, `/files`, `/notifications`, `/dms`) + [`CI 45174768337`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768337) (`frontend-build`) `success`.
- **Evidência formal**: `.specify/validations/117-notifications-ui-and-honest-navigation/evidence.md`.
- **Restante `[NV]`**: tema escuro real no navegador e staging/produção externos.

## Pós-evidência visual e correção WCAG 2.5.3 (append-only, 2026-11-23)

- **Execução local completa (`task-618`, `task-694`, PostgreSQL 18.7 em `127.0.0.1:15433`)**: `contract::notifications::db_notifications::` obteve `6 passed; 0 failed` (`3.05s`). Na execução Playwright com inserção de 2 notificações não lidas, o axe-core identificou `WCAG 2.2 AA: label-content-name-mismatch [serious] ["a[href$=\"notifications\"]"]` (`+layout.svelte`).
- **Correção (`fix/117-notifications-badge-wcag`)**: ajustado `+layout.svelte` (`notificationsCollapsedAriaLabel` apenas quando `!isSidebarOpen`; texto natural + `<Badge>` + `<span class="sr-only">` quando expandida) e consolidado o Passo 6b em `seed-flow.e2e.ts` (não lidas + badge + `WCAG 2.2 AA` + `POST /read-all` `204`).
- **Verificação pós-correção (`task-694`)**: `check`/`lint`/`test:unit`/`build` PASS + Playwright `4 passed (35.1s)` com `WCAG 2.2 AA: sem achados` nas 6 vistas e 8 PNGs publicados em [Gist `962e84502fb4cf5689266e10f2224967`](https://gist.github.com/lfgranja/962e84502fb4cf5689266e10f2224967) (`369fb4fc`).

## Pós-E2E granular, `.dark` real e sidebar recolhida (append-only, 2026-11-23)

- **Escopo (`fix/117-granular-e2e-dark-and-a11y`)**: automação completa em `apps/frontend/tests/e2e/seed-flow.e2e.ts` para upload multipart + reload em `/files` (Passo 5b), marcar notificação única + concordância `"1 não lida"` + filtro `"Não lidas"` + `"Abrir destino"` + sidebar recolhida (Passos 6b–6f) e tema escuro real `.dark` em `/dms`, `/notifications` e `/tasks` (Passo 8).
- **Correções descobertas na auditoria em Chromium**:
  - `apps/frontend/src/routes/(app)/+layout.svelte`: corrigido `button-name [critical]` em `#bits-s1` quando `isSidebarOpen === false` (`nav.myAccount`), adicionados `aria-label` recolhidos em `/channels`, `/tasks`, `/files`, e concordância singular/plural (`unreadSrLabel` com `pt-BR.ts`).
  - `apps/frontend/src/routes/layout.css`: adicionada `@custom-variant dark (&:where(.dark, .dark *));` para Tailwind CSS v4.
  - `apps/frontend/playwright.config.ts`: `FLORA_STORAGE__LOCAL_PATH` configurável com fallback em `os.tmpdir()/flora-e2e-storage`.
- **Evidência (`task-847`)**: `bun run check` (`0/0`), `bun run lint` (`exit 0`), `bun run test:unit -- --run --project server` (`18 files / 71 tests passed`), `npm run build` + `bun run test:e2e` **`4 passed (1.3m)`** com **12/12 `WCAG 2.2 AA: sem achados`** e **17 PNGs** publicados em [Gist `962e8450` @ `8952ffbd`](https://gist.github.com/lfgranja/962e84502fb4cf5689266e10f2224967/8952ffbd89eebc783b8d20de4e9311dbe048a306); testes OIDC unit/contrato (`flora-api oidc::` `5/5`, `flora-tests test_auth_` `7/7`, `test_registration_flow` `2/2` stub). Restam `[NV]` apenas deploy Coolify externo e IdP OIDC HTTP real.

## Correções pós-CI `e2e-browser` e ACR #1 (`82bbec11`, append-only, 2026-11-23)

- Corrigido race visual no toggle de `.dark` em CI (`+layout.svelte`: `transition-all` → `transition-[width]` no `<aside>`), contraste `.dark --destructive: 0 90.6% 70.8%` + `@theme` (`layout.css`), `dark:text-green-400` (`tasks/+page.svelte`), persistência síncrona via `localStorage` (`flora.theme` em `app.html`), chave órfã `pt-BR.ts`, fallback `FLORA_STORAGE__LOCAL_PATH?.trim()` (`playwright.config.ts`) e asserções `seed-flow.e2e.ts` (`settleAnimations`, item não lido em `.dark`, persistência pós-`page.goto`).
- Reexecução local idêntica à CI (`task-948`): `bun run test:e2e` **`4 passed (31.3s)`**, **12/12 `WCAG 2.2 AA: sem achados`**. Permanecem `[NV]` seletor interativo de tema em `/settings`, deploy Coolify externo e IdP OIDC HTTP externo.
