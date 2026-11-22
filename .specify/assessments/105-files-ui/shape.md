# Shape: 105 — Tela de arquivos e limite coerente

**Data:** 2026-11-21 · **Branch**: `feature/105-files-ui@c5e4f886ee91125cfe1215c436c144476fe42415`

## Forma escolhida

Ligar blocos existentes, sem criar superfície nova: uma página de rota que reusa `FileUpload`/`FilePreview`/`fileStore`, um item de navegação com uma chave pt-BR, uma constante de limite espelhando a configuração do servidor e uma tabela de casos Vitest. Nenhum endpoint, estado global, dependência, gate ou abstração compartilhada novo. O backend continua sendo a autoridade sobre limite, tenant e gravação (`flora-core/src/services/file_service.rs:139-155`, `flora-api/src/routes/files.rs:134,253`).

1. **Causa e rastro (T001 / intake, research)**: fixar o item 8 do roteiro (`roteiro:159-164`), a ausência de `/files` e de `nav.files`, os componentes órfãos e a divergência 50 MB × 10 MB (`files.ts:2` × `storage.rs:35-37,54`). Registrar como dívida/**[NV]** a lista volátil, o erro genérico, o número duplicado no texto e o “< 50 MB” do roteiro. Fechar define/shape/decision com veredito **go** (FR-001..FR-005, SC-001, SC-002, SC-003).
2. **Limite e mensagem (T002 / FR-003)**: em `apps/frontend/src/lib/utils/files.ts`, exportar `MAX_UPLOAD_BYTES = 10 * 1024 * 1024` e aceitar somente `0 <= sizeBytes <= MAX_UPLOAD_BYTES` com `Number.isFinite`. Trocar `file.tooLarge` (`pt-BR.ts:103`) para “Arquivo muito grande. O tamanho máximo é 10 MB.”. Nenhuma outra chave de i18n muda.
3. **Testes de fronteira (T003 / FR-004)**: ampliar `tests/unit/fileUpload.test.ts` com 0 e 10 MB exato (`true`), 10 MB + 1 byte/50 MB/100 MB e inválidos (`false`), asserção da constante e do texto. Projeto `server`, `requireAssertions` (`vite.config.ts:28,49`), sem componente, rede ou navegador.
4. **Rota (T004 / FR-001)**: criar `(app)/files/+page.svelte` no padrão de `/tasks` (`tasks/+page.svelte:41-51,116-141`): `GET /api/v1/workspaces`, preferência por `authState.workspaceId`, guard de auth/org, `FileUpload` com `workspaceId` e `onUploadComplete={(file) => fileStore.addFile(file)}`, lista `fileStore.files` via `FilePreview`. Sem listagem backend — a lista é da sessão.
5. **Navegação (T005 / FR-002)**: adicionar `nav.files: 'Arquivos'` em `pt-BR.ts:48-57` e um `Button href="/files"` com ícone `@lucide/svelte` após `/tasks` (`+layout.svelte:142-149`), mantendo classes e comportamento de sidebar recolhida. `enforceAuth`, `$effect`s e dropdown intactos.
6. **Fecho documental (T006, T007, T008 / FR-005, SC-001, SC-002, SC-003)**: registrar dívidas, confrontar FR/SC com o baseline, verificar que apenas a nova chave de i18n apareceu, que nenhum endpoint/gate/artefato de outro ciclo foi tocado e que não houve commit. Testes, gates e navegador ficam **[NV]** no handoff.

- **Limite divergente (motivo do ciclo)**: 50 MB no cliente (`files.ts:2`) e 10 MB no servidor (`storage.rs:35-37,54`) causam rejeição tardia e mensagem genérica; mitigado espelhando 10 MB no helper e no texto, com testes nos dois lados da fronteira.
- **Arquivo entre 10 e 50 MB gravado antes de falhar**: não ocorre — `file_service.rs:139-155` passa o limite a `put_stream`, e o guard de `FileUpload.svelte:20-23` bloqueia o `fetch`; os dois lados são verificados.
- **Texto do roteiro desatualizado**: `roteiro:161` diz “< 50 MB”; a configuração efetiva é 10 MB. Mantido como dívida **[NV]** — o roteiro não é alterado neste ciclo; a mensagem exibida segue a configuração.
- **Lista que some ao recarregar**: `FileStore.svelte.ts:4-16` é memória e `files.rs:81-89` não tem listagem. Aceito como comportamento da sessão e registrado como dívida; não se inventa endpoint listagem.
- **Erro do servidor genérico**: fora do intervalo aceito pelo cliente (por exemplo mime type não permitido, `file_service.rs:135-137`) continua caindo em `file.uploadFailed` (`FileUpload.svelte:49-52`). Mapear códigos de erro exigiria contrato novo; **[NV]**.
- **Número do limite em três lugares**: config Rust, constante TS e literal pt-BR. Sem injeção de valor no dicionário (mudaria a arquitetura de i18n); dívida registrada e teste de texto protege contra drift.
- **Regressão de navegação/autorização**: adicionar link pode tentar mudar `enforceAuth` ou estados de sessão; mitigado mantendo `+layout.svelte:36-86,168-216` idênticos e reaproveitando o guard da página `/tasks`.
- **Resultado de testes/gates e navegador**: **[NV]** neste artefato — não executamos Vitest, `check`, `lint`, build, gates nem validação visual aqui.

## Entrega

Diff de implementação restrito a `apps/frontend/src/routes/(app)/files/+page.svelte` (novo), `apps/frontend/src/routes/(app)/+layout.svelte` (import de ícone + um `Button`), `apps/frontend/src/lib/i18n/pt-BR.ts` (`nav.files` + texto `file.tooLarge`), `apps/frontend/src/lib/utils/files.ts` (constante exportada + guard) e `apps/frontend/tests/unit/fileUpload.test.ts` (fronteira), além dos artefatos do ciclo 105. Arquivos lidos, não modificados: `flora-api/src/routes/files.rs`, `flora-core/src/config/storage.rs`, `flora-core/src/services/file_service.rs`, componentes `files/*`, `FileStore.svelte.ts`, `models.ts`, página `/tasks`, `vite.config.ts`, `package.json` e o roteiro. Verificação documental cobre T001–T008; gates, build, testes e validação em navegador ficam **[NV]** até o handoff.
