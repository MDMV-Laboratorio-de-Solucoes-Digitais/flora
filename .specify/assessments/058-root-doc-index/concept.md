# Concept: Índice único na raiz

- **Slug**: 058-root-doc-index
- **Created**: 2026-11-25
- **Recommended option**: Índice curto de entrada na raiz

## Options

### Option A — Índice curto de entrada na raiz
- **Sketch**: Um único arquivo na raiz funciona como porta de entrada: agrupa, em poucas linhas, os documentos de direção, o estado verificado e a referência, com um link relativo e uma frase de descrição para cada um. Declara a data da verificação e sinaliza explicitamente o que é estado local confirmado e o que depende de conferência remota. Não repete o conteúdo dos documentos — só aponta para eles.
- **Appetite**: small — um arquivo novo, sem código nem gates.
- **Trade-offs**: Ganha descoberta sem exigir que o leitor conheça os nomes dos arquivos e sem duplicar informação. Sacrifica apenas a conveniência de ter resumos embutidos; quem quiser detalhe segue o link. Risco baixo: arquivo novo, reversível.
- **Rabbit holes**: Virar índice exaustivo de specs/assessments/validations; copiar resumos de `PROGRESSO.md` e `research/index.md` (drift garantido); transformar em documento gerado.

### Option B — Reaproveitar o README como índice
- **Sketch**: Em vez de arquivo novo, o próprio README passa a listar os pontos de entrada junto da apresentação do projeto.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Evita um arquivo adicional, mas mistura dois propósitos e altera a apresentação do projeto, que hoje não é um índice de estado verificado.
- **Rabbit holes**: Reescrever o README inteiro; duplicar o `PROGRESSO.md` dentro dele.

### Option C — Não fazer
- **Sketch**: Manter a situação atual: quem procura precisa conhecer os nomes dos arquivos ou percorrer diretórios.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Nenhum custo nem risco, mas a descoberta continua dependendo de conhecimento prévio — justamente o problema registrado no intake.
- **Rabbit holes**: — (nenhum; a lacuna simplesmente permanece).

## Recommendation

**Option A**. Os pontos de entrada já existem desde o ciclo 056, mas não havia um lugar que os conectasse (intake.md). O índice curto satisfaz FR-001 (links relativos válidos), FR-002 (data/âncora/`[NV]`) e FR-003 (sem duplicação), com verificação simples de links e contagem de linhas (SC-001/SC-002), sem tocar em README, ADRs, código ou CI (FR-004).

## Out of Scope (for the recommended option)

- Editar `README.md`, ADRs, constituição, código ou workflows.
- Criar índices dentro de `docs/` (já existem).
- Listar specs/assessments individualmente ou duplicar contagens e resumos.

## Assumptions to Validate

- A lista de pontos de entrada do define.md é exaustiva para uma página de entrada na raiz; confirmar se `AGENTS.md` e `docs/adr/` devem mesmo constar.
- Links relativos continuam resolvendo após o merge; estado remoto/CI permanece **[NV]** nesta avaliação.
- Um índice com ≤ 40 linhas é suficiente sem sacrificar clareza.
