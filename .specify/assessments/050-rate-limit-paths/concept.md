# Concept: Limites de taxa nos caminhos certos

- **Slug**: 050-rate-limit-paths
- **Created**: 2026-11-25
- **Recommended option**: Classificação isolada e testada

## Options

### Option A — Classificação isolada e testada
- **Sketch**: Separar em uma regra única e testável a decisão de qual limite se aplica a cada caminho, de modo que login, refresh e convite caiam nos limites específicos pretendidos — inclusive nas formas antigas de caminho — e todo o resto continue no limite genérico. A decisão é exercitada por testes que não dependem de servidor HTTP.
- **Appetite**: small (incerteza: baixa — nenhum número, janela ou resposta muda).
- **Trade-offs**: Faz os limites específicos passarem a valer de fato, sem alterar limites, janelas, estado ou mensagens, com risco baixo e regressão fácil de detectar. Em troca, introduz uma pequena indireção onde antes havia comparações inline.
- **Rabbit holes**: Casar caminhos por expressão regular e trazer dependência ou complexidade sem necessidade; mover a verificação para dentro das rotas, mudança arquitetural fora de escopo; multiplicar sufixos sem critério.

### Option B — Só ajustar as comparações no lugar (menor coisa que poderia funcionar)
- **Sketch**: Corrigir os caminhos comparados diretamente onde a decisão já é tomada, sem extrair regra nem testes isolados.
- **Appetite**: small (incerteza: baixa).
- **Trade-offs**: Diff mínimo e correção imediata do bug; porém sem teste isolado, uma futura mudança de rota pode reintroduzir o problema sem ninguém perceber.
- **Rabbit holes**: Aumentar a lista de sufixos sem saber quais formas antigas ainda são usadas; confundir "teste do contador" com "teste do casamento de caminho".

### Option C — Não fazer / confiar no limite genérico
- **Sketch**: Manter a classificação atual e considerar que o limite genérico já protege os caminhos sensíveis.
- **Appetite**: none (sem custo de implementação; o custo é a proteção ineficaz).
- **Trade-offs**: Zero mudança, mas força bruta em login/refresh e spam de convite seguem sem os limites específicos — o que contradiz diretamente o problema registrado.
- **Rabbit holes**: Tratar o limite padrão por IP como substituto dos limites por caminho; adiar a correção para uma reforma maior de autenticação.

## Recommendation

Recomendo a **Option A**. Ela atende aos requisitos de `define.md` — reconhecer os caminhos reais e legados, manter os demais no limite genérico e cobrir os casos com testes sem servidor — respeitando a decisão de não alterar limites, janelas, estado ou respostas. A forma isolada é a que melhor protege contra a regressão futura apontada em `intake.md` (os testes atuais cobrem só o contador, não o casamento de path).

## Out of Scope (for the recommended option)

- Alterar limites, janelas, estado do limitador ou mensagens de resposta.
- Mexer em rotas, autenticação, CORS, constituição, ADRs ou workflows.
- C4/C5/C6 e quaisquer mudanças de arquitetura do middleware.
- Mudanças em proteção de branch, deploy, tag ou release.

## Assumptions to Validate

- Os testes do pacote da API passam com os novos casos de classificação.
- Não há regressão nos gates Rust e frontend.
- As formas legadas de caminho citadas continuam realmente em uso por algum cliente.
- Os sufixos escolhidos cobrem todas as variantes que precisam de limite específico.
- Comportamento do middleware em produção e estado distribuído do limitador: [NV].
