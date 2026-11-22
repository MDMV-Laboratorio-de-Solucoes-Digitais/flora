# Analyze: 105 — UI de arquivos escopada ao workspace (READ-ONLY)

**Data**: 2026-11-22 · **Branch**: `feature/105-files-ui` @ `c5e4f886e` · **Baseline**: `c5e4f886e`
**Artefatos revisados**: `.specify/assessments/105-files-ui/*` (`go`), `specs/105-files-ui/{spec,plan,tasks}.md`, `docs/governance/2026-11-21-roteiro-staging-flora.md` item 8, componentes `files/`, `pt-BR.ts`, `files.ts`, `flora-core/src/config/storage.rs`, `flora-api/src/routes/files.rs`.

## Diagnóstico determinístico

- Roteiro item 8 exige upload, preview/download e erro pt-BR para arquivo acima do limite. Backend já respondia 201/200 em staging local, mas não havia rota `/files` nem link de navegação — os componentes `FileUpload`/`FilePreview`/`FileStore` estavam órfãos.
- Limite divergente: `files.ts` usava 50 MB, enquanto `StorageConfig::max_upload_bytes` é 10 MB (`storage.rs:54`) e `FileService` aplica `min(max_upload_bytes, quotas)`. Arquivos entre 10 e 50 MB passavam no cliente e falhavam no servidor com mensagem genérica.
- Correção: rota `(app)/files/+page.svelte` com seletor de workspace (padrão de `/tasks`), upload via componente existente, lista da sessão via `FileStore`, link "Arquivos" com `aria-label`, limite exportado `MAX_FILE_SIZE_BYTES=10 MiB` e mensagem pt-BR alinhada. `FilePreview` passou a usar `API_BASE_URL` para download (a URL relativa apontava para a origem do frontend).

## Checagem determinística

```
artifacts=8  critical=0  fr=5/5  sc=3/3  tasks=8/8  go=True  divergences=1
FR match= True ['FR-001','FR-002','FR-003','FR-004','FR-005']
SC match= True ['SC-001','SC-002','SC-003']
```

Divergência registrada: não há endpoint de listagem de arquivos no backend; a lista da rota é da sessão corrente (`FileStore`). Persistência da lista após reload permanece **[NV]**/dívida, fora da fronteira deste ciclo.

## Rastreabilidade

| # | Requisito | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — limite client alinhado ao Seed (10 MiB) | T002, T003 | `MAX_FILE_SIZE_BYTES`, testes de fronteira; Vitest 50/50 |
| R2 | FR-002 — rota `/files` com seletor de workspace | T004 | navegador real: `/files`, workspaces carregados, upload 201 |
| R3 | FR-003 — erro pt-BR para arquivo acima do limite | T002 | captura: "Arquivo muito grande. O tamanho máximo é 10 MB." e nada adicionado à lista |
| R4 | FR-004 — link de navegação e download na origem da API | T005 | `aria-label` no link; `FilePreview` usa `API_BASE_URL` |
| R5 | FR-005 — documentação e fronteira | T006–T008 | assessoria/spec/plan/tasks + R4/R7; nenhum gate/backend tocado |

SC-001 (upload válido), SC-002 (rejeição acima do limite) e SC-003 (pt-BR/escopo) cobertos por R1–R4.

## [NV]

- Lista persistida após reload (sem endpoint de listagem) — dívida.
- FQDN de staging real, CI remota e deploy: **[NV]**.
- Verificação visual final do link de download após a troca para `API_BASE_URL`: observada indiretamente (URL montada a partir da origem configurada); **[NV]** para clique efetivo em produção.

## Resultado

**PASS — 5/5 FR, 3/3 SC, 8/8 tarefas, 0 críticos, 1 divergência documentada.**
