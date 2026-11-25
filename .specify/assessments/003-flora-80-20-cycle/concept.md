# Concept: Ciclo 80/20 com baseline verificável

- **Slug**: 003-flora-80-20-cycle
- **Created**: 2026-11-25
- **Recommended option**: Estabilizar, fechar PKCE e reconciliar direção

## Options

### Option A — Provar o que já existe

- **Sketch**: Antes de qualquer nova capacidade, o mantenedor passa a ter um registro curto e honesto do que realmente roda no ambiente: o que foi executado com evidência, o que permanece NV e qual lacuna de confiança ainda está aberta. A escolha do próximo passo sai desse registro, e não da marcação de tarefas nos checklists.
- **Appetite**: small
- **Trade-offs**: Ganha critério de priorização e transparência com custo mínimo, sem tocar em runtime; sacrifica a sensação de avanço em código. Risco: a evidência disponível é parcial — verificações locais pontuais e claims de convergência sem runtime demonstrado — então "verificado" precisa vir sempre acompanhado de seu limite.
- **Rabbit holes**: Transformar verificações esparsas em selo de "tudo verde"; tratar a T116 como resolvida sem registrar casos de falha/state/nonce; reabrir a discussão de ferramentas de engenharia reversa como se fosse pré-requisito do ciclo.

### Option B — Estabilizar, fechar PKCE e reconciliar direção

- **Sketch**: Um recorte estreito e defendível: provar o baseline executável, tratar a PKCE (T116) como lacuna de segurança a fechar antes de ampliar superfície, e alinhar o que é canônico entre frontend, CI e documentação. O peso da decisão passa a ser "o que consigo demonstrar", com guardrails para manter o Seed perto do que o Macro oferece de relevante sem copiar sua operação.
- **Appetite**: medium
- **Trade-offs**: Cobre os quatro goals do problema — critério ancorado em evidência, visibilidade do NV, T116 como lacuna de confiança e fim da ambiguidade de caminho — e reduz drift entre artefatos e realidade. Custa mais que A e depende de ambiente para converter NV em evidência; appetite médio com incerteza, pois o runtime não está evidenciado neste ambiente.
- **Rabbit holes**: Virar migração ampla entre `apps/frontend` e `apps/web`; prometer paridade com o Macro; dar a PKCE como concluída apenas no caminho de sucesso; adicionar REA/Craft-Anything ao toolchain por curiosidade, contrariando a decisão de não serem estratégia central.

### Option C — Engenharia reversa como via estratégica (não fazer)

- **Sketch**: Adotar REA/Craft-Anything como método central para "reimplementar o Macro", usando artefatos e comportamentos observados como fonte primária de requisitos, em vez do código e da documentação abertos.
- **Appetite**: large
- **Trade-offs**: Parece atalho, mas o Macro é AGPL-3.0 de código aberto: fonte e docs são a via autoritativa e mais limpa. A opção acrescenta risco legal/ToS, contaminação de clean room e toolchain com permissões de host, com baixa validade e viabilidade para o objetivo real.
- **Rabbit holes**: Tentar "validar" a direção com ferramentas de terceiros; importar código ou assets incompatíveis com AGPL; transformar uma referência tática em dependência do produto.

## Recommendation

Option B. Ela responde diretamente aos goals do `problem.md`: substitui marcação de tarefas por evidência executável, torna explícito o que é verificável versus NV, coloca a T116 na frente de qualquer ampliação de superfície e reduz a ambiguidade de caminho canônico. Também honra a decisão registrada de manter REA e Craft-Anything fora da estratégia — o ganho vem de estabilizar e esclarecer, não de nova ferramenta. Onde o ambiente não executa runtime, a recomendação permanece NV por princípio.

## Out of Scope (for the recommended option)

- Implementação de features de usuário final, API, modelo de dados ou plano de tarefas além do necessário para estabilizar e verificar.
- Deploys, infra/produção, merges/releases, manuseio de segredos e serviços pagos ou proprietários.
- Adoção de REA/Craft-Anything como dependência ou método central, reuso AGPL-incompatível e engenharia reversa de alvo não autorizado.
- Afirmar builds, testes ou integrações como verdes sem execução observável.

## Assumptions to Validate

- O ambiente permite executar ao menos o conjunto determinístico local (fmt/clippy/test/build) com evidência registrada; o restante segue NV.
- O aceite da T116 inclui ou não casos de falha/state/nonce — a resposta muda o que conta como "fechada".
- Existe autorização para definir o caminho canônico de frontend/CI/documentação sem uma migração ampla não verificável.
- A referência do Macro continua sendo código/documentação AGPL, não artefatos opacos sujeitos a análise tactically autorizada.
