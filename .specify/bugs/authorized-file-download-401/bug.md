# Bug: download de arquivo em /files retorna 401

**Data**: 2026-11-25 · **Severidade**: alta (fluxo P0 de arquivos) · **Status**: aberto

## Reprodução

1. Autenticar no Flora e selecionar organização/workspace com ao menos um arquivo enviado.
2. Abrir `/files` e conferir o arquivo na lista.
3. Clicar em “Baixar” (`apps/frontend/src/lib/components/files/FilePreview.svelte:25-31`).

## Observado

O link é um `<a href="${API_BASE_URL}/api/v1/files/download/<id>" download>`: a navegação não inclui `Authorization`. O endpoint valida Bearer (`flora-api/src/extractors/auth.rs:116,120`; `flora-api/src/routes/files.rs:258`) e responde `401`. `ApiClient.request` já injeta o token e trata 401 (`apps/frontend/src/lib/api/client.ts:108-127`), mas não é usado nesse caminho.

Limitação já registrada em `docs/governance/2026-11-22-roteiro-staging-ciclos-102-109.md:31`.

## Esperado

Com sessão válida, o clique baixa exatamente o arquivo enviado (status 200, nome e bytes preservados) e uma resposta 401 continua acionando `authState.lockSession()` como nos demais clientes autenticados.
