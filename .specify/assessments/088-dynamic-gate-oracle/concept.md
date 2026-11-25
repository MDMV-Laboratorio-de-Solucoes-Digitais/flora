# Concept: Oráculo dinâmico de pisos no gate de contratos

- **Slug**: 088-dynamic-gate-oracle
- **Created**: 2026-11-25
- **Recommended option**: Descoberta dos contratos ignorados como fonte de verdade

## Options

### Option A — Descoberta dos contratos ignorados como fonte de verdade
- **Sketch**: o guarda de integridade deixa de ser um espelho de duas listas escritas à mão e passa a perguntar ao próprio código quais contratos estão ignorados, conferindo se cada um tem uma lega correspondente e um piso coerente. Para o mantenedor, um contrato novo sem lega — ou um piso defasado — passa a falhar o `rust-build` em vez de passar despercebido; se a listagem falhar ou vier vazia, o guarda falha fechado. As checagens estáticas atuais e o modo de comparação remota continuam disponíveis, e a linha de sucesso passa a refletir a contagem realmente descoberta.
- **Appetite**: small — uma função a mais no script existente, sem novo check obrigatório e sem tocar no workflow; incerteza registrada: o custo de tempo da listagem/compilação no CI não foi medido nos artefatos **[NV]**.
- **Trade-offs**: ganha independência de fonte (o código dos testes, não o hábito humano), detecta o terceiro *confidence gap* que 086/087 não detectaram e preserva a autoridade do gate ADR-0003. Sacrifica a simplicidade de "comparar duas constantes" e passa a depender da disponibilidade da ferramenta de listagem no job: falha de toolchain ou compilação agora também falha o gate, mesmo com pisos corretos. A mensagem de sucesso muda de valor (o checkout registra 11/87 contra 10/58 no handoff), o que pode afetar consumidores que comparam texto exato.
- **Rabbit holes**: transformar a descoberta em um novo required check ou mover a lógica para o workflow; tentar fazer associação perfeita de órfãos e nomes ambíguos entre prefixos antes de entregar o valor central; reescrever `_remote` e as checagens estáticas "aproveitando" a refatoração; medir/otimizar tempo de CI sem evidência; corrigir a defasagem documental 10/58 em todos os artefatos históricos.

### Option B — Espelho mais auditoria humana periódica (a menor coisa que poderia funcionar)
- **Sketch**: manter o guarda-espelho como está e criar o hábito de revisar, fora do gate, a lista de contratos ignorados versus as legs — por exemplo, uma nota de checagem manual a cada ciclo de contratos de banco. Nenhuma falha automática é introduzida; a detecção continua dependendo de alguém olhar.
- **Appetite**: small — nenhuma mudança de código, apenas processo; é a opção de menor custo imediato.
- **Trade-offs**: custo quase zero e risco de regressão nulo no gate, mas não resolve a causa raiz: os dois lados continuam sendo alimentados por humanos, e um terceiro esquecimento passaria como os dois anteriores. Não atende às métricas de falha fechada nem de detecção de órfão; útil apenas como medida temporária.
- **Rabbit holes**: transformar a checklist em mais um artefato obrigatório de governança; propagar a revisão manual para outros domínios sem evidência de necessidade; tratar a auditoria periódica como substituto do gate.

### Option C — Manifesto de contratos gerado e versionado como nova fonte
- **Sketch**: em vez de consultar o código dentro do guarda, gerar em outro momento um inventário dos contratos ignorados e guardá-lo como lista autoritativa; o guarda compararia esse inventário com as legs e os pisos. A experiência para o mantenedor seria parecida com a Opção A, mas com uma etapa extra de geração/atualização do inventário.
- **Appetite**: medium — envolve uma etapa de geração, versionamento e disciplina de atualização; sem evidência nos artefatos de que esse mecanismo já exista ou seja desejado.
- **Trade-offs**: desloca a dependência de `cargo` para fora do guarda, o que pode reduzir tempo de CI, mas reintroduz uma lista mantida por processo — se o inventário não for regenerado no mesmo commit do contrato novo, o problema original retorna. Adiciona superfície de sincronização e possibilidade de divergência entre código e artefato; pode tentar virar gate separado, contrariando a decisão de não criar required check.
- **Rabbit holes**: disputa sobre quem gera e quem confirma o inventário; inclusão do artefato em `validate/`/`release/`; tentativa de reconciliar as contagens históricas (10/58 vs 11/87) por meio do manifesto; promoção do gerador a check obrigatório.

## Recommendation

Recomendo a **Opção A**. Ela ataca diretamente as métricas do problema: um contrato `#[ignore]` sem lega passa a provocar falha do guarda (baseline: dois gaps descobertos só manualmente), divergência de piso ou lista vazia falha fechado, e a linha de sucesso pode passar a derivar da contagem descoberta em vez das constantes copiadas. Mantém o gate único sob ADR-0003 e o diff confinado ao guarda; a incerteza de tempo de CI e a cobertura global de órfãos ficam explícitas como premissas a validar, não como promessa de desempenho.

## Out of Scope (for the recommended option)
- Novo required check, alteração de `REQUIRED`/`ALLOWED_JOB_IF` ou promoção de scripts SDD a gate/evidência de verificação.
- Mudanças em `.github/workflows/rust.yml`, nas legs/pisos específicos ou nos próprios testes `#[ignore]`.
- Redesenho do modo `--remote` e das checagens estáticas existentes (jobs, `continue-on-error`, path filters, comandos `cargo build/test --workspace`).
- Afirmação de custo/tempo de compilação acrescentado ao gate; estado remoto **[NV]**.
- Reescrita do registro histórico dos ciclos 086/087/088 ou de `validate/`, `release/`, `STATE.md`, ADRs e constituição.
- Promessa de que toda a contagem exibida já vem da descoberta — a divergência entre decisão e código está em aberto nos artefatos.

## Assumptions to Validate

- `cargo test -p flora-tests -- --ignored --list` é determinístico e seguro no job `rust-build`, inclusive sem Postgres **[NV]**.
- A associação por prefixo (`startswith`) separa corretamente as legs atuais e futuras, sem órfãos nem testes que casam com mais de um filtro; cobertura global não foi verificada nos artefatos.
- Consumidores da linha `all N db contract floors intact (M total)` toleram a mudança para a contagem descoberta (checkout: 11/87; handoff: 10/58).
- A dependência de compilação do workspace de testes não acrescenta tempo/custo inaceitável ao gate — não medido **[NV]**.
- O modo `--remote` continua comparando os contextos required ao vivo como registrado; resultado ao vivo **[NV]**.
