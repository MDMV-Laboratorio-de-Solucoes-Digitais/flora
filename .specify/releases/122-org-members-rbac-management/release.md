# Release: 122 — Gestão de membros e papéis RBAC em `/settings`

**Ciclo**: 122 · **Data**: 2026-11-24 · **Baseline**: `github/dev@8f984adb7` · **Fecha**: `BF-02 -> fechado` (conclui todos os itens acionáveis do `backlog_de_fechamento`)

## Entregue

- `FR-001` + `FR-002` / `T-001` ([#420](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/420)):
  - `flora-core/src/traits/repository.rs` e `repositories/membership_repository.rs`: `MembershipRepository::update_role` atômico preservando `joined_at` e `metadata`.
  - `flora-api/src/routes/rbac.rs`: `GET /api/v1/roles/members` (batch `find_by_ids` sem N+1 nem dados sensíveis), `POST /api/v1/roles/members` (adiciona usuário ativo existente por e-mail + `role_id` da org ativa), hardening de `assign_role` (rejeita auto-rebaixamento `403`, atualiza apenas membros existentes via `update_role`), hardening de `revoke_role` (valida papel alvo na org, exige correspondência `membership.role_id == id` `409`, rejeita auto-revogação `403`), `#[serde(deny_unknown_fields)]` em `AddMemberRequest`/`UserRoleRequest` e `2` testes unitários.
  - `flora-tests/src/contract/rbac.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`: `+2` contratos PostgreSQL 18 (`rbac: 3 -> 5`, total `85 -> 87`).
- `FR-003` / `T-002` ([#421](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/421)): `OrganizationMemberItem` (`src/lib/types/models.ts`), `utils/members.ts`, `api/roles.ts` (`encodeURIComponent(roleId)` + endpoints `/members`, `/assign`, `/revoke`), `MemberStore.svelte.ts` (Svelte 5 Runes) e `5` testes TDD em `tests/unit/members.test.ts`.
- `FR-003` + `FR-004` / `T-003` ([#422](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/422)): `pt-BR.ts` (`members.*`, `error.*`), `MemberManager.svelte` montado em `routes/(app)/settings/+page.svelte` e extensão Passo 10 em `tests/e2e/seed-flow.e2e.ts` (reatribuição de papel + `page.reload()` + restauração de `Bob Builder`, remoção e readição de `Carol Viewer`, WCAG 2.2 AA e Gist `2458075a6526da72fd23bd43cf9f2fda`).
- `FR-004` / `T-004` ([#423](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/423)): Briefing de Compreensão (`auth`/`rbac`), R1/SDD/R4/R7 (`0.14.0`), docs append-only fechando `BF-02 -> fechado`, ACR independente e promoção `dev -> main`.

## SemVer

MINOR `0.13.0` → `0.14.0`: nova superfície completa de gestão de membros e atribuição/revogação de papéis RBAC (endpoints REST `/api/v1/roles/members`, hardening de `assign`/`revoke` e UI `MemberManager.svelte` em `/settings`) sem breaking change em contratos existentes nem migrations SQL (`87` pisos DB em lockstep).

## Evidência

Portas locais verdes (`task-2207`, `task-2234`, `task-2283`, `task-2315`, `task-2376`): `fmt`, `clippy --all-targets -- -D warnings`, `cargo deny`, `cargo test --workspace` (`flora-api` `55/55`, `flora-core` `36+21`, `flora-tests` `36/87`), `contract::rbac::db::db_` **`5/5`**, `check_gate_integrity.py` (`PASS`, `87` pisos), `check_deploy_workflows.py` + `--self-test` (`PASS`), `svelte-check` `0/0`, `prettier`/`eslint` `exit 0`, Vitest `21/21` / `89/89`, `vite build` `1m 20s`, Playwright E2E Chromium real + PG 18.7 (**`4 passed (37.5s)`**, **`15/15` auditorias WCAG 2.2 AA: `sem achados`**) e capturas em <https://gist.github.com/lfgranja/2458075a6526da72fd23bd43cf9f2fda>.

## Rollback

Reverter o commit deste ciclo (restaurando `rbac: 3` / `85` pisos em `check_gate_integrity.py` e `.github/workflows/rust.yml`); nenhuma migration SQL foi criada e nenhuma tabela foi alterada.

## [NV]

- CI remoto pós-PR em `dev`, ACR independente e deploy manual em staging/produção via Coolify (`workflow_dispatch`).
