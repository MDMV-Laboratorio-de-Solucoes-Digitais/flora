# Decision: 105 — Tela de arquivos e limite coerente

- **Slug**: 105-files-ui
- **Decided**: 2026-11-25
- **Verdict**: go (decisão original 2026-11-21, preservada)
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (anterior) | roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md:159-164` | upload 201/download 200 relatados como entrada | spec/plan/tasks 105

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Item 8 do roteiro exige upload, preview/download e erro pt-BR com nada gravado, mas não há rota `/files` nem link, e o cliente aceita 50 MB contra 10 MB do servidor (`roteiro:159-164`; `files.ts:2`; `storage.rs:35-37,54`). |
| Evidence strength | adequate | Inspeção determinística do baseline `c5e4f886e` e upload 201/download 200 relatados como entrada do ciclo; reexecução ao vivo, gates e navegador **[NV]** (`intake.md`; `research.md:1-13`). |
| Value vs. inaction | strong | Liga componentes e endpoints órfãos e alinha a mensagem ao limite real sem tocar em autorização, tenant, schema ou contrato de API (`decisão anterior:42`). |
| Feasibility / appetite | adequate | Uma página no padrão de `/tasks`, um link com nova chave pt-BR, uma constante e testes de fronteira; sem endpoint, estado global ou dependência nova (`shape.md`; `decisão anterior:25-30`). |
| Strategic fit | strong | Backend continua autoridade sobre limite/tenant/gravação; `enforceAuth`, websocket e padrão de sidebar preservados; serviço obrigatório novo inexistente (`define.md:FR-001..FR-005`). |
| Risk posture | adequate | Guard de tamanho roda antes do `fetch` e o servidor aplica `min(...)` antes de gravar, garantindo "nada gravado"; lista volátil e erro genérico registrados como dívida/**[NV]** (`decisão anterior:42,46-49`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**; nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. Registro original de 2026-11-21, branch `feature/105-files-ui` @ `c5e4f886ee91125cfe1215c436c144476fe42415`, âncora `.specify/feature.json` → `specs/105-files-ui` (cujo `spec.md` modelo é substituído por spec real), evidência do roteiro item 8 e inspeção determinística do baseline `c5e4f886e`, com upload 201/download 200 e `FLORA_STORAGE__LOCAL_PATH` gravável relatados como entrada do ciclo.

Causa determinística e dupla: `apps/frontend/src/routes/(app)/` não tem `files/`, `+layout.svelte:122-151` só oferece `/channels`, `/dms`, `/tasks` e `pt-BR.ts:48-57` não tem `nav.files`; os blocos existem mas não são usados — `FileUpload.svelte:8-54` (guards, callback), `FilePreview.svelte:5-27` (nome + link de download), `FileStore.svelte.ts:3-16` (`fileStore`, sem consumidor). O backend já fecha o ciclo: `flora-api/src/routes/files.rs:81-89,127-204,247-296` expõe upload/download, `FileAttachmentResponse` (`:60-73`) gera `rust_fs_url = /api/v1/files/download/{id}` (`:110`). O limite do cliente é maior que o do servidor: `apps/frontend/src/lib/utils/files.ts:2` usa 50 MB, enquanto `flora-core/src/config/storage.rs:35-37,54` fixa 10 MB e `flora-core/src/services/file_service.rs:139-155` aplica o mínimo antes de gravar; entre 10 e 50 MB o `fetch` é feito, o servidor recusa e `FileUpload.svelte:49-52` mostra `file.uploadFailed`, não a mensagem do roteiro. `tests/unit/fileUpload.test.ts:1-12` cobre apenas 10 bytes e 100/51 MB, sem provar a fronteira de 10 MB. Não há falha intermitente: é lacuna de ligação mais constante desatualizada.

| Compromisso rastreável | Conteúdo preservado |
|---|---|
| FR-001 / T004 — rota `/files` | Criar `apps/frontend/src/routes/(app)/files/+page.svelte` no padrão de `/tasks` (`tasks/+page.svelte:41-51,116-141`): workspaces via `GET /api/v1/workspaces`, preferência `authState.workspaceId`, guard auth/org, `FileUpload` ligado a `fileStore.addFile`, lista `fileStore.files` via `FilePreview`; sem endpoint ou estado global novo (lista da sessão). |
| FR-002 / T005 — navegação | Nova chave `nav.files: 'Arquivos'` em `pt-BR.ts:48-57` e `Button href="/files"` com ícone `@lucide/svelte` após `/tasks` (`+layout.svelte:142-149`), mesmo padrão de sidebar recolhida; `enforceAuth` (`:36-46`), `$effect` (`:57-86`) e dropdown (`:168-216`) intactos; `TranslationKey` (`:188`) reconhece a chave. |
| FR-003 / T002 — limite e mensagem | Exportar `MAX_UPLOAD_BYTES = 10 * 1024 * 1024` em `utils/files.ts`, aceitar `0 <= sizeBytes <= MAX_UPLOAD_BYTES` com `Number.isFinite`; trocar `file.tooLarge` (`pt-BR.ts:103`) para "Arquivo muito grande. O tamanho máximo é 10 MB.", espelhando `storage.rs:35-37,54` e `file_service.rs:139-155`; nenhuma outra chave muda. |
| FR-004 / T003 — fronteira testada | Ampliar `tests/unit/fileUpload.test.ts` com 0 e 10 MB exato aprovados, 10 MB + 1, 50 MB, 100 MB, negativo/`NaN`/`Infinity` recusados e asserções da constante e do texto; projeto `server` do Vitest (`vite.config.ts:49`) com `requireAssertions` (`:28`). |
| FR-005 / T001, T006–T008 — fronteira/dívida | Diff restrito a cinco arquivos de implementação e artefatos do ciclo 105; dívidas fora de escopo/**[NV]** (lista volátil, erro genérico do servidor, número duplicado no texto, "< 50 MB" do roteiro); sem backend, gates, `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, workflows, ADRs, changelog ou constituição; factory somente leitura; sem commit. |

