# Define: 058 — Root doc index

## Problema

Não há um único ponto de entrada na raiz que conecte visão, roadmap, estado
verificado, índice de research e ADRs — a descoberta depende de conhecer os
nomes dos arquivos.

## Requisitos funcionais

- **FR-001**: `index.md` DEVE existir na raiz com links relativos válidos para
  `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md`, `docs/research/index.md`,
  `README.md`, `AGENTS.md` e `docs/adr/`.
- **FR-002**: DEVE declarar data de verificação, âncora e `[NV]` para estado remoto.
- **FR-003**: NÃO DEVE duplicar o conteúdo dos documentos linkados.
- **FR-004**: NENHUM código, workflow, ADR ou constituição DEVE mudar.

## Critérios de sucesso

- **SC-001**: todos os links relativos resolvem para arquivos/diretórios existentes.
- **SC-002**: índice com ≤ 40 linhas.
- **SC-003**: verificador isolado confirma cobertura dos pontos de entrada.
- **SC-004**: gates do alvo permanecem verdes.

## Não-objetivos

- Não editar `README.md`, ADRs, constituição, código ou CI.
- Não criar índices em `docs/` (já existem).
