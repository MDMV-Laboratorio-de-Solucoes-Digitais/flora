# Converge: 115 — listagem persistida de arquivos por workspace

**Data**: 2026-11-23 · **Modo**: append-only · **Status: Converged (pré-CI)**

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#379](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/379) (`flora-api/src/routes/files.rs`: `ListFilesQuery`, `resolve_list_limit`, `list_files`, `files::tests`)
- `T-002` → [#380](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/380) (`flora-tests/src/contract/files.rs`: `db_list_files_scoped_to_workspace_and_org`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`)
- `T-003` → [#381](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/381) (`files.ts`, `FileStore.svelte.ts`, `FileStore.test.ts`, `pt-BR.ts`, `files/+page.svelte`)
- `T-004` → [#382](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/382) (`apps/frontend/tests/e2e/seed-flow.e2e.ts`)
- `T-005` → [#383](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/383) (Briefing de Compreensão `tenant`, governança, docs append-only, R4/R7 `0.8.0`, ACR)

## Entrega e verificação local

- **TDD frontend RED → GREEN**: `apps/frontend/tests/unit/FileStore.test.ts` falhou em `4/4` testes antes da implementação (`TypeError: filesQueryPath is not a function`, `store.setFiles is not a function`); após implementação, `4/4 passed`.
- **Rust e gate (`task-402`)**:
  - `cargo fmt --all -- --check`: exit 0.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 erros em todo o workspace (`flora-api` e `flora-tests` após refatoração de helpers locais).
  - `cargo test -p flora-api files::tests`: `2 passed; 0 failed; 40 filtered out`.
  - `python3 scripts/ci/check_gate_integrity.py`: `gate-integrity: PASS`, `all 10 db contract floors intact (78 total)` (`files: 7`).
  - `git diff --check`: exit 0.
- **Frontend (`task-402`)**:
  - `bun run check` (`svelte-check`): `svelte-check found 0 errors and 0 warnings`.
  - `bun run lint` (`prettier --check . && eslint .`): `All matched files use Prettier code style!` / 0 erros ESLint.
  - `bun run test:unit -- --run --project server`: `Test Files  16 passed (16)`, `Tests  58 passed (58)`.
  - `bun run build`: `✓ built in 31.56s` (`@sveltejs/adapter-node` done).
- **Briefing de Compreensão (`tenant`)**: preenchido em `docs/governance/2026-11-22-115-briefing-compreensao.md` seguindo `.specify/templates/briefing-compreensao.md`.
- **[NV]**: execução remota de `db-tests` (Postgres 18), `e2e-browser` (Chromium + WCAG `/tasks`, `/channels`, `/files`), ACR independente, modo escuro em navegador real e staging/produção externos.
