# Shape: 069 — corrigir nota falsa de dívida no índice de research

## Forma escolhida

Um único apêndice datado ao fim de `docs/research/index.md`, afirmando:

1. que a verificação de conjunto (arquivos vs links) resulta em 137/137, sem faltantes;
2. que as notas anteriores que falam em dívida 047+ permanecem como registro histórico, mas foram superadas por esta;
3. que o resíduo real é a ausência de prefixo de ciclo nos títulos de várias entradas — não cobertura;
4. o comando reprodutível de verificação.

Nenhuma linha existente é removida; nenhum título é renomeado em massa (isso seria um ciclo separado, com risco maior e pouco ganho).

## Riscos e mitigação

- **Risco:** apagar ou reescrever notas append-only → mitigado por diff de adição pura (`+N/-0`) verificado no validate.
- **Risco:** afirmar cobertura sem evidência → mitigado pelo comando de conjunto colado em `research.md`.
- **Risco:** escopo acidental em outros docs → mitigado por `git diff --name-only`.

## Entrega

Um PR para `dev/`, título conventional em inglês, descrição com evidências, riscos e rollback; merge deliberado somente após CI verde observada ao vivo (ADR-0003).
