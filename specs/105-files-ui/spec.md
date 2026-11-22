# Feature Specification: 105 — Tela de arquivos e limite coerente

**Feature Branch**: `feature/105-files-ui`

**Created**: 2026-11-21

**Status**: Specified — `go` para implementação (decisão em `.specify/assessments/105-files-ui/decision.md`)

**Input**: roteiro de staging item 8 (`docs/governance/2026-11-21-roteiro-staging-flora.md:159-164`), inspeção determinística do baseline `c5e4f886e` (sem rota `/files`, sem link “Arquivos”, limite 50 MB no cliente × 10 MB no servidor) e decisão `go` do ciclo 105

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Alcançar os arquivos pela navegação (Priority: P1)

Como usuário autenticado, preciso ver um item “Arquivos” na navegação e abrir a tela de arquivos, para poder enviar e baixar arquivos do workspace sem conhecer a URL.

**Why this priority**: o item 8 do roteiro (`roteiro:159-164`) não pode ser executado porque `apps/frontend/src/routes/(app)/` não tem `files/` e `+layout.svelte:122-151` só oferece Canais, Mensagens diretas e Tarefas. Os componentes `FileUpload.svelte`, `FilePreview.svelte` e o `fileStore` (`apps/frontend/src/lib/state/FileStore.svelte.ts:3-16`) já existem, e o backend já responde (`flora-api/src/routes/files.rs:81-89,127-204,247-296`): falta apenas a ligação.

**Independent Test**: com uma sessão autenticada e um workspace selecionado, clicar em “Arquivos” na sidebar abre `/files`; a página mostra o seletor de workspace e o campo de envio, sem erro de navegação nem redirecionamento indevido.

**Acceptance Scenarios**:

1. **Given** uma sessão autenticada com organização selecionada, **When** a sidebar é renderizada, **Then** há um botão “Arquivos” (`nav.files`) após “Tarefas”, usando o mesmo padrão de ícone/classe dos demais itens.
2. **Given** o clique em “Arquivos”, **When** a navegação ocorre, **Then** `/files` é exibido com o seletor de workspace, o componente de upload e a lista (vazia no início).
3. **Given** a sidebar recolhida, **When** o botão “Arquivos” é exibido, **Then** apenas o ícone aparece, como nos botões existentes (`+layout.svelte:126-149`).
4. **Given** uma sessão sem autenticação ou sem organização, **When** `/files` é aberto diretamente, **Then** o guard da página (padrão de `tasks/+page.svelte:131-141`) redireciona para `/` ou `/select-org`, sem chamada de upload.

---

### User Story 2 — Enviar, listar e baixar um arquivo válido (Priority: P1)

Como usuário, preciso enviar um arquivo de até 10 MB para o workspace selecionado e vê-lo na lista com link de download, para completar o item 8 do roteiro.

**Why this priority**: `FileUpload.svelte:15-54` já monta o `FormData`, chama `POST /api/v1/files/upload?workspace_id=...` e notifica `onUploadComplete`; `FileStore.addFile` (`FileStore.svelte.ts:6-10`) deduplica por `id`; `FilePreview.svelte:7-9` aponta para `GET /api/v1/files/download/{id}`. A verificação ao vivo relatada obteve 201 no upload e 200 no download com `FLORA_STORAGE__LOCAL_PATH` gravável; o que falta é o consumidor desses blocos na página.

**Independent Test**: selecionar um workspace, enviar um arquivo pequeno e confirmar que o item aparece na lista com nome e link “Baixar” cujo destino é `/api/v1/files/download/{id}`.

**Acceptance Scenarios**:

1. **Given** `/files` com workspace selecionado, **When** um arquivo de 5 MB é escolhido, **Then** `FileUpload` envia o multipart e, com resposta 201 (`FileAttachment`), chama `fileStore.addFile`.
2. **Given** o upload concluído, **When** a lista é renderizada, **Then** cada item usa `FilePreview` com `fileName` e link `t['file.download']` para `/api/v1/files/download/{file.id}`.
3. **Given** a troca de workspace no seletor, **When** `onWorkspaceChange` roda, **Then** o `workspaceId` passado a `FileUpload` é atualizado e `authState.setWorkspace` é chamado, como em `tasks/+page.svelte:116-129`.
4. **Given** nenhum workspace selecionado, **When** um arquivo é escolhido, **Then** `FileUpload.svelte:24-27` exibe `file.workspaceRequired` e nenhuma requisição é feita.

