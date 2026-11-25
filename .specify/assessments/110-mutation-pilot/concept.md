# Concept: Sensibilidade dos testes medida por piloto

- **Slug**: 110-mutation-pilot
- **Created**: 2026-11-25
- **Recommended option**: Piloto informativo em um arquivo

## Options

### Option A — Piloto informativo em um arquivo
- **Sketch**: Em vez de acrescentar mais testes, mede-se se os testes que já existem percebem pequenas alterações deliberadas na lógica de papéis. O resultado é um relatório local: o que foi detectado, o que sobreviveu e o que não pôde ser avaliado. Mutantes sobreviventes viram dívida registrada, e um defeito real encontrado no caminho é corrigido como desdobramento. Nada disso passa a bloquear merge.
- **Appetite**: small (dias) com incerteza operacional: a execução registrada levou cerca de 16 minutos para um único arquivo, e o custo de repetição ou ampliação ainda não foi medido.
- **Trade-offs**: Entrega sinal concreto de sensibilidade onde o gate só via contagem, e já revelou um comportamento de deduplicação quebrado. Limita-se a uma fatia do código e consome tempo de execução; exige ambiente com espaço suficiente e atenção para não transformar a prática em teatro de relatório.
- **Rabbit holes**: Promover o resultado a required check sem autorização; estender a medição a todo o workspace; aplicar exclusões generalizadas para embelezar a taxa; perseguir casos classificados como inviáveis como se fossem sensibilidade; adotar o piloto em CI antes de calibrar custo.

### Option B — Confiar nos pisos do gate e não medir
- **Sketch**: Mantém-se a rotina atual: pisos de testes e contagem de casos são a evidência de rigor, sem nenhum relatório sobre sobreviventes.
- **Appetite**: small (dias).
- **Trade-offs**: Custo zero e nenhuma mudança de CI, mas preserva exatamente o ponto cego que motivou a demanda: uma alteração de comparação em lógica de autorização pode passar despercebida mesmo com pisos altos.
- **Rabbit holes**: Tratar piso alto como sinônimo de sensibilidade; usar a ausência de mutantes relatados como prova de que não há dívida.

### Option C — Mutação contínua no fluxo de mudanças
- **Sketch**: Cada mudança receberia um relatório de sensibilidade das linhas alteradas, de forma recorrente, com eventual promoção a condição de merge.
- **Appetite**: large (meses); incerteza alta: o orçamento de tempo necessário em CI não foi medido e a promoção a gate depende de autorização de governança.
- **Trade-offs**: Daria sinal contínuo e impediria regressões de sensibilidade, mas traz ruído, custo de execução, calibração de exceções e risco de mudar required checks — exatamente o que o veredito ADAPT evitou neste ciclo.
- **Rabbit holes**: Ajustar limites de tempo até o relatório perder significado; criar política de exclusões que esconda dívida; transformar o relatório em novo gate; reexecutar medição completa a cada mudança pequena.

## Recommendation

Recomendo a **Option A**. O enquadramento F6 classificou a iniciativa como ADAPT: piloto informativo, sem required check, com ADR-0003 e o guarda do gate como autoridade. A evidência registrada — 39 mutantes detectados de 42, um sobrevivente corrigido, dois inviáveis e nenhum check novo — mostra que a fatia local gerou aprendizado acionável com custo controlado. Afirmar cobertura de autenticação, tenant ou bootstrap estaria além do que foi observado, portanto permanece **[NV]**.

## Out of Scope (for the recommended option)

- Job de CI, gate, pisos e required checks novos.
- Aplicação do piloto a todo o workspace e a outros domínios não medidos.
- Mudanças em produção, migrations e dependências de runtime.
- Tratar mutantes inviáveis como sinal de sensibilidade.
- Afirmar cobertura de autenticação, tenant, bootstrap ou comportamento remoto sem evidência.
- Exclusões generalizadas que transformem o relatório em teatro.

## Assumptions to Validate

- A ferramenta já disponível é suficiente, sem instalação adicional neste ciclo.
- O ambiente de execução tem espaço em disco suficiente para a cópia temporária do workspace.
- A correção colateral encontrada pelo piloto é aceita como desdobramento, sem reabrir escopo de produção.
- Mutantes sobreviventes podem ser registrados como dívida sem virar bloqueio de merge.
- O custo registrado para um arquivo é aceitável como insumo para decidir sobre incrementalismo futuro.
- A verificação focal do sobrevivente corrigido é suficiente como evidência, sem reexecutar a bateria completa.
