# Release: 115 — listagem persistida de arquivos por workspace

**Ciclo**: 115 · **Data**: 2026-11-23 · **Baseline**: `github/dev@07556dd0e`

## Entregue

- `FR-001` / `T-001` (`#379`): exposição da rota autenticada `GET /api/v1/files?workspace_id=<uuid>[&limit=<n>]` em `flora-api/src/routes/files.rs` (`ListFilesQuery`, `resolve_list_limit` em `1..=100` / default `50`, `list_files` com `ensure_member` + `ensure_workspace_in_org`, 2 testes unitários em `files::tests`).
- `FR-002` / `T-002` (`#380`): contrato DB `db_list_files_scoped_to_workspace_and_org` em `flora-tests/src/contract/files.rs` e elevação em lockstep do piso `files: 6 → 7` (total `78`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
- `FR-003` / `T-003` (`#381`): `filesQueryPath` (`utils/files.ts`), `FileStore.setFiles`/`clear` + ordenação newest-first com dedup em `addFile` (`FileStore.svelte.ts`), suíte TDD `FileStore.test.ts` (4/4), chaves pt-BR (`pt-BR.ts`) e carregamento/limpeza com guarda `loadSeq` em `files/+page.svelte`.
- `FR-004` / `T-004` (`#382`): verificação de `t['files.empty']` visível em Passo 5 de `seed-flow.e2e.ts` antes da auditoria WCAG 2.2 AA em `/files`.
- `FR-005` / `T-005` (`#383`): Briefing de Compreensão (`tenant`), registro de governança, docs append-only com recontagem determinística, roteiro/evidência, R4/R7 e ACR independente.

## SemVer

MINOR `0.7.16` → `0.8.0`: nova funcionalidade pública retrocompatível (`GET /api/v1/files`) com persistência de listagem de arquivos no cliente, sem quebra de contrato existente ou alteração de schema/required checks.

## Evidência

Portas locais verdes (`cargo fmt`, `clippy -D warnings`, `flora-api files::tests` `2/2`, `check_gate_integrity.py` PASS 10 legs / 78 pisos, `svelte-check` 0/0, `lint` exit 0, Vitest `server` `16/58`, `vite build` exit 0). Evidência remota de CI (`db-tests`, `e2e-browser`) e ACR será anexada em `.specify/validations/115-files-list-persistence/evidence.md`.

## Rollback

Novo commit de reversão dos arquivos do ciclo 115 (reduzindo em lockstep o piso `files` `7 → 6` caso `db_list_files_scoped_to_workspace_and_org` seja revertido); sem alteração de banco de dados e sem force-push.

## [NV]

Execução local direta com Postgres de staging real, modo escuro em navegador real, espelho Forgejo e deploy em produção.