---

### User Story 3 — Limite coerente e erro pt-BR na fronteira (Priority: P1)

Como usuário e mantenedor, preciso que o cliente rejeite arquivos acima de 10 MB antes do envio, mostrando “Arquivo muito grande...” em pt-BR, e que a fronteira seja provada por testes.

**Why this priority**: `files.ts:2` aceita 50 MB, mas `storage.rs:35-37,54` configura 10 MB e `file_service.rs:139-155` aplica o limite antes de gravar. Hoje um arquivo de 20 MB passa no cliente, falha no servidor e mostra `file.uploadFailed` (`FileUpload.svelte:49-52`), contrariando `roteiro:164` (“erro pt-BR ... nada é gravado”). A correção é uma constante exportada, um texto e uma tabela de testes.

**Independent Test**: chamar `isValidFileSize` com 10 MB exato e com 10 MB + 1 byte; verificar respectivamente `true` e `false`, que `MAX_UPLOAD_BYTES` vale `10 * 1024 * 1024` e que `file.tooLarge` contém “10 MB”.

**Acceptance Scenarios**:

1. **Given** `MAX_UPLOAD_BYTES = 10 * 1024 * 1024`, **When** `isValidFileSize` recebe exatamente esse valor, **Then** retorna `true` (limite inclusivo).
2. **Given** um arquivo com `byteSize = 10 * 1024 * 1024 + 1`, **When** ele é escolhido em `FileUpload`, **Then** `isValidFileSize` retorna `false`, `file.tooLarge` aparece com “O tamanho máximo é 10 MB.” e **nenhum `fetch` de upload é feito**.
3. **Given** 50 MB ou 100 MB, **When** `isValidFileSize` é chamado, **Then** retorna `false`; a antiga expectativa “≤ 50 MB” deixa de valer.
4. **Given** entradas `NaN`, `Infinity` ou negativas, **When** `isValidFileSize` é chamado, **Then** retorna `false`, sem lançar.
5. **Given** a suíte `tests/unit/fileUpload.test.ts`, **When** o projeto `server` do Vitest é executado, **Then** todos os casos passam com asserção explícita (`vite.config.ts:28,49`).

---

### Edge Cases

- Limite inclusivo: 10 MB exato é aceito; 10 MB + 1 byte é o primeiro valor recusado. `File.size` é inteiro e não negativo, mas a função também recusa `NaN`, `Infinity` e negativos.
- O número 10 MB aparece em três lugares (config Rust, constante TS e texto pt-BR); não há injeção de valor no dicionário neste ciclo — o teste do texto protege contra divergência futura.
- O texto “< 50 MB” do roteiro (`roteiro:161`) está desatualizado frente à configuração; permanece como dívida **[NV]** e não é editado.
- Erros do servidor fora da faixa de tamanho (por exemplo mime type não permitido, `file_service.rs:135-137`) continuam caindo em `file.uploadFailed` (`FileUpload.svelte:49-52`); mapeamento específico é dívida **[NV]**.
- A lista de arquivos existe apenas na sessão: `FileStore` é memória do navegador e `files.rs:81-89` não oferece listagem; recarregar limpa a lista sem apagar objetos no servidor.
- `FileStore.addFile` deduplica por `id` (`FileStore.svelte.ts:6-10`): o mesmo arquivo não é listado duas vezes.
- Sem workspace selecionado, `FileUpload.svelte:24-27` bloqueia antes do envio; a página deve garantir que o seletor esteja carregado antes de habilitar o envio.
- Nenhuma decisão de autenticação, autorização ou tenant passa a depender do seletor local: `enforceAuth` (`+layout.svelte:36-46`), o `$effect` de websocket (`:65-86`) e a checagem de membro no backend (`files.rs:134,253`) permanecem intocados.
- A nova chave `nav.files` é a única chave de i18n adicionada; `file.download`, `file.workspaceRequired` e `file.uploadFailed` são reaproveitadas sem alteração.
- Execução de testes/gates e validação em navegador real: **[NV]** neste artefato, registrada no handoff.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001 — rota `/files` com upload e lista da sessão**: `apps/frontend/src/routes/(app)/files/+page.svelte` DEVE ser criada como rota do grupo autenticado, seguindo `tasks/+page.svelte:41-51,116-141`: carregar workspaces via `GET /api/v1/workspaces`, preferir `authState.workspaceId` e chamar `setWorkspace`, aplicar guard de autenticação/organização no `onMount` e atualizar o workspace ao trocar o seletor. A página DEVE renderizar `FileUpload` com `workspaceId` e `onUploadComplete` ligado a `fileStore.addFile`, listar `fileStore.files` com `FilePreview` e reaproveitar `file.workspaceRequired`, `file.uploadFailed` e `file.download`. NÃO DEVE introduzir endpoint, chamada de rede ou estado global novo; a lista é da sessão (`FileStore.svelte.ts:4-16`, `files.rs:81-89`).

