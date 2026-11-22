# Decision: 105 — Tela de arquivos e limite coerente

- **Slug**: 105-files-ui
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/105-files-ui` @ `c5e4f886ee91125cfe1215c436c144476fe42415`
- **Âncora**: `.specify/feature.json` → `specs/105-files-ui`
- **Evidência**: roteiro item 8 + inspeção determinística do baseline `c5e4f886e`; upload 201/download 200 com `FLORA_STORAGE__LOCAL_PATH` gravável relatados como entrada do ciclo
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

O item 8 do roteiro (`docs/governance/2026-11-21-roteiro-staging-flora.md:159-164`) exige upload válido, preview/download e erro pt-BR “Arquivo muito grande...” com nada gravado. No baseline `c5e4f886e` isso não é executável pela interface:

1. `apps/frontend/src/routes/(app)/` não tem `files/`; `+layout.svelte:122-151` só oferece `/channels`, `/dms` e `/tasks`, e `pt-BR.ts:48-57` não tem `nav.files`.
2. Os blocos existem, mas não são usados: `FileUpload.svelte:8-54` (upload com guards e callback), `FilePreview.svelte:5-27` (nome + link de download), `apps/frontend/src/lib/state/FileStore.svelte.ts:3-16` (`fileStore`, sem consumidor).
3. O backend já fecha o ciclo: `flora-api/src/routes/files.rs:81-89,127-204,247-296` expõe upload e download; `FileAttachmentResponse` (`:60-73`) gera `rust_fs_url = /api/v1/files/download/{id}` (`:110`). Upload 201/download 200 foram observados na entrada do ciclo com armazenamento local gravável.
4. O limite do cliente é maior que o do servidor: `apps/frontend/src/lib/utils/files.ts:2` usa 50 MB; `flora-core/src/config/storage.rs:35-37,54` configura 10 MB e `flora-core/src/services/file_service.rs:139-155` aplica o mínimo antes de gravar. Entre 10 e 50 MB o `fetch` é feito, o servidor recusa e `FileUpload.svelte:49-52` mostra `file.uploadFailed`, não a mensagem do roteiro.
5. `apps/frontend/tests/unit/fileUpload.test.ts:1-12` cobre apenas 10 bytes e 100/51 MB; a fronteira de 10 MB não é provada.

Não há falha intermitente: é uma lacuna de ligação entre peças existentes mais uma constante desatualizada no cliente.

## Decisão

Registrar o ciclo 105 como **Tela de arquivos e limite coerente**, com cinco compromissos rastreáveis (FR-001..FR-005), três critérios (SC-001, SC-002, SC-003) e oito tarefas (T001, T002, T003, T004, T005, T006, T007, T008; fila de implementação neste artefato):
1. **Rota `/files` (FR-001 / T004)**: criar `apps/frontend/src/routes/(app)/files/+page.svelte` no padrão de `/tasks` (`tasks/+page.svelte:41-51,116-141`): workspaces via `GET /api/v1/workspaces`, preferência `authState.workspaceId`, guard auth/org, `FileUpload` ligado a `fileStore.addFile` e lista `fileStore.files` via `FilePreview`. Sem endpoint ou estado global novo; a lista é da sessão.
2. **Navegação (FR-002 / T005)**: nova chave `nav.files: 'Arquivos'` em `pt-BR.ts:48-57` e um `Button href="/files"` com ícone `@lucide/svelte` após `/tasks` (`+layout.svelte:142-149`), com o mesmo padrão de sidebar recolhida. `enforceAuth` (`:36-46`), os `$effect` (`:57-86`) e o dropdown (`:168-216`) permanecem idênticos; `TranslationKey` (`pt-BR.ts:188`) reconhece a chave automaticamente.
3. **Limite e mensagem (FR-003 / T002)**: exportar `MAX_UPLOAD_BYTES = 10 * 1024 * 1024` em `utils/files.ts`, aceitar `0 <= sizeBytes <= MAX_UPLOAD_BYTES` com `Number.isFinite` e trocar `file.tooLarge` (`pt-BR.ts:103`) para “Arquivo muito grande. O tamanho máximo é 10 MB.” — espelhando `storage.rs:35-37,54` e `file_service.rs:139-155`. Nenhuma outra chave muda.
4. **Fronteira testada (FR-004 / T003)**: ampliar `tests/unit/fileUpload.test.ts` com 0 e 10 MB exato aprovados, 10 MB + 1, 50 MB, 100 MB, negativo/`NaN`/`Infinity` recusados, e asserções da constante e do texto; projeto `server` do Vitest (`vite.config.ts:49`) com `requireAssertions` (`:28`).
5. **Fronteira e dívida (FR-005 / T001, T006, T007, T008)**: diff restrito a cinco arquivos de implementação e aos artefatos do ciclo 105; dívidas registradas como fora de escopo/**[NV]** (lista volátil, erro genérico do servidor, número duplicado no texto, “< 50 MB” do roteiro); sem backend, gates, required checks, `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição; factory somente leitura; sem commit.
## Evidência de entrada

