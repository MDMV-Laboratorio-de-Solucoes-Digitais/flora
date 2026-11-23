# Plan: 115 — listagem persistida de arquivos por workspace

**Data**: 2026-11-23 · **Baseline**: `github/dev@07556dd0e`

## Mapeamento FR/SC → Arquivos e Verificação

| FR / SC | Arquivos alvo | Verificação |
|---|---|---|
| FR-001 / SC-001 | `flora-api/src/routes/files.rs` (`ListFilesQuery`, `resolve_list_limit`, `list_files`, `#[cfg(test)]`) | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p flora-api files::tests` |
| FR-002 / SC-002 | `flora-tests/src/contract/files.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml` | `python3 scripts/ci/check_gate_integrity.py` (78 total) + job `db-tests` (`files: passed=7 floor=7`) |
| FR-003 / SC-003 | `apps/frontend/src/lib/utils/files.ts`, `apps/frontend/src/lib/state/FileStore.svelte.ts`, `apps/frontend/tests/unit/FileStore.test.ts`, `apps/frontend/src/lib/i18n/pt-BR.ts`, `apps/frontend/src/routes/(app)/files/+page.svelte` | TDD RED→GREEN (`FileStore.test.ts`), `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build` |
| FR-004 / SC-004 | `apps/frontend/tests/e2e/seed-flow.e2e.ts` | Job `e2e-browser` (`4 passed`, `/files` com `"sem achados"`) |
| FR-005 / SC-005 | `docs/governance/2026-11-22-115-briefing-compreensao.md`, `docs/governance/2026-11-22-115-files-list-persistence.md`, `specs/115-files-list-persistence/converge.md`, `.specify/validations/115-files-list-persistence/*`, `.specify/releases/115-files-list-persistence/release.md`, `STATE.md`, `PROGRESSO.md`, `research/index.md`, `OBJETIVO.md`, `ROADMAP.md` | `git diff --check`, recontagem `find`, ACR independente e CI remoto verde |

## Passos ordenados

1. Fechar `checklist.md`, `tasks.md` e `analyze.md` (read-only após `tasks`).
2. Executar `11b taskstoissues` (`T-001`..`T-005`) no GitHub sem label `dark-factory:eligible`.
3. Implementar backend + contrato DB + lockstep floor (`files: 6 -> 7`, total `78`).
4. Implementar frontend em TDD estrito (RED `FileStore.test.ts` → GREEN `files.ts` + `FileStore.svelte.ts` → `pt-BR.ts` → `files/+page.svelte` → `seed-flow.e2e.ts`).
5. Validar localmente com `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target`, preencher Briefing de Compreensão, `test-roadmap.md`, R4/R7, docs append-only e abrir PR para `dev`.
6. Acompanhar CI remoto + ACR independente, anexar `evidence.md`, revalidar CI no head final, mesclar em `dev` e atualizar PR #371 (`release/112-promotion` → `main`) para escalonamento ao SWE.
