# Shape: refresh verificado de estado (072)

- **Slug**: 072-verified-state-refresh
- **Shapeado**: 2026-11-19T04:23Z
- **Opção recomendada**: A — apêndice datado nos quatro documentos

## Opção A — apêndice datado (menor coisa que funciona)

- **Sketch**: cada um dos quatro documentos recebe, no fim, uma nota curta intitulada “Estado verificado — 2026-11-19T…Z” com âncora local, contagens reais, intervalo 001–070 e a lista explícita do que permanece `[NV]`. O histórico existente fica intacto.
- **Apetite**: small.
- **Trade-offs**: corrige a desinformação para leitores futuros sem risco de sobrescrever histórico; não atualiza o corpo antigo das notas (isso é intencional).
- **Rabbit holes**: tentar “consertar” todas as cifras antigas; transformar notas em relatório completo; reabrir dívidas já fechadas.

## Opção B — não fazer nada

- **Sketch**: manter as notas como estão e confiar que leitores consultem os artefatos SDD diretamente.
- **Apetite**: none.
- **Trade-offs**: custo zero agora; preserva a desatualização e a chance de agentes reabrirem dívida 047+ já eliminada por evidência.
- **Rabbit holes**: nenhum.

## Opção C — reescrever/regenerar os documentos

- **Sketch**: substituir as notas antigas por uma visão consolidada e única do estado atual, potencialmente com geração automática.
- **Apetite**: medium.
- **Trade-offs**: leitura mais limpa; destrói append-only, exige fonte regenerável que não existe e viola a convenção do repo.
- **Rabbit holes**: reconstruir histórico, inventar `.generated.md`, decidir quais notas descartar.

## Recomendação

**A**. Ela satisfaz SC-001/SC-003 com menor risco e é o padrão já usado nos ciclos 068/069. B falha SC-001; C viola SC-003 e a convenção append-only.

## Fora de escopo da opção A

Código, CI, gates, pisos, ADRs, constituição, worktree 071, leg RBAC, emenda constitucional, afirmações remotas.

## Premissas

- As contagens obtidas por `find` nesta sessão são a fonte da verdade local.
- O intervalo de specs até 070 é estável durante a execução deste ciclo (o 071 vive em worktree isolada).
- `gh` é a CLI de PR autoritativa do alvo (constituição v1.3.0 §VI), não `fj`.
