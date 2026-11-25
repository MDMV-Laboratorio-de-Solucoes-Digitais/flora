# Concept: Fim da affordance de convite morta

- **Slug**: 051-dead-invite-ui
- **Created**: 2026-11-25
- **Recommended option**: Remover a affordance e marcar o drift

## Options

### Option A — Remover a affordance e marcar o drift
- **Sketch**: Retirar da página de configurações o formulário que nunca pode funcionar e a chamada que o alimenta, mantendo o restante da página intacto, e anotar no documento de produto que aquele cenário não foi implementado nesta baseline — preservando o texto original.
- **Appetite**: small (incerteza: baixa — componente isolado, sem estado compartilhado).
- **Trade-offs**: Elimina a falsa affordance e o ruído de falha silenciosa sem criar backend nem tomar decisão de autoridade. Em troca, descarta a ideia de convites neste recorte, que dependeria de serviço de e-mail/token e decisão de dono.
- **Rabbit holes**: Implementar convite real com token/e-mail e definir `OrgAdmin` vs `RoleWrite`; mexer no gerenciamento de organização/workspace ou no layout geral; esconder o erro sem remover a causa.

### Option B — Só esconder ou desabilitar o formulário (menor coisa que poderia funcionar)
- **Sketch**: Manter o componente, mas não exibi-lo ou desabilitá-lo na página de configurações.
- **Appetite**: small (incerteza: baixa).
- **Trade-offs**: Diff ainda menor, mas a chamada morta continua no código e a affordance enganosa permanece latente — o que contraria o requisito de não deixar chamada ao endpoint no frontend.
- **Rabbit holes**: Esconder o erro em vez de remover a causa; deixar o componente importado e pronto para ser reativado sem backend.

### Option C — Comprar / implementar convites de verdade
- **Sketch**: Criar o endpoint de convite, com persistência, emissão de token/e-mail e definição de quem tem autoridade para convidar.
- **Appetite**: large (incerteza: alta — depende de decisão de dono que os artefatos marcam como fora de escopo).
- **Trade-offs**: Entregaria a funcionalidade que a UI promete, mas exige serviço de e-mail/token e uma decisão de autoridade que não foi tomada neste ciclo.
- **Rabbit holes**: Arrastar RBAC/RLS e proteção de último Owner; ampliar para fluxo completo de onboarding de membros.

## Recommendation

Recomendo a **Option A**. Ela resolve o problema registrado em `define.md` — falsa affordance e falha silenciosa — sem tocar em backend, autorização ou no restante da interface, e preserva o texto original do cenário obsoleto ao marcá-lo, conforme o requisito append-only. Não toma nenhuma decisão sobre convites futuros.

## Out of Scope (for the recommended option)

- Implementar convites, e-mail ou token.
- Backend, autorização, RBAC/RLS e constituição, ADRs ou workflows.
- Alterar o gerenciamento de organização, o gerenciamento de workspace ou o layout geral.
- PR #265 e qualquer trabalho alheio.
- Apagar o texto original do cenário na documentação.

## Assumptions to Validate

- Não resta nenhuma chamada ao endpoint de convite nem referência ao componente no frontend.
- Os gates frontend (check/lint/test/build) e Rust permanecem verdes.
- O texto original do cenário é preservado ao se marcar o drift.
- Nenhum outro consumidor depende do componente removido.
- Intenção futura do dono sobre convites: [NV].
