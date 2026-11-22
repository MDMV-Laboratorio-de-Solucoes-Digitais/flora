# Converge: 105 — UI de arquivos escopada ao workspace

**Data**: 2026-11-22 · **Branch**: `feature/105-files-ui` @ `c5e4f886e`
**Modo**: artefato append-only.

## Entrada 1 — 2026-11-22 (implementação e convergência)

**Status: Converged — sem novas tarefas.**

### Mudanças entregues

- `(app)/files/+page.svelte` (novo): seletor de workspace com preseleção de `authState.workspaceId`, `FileUpload` com callback que alimenta `fileStore`, lista com `FilePreview`, estados de erro/sucesso e botão de retorno.
- `+layout.svelte`: link "Arquivos" na sidebar com ícone `Files` e `aria-label`.
- `lib/utils/files.ts`: `MAX_FILE_SIZE_BYTES = 10 MiB` exportado; `isValidFileSize` usa a constante.
- `pt-BR.ts`: `nav.files`, `files.title/description/workspace/empty/uploaded`, `error.loadWorkspaces`; `file.tooLarge` corrigido de 50 MB para 10 MB.
- `FilePreview.svelte`: URL de download construída com `API_BASE_URL` (antes relativa ao frontend).
- `tests/unit/fileUpload.test.ts`: casos de fronteira (exatamente 10 MiB aceito; +1 byte rejeitado; 100 MiB rejeitado).

### Decisões e correções

1. Lista apenas da sessão: backend não expõe listagem; registrado como dívida, sem novo endpoint.
2. Prettier aplicado à nova rota; eslint sem erros após a formatação.
3. Corrigido import de ícones e chave `error.loadWorkspaces` faltante apontados por revisão durante a implementação.
4. `FilePreview` passou a usar a origem da API — URL relativa anterior apontaria para o frontend.

### Verificação observada

- Gate do alvo PASS (10 legs / 77 pisos); `svelte-check` 0 erros; Vitest 14 arquivos / **50 testes**; prettier/eslint exit 0 nos arquivos alterados.
- Navegador real: rota `/files` renderiza, workspaces carregam, upload pequeno retorna 201 e aparece na lista com "Baixar"; arquivo de 11 MiB é rejeitado com "Arquivo muito grande. O tamanho máximo é 10 MB." e não entra na lista.

### Fronteira preservada

Nenhum toque em backend, gate, required checks, ADR, constituição, schema, flags ou rotas de API. Nenhuma credencial real registrada.
