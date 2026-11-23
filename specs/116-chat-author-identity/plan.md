# Plan: 116 — identidade real de autores no chat

**Data**: 2026-11-23 · **Baseline**: `github/dev@2052ad7f1`

## Mapeamento FR/SC → Arquivos e Verificação

| FR / SC | Arquivos alvo | Verificação |
|---|---|---|
| FR-001 / SC-001 | `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/user_repository.rs`, `flora-api/src/realtime.rs`, `flora-api/src/routes/{channels,websocket}.rs` | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (incl. `realtime::tests`) |
| FR-002 / SC-002 | `flora-tests/src/contract/messaging.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml` | `python3 scripts/ci/check_gate_integrity.py` (79 total) + job `db-tests` (`messaging: passed=5 floor=5`) |
| FR-003 / SC-003 | `chatAuthor.ts`, `chatAuthor.test.ts`, `models.ts`, `mappers.ts`, `websocket.ts`, `MessageStore.svelte.ts`, `mappers.test.ts`, `MessageStore.test.ts`, `ChatLayout.svelte` | TDD RED→GREEN (`chatAuthor.test.ts`), `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build` |
| FR-004 / SC-004 | `apps/frontend/tests/e2e/seed-flow.e2e.ts` | Job `e2e-browser` (`4 passed`, `'Alice Admin'`/`'Bob Builder'` visíveis, WCAG `"sem achados"`) |
| FR-005 / SC-005 | `2026-11-23-116-briefing-compreensao.md`, `2026-11-23-116-chat-author-identity.md`, `converge.md`, `.specify/validations/116-chat-author-identity/*`, `.specify/releases/116-chat-author-identity/release.md`, `STATE.md`, `PROGRESSO.md`, `research/index.md`, `OBJETIVO.md`, `ROADMAP.md` | `git diff --check`, recontagem `find`, ACR independente e CI remoto verde |

## Passos ordenados

1. Fechar `checklist.md`, `tasks.md` e `analyze.md` (somente leitura após `tasks`).
2. Executar `11b taskstoissues` (`T-001`..`T-005`) no GitHub sem label `dark-factory:eligible`.
3. Implementar backend em lote + contrato DB + trava lockstep (`messaging: 4 -> 5`, total `79`).
4. Implementar frontend em TDD estrito (`chatAuthor.test.ts` RED→GREEN, mappers/store/testes, `ChatLayout.svelte`, `seed-flow.e2e.ts`).
5. Validar localmente com `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target`, preencher Briefing (`tenant`), R4/R7 (`0.9.0`), docs append-only e abrir PR para `dev`.
6. Acompanhar CI + ACR independente, anexar `evidence.md`, revalidar CI no head final, mesclar em `dev` e atualizar PR #371 (`release/112-promotion` → `main`) para escalonamento ao SWE.
