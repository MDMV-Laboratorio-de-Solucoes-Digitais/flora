# 115 — listagem persistida de arquivos por workspace

**Data**: 2026-11-23 · **Critério 80/20**: `400/625` · **Issues**: `#379`..`#383` · **Domínio crítico**: `tenant`

## O que mudou

- **Endpoint backend `GET /api/v1/files` (`T-001` / `#379`)**:
  - `flora-api/src/routes/files.rs`: adicionados `ListFilesQuery`, `resolve_list_limit` (clamp `1..=100`, default `50`), handler `list_files` com `ensure_member` + validação de `workspace_id` + `ensure_workspace_in_org` (`flora-api/src/tenant.rs`), e 2 testes unitários em `files::tests`.
- **Contrato DB e trava lockstep (`T-002` / `#380`)**:
  - `flora-tests/src/contract/files.rs`: adicionado `db_list_files_scoped_to_workspace_and_org` com helpers locais `authed_json_request`, `upload_test_file`, `setup_extra_list_workspaces` e `list_workspace_files`, provando `422` sem `workspace_id`, `422` BOLA cross-org, isolamento entre dois workspaces da mesma org, ausência de `storage_path` no DTO e omissão pós-`DELETE`.
  - Atualizado em lockstep `FLOORS["files"] = 7` em `scripts/ci/check_gate_integrity.py` e `run_leg files contract::files::db_ 7` em `.github/workflows/rust.yml` (total `78` testes `#[ignore]`). Nenhum required check alterado.
- **Store, utilitário TDD e página `/files` (`T-003` / `#381`)**:
  - `apps/frontend/src/lib/utils/files.ts`: helper `filesQueryPath(workspaceId)` com `encodeURIComponent`.
  - `apps/frontend/src/lib/state/FileStore.svelte.ts`: métodos `setFiles(files)`, `clear()` e `addFile(file)` inserindo no topo (mais recente primeiro) com deduplicação por `id`.
  - `apps/frontend/tests/unit/FileStore.test.ts`: 4 testes unitários escritos em TDD (RED antes da implementação → GREEN `4 passed`).
  - `apps/frontend/src/lib/i18n/pt-BR.ts`: `'files.empty'` atualizado para `'Nenhum arquivo neste workspace.'` e novas chaves `'files.loading'`, `'files.refresh'`, `'error.loadFiles'`.
  - `apps/frontend/src/routes/(app)/files/+page.svelte`: carregamento automático em `onMount`, botão Atualizar, `fileStore.clear()` imediato na troca de workspace e guarda contra race condition via `loadSeq`.
- **Blindagem E2E (`T-004` / `#382`)**:
  - `apps/frontend/tests/e2e/seed-flow.e2e.ts`: Passo 5 verifica `t['files.empty']` visível (após resolução de `GET /api/v1/files?workspace_id=...`) antes de `auditAccessibility(page, '/files')`.
- **Briefing `tenant`, SDD e governança (`T-005` / `#383`)**:
  - Briefing de Compreensão em `docs/governance/2026-11-22-115-briefing-compreensao.md`, artefatos R4/R7 (MINOR `0.7.16` → `0.8.0`) e atualizações append-only com recontagem `find`.

## [NV]

- Execução remota de `db-tests` em Postgres 18, `e2e-browser` em Chromium, ACR independente, modo escuro em navegador real e staging/produção externos.

## Rollback

Novo commit de reversão dos arquivos do ciclo 115 (incluindo retorno do piso `files` `7 → 6` em lockstep se o teste de contrato for removido); sem alteração de schema/migrations e sem force-push.