- **FR-002 — link “Arquivos” na navegação**: `apps/frontend/src/lib/i18n/pt-BR.ts:48-57` DEVE receber a chave nova `nav.files` com valor `'Arquivos'`. `apps/frontend/src/routes/(app)/+layout.svelte` DEVE adicionar um `Button href="/files"` com ícone de arquivo de `@lucide/svelte` (importado em `:7-19`) logo após o botão `/tasks` (`:142-149`), repetindo as classes condicionais de `isSidebarOpen`. Nenhum outro `href`, chave de navegação, `enforceAuth` (`:36-46`), `$effect` (`:57-86`) ou trecho do dropdown (`:168-216`) DEVE mudar; `TranslationKey` (`pt-BR.ts:188`) passa a incluir a chave automaticamente.

- **FR-003 — limite de 10 MB e mensagem pt-BR**: `apps/frontend/src/lib/utils/files.ts` DEVE exportar `MAX_UPLOAD_BYTES = 10 * 1024 * 1024` e `isValidFileSize(sizeBytes: number): boolean` DEVE retornar `true` somente para números finitos com `0 <= sizeBytes <= MAX_UPLOAD_BYTES`. O valor DEVE espelhar `flora-core/src/config/storage.rs:35-37,54` e o `min` aplicado em `flora-core/src/services/file_service.rs:139-155`. A entrada `file.tooLarge` de `pt-BR.ts:103` DEVE ser `'Arquivo muito grande. O tamanho máximo é 10 MB.'`; nenhuma outra chave DEVE ser alterada.

- **FR-004 — testes unitários de fronteira**: `apps/frontend/tests/unit/fileUpload.test.ts` DEVE usar `describe/it/expect` no padrão de `tests/unit/tasks-query.test.ts:1-21`, ser descoberto pelo projeto `server` (`vite.config.ts:49`, node) e conter ao menos uma asserção por teste (`:28`). Casos obrigatórios: 0 e 10 MB exato → `true`; 10 MB + 1 byte, 50 MB, 100 MB, negativo, `NaN` e `Infinity` → `false`; `MAX_UPLOAD_BYTES === 10 * 1024 * 1024`; `t['file.tooLarge']` contém `10 MB` e não contém `50 MB`. Nenhum teste de componente, rede ou navegador é necessário.

