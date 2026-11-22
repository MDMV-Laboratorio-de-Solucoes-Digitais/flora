# Intake: 105 — Tela de arquivos e limite coerente

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/105-files-ui@c5e4f886ee91125cfe1215c436c144476fe42415` · `.specify/feature.json` → `specs/105-files-ui`

## Sinal

O item 8 do roteiro de staging (`docs/governance/2026-11-21-roteiro-staging-flora.md:159-164`) pede upload de arquivo pequeno, preview/download e, acima do limite, erro pt-BR “Arquivo muito grande...” com nada gravado. No baseline `c5e4f886e` esse roteiro não pode ser executado pela interface: não existe rota `/files` nem link “Arquivos” na navegação. Os blocos já estão prontos e órfãos — `FileUpload.svelte`, `FilePreview.svelte`, `FileStore.svelte.ts` (`apps/frontend/src/lib/state/FileStore.svelte.ts`) e os endpoints `POST /api/v1/files/upload` / `GET /api/v1/files/download/{id}` (`flora-api/src/routes/files.rs:81-89,127-204,247-296`). A verificação ao vivo relatada como entrada do ciclo confirma que o backend funciona: upload 201 e download 200 com `FLORA_STORAGE__LOCAL_PATH` gravável; o que falta é superfície de UI e coerência do limite.

Há uma segunda inconsistência, silenciosa e mais grave: `apps/frontend/src/lib/utils/files.ts:1-4` aceita até 50 MB, enquanto `flora-core/src/config/storage.rs:35-37,54` fixa `max_upload_bytes = 10 * 1024 * 1024`, aplicado em `flora-core/src/services/file_service.rs:139-155`. Um arquivo entre 10 e 50 MB passa no cliente, chega ao servidor e falha com erro genérico, contrariando a expectativa de mensagem pt-BR e de “nada gravado” do roteiro. A correção é exclusivamente de frontend: alinhar o limite e a mensagem ao valor já configurado, sem tocar no backend.

## Evidência

```
ROTEIRO (entrada do ciclo):
docs/governance/2026-11-21-roteiro-staging-flora.md:159-164  item 8 "Arquivos";
:161  upload pequeno "< 50 MB" (texto desatualizado frente ao limite configurado);
:162  preview/download; :163  upload acima do limite;
:164  esperado: erro pt-BR "Arquivo muito grande..."; nada é gravado.

SEM ROTA / SEM LINK:
apps/frontend/src/routes/(app)/                apenas +layout.svelte, channels/, dms/,
                                              select-org/, settings/, tasks/ — nenhum files/;
apps/frontend/src/routes/(app)/+layout.svelte:7-19   ícones importados (Hash, MessageSquare,
                                              CheckSquare, Settings ...), nenhum de arquivo;
apps/frontend/src/routes/(app)/+layout.svelte:122-151  grupo de navegação com /channels (:126-133),
                                              /dms (:134-141) e /tasks (:142-149); sem /files;
apps/frontend/src/lib/i18n/pt-BR.ts:48-57     seção "Navegação / layout do app" sem nav.files;
apps/frontend/src/lib/i18n/pt-BR.ts:188       TranslationKey = keyof typeof t.

BLOCOS JÁ PRESENTES (reuso, sem implementação nova de backend):
apps/frontend/src/lib/components/files/FileUpload.svelte:8-13    props onUploadComplete/workspaceId;
apps/frontend/src/lib/components/files/FileUpload.svelte:15-54   handleUpload: guard de tamanho
                                              (:20-23), guard de workspace (:24-27), FormData +
                                              POST /api/v1/files/upload?workspace_id=... (:30-41),
                                              callback (:43-45) e catch (:49-52);
apps/frontend/src/lib/components/files/FilePreview.svelte:5-9    file + GET /api/v1/files/download/{id};
apps/frontend/src/lib/components/files/FilePreview.svelte:12-27  nome, link "Baixar" (t['file.download']);
apps/frontend/src/lib/state/FileStore.svelte.ts:3-16   files $state, addFile dedup por id, removeFile,
                                              singleton fileStore — sem consumidor hoje;
apps/frontend/src/lib/types/models.ts:65-72  FileAttachment { id, fileName, mimeType, byteSize,
                                              rustFsUrl, uploadedAt };
