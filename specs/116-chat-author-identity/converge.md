# Converge: 116 — identidade real de autores no chat

**Data**: 2026-11-23 · **Modo**: append-only · **Status: Converged (pré-CI)**

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#385](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/385) (`flora-core`: `UserRepository::find_by_ids`; `flora-api`: `realtime.rs`, `channels.rs`, `websocket.rs`)
- `T-002` → [#386](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/386) (`flora-tests/src/contract/messaging.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`)
- `T-003` → [#387](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/387) (`chatAuthor.ts`, `chatAuthor.test.ts`, `models.ts`, `mappers.ts`, `websocket.ts`, `MessageStore.svelte.ts`, `mappers.test.ts`, `MessageStore.test.ts`, `ChatLayout.svelte`)
- `T-004` → [#388](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/388) (`apps/frontend/tests/e2e/seed-flow.e2e.ts`)
- `T-005` → [#389](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/389) (Briefing de Compreensão `tenant`, governança, docs append-only, `test-roadmap.md`/`evidence.md`, R4/R7 `0.9.0`, ACR)

## Entrega e verificação local

- **TDD frontend RED → GREEN**: `apps/frontend/tests/unit/chatAuthor.test.ts` falhou em `RED` (`Cannot find module '../../src/lib/utils/chatAuthor'`, exit `1`) antes da criação de `chatAuthor.ts`; após criação, `4/4 passed`.
- **Rust e gate (`task-637` + `task-673`)**:
  - `cargo fmt --all -- --check`: exit `0`.
  - `cargo clippy --workspace --all-targets -- -D warnings`: `0` erros.
  - `cargo test --workspace`: `flora-api` `44 passed` (incl. `realtime::tests` `2/2`), `flora-core` `35 + 21 passed`, `flora-tests` `36 passed` / `79 ignored`.
  - `python3 scripts/ci/check_gate_integrity.py`: `gate-integrity: PASS`, `all 10 db contract floors intact (79 total)` (`messaging: 5`).
  - `git diff --check`: limpo.
- **Frontend (`task-673`)**:
  - `bun run check` (`svelte-check`): `svelte-check found 0 errors and 0 warnings`.
  - `bun run lint` (`prettier --check . && eslint .`): `All matched files use Prettier code style!` / `0` problemas ESLint, exit `0`.
  - `bun run test:unit -- --run --project server`: `Test Files  17 passed (17)`, `Tests  64 passed (64)` (`chatAuthor.test.ts` `4`, `mappers.test.ts` `3`, `MessageStore.test.ts` `2`).
  - `bun run build`: `✓ built in 26.46s` (`@sveltejs/adapter-node done`).
- **Briefing de Compreensão (`tenant`)**: preenchido em `docs/governance/2026-11-23-116-briefing-compreensao.md`.
- **[NV]**: execução remota de `db-tests` (Postgres 18), `e2e-browser` (`# general` `'Alice Admin'` / `'Bob Builder'` + WCAG), ACR independente, WS multi-usuário ao vivo no E2E, modo escuro real e staging/produção.

## Fechamento pós-CI e ACR (2026-11-23 UTC; append-only)

- **CI remoto (`a5a35fd7b`)**: PR #390 `SUCCESS` em `CI/frontend-build` (`45113661689` / `137491474792`), `Rust CI/rust-build` (`45113661700`), `db-tests` (`137491474891`: `db_messages_include_author_display_name_across_post_list_and_catchup ... ok`, `messaging: passed=5 floor=5`, **79/79**), `dependency-review`, `supply-chain-vet` e `e2e-browser` (`137491474873`: `4 passed (37.8s)` com `'Alice Admin'` / `'Bob Builder'` em `# general` e WCAG `"sem achados"` em `/tasks`, `/channels`, `/files`).
- **ACR independente (`a5a35fd7b`, revisor `4ff7319c`, `pro`)**: **`APPROVE`** (0 Critical, 0 High, 0 Medium; 3 observações Low não bloqueantes documentadas em `evidence.md` e placeholder do PR atualizado para `#390` em `docs/governance/2026-11-23-116-briefing-compreensao.md`). R4 atualizado para `APROVADO com ressalva [NV]`.