- **FR-005 — fronteira, dívidas e rastreabilidade**: o diff DEVE ficar restrito a `(app)/files/+page.svelte` (novo), `(app)/+layout.svelte`, `i18n/pt-BR.ts`, `utils/files.ts` e `tests/unit/fileUpload.test.ts`, além dos artefatos do ciclo 105. Dívidas DEVEM ser registradas como fora de escopo/**[NV]**: lista apenas em memória, ausência de mapeamento de erro genérico do servidor, número do limite duplicado no texto i18n e texto “< 50 MB” do roteiro. O diff NÃO DEVE tocar backend, gates, required checks, `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição; a factory `mdmv-dark-factory` é somente leitura e nenhum commit é feito.

### Key Entities

- **`/files`**: nova rota em `apps/frontend/src/routes/(app)/files/+page.svelte`, consumidora dos componentes de arquivo já existentes.
- **`FileUpload`**: `apps/frontend/src/lib/components/files/FileUpload.svelte:8-54`; prop `workspaceId`, callback `onUploadComplete`, guards de tamanho/workspace e POST multipart (inalterado).
- **`FilePreview`**: `apps/frontend/src/lib/components/files/FilePreview.svelte:5-27`; item da lista com nome e link para `/api/v1/files/download/{id}` (inalterado).
- **`fileStore`**: singleton em `apps/frontend/src/lib/state/FileStore.svelte.ts:3-16`; lista `FileAttachment` da sessão, deduplicada por `id`.
- **`FileAttachment`**: `apps/frontend/src/lib/types/models.ts:65-72`; contrato `{ id, fileName, mimeType, byteSize, rustFsUrl, uploadedAt }`, compatível com `FileAttachmentResponse` do backend (`files.rs:60-73`).
- **`MAX_UPLOAD_BYTES` / `isValidFileSize`**: `apps/frontend/src/lib/utils/files.ts`; limite de cliente alinhado a 10 MB e sua verificação pura.
- **`nav.files` / `file.tooLarge`**: `apps/frontend/src/lib/i18n/pt-BR.ts:48-57,101-105`; única chave nova e texto de erro corrigido.
- **`StorageConfig.max_upload_bytes`**: `flora-core/src/config/storage.rs:30-37,54`, aplicado em `flora-core/src/services/file_service.rs:139-155`; fonte da verdade do limite, não alterada.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001 — roteiro executável pela UI**: o link “Arquivos” abre `/files`; com workspace selecionado, o upload de arquivo ≤ 10 MB retorna 201, o item aparece na lista via `fileStore`/`FilePreview` e o link “Baixar” aponta para `GET /api/v1/files/download/{id}` (200 observado na entrada do ciclo; reexecução **[NV]**).
- **SC-002 — fronteira e “nada gravado”**: 10 MB exato é aceito; 10 MB + 1 byte é recusado antes do `fetch` com `Arquivo muito grande. O tamanho máximo é 10 MB.`, sem requisição e sem objeto gravado. A suíte Vitest prova os dois lados da fronteira, as entradas inválidas e o texto da mensagem.
- **SC-003 — escopo e segurança preservados**: nenhum endpoint, gate, required check, flag, schema, autorização, chave de i18n além de `nav.files`, rota além de `/files`, artefato de outro ciclo ou commit aparece no diff; auth/tenant/websocket permanecem intactos. Execução de testes/gates e validação em navegador são **[NV]** neste artefato e ficam no handoff.

## Assumptions

- O baseline é `c5e4f886e`; os números de linha citados se referem a esse commit e podem se deslocar após a implementação.
- O limite efetivo é 10 MB conforme `storage.rs:35-37,54` e `file_service.rs:139-155`; o “< 50 MB” do roteiro (`roteiro:161`) é texto desatualizado e não é alterado neste ciclo.
- A verificação ao vivo de upload 201/download 200 com `FLORA_STORAGE__LOCAL_PATH` gravável é entrada relatada ao ciclo; nenhuma requisição ao vivo foi feita neste artefato.
- `File.size` é um inteiro não negativo; a função ainda recusa `NaN`, `Infinity` e negativos para tornar o contrato explícito e testável.
- A lista da tela é da sessão por construção: não existe endpoint de listagem em `files.rs:81-89`, e `FileStore` não persiste. Isso é comportamento aceito, não pendência funcional do ciclo.
- `@lucide/svelte` já é dependência do frontend (`+layout.svelte:7-19`) e exporta ícones de arquivo; nenhuma dependência nova é adicionada.
- Svelte 5 roda em modo runes (`vite.config.ts:15-18`); a página usa `$state` e o padrão reativo já empregado em `/tasks`.
- Resolver a listagem persistente, o mapeamento de erros do servidor ou a injeção do limite no texto i18n exige trabalho próprio; nenhum dado de produção foi lido neste artefato.
