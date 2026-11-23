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

## Fechamento pós-CI e ACR (2026-11-23 UTC; append-only)

- **ACR Round 1 (`51b55c252`, revisor `33c01dfa`)**: `REQUEST_CHANGES` (1 High, 2 Medium, 2 Low) — apontou `"error": "validation"` em vez de `"validation_error"` no contrato DB e no Briefing (confirmado no run `45107507844`), limpeza de `fileStore` e guarda `isLoading || isLoadingFiles` no `onMount` de `files/+page.svelte`, checagem explícita de HTTP `200` + ausência de alerta em `seed-flow.e2e.ts` e captura de `targetWorkspaceId` no upload concorrente (`FileUpload.svelte`). Todos corrigidos no commit `df6382dd7`.
- **Revalidação local (`df6382dd7`)**: `cargo fmt`/`clippy`/`cargo test --workspace` (`flora-api` 42/42, `flora-core` 35+21, `flora-tests` 36 passed / 78 ignored), `check_gate_integrity.py` PASS (10 legs / 78 pisos), `svelte-check` 0/0, `prettier`/`eslint` exit 0, Vitest `server` `16 passed / 58 passed`, `vite build` `33.20s`, `git diff --check` limpo.
- **CI remoto (`df6382dd7`)**: PR #384 `CLEAN`; `frontend-build` (`45108788166`), `rust-build`, `db-tests` (`45108788116` / `137476110671`: `db_list_files_scoped_to_workspace_and_org ... ok`, `files: passed=7 floor=7`, **78/78**), `dependency-review`, `supply-chain-vet` e `e2e-browser` (`45108788116` / `137476110594`: `4 passed (32.5s)` e WCAG `"sem achados"` em `/tasks`, `/channels`, `/files`) = `SUCCESS`.
- **ACR Round 2 (`df6382dd7`, revisor `2600ac2c`)**: **`APPROVE`** (0 críticos/high/medium; pequenas referências de linha no Briefing ajustadas). R4 atualizado para `APROVADO com ressalva [NV]`; evidência anexada em `.specify/validations/115-files-list-persistence/evidence.md`.
