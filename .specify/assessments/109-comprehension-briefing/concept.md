# Concept: Compreensão registrada em mudanças críticas

- **Slug**: 109-comprehension-briefing
- **Created**: 2026-11-25
- **Recommended option**: Briefing curto nos domínios críticos

## Options

### Option A — Briefing curto nos domínios críticos
- **Sketch**: Toda mudança em autenticação/sessão, RBAC, tenant, bootstrap/recuperação ou HITL passa a vir acompanhada de uma nota curta e padronizada: o que se pretendia, quais invariantes mudaram (antes e depois), quais decisões ficaram implícitas, como verificar o resultado e uma pergunta que o humano deve responder ao revisar. O SE consegue revisar sem reler o diff inteiro, e o histórico de ciclos anteriores permanece como estava.
- **Appetite**: small (dias); incerteza: a adesão real nos próximos ciclos ainda não foi observada, então o benefício de compreensão é uma hipótese, não medição.
- **Trade-offs**: Acrescenta rastreabilidade sem criar checagem automática e preserva a autoridade dos gates previstos em ADR-0003. O custo é disciplina de escrita e o risco de o briefing virar formulário preenchido por rotina; não há como provar compreensão efetiva apenas pelo artefato existir.
- **Rabbit holes**: Transformar a nota em gate ou required check; tentar padronizar retroativamente a nomenclatura dos ciclos grandfathered; incluir pagamentos na obrigatoriedade antes de resolver a divergência com o diagnóstico original; medir compreensão por heurística de texto.

### Option B — Confiar na spec e na revisão integral do diff
- **Sketch**: Nenhum artefato novo: a compreensão continua dependendo da spec escrita e da leitura completa da mudança por quem revisa.
- **Appetite**: small (dias).
- **Trade-offs**: Não adiciona processo nem automação, mas mantém a revisão lenta e não reproduzível. Decisões implícitas continuam invisíveis, e a alternativa de revisão linha a linha foi explicitamente rejeitada pelo assessment por custo e por conflito com o regime de merge autônomo.
- **Rabbit holes**: Reintroduzir revisão humana detalhada como condição de merge; tratar a existência da spec como prova de que as invariantes foram entendidas.

### Option C — Briefing como checagem automática de conformidade
- **Sketch**: A presença e o conteúdo do briefing passam a ser validados automaticamente antes do merge, tornando a adesão uma condição verificável do processo.
- **Appetite**: medium (semanas); incerteza alta: muda governança e exigiria autorização separada, além de definir o que conta como conteúdo suficiente.
- **Trade-offs**: Ganha adesão garantida e sinal único para revisores, mas cria required check, contrariando o enquadramento atual, e corre o risco de virar preenchimento mecânico — conformidade sem compreensão.
- **Rabbit holes**: Classificar texto para "medir compreensão"; estender a checagem a todos os domínios e ciclos passados; reescrever o histórico grandfathered; transformar a nota em novo piso do guarda do gate.

## Recommendation

Recomendo a **Option A**. Ela responde ao problema central — gates provam que checks rodaram, não que alguém entendeu o que mudou — com um artefato leve, prospectivo a partir do ciclo 109 e obrigatório nos cinco domínios críticos já registrados na nota de governança. O alvo é 100% das mudanças críticas com briefing referenciado, sem required check, sem alteração de contrato e sem reescrita de histórico. A eficácia sobre a compreensão real depende do uso humano e permanece **[NV]** até observação nos próximos ciclos.

## Out of Scope (for the recommended option)

- Gate, required check, automação de CI e alteração de contrato de código.
- Reescrita dos ciclos grandfathered e renomeação retroativa do template existente.
- Incluir pagamentos na obrigatoriedade sem clarificar a divergência com o diagnóstico.
- Afirmar compreensão efetiva ou adesão futura sem uso observado.
- Credenciais, segredos e novos pisos no guarda do gate.

## Assumptions to Validate

- Os cinco domínios registrados na nota de governança são a lista autoritativa de "críticos".
- A pergunta-teste será efetivamente usada na revisão, não apenas preenchida.
- A nota curta é suficiente para registrar intenção, invariantes e decisões implícitas.
- A aplicação prospectiva a partir do ciclo 109 é aceita como critério de adesão.
- Ausência de automação é compatível com a autoridade atual de ADR-0003 e do guarda do gate.
- A divergência sobre pagamentos será resolvida em decisão separada, sem mudança silenciosa do escopo.