- Baseline `c5e4f886e`, branch `feature/105-files-ui`; `.specify/feature.json` aponta para `specs/105-files-ui` (cujo `spec.md` era modelo e é substituído por spec real).
- `docs/governance/2026-11-21-roteiro-staging-flora.md:159-164` — item 8 e o esperado em `:164`.
- `apps/frontend/src/routes/(app)/+layout.svelte:7-19,122-151` e `apps/frontend/src/lib/i18n/pt-BR.ts:48-57,101-105,188` — sem link, sem rota, limite no texto.
- `apps/frontend/src/lib/components/files/FileUpload.svelte:8-54`, `FilePreview.svelte:5-27`, `apps/frontend/src/lib/state/FileStore.svelte.ts:3-16`, `apps/frontend/src/lib/types/models.ts:65-72` — blocos prontos.
- `flora-api/src/routes/files.rs:60-73,81-89,104-113,127-204,247-296`, `flora-core/src/config/storage.rs:30-37,54`, `flora-core/src/services/file_service.rs:139-155` — contrato e limite efetivo, sem alteração.
- `apps/frontend/src/routes/(app)/tasks/+page.svelte:41-51,116-141`, `apps/frontend/tests/unit/fileUpload.test.ts:1-12`, `apps/frontend/vite.config.ts:27-53` — padrões de página e de teste.

## Racional

**go.** A lacuna é de ligação e de coerência, não de capacidade: o servidor já recebe e devolve arquivos (201/200 observados), os componentes já implementam guards, callback e download, e a página `/tasks` já mostra como selecionar workspace e proteger a rota. Uma página nova, um link e uma constante alinhada a 10 MB fecham o item 8 sem tocar em autorização, tenant, schema ou contrato de API. O guard de tamanho roda antes do `fetch` (`FileUpload.svelte:20-23`), então a fronteira de 10 MB garante a mensagem pt-BR e “nada gravado” sem depender do erro do servidor; os testes de fronteira tornam isso verificável. Registrar a lista como volátil e o erro genérico como dívida mantém o ciclo honesto: não inventamos endpoint de listagem nem mapeamento de erro só para embelezar a tela. A única chave nova de i18n é a do rótulo do link; `enforceAuth`, websocket, gates e required checks permanecem idênticos.

## [NV]

- Reexecução de upload/download ao vivo e execução de `npm run test:unit`, `check`, `lint`, build e gates: **[NV]** neste artefato — a verificação ao vivo é entrada relatada; consolidação no handoff.
- Aparência de `/files`, do link “Arquivos” e da mensagem em navegador real: **[NV]** — validada por construção sobre os padrões existentes, não medida aqui.
- Listagem persistente de arquivos por workspace, mapeamento dos erros genéricos do servidor e injeção do limite no texto i18n: **[NV]** / dívida — fora do ciclo 105.
- Números de linha referem-se a `c5e4f886e`; nenhum segredo, token real ou dado de produção foi lido ou registrado neste ciclo.

## Fora de âmbito

Mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*` ou `AGENTS.md`; alteração de required checks, gates, workflows, ADRs, constituição ou changelog; mudança em `flora-api/src/routes/files.rs`, `flora-core/src/config/storage.rs`, `flora-core/src/services/file_service.rs`, `AuthState`, chat, DMs, busca, notificações, rotas além de `/files`, flags, schema ou migrações; edição do roteiro; deploy, escrita em staging/produção e commits.

## Handoff

`/speckit.specify` — especificar a tela de arquivos como requisitos verificáveis (FR-001..FR-005 / SC-001..SC-003) e fila T001..T008: rota `/files` com seletor de workspace e lista da sessão, link “Arquivos” com `nav.files`, limite de 10 MB alinhado ao servidor com mensagem pt-BR e testes de fronteira. Testes, gates, reexecução ao vivo e validação em navegador ficam no handoff **[NV]**.
