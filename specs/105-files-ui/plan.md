# Implementation Plan: 105 — Tela de arquivos e limite coerente

**Branch**: `feature/105-files-ui` @ `c5e4f886ee91125cfe1215c436c144476fe42415` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/105-files-ui/decision.md` (go), roteiro item 8 (`docs/governance/2026-11-21-roteiro-staging-flora.md:159-164`) e inspeção determinística do baseline `c5e4f886e`

## Summary

Ligar os blocos de arquivos já existentes e alinhar o limite do cliente ao do servidor: criar `/files` com seletor de workspace, upload via `FileUpload` e lista da sessão via `fileStore`/`FilePreview`; adicionar o link “Arquivos” com a chave `nav.files`; exportar `MAX_UPLOAD_BYTES = 10 MB` e corrigir a mensagem `file.tooLarge`; provar a fronteira com Vitest. Nenhuma mudança de backend, autorização, gate, required check, rota adicional, flag, schema, outro ciclo ou commit. Lista volátil, erro genérico do servidor, número duplicado no texto e “< 50 MB” do roteiro ficam registrados como dívida/**[NV]**.

## Technical Context

**Root cause**: não existe rota `files/` em `apps/frontend/src/routes/(app)/` nem link em `+layout.svelte:122-151`; `pt-BR.ts:48-57` não tem `nav.files`. Os componentes `FileUpload.svelte:8-54`, `FilePreview.svelte:5-27` e `FileStore.svelte.ts:3-16` estão prontos, mas sem consumidor. Em paralelo, `files.ts:2` fixa 50 MB enquanto `storage.rs:35-37,54` fixa 10 MB (`file_service.rs:139-155` aplica o limite), produzindo rejeição tardia e mensagem genérica (`FileUpload.svelte:49-52`).

**Target Path**: `apps/frontend/src/routes/(app)/files/+page.svelte` (novo), `apps/frontend/src/routes/(app)/+layout.svelte`, `apps/frontend/src/lib/i18n/pt-BR.ts`, `apps/frontend/src/lib/utils/files.ts` e `apps/frontend/tests/unit/fileUpload.test.ts`. Somente leitura: `flora-api/src/routes/files.rs`, `flora-core/src/config/storage.rs`, `flora-core/src/services/file_service.rs`, `apps/frontend/src/lib/components/files/*`, `apps/frontend/src/lib/state/FileStore.svelte.ts`, `apps/frontend/src/lib/types/models.ts`, `(app)/tasks/+page.svelte`, `apps/frontend/vite.config.ts`, `apps/frontend/package.json` e o roteiro.

**Testing**: projeto `server` do Vitest (`vite.config.ts:49`, node), `tests/unit/fileUpload.test.ts` ampliado com 0/10 MB exato → `true`, 10 MB + 1/50 MB/100 MB/negativo/`NaN`/`Infinity` → `false`, asserções de `MAX_UPLOAD_BYTES` e do texto `file.tooLarge`; `requireAssertions` em `:28`. Convenção `describe/it/expect` de `tests/unit/tasks-query.test.ts:1-21`. Sem componente, rede ou navegador. Gates do alvo no handoff **[NV]**.

**Constraints**: reaproveitar componentes e endpoints sem alterá-los; a página segue o padrão de `/tasks` (`tasks/+page.svelte:41-51,116-141`) para workspaces e guard; `enforceAuth` (`+layout.svelte:36-46`), os `$effect` (`:57-86`) e o dropdown (`:168-216`) não mudam; única chave i18n nova é `nav.files`; `file.download`, `file.workspaceRequired` e `file.uploadFailed` permanecem idênticos; sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição; sem commits.

## Steps

1. **Causa (T001)** — fixar em intake/research a cadeia do baseline `c5e4f886e`: item 8 do roteiro (`roteiro:159-164`), ausência de `/files`/`nav.files`, blocos órfãos e divergência 50 MB × 10 MB (`files.ts:2` × `storage.rs:35-37,54`). Registrar dívidas/**[NV]**. Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001..SC-003).
2. **Limite e mensagem (T002)** — em `utils/files.ts`, exportar `MAX_UPLOAD_BYTES = 10 * 1024 * 1024` e fazer `isValidFileSize` aceitar somente `Number.isFinite(sizeBytes) && sizeBytes >= 0 && sizeBytes <= MAX_UPLOAD_BYTES`. Trocar `file.tooLarge` (`pt-BR.ts:103`) para “Arquivo muito grande. O tamanho máximo é 10 MB.”; nenhuma outra chave muda (FR-003).
3. **Testes de fronteira (T003)** — ampliar `tests/unit/fileUpload.test.ts`: aprovados 0 e `MAX_UPLOAD_BYTES`; recusados `MAX_UPLOAD_BYTES + 1`, 50 MB, 100 MB, `-1`, `NaN`, `Infinity`; asserções da constante e de `t['file.tooLarge']` contendo `10 MB` e não `50 MB`. Projeto `server`, asserções explícitas (FR-004, SC-002).
4. **Rota (T004)** — criar `(app)/files/+page.svelte` no padrão de `/tasks`: `workspaces = $state<Workspace[]>([])`, `GET /api/v1/workspaces`, preferência por `authState.workspaceId` com `authState.setWorkspace`, `onWorkspaceChange`, guard auth/org no `onMount`. Renderizar `FileUpload` com `workspaceId={selectedWorkspaceId}` e `onUploadComplete={(file) => fileStore.addFile(file)}`; listar `fileStore.files` com `<FilePreview {file} />`. Sem endpoint, estado global ou chave de i18n adicional (FR-001, SC-001).
5. **Navegação (T005)** — adicionar `'nav.files': 'Arquivos'` na seção de navegação de `pt-BR.ts:48-57`; importar um ícone de arquivo de `@lucide/svelte` em `+layout.svelte:7-19` e inserir o `Button href="/files"` após `/tasks` (`:142-149`) com as mesmas classes condicionais. Nada mais no layout muda (FR-002).
6. **Dívidas (T006)** — registrar em assessment/spec/plan/tasks que a lista é apenas da sessão (`FileStore.svelte.ts:4-16`, sem listagem em `files.rs:81-89`), que erros não relacionados a tamanho continuam genéricos (`FileUpload.svelte:49-52`), que o número 10 MB está duplicado em config/helper/texto e que `roteiro:161` diz “< 50 MB”. Nenhum desses pontos é corrigido aqui; nenhum arquivo de backend ou roteiro é alterado.
7. **Validação documental (T007)** — conferir que `enforceAuth`, o `$effect` de websocket, a checagem de tenant no backend e os endpoints permanecem idênticos; que apenas `nav.files` foi acrescentada; que o guard de tamanho continua antes do `fetch` (`FileUpload.svelte:20-23`). Registrar `npm run test:unit`, `check`, `lint`, build, gates e validação em navegador no handoff **[NV]**, sem escrita em staging/produção e sem segredos em artefatos.
8. **Fecho (T008)** — rastreabilidade T001..T008 em decision/spec/plan/tasks, conferência da fronteira proibida e confirmação de que nenhum required check/gate, schema, flag, endpoint, autorização ou chave de i18n adicional foi introduzido e de que não houve commit (FR-005, SC-003).

## Constitution Check

Sem violações: a mudança é local ao frontend e não cria superfície de autorização, listagem ou contrato de API. O limite é apenas espelhado do servidor (`storage.rs:35-37,54`, `file_service.rs:139-155`), que continua sendo a autoridade sobre gravação e tenant (`files.rs:134,253`). `FileUpload`, `FilePreview`, `FileStore`, `models.ts` e os endpoints são reaproveitados sem alteração; `enforceAuth`, websocket, rotas existentes, gates, required checks, workflows, ADRs e constituição seguem idênticos; o diff fica restrito a um arquivo novo, quatro existentes e aos artefatos do ciclo 105.

## Project Structure

```text
.specify/assessments/105-files-ui/{intake,research,define,shape,decision}.md
specs/105-files-ui/
├── spec.md      # FR-001..FR-005 / SC-001..SC-003
├── plan.md      # T001..T008
└── tasks.md     # fila T001..T008 (a executar)
apps/frontend/src/routes/(app)/files/+page.svelte    # novo: seletor + upload + lista da sessão
apps/frontend/src/routes/(app)/+layout.svelte        # ícone + Button href="/files"
apps/frontend/src/lib/i18n/pt-BR.ts                  # nav.files (novo) + file.tooLarge (10 MB)
apps/frontend/src/lib/utils/files.ts                 # MAX_UPLOAD_BYTES exportado + guard
apps/frontend/tests/unit/fileUpload.test.ts         # fronteira de 10 MB + texto da mensagem
apps/frontend/src/lib/components/files/FileUpload.svelte  # reuso inalterado (guard antes do fetch)
apps/frontend/src/lib/components/files/FilePreview.svelte # reuso inalterado (link de download)
apps/frontend/src/lib/state/FileStore.svelte.ts      # reuso inalterado (lista da sessão)
flora-api/src/routes/files.rs                        # endpoints existentes (somente leitura)
flora-core/src/config/storage.rs                     # max_upload_bytes = 10 MB (somente leitura)
flora-core/src/services/file_service.rs              # aplicação do limite (somente leitura)
```

## Complexity Tracking

Nenhuma. A página reusa um padrão já presente em `/tasks`, os três componentes de arquivo e os endpoints existentes; a mudança de limite é uma constante exportada mais um texto, e os testes são uma tabela de casos. Não há dependência nova, estado global novo, chamada de rede nova, abstração compartilhada ou trabalho de backend. A repetição do número 10 MB em config/helper/texto é intencional e registrada como dívida: injetá-lo no dicionário exigiria redesenhar a i18n, fora do escopo.
