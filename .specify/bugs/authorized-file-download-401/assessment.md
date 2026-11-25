# Bug assessment — authorized-file-download-401

**Data**: 2026-11-25 · **Origem**: auditoria de superfície do ciclo 80/20 (código + `docs/governance/...102-109.md:31`)

## Achado confirmado

`FilePreview.svelte:8-9,25-31` constrói um `<a download>` apontando diretamente para o endpoint autenticado. O cliente HTTP canônico (`client.ts:104-127`) é o único caminho que injeta `Authorization: Bearer` e aciona `authState.lockSession()` em 401. Consequentemente, o botão “Baixar” existente não pode funcionar para nenhum usuário autenticado.

## Impacto

- **Consumidor**: membro autenticado que fez upload em `/files` e precisa recuperar o arquivo (fluxo P0 “upload/download real” de `OBJETIVO.md:28`).
- **Severidade**: alta — funcionalidade principal do módulo de arquivos está inutilizável na UI, embora o endpoint esteja correto.
- **Blast radius**: somente frontend; nenhuma mudança de API, gate, migration ou sessão é necessária.

## Decisão

Corrigir no ciclo 123 via R5: adicionar download autenticado ao `ApiClient` (blob), trocar o `<a>` por ação explícita com `URL.createObjectURL`/`revokeObjectURL`, e cobrir com teste unitário do helper e E2E do clique. Veredito: **fix now**; não é necessário abrir spec nova nem autorização do SE.
