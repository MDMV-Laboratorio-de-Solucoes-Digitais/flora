# Concept: Listagem de arquivos fail-closed

- **Slug**: 115-files-list-persistence
- **Created**: 2026-11-25
- **Recommended option**: Opção B — DTO reutilizado + `workspace_id` obrigatório

## Options

### Option A — Listar tudo e filtrar no frontend
- **Sketch**: endpoint sem filtro e filtragem local no stores.
- **Appetite**: small
- **Trade-offs**: implementação rápida; vaza escopo entre workspaces e exige confiar no cliente.
- **Rabbit holes**: BOLA cross-org e auditoria de tenancy.

### Option B — DTO reutilizado + workspace obrigatório
- **Sketch**: `GET /api/v1/files?workspace_id=...` valida sessão/membership, retorna `FileAttachmentResponse` sem `storage_path`, e o frontend carrega no mount com guarda de concorrência.
- **Appetite**: medium
- **Trade-offs**: fonte de verdade determinística e tenant-safe; exige contrato DB e lockstep do piso `files`.
- **Rabbit holes**: race condition na troca de workspace e divergência de lockfile/piso.

### Option C — Listagem paginada genérica com S3/RustFS
- **Sketch**: abstração de storage com paginação e backend substituível.
- **Appetite**: large
- **Trade-offs**: prepara Sprout/Grove; complexidade desnecessária no Seed.
- **Rabbit holes**: contrato de storage, migrações e configuração de perfis.

## Recommendation

Opção B: reaproveita o DTO existente, mantém o escopo fail-closed e é verificável pelo contrato DB e pelo smoke (`define.md:5-11`; `shape.md:5-15`).

## Out of Scope (for the recommended option)

- Migrations/schema, required checks, listagem sem `workspace_id`, S3/RustFS e paginação avançada.

## Assumptions to Validate

- `FileAttachmentResponse` já tem os campos necessários e nunca inclui `storage_path` (`shape.md:5-7`).
- O piso `files` e o total do gate serão atualizados em lockstep no ciclo de implementação (`shape.md:15`).
