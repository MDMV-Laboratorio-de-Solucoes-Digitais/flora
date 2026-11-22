# Tasks: 105 — Tela de arquivos e limite coerente

**Input**: `specs/105-files-ui/spec.md`, `plan.md`,
`.specify/assessments/105-files-ui/decision.md`, roteiro item 8 e inspeção do baseline `c5e4f886e`

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T008 (fila original; execução registrada no status abaixo); gates do alvo **PASS** no handoff. Os números de linha citados são do baseline `c5e4f886e`.
**Status de execução (2026-11-21):** T001–T008 executados e verificados; os `[ ]` abaixo são a fila original, não o estado corrente. Correções de lint registradas no `converge.md`.

## Phase 1 — Diagnóstico e artefatos de assessoria

- [ ] T001 Fixar em `.specify/assessments/105-files-ui/{intake,research}.md` a cadeia: roteiro item 8 (`docs/governance/2026-11-21-roteiro-staging-flora.md:159-164`) exige upload/preview/download e erro pt-BR “Arquivo muito grande...” com nada gravado; `apps/frontend/src/routes/(app)/` não tem `files/` e `+layout.svelte:122-151` não tem link “Arquivos” (nem `nav.files` em `pt-BR.ts:48-57`); `FileUpload.svelte:8-54`, `FilePreview.svelte:5-27` e `apps/frontend/src/lib/state/FileStore.svelte.ts:3-16` estão órfãos enquanto `flora-api/src/routes/files.rs:81-89,127-204,247-296` já responde; `files.ts:2` usa 50 MB contra 10 MB de `flora-core/src/config/storage.rs:35-37,54` aplicado em `flora-core/src/services/file_service.rs:139-155`. Registrar dívidas/**[NV]** (lista volátil, erro genérico, número duplicado no texto, “< 50 MB” do roteiro). Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).

## Phase 2 — Limite, mensagem e testes

- [ ] T002 Em `apps/frontend/src/lib/utils/files.ts`, exportar `export const MAX_UPLOAD_BYTES = 10 * 1024 * 1024;` e fazer `isValidFileSize(sizeBytes: number): boolean` retornar `true` somente para `Number.isFinite(sizeBytes) && sizeBytes >= 0 && sizeBytes <= MAX_UPLOAD_BYTES`. Em `apps/frontend/src/lib/i18n/pt-BR.ts:103`, trocar `file.tooLarge` para `'Arquivo muito grande. O tamanho máximo é 10 MB.'`. O valor espelha `storage.rs:35-37,54` e `file_service.rs:139-155`; nenhuma outra chave de i18n muda, nenhum backend é tocado (FR-003, SC-002).

- [ ] T003 Ampliar `apps/frontend/tests/unit/fileUpload.test.ts` com `describe/it/expect` e import relativo no padrão de `tests/unit/tasks-query.test.ts:1-21`. Aprovar 0 e `MAX_UPLOAD_BYTES` (10 MB exato); recusar `MAX_UPLOAD_BYTES + 1`, 50 MB, 100 MB, `-1`, `NaN` e `Infinity`; afirmar `MAX_UPLOAD_BYTES === 10 * 1024 * 1024` e que `t['file.tooLarge']` contém `10 MB` e não `50 MB`. Cada teste com asserção explícita (`vite.config.ts:28`), descoberto pelo projeto `server` (`:49`), sem componente, rede ou navegador (FR-004, SC-002).

## Phase 3 — Frontend: rota e navegação

- [ ] T004 Criar `apps/frontend/src/routes/(app)/files/+page.svelte` no padrão de `tasks/+page.svelte:41-51,116-141`: `workspaces` e `selectedWorkspaceId` em `$state`, `GET /api/v1/workspaces`, preferência por `authState.workspaceId` com `authState.setWorkspace`, `onWorkspaceChange` e guard de autenticação/organização no `onMount`. Renderizar `FileUpload` com `workspaceId={selectedWorkspaceId}` e `onUploadComplete={(file) => fileStore.addFile(file)}`, e listar `fileStore.files` com `<FilePreview {file} />` (nome e link “Baixar” para `/api/v1/files/download/{file.id}`). Reaproveitar `file.workspaceRequired`, `file.uploadFailed` e `file.download`; sem endpoint, chamada de rede, estado global ou chave de i18n nova (FR-001, SC-001).

- [ ] T005 Em `apps/frontend/src/lib/i18n/pt-BR.ts:48-57`, adicionar `'nav.files': 'Arquivos'`. Em `apps/frontend/src/routes/(app)/+layout.svelte`, importar um ícone de arquivo de `@lucide/svelte` em `:7-19` e inserir, após o botão `/tasks` (`:142-149`), um `Button` com `href="/files"`, o ícone e as mesmas classes condicionais de `isSidebarOpen`. Manter `enforceAuth` (`:36-46`), os `$effect` (`:57-86`), o dropdown (`:168-216`) e os outros `href` idênticos; `TranslationKey` (`pt-BR.ts:188`) passa a reconhecer a chave (FR-002).

## Phase 4 — Fecho

- [ ] T006 Registrar as dívidas: a lista de `/files` é apenas da sessão (`FileStore.svelte.ts:4-16`; `files.rs:81-89` não tem listagem), então recarregar limpa a lista sem apagar objetos; erros não relacionados a tamanho continuam genéricos em `file.uploadFailed` (`FileUpload.svelte:49-52`); o número 10 MB aparece em config, helper e texto; `roteiro:161` ainda diz “< 50 MB”. Tudo fora de escopo e **[NV]**; nenhum arquivo de backend ou do roteiro é alterado.

- [ ] T007 Conferir que o guard de tamanho continua antes do `fetch` (`FileUpload.svelte:20-23`), que o `fetch` usa o `workspaceId` selecionado (`:30-41`), que `enforceAuth` (`+layout.svelte:36-46`), o `$effect` de websocket (`:65-86`) e a checagem de tenant no backend (`files.rs:134,253`) permanecem idênticos e que apenas `nav.files` foi acrescentada ao dicionário. Registrar `npm run test:unit`, `check`, `lint`, build, gates, reexecução do roteiro ao vivo e validação em navegador no handoff **[NV]**; nenhuma requisição ou escrita em staging/produção e nenhum segredo em artefatos.

- [ ] T008 Fechar a rastreabilidade T001..T008 em decision/spec/plan/tasks, verificar a fronteira proibida (`analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog, constituição) e confirmar que nenhum required check/gate, schema, flag, endpoint, autorização ou chave de i18n adicional foi introduzido e que não houve commit (FR-005, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008. T002 depende de T001 (fonte da verdade do limite e texto fixados); T003 de T002 (testa a constante, o guard e a mensagem); T004 de T002 e T003 (a página consome o componente cujo contrato de limite já foi provado); T005 de T004 (o link deve apontar para a rota existente); T006 registra dívidas sem bloquear o código; T007 depende de T004–T006 (valida rota, testes e fronteira); T008 verifica T001–T007 em conjunto e consolida o rastro documental. Em todos os passos, `FileUpload`, `FilePreview`, `FileStore`, os endpoints de arquivos, `enforceAuth`, o websocket, `StorageConfig` e o roteiro permanecem inalterados. Execução de testes/gates, reexecução ao vivo e validação em navegador permanecem **[NV]** até o handoff.
