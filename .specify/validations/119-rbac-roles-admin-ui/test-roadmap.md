# Test roadmap — ciclo 119

**Data**: 2026-11-23 · **Baseline**: `github/dev@df60db4b8` · **Superfície**: UI real Svelte 5 (`/settings` — `RoleManager.svelte`), `RoleStore` + helpers Vitest e endpoints backend `/api/v1/roles` (`GET/POST/PATCH/DELETE`) com PostgreSQL 18

## Escopo

FR-001/002/003/004 · SC-001..SC-004 · Domínio crítico `auth`/`rbac` (sem alteração de schema SQL ou pisos DB `82`).

## Pré-condições

- Postgres 18 ativo (`FLORA_DATABASE__POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:15433/flora'`, `PGPORT=15433`).
- Dependências frontend instaladas (`bun install --frozen-lockfile`) e `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target`.
- Preview SvelteKit em `http://127.0.0.1:4173` + `flora-api` em `http://127.0.0.1:3000` (seed `dev-seed.sql` + `scripts/dev-smoke-session.sql`).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `bun run check && bun run lint` em `apps/frontend` | `svelte-check` 0 erros/0 avisos; `prettier` + `eslint` exit 0 | CLI / Frontend | sim |
| 2 | `bun run test:unit -- --run --project server` em `apps/frontend` | `roles.test.ts` e toda a suíte unitária passam (TDD RED→GREEN) | CLI / Vitest | sim |
| 3 | `bun run build` em `apps/frontend` | Build SvelteKit `adapter-node` exit 0 | CLI / Frontend | sim |
| 4 | `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`, 4 required checks, `82` pisos intactos | CLI / Gate | sim |
| 5 | `bun run test:e2e` (Chromium real + PG 18) — Passo 9 `/settings` `.dark` inicial | `Admin` e badge `Nativo` visíveis sem ações destrutivas; WCAG 2.2 AA `sem achados`; captura `18-settings-roles-dark.png` | Browser / UI | sim |
| 6 | Passo 9 E2E — Criar papel `Auditor E2E` (`POST /api/v1/roles` → `201`) + `page.reload()` | Papel personalizado aparece na lista, persiste no PostgreSQL após `reload` | Browser + API + DB | sim |
| 7 | Passo 9 E2E — Editar papel (`PATCH /api/v1/roles/{id}` → `200`) + alternar modo claro + excluir (`DELETE` → `204`) | Papel atualizado visível em modo claro, WCAG 2.2 AA `sem achados`, captura `19-settings-roles-light.png`, exclusão remove linha da lista | Browser + API + DB | sim |
| 8 | `gh pr checks <PR> --watch` + inspeção ao vivo de `frontend-build`, `db-tests` (`82/82`), `e2e-browser` + ACR independente (`pro`) | Todos os required checks GREEN, `e2e-browser` PASS (`14/14` WCAG 2.2 AA `sem achados`) e ACR `APROVADO` | CI / GitHub | sim |
| 9 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Produção | não (`[NV]`) |

## Plano de capturas

Capturas geradas com `E2E_SCREENSHOT_DIR=/tmp/flora-e2e-119`, publicadas em Gist público sem incluir binários no git e referenciadas por URL em `.specify/validations/119-rbac-roles-admin-ui/evidence.md` e no PR.

## Rollback

Novo commit de reversão removendo `RoleManager.svelte`, `roles.ts`, `RoleStore.svelte.ts`, helpers/testes em `apps/frontend` e restaurando `settings/+page.svelte`, `layout.css`, `pt-BR.ts` e `seed-flow.e2e.ts`; zero migrations SQL e zero alteração nos 82 pisos DB.
