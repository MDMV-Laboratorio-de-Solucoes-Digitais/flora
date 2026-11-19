# Shape: 068 — refresh verificado de documentação

## Forma escolhida

Um "sweep de verdade documental" em cinco arquivos, cada um com diff mínimo e auditável:

1. `docs/research/index.md` — dois números no cabeçalho e sete células da tabela, mais um apêndice datado que preserva a nota de dívida 047+ existente.
2. `docs/PROGRESSO.md` — nova seção `## Estado verificado — 2026-11-19T…Z (append-only)` no fim, com a mesma estrutura das notas de 15:53Z (ciclo 067).
3. `OBJETIVO.md` e `ROADMAP.md` — uma nota datada cada, no fim, dizendo que o intervalo verificado vai até 067 e apontando as contagens canônicas.
4. `flora-api/src/routes/rbac.rs` — substituição das 2 linhas de comentário por texto compatível com o código atual; nenhuma linha executável tocada.
5. `STATE.md` — apêndice de execução; nenhuma linha histórica reescrita.

## Por que não regenerar o índice inteiro

A indexação retroativa de 047–067 é um candidato separado (E=3.75) e exigiria 21 entradas novas. Misturá-la aqui inflaria o esforço e o risco de erro sem aumentar a confiança das contagens. A dívida permanece registrada explicitamente.

## Riscos e mitigação

- **Risco:** sobrescrever histórico append-only → mitigado por `git diff` por arquivo antes de cada commit; nenhuma linha antiga removida exceto as cifras explicitamente substituídas no cabeçalho/tabela do índice.
- **Risco:** contagem com semântica diferente (com/sem `index.md`) → mitigado pelo uso do mesmo comando da linha 4 do índice (137 sem `index.md`; 138 com ele).
- **Risco:** tocar comportamento em `rbac.rs` → mitigado por diff restrito ao bloco `//!` e reexecução dos gates Rust.

## Entrega

Um PR para `dev/`, título conventional em inglês, descrição com evidências coladas, riscos e rollback; merge deliberado somente após CI verde observada ao vivo (ADR-0003).
