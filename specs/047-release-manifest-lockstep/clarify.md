# Clarify: 047 — Release manifest lockstep

**Data:** 2026-11-17T13:45Z

## Perguntas resolvidas

1. **O número correto é 5 ou 6?** É 6: `cargo metadata --no-deps` inclui o pacote
   raiz `flora` além dos 5 membros de `Cargo.toml`.
2. **A checagem de versão deve ser removida?** Não. Apenas a cláusula de contagem
   está errada; a de lockstep continua protegendo contra versão parcial.
3. **O número deve ser dinâmico?** Não neste ciclo: não há segunda fonte
   autoritativa e a solução dinâmica aumentaria a superfície de falha.
4. **Qual identidade do ciclo?** 047. `feature.json` e `check-prerequisites.sh`
   não existiam; a resolução veio do inventário de artefatos (specs até 042,
   validations/releases até 046), não do nome do branch.

## Pendências

Nenhum marcador `NEEDS CLARIFICATION` permanece em `spec.md`.
Itens dependendo de estado vivo (GitHub Actions, tags) são **[NV]** e não bloqueiam.
