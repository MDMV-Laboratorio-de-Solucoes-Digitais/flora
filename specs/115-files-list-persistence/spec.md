# Feature Specification: Listagem persistida de arquivos por workspace

**Feature ID**: 115 · **Status**: specified · **Data**: 2026-11-23

## User Scenario

Como membro autenticado de uma organização no Flora, quero que os arquivos enviados para um workspace permaneçam visíveis na rota `/files` após recarregar a página ou alternar de workspace, para não perder acesso aos arquivos já persistidos no PostgreSQL.

## Acceptance Scenarios

1. **SC-001 (Endpoint `GET /api/v1/files`)**: **Given** usuário autenticado membro da organização, **when** chama `GET /api/v1/files?workspace_id=<uuid>[&limit=<n>]`, **then** recebe `200 OK` com `Vec<FileAttachmentResponse>` (`id`, `fileName`, `mimeType`, `byteSize`, `rustFsUrl`, `uploadedAt`, sem `storage_path`) ordenado por `created_at DESC`, com `limit` clamped em `1..=100` (default `50`); ausência de `workspace_id` ou workspace fora da organização retorna `422`.
2. **SC-002 (Contrato DB e trava lockstep)**: **Given** PostgreSQL 18 com migrações aplicadas, **when** `db_list_files_scoped_to_workspace_and_org` executa em `flora-tests/src/contract/files.rs`, **then** prova `422` sem `workspace_id`, `422` BOLA cross-org, isolamento estrito entre dois workspaces da mesma org e omissão após `DELETE`; o piso `files` sobe em lockstep `6 -> 7` (total `78`) em `check_gate_integrity.py` e `.github/workflows/rust.yml`.
3. **SC-003 (Store, helpers TDD e página `/files`)**: **Given** testes unitários TDD em `FileStore.test.ts`, **when** `filesQueryPath` e `FileStore` (`setFiles`, `clear`, `addFile` no topo + dedup) são executados e `/files` é montada/alterada, **then** a UI carrega arquivos do workspace selecionado via `GET /api/v1/files?workspace_id=...` no mount, na troca de workspace (com guarda `loadSeq`) e no botão Atualizar.
4. **SC-004 (E2E Chromium + WCAG 2.2 AA)**: **Given** o Passo 5 em `apps/frontend/tests/e2e/seed-flow.e2e.ts`, **when** navega para `/files` com a sessão rotacionada após `select-org`, **then** `t['files.empty']` fica visível após o `GET /api/v1/files?workspace_id=...` concluir e `auditAccessibility(page, '/files')` passa com zero violações WCAG 2.2 AA.
5. **SC-005 (Governança e entregas)**: **Given** ciclo tocando fronteira `tenant`, **when** fechado, **then** inclui Briefing de Compreensão (`2026-11-22-115-briefing-compreensao.md`), nota de governança, contagens append-only (`find`), roteiro/evidência, R4 `validate.md`, R7 `release.md` (MINOR `0.7.16` → `0.8.0`) e Code Review agêntico independente.

## Requirements

- **FR-001**: Adicionar `GET /api/v1/files` (`ListFilesQuery`, `resolve_list_limit`, `list_files`, testes unitários) em `flora-api/src/routes/files.rs`.
- **FR-002**: Adicionar `db_list_files_scoped_to_workspace_and_org` em `flora-tests/src/contract/files.rs` e atualizar `FLOORS["files"] = 7` / `run_leg files ... 7` (total `78`).
- **FR-003**: Atualizar `files.ts`, `FileStore.svelte.ts`, `pt-BR.ts`, `files/+page.svelte` e cobrir com `tests/unit/FileStore.test.ts` (TDD RED→GREEN).
- **FR-004**: Verificar resposta ao vivo em `seed-flow.e2e.ts` Passo 5 (`t['files.empty']` visível antes de `auditAccessibility(page, '/files')`).
- **FR-005**: Preencher Briefing de Compreensão (`tenant`), governança, docs append-only, roteiro, `evidence.md`, R4, R7 (`0.8.0`) e ACR independente.

## [NV]

Execução local com Postgres de staging real e tema escuro em navegador real permanecem `[NV]` (o job `db-tests` e `e2e-browser` no GitHub Actions validam Postgres 18 e Chromium limpo).
