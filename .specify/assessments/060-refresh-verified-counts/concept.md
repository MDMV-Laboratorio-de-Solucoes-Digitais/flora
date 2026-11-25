# Concept: Contagens verificadas atualizadas

- **Slug**: 060-refresh-verified-counts
- **Created**: 2026-11-25
- **Recommended option**: Apêndice datado com as contagens atuais

## Options

### Option A — Apêndice datado com as contagens atuais
- **Sketch**: O documento de estado verificado ganha uma nova seção no fim, com a data, a âncora e as contagens obtidas diretamente do checkout. A nota anterior continua intacta — o leitor vê o histórico e, logo abaixo, o estado corrente, com a ressalva de que o estado remoto não foi verificado.
- **Appetite**: small — um acréscimo a um documento existente.
- **Trade-offs**: Ganha precisão do "estado verificado" sem apagar o registro do que era verdade antes e sem automatizar nada. Sacrifica a existência de uma única tabela canônica (passam a ser duas, uma histórica e uma atual). Risco nulo: só acréscimo.
- **Rabbit holes**: Reescrever a tabela antiga (quebra o append-only aprovado no ciclo 056); criar um gerador de contagens; atualizar o índice da raiz, que só deferencia a contagem.

### Option B — Reescrever a nota no lugar
- **Sketch**: A tabela existente é substituída pelos números de agora, deixando apenas uma versão da contagem no documento.
- **Appetite**: small.
- **Trade-offs**: Uma só fonte visual, sem acumular seções; mas destrói o histórico datado e contraria a convenção append-only já adotada.
- **Rabbit holes**: Reconciliar ou reconstruir notas de ciclos passados; explicar a mudança em outros artefatos.

### Option C — Gerador automático das contagens
- **Sketch**: As contagens passam a ser produzidas por um mecanismo que atualiza o documento sempre que os diretórios mudam, eliminando a defasagem na origem.
- **Appetite**: medium — exige automação e validação da fonte.
- **Trade-offs**: Ganha atualização contínua, mas troca uma nota datada por um processo e adiciona uma dependência de execução.
- **Rabbit holes**: Integrar ao CI; revalidar fonte específica; mudar a natureza do registro de "verificado" para "gerado".

## Recommendation

**Option A**. O intake mostra que a nota ficou desatualizada pelos ciclos 057–059 enquanto `index.md` deferencia explicitamente a contagem a `PROGRESSO.md`. O apêndice atende FR-001/FR-003 (contagens por `find`, âncora e `[NV]`) e FR-002/SC-002 (nota anterior preservada, diff só de acréscimo), com verificação direta dos números (SC-001) e sem tocar em código, migração ou gate.

## Out of Scope (for the recommended option)

- Reescrever a nota datada de 02:23Z.
- Atualizar `docs/research/index.md` ou `index.md` (não contêm números próprios).
- Executar a migração N3 ou corrigir ADRs (escalados, sem autorização).
- Criar script gerador ou alterar CI.

## Assumptions to Validate

- As contagens obtidas por `find` no checkout de referência são as que devem ser publicadas; confirmar a âncora e a data antes do merge.
- A convenção append-only do ciclo 056 continua válida para este documento.
- Estado remoto/CI permanece **[NV]** nesta avaliação documental.
