# Assess — shape: 115

**Data**: 2026-11-23

## Abordagem

1. **DTO reutilizado (`FileAttachmentResponse`)**: `GET /api/v1/files` retorna o mesmo `FileAttachmentResponse` já retornado por `POST /api/v1/files/upload` e esperado por `FilePreview.svelte` (`id`, `fileName`, `mimeType`, `byteSize`, `rustFsUrl`, `uploadedAt`), mantendo `storage_path` interno fora da resposta JSON.
2. **Fail-closed no `workspace_id`**: diferente de `/tasks` (que mantém retrocompatibilidade sem filtro), `/files` já exige seleção de workspace na UI e o repositório `find_by_workspace_id` exige `(organization_id, workspace_id)`. Exigir `workspace_id` na query retorna `422` (`Error::Validation`) se omitido e valida `ensure_workspace_in_org` antes de consultar o banco.
3. **Prevenção de race condition no frontend**: `loadSeq` monotônico em `files/+page.svelte` descarta respostas atrasadas se o usuário trocar rapidamente de workspace antes do `fetch` concluir.
4. **Lockstep de gate (ADR-0003)**: ao adicionar 1 novo teste `#[ignore]` em `flora-tests/src/contract/files.rs`, atualizar o piso `files` de `6` para `7` simultaneamente em `check_gate_integrity.py` e `.github/workflows/rust.yml` sem alterar os nomes dos 4 required checks.

## Riscos e mitigação

- **Risco**: `e2e-browser` detectar erro de autorização/rede em `GET /api/v1/files` ou falha WCAG no novo botão/estado de carregamento. **Mitigação**: `<Button variant="outline" size="sm">` já usa tokens contrastados e `seed-flow.e2e.ts` valida tanto `t['files.empty']` quanto `auditAccessibility(page, '/files')` no Chromium do CI.
