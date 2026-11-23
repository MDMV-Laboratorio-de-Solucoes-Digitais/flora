# Assess — research: 115

**Data**: 2026-11-23 · **Cobertura idiomática**: pt-BR (UI/governança) + en (Rust/Axum/SQLx)

## Evidência local (`github/dev@07556dd0e`)

- `flora-core/src/traits/repository.rs:257-262`: `FileRepository::find_by_workspace_id(organization_id, workspace_id, pagination) -> Result<Page<File>>`.
- `flora-core/src/repositories/file_repository.rs:42-61`: consulta SQL já filtra `WHERE organization_id = $1 AND workspace_id = $2 AND is_deleted = false ORDER BY created_at DESC LIMIT $3` usando `pagination.resolved_limit()` (capado em `MAX_LIMIT = 100`, `flora-core/src/models/mod.rs:51-67`).
- `flora-core/src/services/file_service.rs:208-217`: `FileService::list_files` delega diretamente para `find_by_workspace_id`.
- `flora-api/src/routes/files.rs:82-89`: `create_files_router()` registra apenas `POST /`, `POST /upload`, `GET /download/{id}`, `GET /{id}/download`, `GET /{id}`, `DELETE /{id}` — sem `GET /`.
- `flora-api/src/routes/files.rs:101-110`: `to_attachment(&file)` converte `&File` em `FileAttachmentResponse` (`id`, `fileName`, `mimeType`, `byteSize`, `rustFsUrl`, `uploadedAt`) sem expor `storage_path`.
- `apps/frontend/src/lib/state/FileStore.svelte.ts:1-16`: possui apenas `files`, `addFile` (`push`, ordem inversa ao `ORDER BY created_at DESC` do backend) e `removeFile`.
- `apps/frontend/src/routes/(app)/files/+page.svelte:1-60`: carrega workspaces no `onMount`, mas não busca arquivos nem escuta troca de workspace.
- `scripts/ci/check_gate_integrity.py:48` e `.github/workflows/rust.yml:112`: piso atual de `files` é `6` (total `77`).
- `docs/governance/2026-11-22-briefing-compreensao.md`: ciclo toca rota escopada por `tenant` (`ensure_workspace_in_org`) — exige Briefing de Compreensão preenchido a partir de `.specify/templates/briefing-compreensao.md`.

## [NV]

Execução em staging real com arquivos pré-existentes de múltiplos usuários e deploy em produção permanecem `[NV]`.
