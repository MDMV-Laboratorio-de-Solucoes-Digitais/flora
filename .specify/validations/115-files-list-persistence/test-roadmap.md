# Test roadmap — ciclo 115

**Data**: 2026-11-23 · **Baseline**: `github/dev@07556dd0e` · **Superfície**: API HTTP (`flora-api`), Contrato DB (`flora-tests`) e UI real (`/files` SvelteKit)

## Escopo

FR-001/002/003/004/005 · Tarefas `T-001`..`T-005` (issues `#379`..`#383`) · Domínio crítico `tenant`. Trava lockstep elevada para `files: 7` (total `78`), sem alteração de required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`).

## Pré-condições

- Branch `feature/115-files-list-persistence` publicada em `github`.
- Dependências frontend instaladas via `bun install --frozen-lockfile`.
- Job `db-tests` (Postgres 18) e job `e2e-browser` (preview SvelteKit em `http://127.0.0.1:4173` + Chromium + `flora-api`).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` | Zero erros de formatação ou clippy pedantic | CLI / Rust | sim |
| 2 | `cargo test -p flora-api files::tests` | `2 passed; 0 failed` (`resolve_list_limit` e `to_attachment`) | CLI / Rust | sim |
| 3 | `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`, 4 required checks, 10 legs / 78 pisos (`files: 7`) | CLI / CI Gate | sim |
| 4 | `bun run check && bun run lint` em `apps/frontend` | `svelte-check` 0 erros/0 avisos; `prettier` + `eslint` exit 0 | CLI / Frontend | sim |
| 5 | `bun run test:unit -- --run --project server` em `apps/frontend` | `16 passed (16)` arquivos e `58 passed (58)` testes, incluindo `FileStore.test.ts` (4/4 TDD) | CLI / Frontend | sim |
| 6 | `bun run build` em `apps/frontend` | Build Vite/SvelteKit exit 0 | CLI / Frontend | sim |
| 7 | `gh pr checks <PR> --watch` no head exato do PR | Required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) e informativos (`supply-chain-vet`, `e2e-browser`) SUCCESS | CI | sim |
| 8 | Inspeção de logs `db-tests` no GitHub Actions (`gh run view`) | Perna `files` reporta `passed=7`, `floor=7`, `0 failed` incluindo `db_list_files_scoped_to_workspace_and_org` | CI / Postgres 18 | sim |
| 9 | Inspeção de logs `e2e-browser` (`gh run view`) | `4 passed`, `t['files.empty']` visível após `GET /api/v1/files`, e 3 linhas WCAG `"sem achados"` (`/tasks`, `/channels`, `/files`) | CI / UI (Chromium) | sim |
| 10 | Code Review agêntico independente (contexto limpo, modelo `pro`) | 0 blockers críticos/high em isolamento BOLA/tenant, DTO, `loadSeq` e gate | PR | sim |
| 11 | Verificação visual em modo escuro real e staging/produção externos | `[NV]` — sem toggle `.dark` no E2E nem credenciais externas de staging | UI / Staging | não (`[NV]`) |

## Plano de capturas

Saídas locais coladas em `validate.md`; evidência pós-PR (`gh pr checks`, logs `db-tests` e `e2e-browser` com contagens exatas, resultado da ACR) registrada em `.specify/validations/115-files-list-persistence/evidence.md`. Nenhuma captura com credenciais ou segredos é versionada.

## Rollback

Novo commit de reversão dos arquivos do ciclo 115 (voltando o piso `files` `7 → 6` em lockstep apenas se `db_list_files_scoped_to_workspace_and_org` for revertido); sem alteração de schema e sem force-push.
