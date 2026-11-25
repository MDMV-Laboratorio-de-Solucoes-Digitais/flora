# Concept: Total de pisos dinâmico no guardião

- **Slug**: 057-dynamic-floor-total
- **Created**: 2026-11-25
- **Recommended option**: Derivar a mensagem da fonte canônica de pisos

## Options

### Option A — Derivar a mensagem da fonte canônica de pisos
- **Sketch**: A frase de sucesso do guardião deixa de ser escrita à mão e passa a refletir o que o próprio mapa de pisos já sabe. Quando alguém adiciona ou.remove um piso, a mensagem se ajusta sozinha — sem uma segunda edição e sem nenhuma mudança no que o gate valida ou no que ele imprime em caso de falha.
- **Appetite**: small — evidência local forte; escopo de uma única mensagem.
- **Trade-offs**: Ganha eliminação do drift recorrente (o literal já foi editado em 052, 054 e 055) e saída sempre coerente. Sacrifica a forma por extenso ("eight" → numeral); o texto continua descritivo, não semântico. Risco baixo: apenas a linha de sucesso muda; regex e mensagens de falha ficam idênticas.
- **Rabbit holes**: Tabela de numerais por extenso ("eight", "nine") — complexidade sem benefício; corrigir a dívida dos ADRs no mesmo PR — exige aprovação de maintainer.

### Option B — Manter o literal e corrigir na mão
- **Sketch**: Nada muda: o guardião continua imprimindo o texto fixo e, a cada novo piso, alguém edita a frase. A saída está correta hoje; o defeito fica latente até a próxima alteração.
- **Appetite**: small (zero de mudança).
- **Trade-offs**: Custo imediato nulo e risco nulo agora, mas preserva a duplicação de verdade que já gerou drift três vezes.
- **Rabbit holes**: Transformar a edição manual em checklist/rotina de revisão — institucionaliza o problema em vez de removê-lo.

### Option C — Check separado de consistência da mensagem
- **Sketch**: Criar uma verificação adicional que alerta quando a frase impressa diverge do mapa de pisos, em vez de corrigir a frase na origem.
- **Appetite**: medium — exige novo mecanismo e mexer na proteção de branch.
- **Trade-offs**: Ganha um sinal explícito de divergência, mas adiciona uma peça onde bastava uma linha e altera contextos de branch.
- **Rabbit holes**: Novo required check; mudar os quatro contextos de proteção; duplicar a lógica que já está no guardião.

## Recommendation

**Option A**. Atende diretamente aos critérios do define.md: a saída permanece a mesma hoje (SC-001, FR-003) e, numa cópia com 9º piso, a mensagem passa a dizer o novo total sem editar texto (SC-002). É a menor mudança que remove a causa raiz do drift apontado no intake, sem tocar ADRs, regex, workflow ou testes (FR-002/FR-004).

## Out of Scope (for the recommended option)

- Editar ADR-0003/0006, constituição, testes ou CI.
- Alterar os valores do mapa de pisos ou a regex de validação.
- Numerais por extenso e novos checks de consistência.

## Assumptions to Validate

- O texto com numeral (em vez de "eight") é aceitável como conteúdo informacional idêntico — a correção pós-implementação do define.md já registra essa equivalência; confirmar com o dono do gate.
- A mutação em `/tmp` (9º piso) é suficiente para provar o comportamento dinâmico sem afetar o repositório.
- Estado de CI/PR remoto permanece **[NV]** nesta avaliação documental.
