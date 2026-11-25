# Bug fix — authorized-file-download-401

**Data**: 2026-11-25 · **Ciclo**: 123 · **Branch**: `fix/123-authorized-file-download`

## Plano

1. Adicionar `ApiClient.downloadBlob(endpoint)` em `apps/frontend/src/lib/api/client.ts`: mesma resolução de origem e header `Authorization` de `request()`, `401 → authState.lockSession()` + `ApiError`, demais erros via `apiErrorFromResponse`, sucesso via `response.blob()`. Não reutilizar `request()` porque `parseResponseBody` consome o corpo como texto/JSON.
2. Em `FilePreview.svelte`, substituir o `<a download>` por um `<button type="button">` que chama o helper, cria `URL.createObjectURL(blob)`, dispara o download com `file.fileName` e revoga a URL após o clique; erro exibido via chave pt-BR `file.downloadFailed`.
3. Adicionar `file.downloadFailed` ao dicionário (`pt-BR.ts`) e usar `apiErrorMessage` para não mostrar texto cru do backend.
4. Cobrir com teste unitário do helper (`tests/unit/filesDownload.test.ts`) e com Passo 5b estendido em `seed-flow.e2e.ts`: conferir `200`, header `Authorization` na request e download com o nome esperado.

## Fora de escopo

Backend, migrations, gate floors, required checks, deploy e mudanças de política de sessão. Nenhuma credencial em fixture; o token de teste é sintético e redigido.

## Status

Implementado em `fix/123-authorized-file-download`: `downloadBlob` no ApiClient, botão com object URL em FilePreview, chave pt-BR e testes. Verificação local: check/lint/unit 2/2/gate 87 pisos PASS; E2E local bloqueado por provisionamento (3 tentativas documentadas). Evidência completa em `.specify/validations/123-authorized-file-download/evidence.md`.
