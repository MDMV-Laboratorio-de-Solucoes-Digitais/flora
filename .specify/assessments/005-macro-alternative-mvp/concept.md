# Concept: MVP self-hosted enxuto para PME

- **Slug**: 005-macro-alternative-mvp
- **Created**: 2026-11-25
- **Recommended option**: Reframe estratégico com MVP Seed verificável

## Options

### Option A — Recorte documental mínimo

- **Sketch**: Só registrar, em objetivo e roadmap, para onde vai cada capacidade do workspace de referência — o que entra no Seed, o que é substituído por alternativa mais barata e o que fica adiado — deixando demanda e limites de hardware explicitamente como hipótese não medida.
- **Appetite**: small
- **Trade-offs**: Menor coisa que poderia funcionar para dar norte sem custo de implementação; reduz ambiguidade imediata. Não aproxima o Seed de um fluxo utilizável nem corrige o fato de que integrações runtime permanecem NV, então o risco de priorizar por suposição continua.
- **Rabbit holes**: Transformar a tabela de hipóteses em promessa de features; tratar ausência de telemetria como validação de demanda; super-simplificar o MVP "por enxutez".

### Option B — Reframe estratégico com MVP Seed verificável

- **Sketch**: Alinhar direção e priorização em torno de um MVP "Macro-like" enxuto para PME em VPS modesta: poucas jornadas realmente utilizáveis (chat/tarefas/arquivos básicos com org e auth), componentes opcionais e substituições de baixo custo, e um próximo incremento técnico verificável. A promessa deixa de ser paridade e passa a ser um Seed operável e demonstrável.
- **Appetite**: medium
- **Trade-offs**: Reduz desperdício de esforço, orienta contribuições e entrega valor percebido cedo sem exigir serviços pesados. Custo maior que A e depende de evidência ainda frágil: demanda do segmento e requisitos de CPU/RAM/disco são hipóteses não medidas, e o smoke ponta a ponta é NV. Appetite médio com incerteza.
- **Rabbit holes**: Perseguir paridade ampla disfarçada de "MVP"; adicionar email completo, calls, CRM/databases ou agentes como requisito de Seed; copiar código/assets do Macro sob interpretação frouxa de AGPL; declarar "qualquer VPS barata" sem medição.

### Option C — Paridade ampla com o Macro (não fazer)

- **Sketch**: Replicar a superfície do workspace de referência — email, docs colaborativos, calls, CRM, bancos de dados, agentes, integrações — como critério de sucesso do projeto.
- **Appetite**: large
- **Trade-offs**: Visão completa, mas incompatível com a constituição (debloat, VPS test, monólito modular, Seed→Forest) e com o estado parcial do repositório; eleva custo de build, operação e suporte exatamente onde a proposta promete leveza.
- **Rabbit holes**: Service sprawl, dependências proprietárias obrigatórias, CRDT/local-first pesado e otimizações de escala que não interessam ao perfil Seed.

## Recommendation

Option B. Os goals do `problem.md` pedem distinguir valor típico de PME de custo operacional elevado, tornar visíveis as lacunas de evidência (demanda, hardware, conformidade AGPL) e reduzir a ambiguidade entre "paridade com o Macro" e "Seed utilizável em VPS modesta". B faz isso preservando fatias verticais finas e verificáveis como critério, e é o caminho escolhido na decisão 005. Classificações de capacidade continuam hipóteses até medição; onde não há evidência, marca-se NV em vez de prometer.

## Out of Scope (for the recommended option)

- Lista final e definitiva de features do MVP, API, modelo de dados ou plano de implementação detalhado.
- Promessa de paridade com o Macro, de suporte a cargas específicas ou de performance em VPS sem medição.
- Reuso de código incompatível com AGPL ou engenharia reversa não autorizada de componentes proprietários.
- Deploy, infra de produção, segredos, serviços pagos, merges/releases como pré-condição.

## Assumptions to Validate

- Existe demanda real das PME pelas capacidades candidatas a core Seed (chat/tarefas/arquivos) — hoje é hipótese, sem entrevistas ou telemetria registradas.
- "VPS barata" tem alvo definido de CPU/RAM/disco (a hipótese de ~2 vCPU/4 GB ainda não foi medida).
- Postgres-first com componentes opcionais degradando graciosamente é suficiente para o perfil Seed sem Meilisearch/Valkey obrigatórios.
- A reimplementação limpa sob AGPL é factível sem cópia indevida de código ou assets do Macro.
