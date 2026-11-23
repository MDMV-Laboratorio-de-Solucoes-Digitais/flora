# Test roadmap — ciclo 117

**Data**: 2026-11-23 · **Baseline**: `github/dev@0cc41336f` · **Superfície**: API HTTP (`notifications.rs`), Repositório (`notification_repository.rs`), Contrato DB (`flora-tests`) e UI real (`/notifications`, `/dms`, `+layout.svelte` SvelteKit)

## Escopo

FR-001/002/003/004/005 · Tarefas `T-001`..`T-005` (issues `#391`..`#395`) · Domínio crítico `tenant`. Trava lockstep elevada para `notifications: 6` (total `80`), sem alteração de required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`).

## Pré-condições

- Branch `feature/117-notifications-ui-and-honest-navigation` publicada em `github`.
- Dependências frontend instaladas via `bun install --frozen-lockfile`.
- Job `db-tests` (Postgres 18) e job `e2e-browser` (preview SvelteKit em `http://127.0.0.1:4173` + Chromium + `flora-api` com `dev-seed.sql`).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` | Zero erros de formatação ou clippy pedantic | CLI / Rust | sim |
| 2 | `cargo test --workspace` | Todas as suítes Rust unitárias/integração sem banco passam, incl. `notifications::tests` | CLI / Rust | sim |
| 3 | `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`, 4 required checks, 10 legs / 80 pisos (`notifications: 6`) | CLI / Gate | sim |
| 4 | `bun run check && bun run lint` em `apps/frontend` | `svelte-check` 0 erros/0 avisos; `prettier` + `eslint` exit 0 | CLI / Frontend | sim |
| 5 | `bun run test:unit -- --run --project server` em `apps/frontend` | Todas as suítes passam, incl. TDD `notifications.test.ts` + `mappers.test.ts` | CLI / Frontend | sim |
| 6 | `bun run build` em `apps/frontend` | Build Vite/SvelteKit exit 0 | CLI / Frontend | sim |
| 7 | `gh pr checks <PR> --watch` no head exato do PR | Required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) e informativos (`supply-chain-vet`, `e2e-browser`) SUCCESS | CI | sim |
| 8 | Inspeção de logs `db-tests` no GitHub Actions (`gh run view`) | Perna `notifications` reporta `passed=6`, `floor=6`, `0 failed` incluindo `db_notifications_org_scoped_target_url_and_read_all` (80/80) | CI / Postgres 18 | sim |
| 9 | Inspeção de logs `e2e-browser` (`gh run view`) | Passos 6 e 7 em `seed-flow.e2e.ts`: ausência de atalhos fantasma, `/notifications` (`GET 200` + empty state), `/dms` honesto e WCAG 2.2 AA `"sem achados"` | CI / UI (Chromium) | sim |
| 10 | Code Review agêntico independente (contexto limpo, modelo `pro`) | 0 blockers críticos/high em isolamento cross-org (`404` fail-closed), `POST /read-all`, XSS/open-redirect em `target_url` e WCAG 2.2 AA | PR | sim |
| 11 | Tema escuro em navegador real e staging/produção externos | `[NV]` — `e2e-browser` executa em Chromium tema claro com `dev-seed.sql` | UI / Staging | não (`[NV]`) |

## Plano de capturas

Saídas locais registradas em `validate.md`; evidência pós-PR (`gh pr checks`, logs `db-tests` 80/80 e `e2e-browser`, resultado ACR) registrada em `.specify/validations/117-notifications-ui-and-honest-navigation/evidence.md`. Nenhuma captura com tokens ou segredos é versionada.

## Rollback

Novo commit de reversão dos arquivos do ciclo 117 (voltando `notifications` `6 → 5` em lockstep caso `db_notifications_org_scoped_target_url_and_read_all` seja removido); nenhuma migration SQL alterada e nenhum force-push.
