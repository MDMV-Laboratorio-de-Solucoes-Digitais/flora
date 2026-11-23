# Briefing de Compreensão — 115 / pendente (a atualizar após abertura do PR)

**Data**: 2026-11-23 · **Domínio crítico**: tenant
**PR**: pendente (a atualizar após abertura do PR) · **Autor**: orquestrador

## Intenção

Expor a rota autenticada `GET /api/v1/files?workspace_id=<uuid>[&limit=<n>]` em `flora-api/src/routes/files.rs` ligando o front-end `/files` ao repositório `PgFileRepository::find_by_workspace_id` já existente em `flora-core`, para que arquivos enviados persistam visíveis após reload da página ou alternância de workspace sem quebrar o isolamento multi-tenant.

## Invariantes

- **Antes**:
  - `flora-api/src/routes/files.rs` expunha apenas `POST /`, `POST /upload`, `GET /download/{id}`, `GET /{id}/download`, `GET /{id}` e `DELETE /{id}`; não havia endpoint `GET /api/v1/files`.
  - A página `apps/frontend/src/routes/(app)/files/+page.svelte` populava `fileStore` somente no callback `onUploadComplete`, perdendo todos os arquivos persistidos ao recarregar `/files` ou trocar de workspace.
  - O método `PgFileRepository::find_by_workspace_id` (`flora-core/src/repositories/file_repository.rs:41-59`) filtrava `WHERE organization_id = $1 AND workspace_id = $2 AND is_deleted = false ORDER BY created_at DESC LIMIT $3`, sem handler HTTP correspondente.

- **Depois**:
  - `create_files_router()` (`flora-api/src/routes/files.rs:91-99`) registra `.route("/", get(list_files).post(upload_file_metadata))`.
  - `list_files` (`flora-api/src/routes/files.rs:341-365`) exige em sequência: (1) sessão válida + pertencimento à organização ativa via `ensure_member(&auth, &state)` (`lines 101-112`), (2) presença de `workspace_id` (retorna `422 Error::Validation { field: "workspace_id", .. }` se ausente), e (3) validação explícita de propriedade `ensure_workspace_in_org(&state, org_id, workspace_id)` (`flora-api/src/tenant.rs:26-46`, retornando `422` indistinguível tanto para inexistente quanto para workspace de outra organização).
  - A consulta delegada a `state.file_service.list_files(org_id, workspace_id, Pagination { limit: resolve_list_limit(query.limit), offset: 0 })` usa `org_id` derivado da sessão (nunca de parâmetro do cliente) e retorna `Vec<FileAttachmentResponse>` (`to_attachment`, `lines 114-123`) sem `storage_path`.
  - Na UI (`apps/frontend/src/routes/(app)/files/+page.svelte:37-72`), a troca de workspace chama `fileStore.clear()` imediatamente antes de `await loadFiles(selectedWorkspaceId)` e descarta respostas antigas via contador monotônico `loadSeq`, impedindo vazamento visual de arquivos entre workspaces em respostas fora de ordem.

## Decisões implícitas

1. **Clamping de `limit` no handler (`1..=100`, default `50`)**: `resolve_list_limit` (`flora-api/src/routes/files.rs:331-338`) normaliza `None -> 50` e `Some(0) -> 1`, em vez de falhar com `422` ou usar o default interno `20` de `Pagination`, mantendo payloads previsíveis no Seed sem alteração de API para futuros clientes paginados.
2. **DTO `FileAttachmentResponse` sem `storage_path`**: reutiliza exatamente o mesmo contrato JSON de `POST /api/v1/files/upload` (`id`, `fileName`, `mimeType`, `byteSize`, `rustFsUrl`, `uploadedAt`), não expondo caminhos internos de storage nem campos extras.
3. **Cancelamento lógico no cliente (`loadSeq` + `fileStore.clear()`)**: em vez de depender de `AbortController`, respostas atrasadas têm seu `seq` comparado ao `loadSeq` atual; o trade-off é simplicidade com `ApiClient.request` sem alterações no cliente HTTP, assumindo ausência de paginação infinita nesta fatia.
4. **Sem alteração em RLS em runtime (`ADR-0005` / `ADR-0006`)**: o isolamento continua garantido pela camada de aplicação (`AuthExtractor`, `ensure_member`, `ensure_workspace_in_org` e cláusula `WHERE organization_id = $1 AND workspace_id = $2`).

## Comando de verificação

```bash
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets -- -D warnings && \
cargo test -p flora-api files::tests && \
python3 scripts/ci/check_gate_integrity.py
# Contrato DB no Postgres 18 (executado pelo job db-tests no CI):
cargo test -p flora-tests contract::files::db_files::db_list_files_scoped_to_workspace_and_org -- --ignored --test-threads=1
```

## Pergunta-teste ao SE

**Pergunta**: Se um membro autenticado da Organização A enviar `GET /api/v1/files?workspace_id=<UUID_DO_WORKSPACE_DA_ORG_B>`, em que ponto exato o fluxo falha e qual código/corpo HTTP deve ser devolvido?
**Resposta esperada**: Antes de chamar `state.file_service.list_files`, `list_files` executa `ensure_workspace_in_org(&state, org_id, workspace_id)` (`flora-api/src/tenant.rs:26-46`) comparando `workspace.organization_id != org_id`; como divergem, retorna `422 Unprocessable Entity` (`error: "validation"`, `field: "workspace_id"`, `message: "workspace not found"`), sem distinguir entre workspace inexistente e workspace de outro tenant e sem consultar `files`.

## [NV]

- Execução ao vivo do contrato DB em Postgres 18 e do smoke Chromium `e2e-browser` (verificados remotamente no CI do PR; aguardando conclusão após abertura do PR).
- Políticas `CREATE POLICY` de RLS em runtime permanecem desativadas nesta fase (`ADR-0005`/`ADR-0006`).
- Staging real com credenciais externas e modo escuro em navegador real.
