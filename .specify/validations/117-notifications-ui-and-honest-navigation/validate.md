# Validação R4 — ciclo 117

**Data**: 2026-11-23 UTC · **Veredito: APROVADO com ressalva `[NV]`** *(Passos 1–10 verificados em execução local e no CI do [PR #396](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396) em `189edd4fe`; Passo 11 permanece `[NV]`)*

| Comando | Resultado observado |
|---|---|
| `bun run test:unit -- --run --project server tests/unit/notifications.test.ts` (`task-199` → `task-222`, RED → GREEN) | exit `1` pré-implementação (`Cannot find module '../../src/lib/state/NotificationStore.svelte'`) → `8/8 passed` (`mappers.test.ts` `4`, `notifications.test.ts` `4`) |
| `cargo fmt --all -- --check` (`task-331`) | exit `0` |
| `cargo clippy --workspace --all-targets -- -D warnings` (`task-331`) | `0` erros em todo o workspace (`Finished dev profile`) |
| `cargo test --workspace` (`task-331`) | `flora-api` `46 passed`, `flora-core` `35+21 passed`, `flora-tests` `36 passed; 0 failed; 80 ignored`, exit `0` |
| `python3 scripts/ci/check_gate_integrity.py` (`task-331`) | `gate-integrity: PASS`, `all 10 db contract floors intact (80 total)` (`notifications: 6`), exit `0` |
| `bun run check` (`svelte-check`, `task-496`) | `svelte-check found 0 errors and 0 warnings`, exit `0` |
| `bun run lint` (`prettier --check . && eslint .`, `task-496`) | `All matched files use Prettier code style!` / `0` erros ESLint, exit `0` |
| `bun run test:unit -- --run --project server` (`task-496`) | `Test Files  18 passed (18)`, `Tests  71 passed (71)`, exit `0` |
| `bun run build` (`vite build`, `task-496`) | `✓ built in 25.33s` (`@sveltejs/adapter-node` `✔ done`), exit `0` |
| `db-tests` CI ([`45174768424/137693062993`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768424/job/137693062993)) | `notifications: passed=6 floor=6` (`db_notifications_org_scoped_target_url_and_read_all ... ok`), `80/80` contratos Postgres 18 `success` |
| `e2e-browser` + `frontend-build` CI ([`137693063173`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768424/job/137693063173) / [`137693058969`](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45174768337/job/137693058969)) | `4 passed (42.1s)`, WCAG 2.2 AA `sem achados` em `/tasks`, `/channels`, `/files`, `/notifications`, `/dms`; ACR `dfa9e102` `APROVADO` |
| `db_notifications::` + Playwright local com capturas (`task-618`, `task-694`, PG 18 em `127.0.0.1:15433`) | `6/6 passed (3.05s)` em contrato DB; fix WCAG 2.5.3 (`label-content-name-mismatch`) no badge de não lidas em `+layout.svelte` + guarda Passo 6b em `seed-flow.e2e.ts`; `4 passed (35.1s)`, `6/6` auditorias WCAG `sem achados`, 8 PNGs anexados em [Gist `962e8450`](https://gist.github.com/lfgranja/962e84502fb4cf5689266e10f2224967) |

Roteiro de testes: `.specify/validations/117-notifications-ui-and-honest-navigation/test-roadmap.md` · Evidência completa pós-CI/ACR: `.specify/validations/117-notifications-ui-and-honest-navigation/evidence.md`.
Briefing de Compreensão (`tenant`): `docs/governance/2026-11-23-117-briefing-compreensao.md`.
Issues R6 vinculadas: `#391`..`#395` · PR: [#396](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396).

**Conclusão**: passos `1–10` do roteiro concluídos com `PASS` (inclusive correção `189edd4fe` e ACR final `APROVADO`); apenas o Passo `11` (tema escuro real e staging/produção externos) permanece **`[NV]`**.

## Atualização append-only — Extensão E2E Granular, `.dark` e Sidebar Recolhida (2026-11-23)

| Comando (`fix/117-granular-e2e-dark-and-a11y`, `task-847`) | Resultado observado |
|---|---|
| `bun run check` (`svelte-check`) | `svelte-check found 0 errors and 0 warnings`, exit `0` |
| `bun run lint` (`prettier --check . && eslint .`) | exit `0` |
| `bun run test:unit -- --run --project server` | `Test Files 18 passed (18)`, `Tests 71 passed (71)`, exit `0` |
| `npm run build` + `bun run test:e2e` (PG 18.7 `127.0.0.1:15433`, `FLORA_STORAGE__LOCAL_PATH=/tmp/flora-e2e-storage`) | **`4 passed (1.3m)`**; **12/12 `WCAG 2.2 AA: sem achados`** (incl. `/files (populated after reload)`, `/notifications (1 unread singular)`, `/notifications (sidebar collapsed)` após fix `#bits-s1`, e `.dark` em `/dms`, `/notifications`, `/tasks`); 17 PNGs em [Gist `962e8450` @ `8952ffbd`](https://gist.github.com/lfgranja/962e84502fb4cf5689266e10f2224967/8952ffbd89eebc783b8d20de4e9311dbe048a306) |
| `cargo test -p flora-api oidc::` / `flora-tests test_auth_` / `test_registration_flow` | `5 passed; 0 failed` / `7 passed; 0 failed` / `2 passed; 0 failed` *(JWKS/ID-token HS256 e `callback` state/PKCE com `PgPool` lazy passam; `test_registration_flow_with_mock_oidc` segue stub sem IdP HTTP mock)* |

- **Estado atualizado do Passo 11**: tema escuro `.dark` em navegador real validado (`PASS` com 0 violações WCAG 2.2 AA em `/dms`, `/notifications` e `/tasks`); apenas staging/produção externos (Coolify manual) e fluxo HTTP contra provedor OIDC externo permanecem **`[NV]`**.

## Atualização append-only — Correções pós-CI `e2e-browser` e ACR #1 `REPROVADO` (`82bbec11`, 2026-11-23)

- **Hardening aplicado**: `transition-[width]` no `<aside>` (`+layout.svelte`), `.dark --destructive: 0 90.6% 70.8%` (`7.23:1`) e tokens `@theme` (`layout.css`), `dark:text-green-400` (`tasks/+page.svelte`), bootstrap síncrono de `localStorage.getItem('flora.theme')` (`app.html`), remoção da chave órfã `notifications.unreadCount` (`pt-BR.ts`), `FLORA_STORAGE__LOCAL_PATH?.trim()` (`playwright.config.ts`) e `settleAnimations()` / cobertura de item não lido em `.dark` (`seed-flow.e2e.ts`).
- **Verificação local sem `E2E_SCREENSHOT_DIR` (`task-948`)**: `bun run test:e2e` → **`4 passed (31.3s)`**, **12/12 `WCAG 2.2 AA: sem achados`** com `.dark` persistido em `/notifications` e `/tasks`.
- **Escopo `[NV]` explícito**: a ativação automática por preferência do usuário em UI (`/settings`) continua `[NV]` fora do escopo do ciclo 117 (a persistência em reload é garantida via chave `flora.theme` no `localStorage`), além de deploy Coolify externo e provedor OIDC HTTP externo.
