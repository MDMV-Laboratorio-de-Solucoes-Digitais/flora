# Assess — intake: 115 — listagem persistida de arquivos por workspace

**Data**: 2026-11-23 · **Origem**: lacuna #1 do núcleo Seed verificada em `github/dev@07556dd0e`

## Problema

Em `flora-core/src/repositories/file_repository.rs` e `flora-core/src/services/file_service.rs`, `PgFileRepository::find_by_workspace_id` e `FileService::list_files` já estão implementados com filtro por `organization_id`, `workspace_id`, `is_deleted = false` e ordenação `created_at DESC`. Porém `flora-api/src/routes/files.rs` não expõe nenhuma rota `GET /` em `/api/v1/files`, e `apps/frontend/src/routes/(app)/files/+page.svelte` mantém arquivos apenas no `FileStore` em memória: ao recarregar `/files` ou alternar de workspace, a lista desaparece completamente.

## Pedido

Expor `GET /api/v1/files?workspace_id=<uuid>[&limit=<n>]` protegido por `AuthExtractor` + `ensure_member` + `ensure_workspace_in_org`, adicionar contrato DB-backed em `flora-tests/src/contract/files.rs` com atualização lockstep de piso (`files: 6 -> 7`, total `78`), e carregar a lista persistida na rota `/files` (mount, troca de workspace e atualização manual), estendendo `seed-flow.e2e.ts` para comprovar a resolução ao vivo em Chromium.

## Fora de escopo

Rota de remoção na UI, paginação infinita por cursor, alterações de schema/migration SQL, edição de required checks ou alteração de regras de deploy.
