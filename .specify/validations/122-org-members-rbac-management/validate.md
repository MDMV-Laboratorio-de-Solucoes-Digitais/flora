# Validação R4 — ciclo 122 (`122-org-members-rbac-management`)

**Data**: 2026-11-24 UTC · **Baseline**: `github/dev@8f984adb7` · **Veredito: APROVADO com ressalva `[NV]`** *(Passos 1–5 verificados localmente nas tasks `task-2207`, `task-2234`, `task-2283`, `task-2315` e `task-2376`; Passo 6 CI remoto/ACR após abertura do PR em `dev`; Passo 7 staging/produção `[NV]`)*

| Comando | Resultado observado |
|---|---|
| `cargo test -p flora-tests contract::rbac::db::db_rbac_members_` pré-implementação (`task-2207`) | FAIL esperado (`0 passed; 2 failed` — `400 vs 401` em `/members`, `204 vs 403` auto-rebaixamento) |
| `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources` (`task-2315`) | PASS (`exit 0`, `bans ok, licenses ok, sources ok`) |
| `cargo test --workspace` (`task-2315`) | `flora-api` `55/55`, `flora-core` `36+21`, `flora-tests` `36 passed; 87 ignored` |
| `cargo test -p flora-tests contract::rbac::db::db_ -- --ignored --test-threads=1` (`task-2315`) | **`5 passed; 0 failed`** (`10.96s` em PG 18.7) |
| `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` (`task-2315`) | `gate-integrity: PASS` (**`87` pisos DB intactos**), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS` |
| `bun run check && bun run lint && bun run test:unit -- --run --project server && VITE_API_URL=http://127.0.0.1:3000 bun run build` (`task-2283`, `task-2376`) | `svelte-check` `0/0`, `prettier`/`eslint` `exit 0`, **`21/21` arquivos / `89/89` testes**, `vite build` `exit 0` |
| `bun run test:e2e` (Chromium real + `flora-api` recompilado + `vite preview` + PG 18.7, `task-2376`) | **`4 passed (37.5s)`**; **`15/15` auditorias WCAG 2.2 AA: `sem achados`** (incl. Passo 10 `/settings (members light mode)`) |

Roteiro: `.specify/validations/122-org-members-rbac-management/test-roadmap.md` · Evidência e Gist: `.specify/validations/122-org-members-rbac-management/evidence.md`.
Briefing de Compreensão (`auth` / `rbac`): `docs/governance/2026-11-24-122-briefing-compreensao.md`.
Issues R6 vinculadas: `#420`–`#423`.

**Conclusão**: todas as verificações backend (`fmt`/`clippy`/`deny`/`55`+`5` contratos DB `87`), gates de integridade (`87` pisos), frontend (`svelte-check`, `eslint`, `89` testes unitários, `vite build`) e fluxo E2E real com WCAG 2.2 AA (`15/15` sem achados) estão verdes; permanecem **`[NV]`** até a etapa de PR o CI remoto/ACR e, fora do gate de PR, o deploy manual em staging/produção no Coolify.

## Adendo — Revalidação Pós-Remediação ACR Rodada 1 (`#424`, 2026-11-24T20:35Z)

| Verificação | Resultado observado (`[EV]`) |
|---|---|
| `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources && cargo test --workspace` | `PASS` (`flora-api` `55/55`, `flora-core` `36+21`, `flora-tests` `36 passed; 87 ignored`) |
| `cargo test -p flora-tests contract::rbac::db::db_ -- --ignored --test-threads=1` | **`5 passed; 0 failed`** (incl. `401` de token após `revoke_role`, `422` e-mail inválido, `404` inativo/cross-org) |
| `cargo test -p flora-tests contract::search::db_ -- --ignored --test-threads=1` | **`8 passed; 0 failed`** (após corrigir checagem `.ok_or_else(Forbidden)` em `routes/search.rs`) |
| `check_gate_integrity.py && check_deploy_workflows.py && --self-test` | `gate-integrity: PASS` (`87` pisos), `deploy-workflows: PASS`, `self-test: PASS` |
| `bun run format && bun run check && bun run lint && bun run test:unit -- --run --project server && vite build` | `PASS` (`21/21` arquivos, `89/89` testes, build limpo) |
| `bun run test:e2e` (Chromium + `flora-api` recompilado + PG 18.7) | **`4 passed (1.9m)`**, **`15/15` WCAG 2.2 AA: `sem achados`** |
