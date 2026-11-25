# Concept: Seed enxuto com quadro real

- **Slug**: 040-seed-ui-debloat
- **Created**: 2026-11-25
- **Recommended option**: Enxugar e dar uso real ao quadro

## Options

### Option A — Enxugar e dar uso real ao quadro
- **Sketch**: Em um mesmo movimento, tirar do checkout o cliente antigo que não tem uso funcional e trocar a lista de tarefas por um quadro alimentado pelas tarefas que já existem, com a mudança de status registrada pelo mecanismo de persistência disponível. Antes de mexer, congela-se um conjunto de verificações determinísticas para garantir que build, lint, testes e integridade de gate continuem equivalentes ao baseline; ao final, diz-se explicitamente o que a mudança não entrega.
- **Appetite**: large (incerteza: duas frentes somadas a uma auditoria cega; nenhuma medição de esforço nos artefatos).
- **Trade-offs**: Ataca as duas metas do problema — menos peso morto no checkout e um quadro com uso real — sem tocar em backend ou autorização. Em troca, a remoção é irreversível no checkout e o valor histórico de UX do cliente antigo segue sem auditoria [NV]; exige critério claro para chamar o quadro de "real" e auditoria em contexto fresco.
- **Rabbit holes**: Buscar paridade com o Macro, visão de time, ordem persistida das colunas ou regras de transição de status — todos declarados como não-objetivos; perseguir um "quadro real" sem critério acordado; tentar provar ganho financeiro em VPS sem medição.

### Option B — Só enxugar o checkout (menor coisa que poderia funcionar)
- **Sketch**: Remover apenas o peso morto do checkout e manter a lista de tarefas como está. Entrega a meta de footprint com a menor superfície de mudança.
- **Appetite**: medium (incerteza: tamanho do inventário de referências a validar antes da remoção).
- **Trade-offs**: Reduz o checkout sem o risco de reescrever a interface, mas deixa o quadro órfão sem uso real e não resolve a segunda meta do problema. A irreversibilidade da remoção permanece.
- **Rabbit holes**: Auditar exaustivamente o valor histórico de UX sem evidência primária; generalizar a remoção para outros diretórios além do inventário verificado.

### Option C — Não fazer / manter o baseline
- **Sketch**: Não remover nada e não trocar a lista pelo quadro; registrar as incertezas como pendências e seguir com o checkout atual.
- **Appetite**: none (sem custo de implementação; o custo é o peso morto que continua).
- **Trade-offs**: Risco operacional zero, porém mantém o checkout pesado e o quadro como affordance isolada; a incerteza sobre o cliente antigo permanece [NV] em vez de ser resolvida.
- **Rabbit holes**: Transformar a pendência em projeto de paridade com o Macro ou em auditoria permanente de UX.

## Recommendation

Recomendo a **Option A**. É a única que liga diretamente às duas metas registradas em `problem.md` — reduzir o peso morto com segurança e fazer o quadro refletir tarefas reais com persistência — e ao requisito de congelar verificações antes da implementação. A recomendação fica condicionada a tornar explícitos os não-objetivos e a acordar o critério que define o quadro como "real", dado o NO-GO da auditoria em contexto fresco registrado no baseline.

## Out of Scope (for the recommended option)

- Backend, migration, RBAC/RLS/tenancy/authz e Tauri (ADR-0002).
- Documentos FASE 7 e paridade com o Macro.
- ADR, constituição, deploy, tag, release, secrets e required checks/proteções.
- Afirmar visão de time, ordem persistida de colunas, regras de transição de status ou ganho financeiro medido em VPS.
- Atribuir ao ciclo a auditoria do valor histórico de UX do cliente removido ([NV]).

## Assumptions to Validate

- Qual inventário exato de referências autoriza a remoção integral em todos os checkouts.
- Se os dois arquivos `.spec.ts` citados são os únicos e2e mortos, ou se existem outros fora do `testMatch`.
- Qual comportamento esperado quando uma transição inválida chega ao backend, dado o fallback silencioso.
- Como preservar estados de empty/loading e a ordem das colunas ao substituir a lista pelo quadro.
- Se o valor histórico de UX do cliente removido foi auditado ou permanece [NV].
- Quem valida a auditoria cega em contexto fresco e qual critério define o quadro como "real".
