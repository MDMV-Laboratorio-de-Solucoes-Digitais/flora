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

## Adendo — Remediação ACR Rodada 1 (`#424`, 2026-11-24T20:35Z)

Após a Rodada 1 do ACR independente (`1 HIGH`, `1 MEDIUM`, `4 LOW` sobre `16cc37cb9`), foram aplicados os seguintes ajustes e verificações locais (`[EV]`, PG 18.7 `127.0.0.1:15433`):

- **Backend**: `SessionRepository::revoke_for_user_and_organization` + `MembershipRepository::delete_with_role` atômico em `revoke_role` (revogando todas as sessões ativas do membro na organização removida), correção da checagem de membership em `flora-api/src/routes/search.rs`, validação de e-mail em `validate_member_email` com `Error::Validation` (`422`) e cobertura de contrato DB ampliada em `flora-tests/src/contract/rbac.rs` (`401` pós-revogação, `422` e-mail inválido, `404` usuário inativo/role de outra org/assign de não membro).
- **Frontend**: `members.roleConflict` distinto de `members.duplicate` em `MemberManager.svelte` + `pt-BR.ts`, selo `Você` condicionado a `currentUserId && member.userId === currentUserId`, `roleStore.syncContext(key)` em `loadMembers`, reset de `addRoleId` ao trocar contexto, `novalidate` no formulário de adição e asserções de 0 botões de ação na linha de auto-usuário em `seed-flow.e2e.ts`.
- **Resultados `[EV]` pós-edição**:
  - `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources && cargo test --workspace`: `PASS` (`flora-api` `55/55`, `flora-core` `36+21`, `flora-tests` `36 passed; 87 ignored`).
  - `cargo test -p flora-tests contract::rbac::db::db_ -- --ignored --test-threads=1`: **`5 passed; 0 failed`** (`3.09s`).
  - `cargo test -p flora-tests contract::search::db_ -- --ignored --test-threads=1`: **`8 passed; 0 failed`** (`4.00s`).
  - `check_gate_integrity.py && check_deploy_workflows.py && --self-test`: `PASS` (`87` pisos DB intactos).
  - `apps/frontend`: `bun run format && bun run check && bun run lint && bun run test:unit -- --run --project server && vite build`: `PASS` (`21/21` arquivos, `89/89` testes).
  - `bun run test:e2e` (Chromium + `flora-api` recompilado + PG 18.7): **`4 passed (1.9m)`**, **`15/15` auditorias WCAG 2.2 AA: `sem achados`**.
