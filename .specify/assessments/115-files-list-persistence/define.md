# Assess — define: 115

**Data**: 2026-11-23

## Requisitos funcionais

- **FR-001**: Expor `GET /api/v1/files?workspace_id=<uuid>[&limit=<n>]` em `flora-api/src/routes/files.rs` validando sessão/membership (`ensure_member`), obrigatoriedade de `workspace_id` (`422` se ausente), pertencimento à organização (`ensure_workspace_in_org` → `422` em BOLA/inexistente), clamp de `limit` (`default 50`, `1..=100`) e resposta `Json<Vec<FileAttachmentResponse>>`, com testes unitários Rust.
- **FR-002**: Adicionar contrato DB-backed `db_list_files_scoped_to_workspace_and_org` em `flora-tests/src/contract/files.rs` cobrindo param ausente (`422`), BOLA cross-org (`422`), isolamento entre workspaces da mesma org, exclusão de soft-deleted e ausência de `storage_path`, atualizando o piso lockstep `files: 6 -> 7` (total `78`) em `check_gate_integrity.py` e `.github/workflows/rust.yml`.
- **FR-003**: Atualizar `apps/frontend/src/lib/utils/files.ts` (`filesQueryPath`), `FileStore.svelte.ts` (`setFiles`, `clear`, `addFile` no topo + dedup), `pt-BR.ts` (`files.empty`, `files.loading`, `files.refresh`, `error.loadFiles`) e `apps/frontend/src/routes/(app)/files/+page.svelte` (carga automática no mount, troca de workspace com guarda de concorrência `loadSeq` e botão atualizar), com TDD em `tests/unit/FileStore.test.ts`.
- **FR-004**: Estender Passo 5 em `apps/frontend/tests/e2e/seed-flow.e2e.ts` para verificar que o carregamento inicial de `/files` (`GET /api/v1/files?workspace_id=...`) conclui com `t['files.empty']` visível antes da auditoria `auditAccessibility(page, '/files')`.
- **FR-005**: Entregar Briefing de Compreensão (`tenant`), nota de governança, recontagens append-only (`STATE.md`, `PROGRESSO.md`, `research/index.md`, `OBJETIVO.md`, `ROADMAP.md`), roteiro + `evidence.md`, R4 (`validate.md`), R7 (`release.md` MINOR `0.7.16 -> 0.8.0`) e ACR independente.

## Requisitos não funcionais

- Rigor Rust (`missing_docs`, `unwrap_used`, `expect_used`, `panic`, `clippy::pedantic` = `deny`) e Frontend (`svelte-check` 0 erros, `eslint`/`prettier` exit 0, `no-console: error`, WCAG 2.2 AA sem achados).
