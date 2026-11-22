# Release: 105 — UI de arquivos escopada ao workspace

**Ciclo**: 105 · **Data**: 2026-11-22 · **Branch**: `feature/105-files-ui` @ `c5e4f886e`
**Baseline**: `c5e4f886e`

## Entregue

Rota `/files` com seletor de workspace, upload via componente existente, lista da sessão com preview/download, link "Arquivos" na navegação, limite client alinhado a 10 MiB e mensagem pt-BR correta. URL de download passou a usar `API_BASE_URL`.

## SemVer

**Proposta:** PATCH `0.7.6` → `0.7.7`. Exposição de capacidade já existente no backend, sem nova rota/API, migration, variável ou contrato. Não é MINOR/MAJOR.

## Changelog (FR / T)

- `FR-001` limite `MAX_FILE_SIZE_BYTES=10 MiB` e testes de fronteira (T002, T003).
- `FR-002` rota `/files` com seletor de workspace (T004).
- `FR-003` erro pt-BR acima do limite, nada gravado (T002).
- `FR-004` link de navegação com `aria-label` e download na origem da API (T005).
- `FR-005` rastreabilidade, gates e fronteira (T006–T008).
- **Bug**: item 8 do roteiro — ausência de UI de arquivos e limite divergente. Nenhum ID de `.specify/bugs/` acionado.

## Evidência

`.specify/validations/105-files-ui/validate.md` (**R4 APROVADO**) · `specs/105-files-ui/analyze.md` (5/5 FR, 3/3 SC, 8/8 tarefas, 1 divergência documentada) · `specs/105-files-ui/converge.md` (**Converged**) · assessoria do ciclo · `docs/governance/2026-11-22-105-files-ui.md` · gate do alvo PASS.

## Rollback

Reverter os arquivos frontend do ciclo em **novo commit de reversão**, sem force-push. Nenhuma migration, variável ou dado persistente a desfazer; reapontar o frontend para o build anterior se já publicado.

## [NV]

- Staging real, CI remota, branch protection, deploy, lista após reload e clique final no download: **[NV]** até verificação ao vivo.
