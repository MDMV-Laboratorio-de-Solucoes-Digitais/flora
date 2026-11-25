# Concept: Correção das violações WCAG encontradas

- **Slug**: 112-wcag-fixes
- **Created**: 2026-11-25
- **Recommended option**: Corrigir as três violações sem relaxar regras

## Options

### Option A — Corrigir as três violações sem relaxar regras
- **Sketch**: As pessoas que usam leitor de tela passam a ouvir o nome dos botões ícone e o título da página; quem tem baixa visão encontra contraste legível nos botões desabilitados do tema claro. A varredura automática continua apenas informativa, mas agora exibe um resumo legível no log do CI. A correção é acompanhada de roteiro de testes, evidência e revisão independente antes de considerar a mudança pronta para merge.
- **Appetite**: small (dias); incerteza: não há captura local confirmada para avaliar regressão visual, e o contraste em tema escuro não foi medido neste ciclo.
- **Trade-offs**: Remove as barreiras reais encontradas sem tocar governança nem desativar regras, mantendo o sinal automatizado confiável. O escopo é pontual: não é auditoria completa e deixa o tema escuro fora da medição; há risco de regressão visual nos botões desabilitados.
- **Rabbit holes**: Ajustar variáveis do tema escuro para "aproveitar" a medição; estender a correção a todas as rotas e componentes; transformar o resumo do job em gate; reabrir o sistema de design; perseguir falso positivo relaxando regra axe.

### Option B — Registrar achados como dívida e não corrigir agora
- **Sketch**: A varredura continua reportando as três violações e a interface permanece como está; os achados ficam registrados para priorização futura.
- **Appetite**: small (dias).
- **Trade-offs**: Evita risco visual e custo de implementação imediata, mas mantém barreiras concretas de navegação e contraste, e corrói a confiança no sinal automatizado ao transformar o log em ruído recorrente.
- **Rabbit holes**: Desativar regras para limpar o log; elevar limite de severidade; acumular dívida até que ninguém mais leia o relatório.

### Option C — Auditoria WCAG completa antes do merge
- **Sketch**: Uma revisão especializada cobriria critérios não automatizáveis em todas as rotas, com julgamento humano sobre contraste, foco e navegação, antes de aceitar a mudança.
- **Appetite**: large (meses); incerteza alta: não há executor nomeado nem orçamento registrado para esse trabalho no ciclo.
- **Trade-offs**: Ampliaria de fato a acessibilidade além do que a ferramenta detecta, mas exige custo, agendamento e provável mudança de processo, enquanto os três achados conhecidos continuariam pendentes até sua conclusão.
- **Rabbit holes**: Contratar auditoria externa; revisar todas as variáveis de tema e o sistema de design; transformar a auditoria em novo required check; reabrir decisões já tomadas de governança.

## Recommendation

Recomendo a **Option A**. Os três achados são reais e localizados — nome acessível ausente em botões ícone, contraste de 3,73:1 em botões desabilitados e documento sem título — e o pedido registrado no bug pede exatamente corrigi-los sem desativar regras nem alterar required checks. O alvo é zero botões sem nome, contraste ≥4,5:1 (registro de ≈5,84:1 no tema claro) e `<title>` presente, com resumo "sem achados" mantido na varredura informativa. Roteiro, evidência e revisão independente são condições para tratar o PR como mesclável; tema escuro e auditoria completa ficam fora e permanecem **[NV]** onde não foi observado.

## Out of Scope (for the recommended option)

- Required checks, gate, pisos, migrations, backend, credenciais e deploy.
- Desativar regras axe, relaxar o guarda ou promover o job a bloqueante.
- Auditoria humana completa WCAG 2.2 AA e revisão de todas as rotas.
- Medição ou alteração do tema escuro neste ciclo.
- Afirmar conformidade AA plena a partir do resumo automatizado.
- Comportamento em staging/produção não observado.

## Assumptions to Validate

- As chaves de texto em pt-BR para abrir/fechar sidebar e painel de IA existem e são consistentes.
- A mudança de contraste no tema claro não degrada a aparência de forma inaceitável; captura local, se disponível, ajuda a confirmar.
- O resumo no log do job é suficiente para tornar o resultado visível sem criar gate.
- A revisão agêntica independente ocorrerá antes do merge em `dev`.
- O tema escuro permanecerá inalterado e sem medição, registrado como **[NV]**.
- O padrão de commits em pt-BR e PR em inglês será mantido conforme a convenção registrada.
