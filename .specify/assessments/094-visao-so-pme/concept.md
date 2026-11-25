# Concept: Registro da visão SO-PME

- **Slug**: 094-visao-so-pme
- **Created**: 2026-11-25
- **Recommended option**: Nota de governança datada, não-normativa e append-only com mapa dos três grupos

## Options

### Option A — Nota de governança datada, não-normativa e append-only com mapa dos três grupos
- **Sketch**: a visão do SE passa a existir como um registro com data e rastreabilidade: um mapa único que lista sistemas, pessoas e agents MDMV e, para cada item, diz se está implementado e verificado, deferido ou ausente no alvo. Quem chega depois encontra a intenção e o estado sem precisar inferir dependências, e lê explicitamente que a nota não cria obrigação nem autoriza execução. O registro histórico permanece intacto — inclusive a nota anterior que dizia "deploy em curso, health **[NV]**".
- **Appetite**: small — documentação datada no formato de notas de governança já usado nos ciclos anteriores; incerteza: a visão é sinal único do SE, e estado operacional de sistemas/agents é **[NV]**.
- **Trade-offs**: ganha rastreabilidade e reduz interpretações divergentes sobre o que Flora integra e o que autoriza, sem tocar em gate, constituição, autoridade de merge ou fluxos de execução. Sacrifica a pretensão de descrever capacidade instalada: vários itens são intenção ou deferimento, e listá-los pode sugerir adoção que os artefatos não suportam. O risco de "escopo normativo acidental" continua presente — a nota é declarada não-normativa, mas sua existência datada pode ser lida como prioridade implícita.
- **Rabbit holes**: transformar o mapa em roadmap, prioridade normativa ou emenda constitucional; criar gate, workflow, migração ou código "para realizar" a visão; reescrever ou atualizar notas antigas; afirmar integração viva de Coolify/Forgejo/Grafana/Uptime Kuma ou participação de agents sem **[NV]**; reconsultar produção sem autorização; definir critério de inclusão de novos itens sem decisão própria.

### Option B — Referência mínima em um índice de documentação (a menor coisa que poderia funcionar)
- **Sketch**: registrar apenas a frase da visão com data e um link para o input do ciclo, sem montar o mapa nem classificar estados. É a forma mais barata de deixar a afirmação rastreável.
- **Appetite**: small — uma linha em documentação já existente, nenhum artefato novo.
- **Trade-offs**: custo quase zero e nenhum risco de mutação de processo, mas não entrega o mapa único de sistemas, pessoas e agents nem a distinção entre implementado, deferido e ausente — justamente o que protege o contribuidor futuro de inferências erradas. A visão continuaria sujeita a leitura como prioridade implícita, sem o contrapeso explícito dos estados.
- **Rabbit holes**: transformar o índice em lista de prioridades; tratar a presença da frase como aval da execução; acumular links sem atualização; confundir rastreabilidade com adoção.

### Option C — Plano normativo de integração com prioridades e prazos
- **Sketch**: converter a visão em plano de trabalho com responsáveis, prioridades e datas para integrar os sistemas e ativar os agents listados, incluindo dogfooding na MDMV. O contribuidor veria não só o mapa, mas o que fazer em seguida.
- **Appetite**: medium a large — exige decisões de escopo, sequenciamento e execução para sistemas hoje deferidos e agents ausentes; nenhuma autorização para isso aparece nos artefatos.
- **Trade-offs**: daria direções concretas e poderia reduzir a ambiguidade de longo prazo, mas contradiz diretamente a natureza não-normativa exigida pelo problema: criaria obrigação, autorização de execução e possivelmente trabalho em gate/deploy, deslocando decisões que pertencem a ciclos próprios sob ADR-0003. Não há plano de dogfooding, critério de inclusão ou evidência de adoção para apoiar o custo.
- **Rabbit holes**: abertura de PRs, mudanças de CI ou deploy; promoção de agentes ausentes a capacidade instalada; reinício de emendas pendentes; reescrita de registros append-only para acomodar o plano; exaustão de escopo com sistemas deferidos sem datasource/painel.

## Recommendation

Recomendo a **Opção A**. Ela satisfaz as métricas do problema: nota datada e rastreável cobrindo sistemas, pessoas e agents; zero emendas, gates, workflows ou código criados a partir dela; distinção explícita entre implementado/verificado, deferido e ausente; nenhum PR, merge, tag ou deploy promovido pela nota. Preserva a autoridade de `check_gate_integrity.py` sob ADR-0003 e o histórico append-only, tratando a visão como direção — não como capacidade instalada nem como prioridade normativa. Estado operacional e adoção seguem **[NV]** e são listados como premissas, não como fatos.

## Out of Scope (for the recommended option)

- Obrigação, autorização de execução, emenda constitucional, gate, workflow, migração ou código decorrentes da nota.
- Promoção da visão a prioridade normativa, roadmap, plano de dogfooding ou critério de inclusão de novos sistemas/pessoas/agents.
- Reescrita de registros históricos, inclusive a nota que dizia "deploy em curso, health **[NV]** até confirmação".
- Afirmação de estado operacional de sistemas, agents, CI, Coolify ou Forgejo sem **[NV]**; reconsulta de hosts não allowlisted.
- PR, merge, tag, deploy, alteração de autoridade de merge e mudanças em `validate/`, `release/`, `STATE.md`, ADRs ou constituição.

## Assumptions to Validate

- A visão é atribuída ao SE e registrada como input literal do ciclo; não há evento ou discussão anterior documentada, nem validação externa da direção.
- O destinatário são contribuidores futuros; outros destinatários da nota não foram identificados nos artefatos.
- Coolify e Forgejo estavam implementados/verificados no alvo; Grafana+VictoriaMetrics e Uptime Kuma deferidos; Autonoma/dark-factory como ligação documental; marketing/monetization/seller ausentes no alvo **[NV]** fora dele.
- Evidência de produção registrada (`/health` 200, deploy `finished 21:23:59Z` no commit `cee7b0cda`, refs Forgejo sincronizados, 0 advisories) é fotografia datada de 2026-11-20; ciclos posteriores mudaram FQDN/apps, então reconsulta é necessária antes de reutilizá-la.
- Não há plano de dogfooding com escopo/prazo definidos nos artefatos; ausência de adoção é hipótese, não medição.
