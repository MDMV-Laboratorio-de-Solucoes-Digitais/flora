# Shape: fechamento pós-merge do ciclo 072

- **Slug**: 074-post-merge-closure · **Shapeado**: 2026-11-19T04:51Z
- **Recomendado**: A — apêndice único no STATE

## Opção A — apêndice único (recomendada)

- **Sketch**: uma nota datada no fim de `STATE.md` com merge commits de #294/#295, horários, checks verdes observados e dívidas. Histórico intacto.
- **Apetite**: small.
- **Trade-offs**: corrige a memória com risco mínimo; não altera o corpo da nota antiga (intencional).
- **Rabbit holes**: reescrever a nota 04:36; reabrir a leg RBAC; tocar o worktree 073.

## Opção B — nada fazer

- **Sketch**: confiar nos merge commits como registro suficiente.
- **Apetite**: none.
- **Trade-offs**: mantém a contradição no STATE.

## Opção C — revisar várias notas

- **Sketch**: atualizar OBJETIVO/ROADMAP/PROGRESSO/índice também.
- **Apetite**: medium; risco de conflito com append-only e escopo inflado.
- **Trade-offs**: mais completo, mas sem ganho proporcional — o merge é fato de governança, não de conteúdo desses documentos.

## Fora de escopo

Código, CI, gates, ADRs, constituição, worktree 073, novos merges, títulos do índice de research.

## Premissas

- `gh pr view`/`gh pr checks` observados ao vivo são a verdade para status de PR.
- Nenhuma aprovação de maintainer é necessária para escrever documentação append-only.
