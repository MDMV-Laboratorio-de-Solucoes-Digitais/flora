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

Roteiro de testes: `.specify/validations/117-notifications-ui-and-honest-navigation/test-roadmap.md` · Evidência completa pós-CI/ACR: `.specify/validations/117-notifications-ui-and-honest-navigation/evidence.md`.
Briefing de Compreensão (`tenant`): `docs/governance/2026-11-23-117-briefing-compreensao.md`.
Issues R6 vinculadas: `#391`..`#395` · PR: [#396](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396).

**Conclusão**: passos `1–10` do roteiro concluídos com `PASS` (inclusive correção `189edd4fe` e ACR final `APROVADO`); apenas o Passo `11` (tema escuro real e staging/produção externos) permanece **`[NV]`**.