Critérios preservados: **SC-001, SC-002, SC-003** (upload válido, erro pt-BR "Arquivo muito grande..." com nada gravado, download funcionando e fronteira/artefatos preservados). Fila **T001–T008**. Racional histórico: a lacuna é de ligação e coerência, não de capacidade — o servidor já recebe e devolve arquivos (201/200 observados), os componentes já implementam guards/callback/download e `/tasks` mostra como selecionar workspace e proteger a rota; uma página, um link e uma constante alinhada a 10 MB fecham o item 8 sem tocar em autorização, tenant, schema ou contrato de API; o guard roda antes do `fetch` (`FileUpload.svelte:20-23`), garantindo mensagem pt-BR e "nada gravado" sem depender do erro do servidor; a única chave nova de i18n é o rótulo do link.

**[NV]** preservados: reexecução de upload/download ao vivo e execução de `npm run test:unit`, `check`, `lint`, build e gates (a verificação ao vivo é entrada relatada; consolidação no handoff); aparência de `/files`, do link "Arquivos" e da mensagem em navegador real (validada por construção); listagem persistente por workspace, mapeamento de erros genéricos e injeção do limite no texto i18n (dívida, fora do ciclo); números de linha referem-se a `c5e4f886e`; nenhum segredo, token real ou dado de produção lido ou registrado.

Fora de âmbito preservado: mudanças em `analyze.md`, `converge.md`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, required checks, workflows, ADRs, constituição ou changelog; mudança em `files.rs`, `storage.rs`, `file_service.rs`, `AuthState`, chat, DMs, busca, notificações, rotas além de `/files`, flags, schema ou migrações; edição do roteiro; deploy, escrita em staging/produção e commits.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante registrada nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: O item 8 do roteiro não pode ser executado pela interface — não há rota `/files` nem link "Arquivos", os blocos de upload/preview/store estão órfãos e o cliente aceita 50 MB contra os 10 MB efetivos do servidor, exibindo erro genérico em vez da mensagem pt-BR.
- **Chosen approach**: Ligar blocos existentes: página `/files` no padrão de `/tasks`, link `nav.files`, `MAX_UPLOAD_BYTES = 10 * 1024 * 1024` espelhando a config e `file.tooLarge` corrigido, com testes de fronteira no Vitest.
- **In scope / out of scope**:
  - **In**: FR-001..FR-005 / SC-001..SC-003 e fila T001..T008; `files/+page.svelte` (novo), `+layout.svelte`, `pt-BR.ts`, `utils/files.ts`, `fileUpload.test.ts`; dívidas registradas.
  - **Out/NV**: backend (`files.rs`, `storage.rs`, `file_service.rs`), `AuthState`, chat/DMs/busca/notificações, listagem persistente, mapeamento de erro, edição do roteiro, rotas/flags/schema/migrações; `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog, constituição; reexecução ao vivo, testes/gates e navegador **[NV]**.
- **Success metrics**: upload válido (inclusive 10 MB exato) e download 200 pela tela; 10 MB + 1, 50/100 MB, negativo/`NaN`/`Infinity` recusados no cliente com "Arquivo muito grande. O tamanho máximo é 10 MB." e nada gravado; fronteira provada por asserções da constante e do texto.
- **Carried-forward open questions**: `[NV]` upload/download ao vivo, `npm run test:unit`/`check`/`lint`/build/gates e aparência em navegador; `[NV]` listagem persistente, erro genérico do servidor, número duplicado no i18n e texto "< 50 MB" do roteiro — nenhuma bloqueia o `go`.