apps/frontend/src/lib/i18n/pt-BR.ts:101-105   file.download, file.tooLarge, file.workspaceRequired,
                                              file.uploadFailed.

LIMITE DIVERGENTE:
apps/frontend/src/lib/utils/files.ts:1-4      MAX_SIZE local = 50 * 1024 * 1024;
apps/frontend/tests/unit/fileUpload.test.ts:1-12  cobre 10 bytes (true), 100 MB e 51 MB (false),
                                              mas não a fronteira de 10 MB;
flora-core/src/config/storage.rs:30-32        campo max_upload_bytes com serde default;
flora-core/src/config/storage.rs:35-37        default_max_upload_bytes() = 10 * 1024 * 1024;
flora-core/src/config/storage.rs:54           max_upload_bytes: 10 MB no Default;
flora-core/src/services/file_service.rs:139-155  max = min(max_upload_bytes, workspace_quota,
                                              org_quota) repassado a storage.put_stream;
flora-api/src/routes/files.rs:127-204         upload multipart; :247-296 download;
apps/frontend/src/lib/i18n/pt-BR.ts:103       'file.tooLarge': 'Arquivo muito grande. O tamanho
                                              máximo é 50 MB.' (texto desatualizado).

PADRÃO DE PÁGINA E TESTE:
apps/frontend/src/routes/(app)/tasks/+page.svelte:41-51    GET /api/v1/workspaces + preferência
                                              authState.workspaceId + setWorkspace;
apps/frontend/src/routes/(app)/tasks/+page.svelte:116-141  troca de workspace e guard de auth/org;
apps/frontend/tests/unit/tasks-query.test.ts:1-21 convenção describe/it/expect;
apps/frontend/vite.config.ts:27-53            projetos client/server; :28 requireAssertions;
                                              :49 server inclui tests/**;
apps/frontend/package.json:15,47              test:unit / vitest.
```

Verificação ao vivo de upload 201/download 200 com `FLORA_STORAGE__LOCAL_PATH` gravável é informação de entrada relatada ao ciclo; nenhuma requisição, gate, build, teste ou formatter foi executado neste artefato. Os números de linha se referem ao baseline `c5e4f886e`.

## Autorização

Modo autônomo. Decisão `go` em `.specify/assessments/105-files-ui/decision.md` antes da implementação. Escopo: (1) rota `(app)/files/+page.svelte` com seletor de workspace, upload via `FileUpload`, lista da sessão via `fileStore` e itens via `FilePreview`; (2) link “Arquivos” na navegação com nova chave pt-BR `nav.files`; (3) limite de cliente alinhado a 10 MB com `MAX_UPLOAD_BYTES` exportado e mensagem `file.tooLarge` corrigida; (4) testes unitários de fronteira (0, 10 MB exato, 10 MB + 1, 50 MB e entradas inválidas); (5) dívidas registradas como fora de escopo/**[NV]**: lista apenas em memória (sem endpoint de listagem), ausência de mapeamento de erro genérico do servidor e o texto “< 50 MB” do roteiro. Nenhuma mudança de backend, gate, required check, rota adicional, flag, schema, autorização, DMs, chat ou CHANGES; nenhum commit.

## Fronteira

Artefatos novos do ciclo 105 (`.specify/assessments/105-files-ui/` e `specs/105-files-ui/{spec,plan,tasks}.md`, com substituição do `spec.md` modelo) e, na implementação, `apps/frontend/src/routes/(app)/files/+page.svelte` (novo), `apps/frontend/src/routes/(app)/+layout.svelte`, `apps/frontend/src/lib/i18n/pt-BR.ts`, `apps/frontend/src/lib/utils/files.ts` e `apps/frontend/tests/unit/fileUpload.test.ts`. Somente leitura: componentes `files/*`, `FileStore.svelte.ts`, `models.ts`, `flora-core/src/config/storage.rs`, `flora-core/src/services/file_service.rs`, `flora-api/src/routes/files.rs`, página `/tasks`, `vite.config.ts`, `package.json` e o roteiro. Sem toques em `analyze.md`, `converge.md`, `validate/`, `release/`, `.specify/memory/STATE.md`, `PROGRESSO*`, `AGENTS.md`, gates, workflows, ADRs, changelog ou constituição. A factory `mdmv-dark-factory` é somente leitura; nenhum commit.
