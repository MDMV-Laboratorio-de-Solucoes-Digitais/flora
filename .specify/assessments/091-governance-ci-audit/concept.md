# Concept: Auditoria viva da governança do CI

- **Slug**: 091-governance-ci-audit
- **Created**: 2026-11-25
- **Recommended option**: Auditoria viva, documental e não mutante dos cinco compromissos

## Options

### Option A — Auditoria viva, documental e não mutante dos cinco compromissos
- **Sketch**: o maintainer passa a dispor de um registro único que responde, ponto por ponto, se a prática coincide com o que a governança afirma: quais checks são exigidos e como `dev`/`main` estão protegidos, se a emenda vigente do ADR-0003 vale sem aprovador humano, se o deploy é uma decisão separada do gate, o que foi realmente verificado e o que ficou diferido, e se os scripts da factory continuam fora de relatórios como gate ou evidência. Tudo é escrito como explicação auditável, com fatos verificados separados de itens **[NV]**, sem mudar o que o CI exige.
- **Appetite**: medium — cinco frentes de verificação, redação de governança e um workflow advisory já existente como referência; incerteza: estado ao vivo de CI/proteção/deploy **[NV]**, e audiência/periodicidade do relatório não foram definidas nos artefatos.
- **Trade-offs**: ganha rastreabilidade e uma linguagem comum entre prática e documentação, preservando o merge autônomo sob CI green e a autoridade exclusiva de `check_gate_integrity.py`. Sacrifica a pretensão de verificação independente plena: a auditoria se apoia em registros locais e em leituras pontuais, e o workflow diário só sinaliza divergência — não bloqueia merge. Mantém itens diferidos visíveis sem resolvê-los, o que exige disciplina para não transformar o relatório em promessa de verde não observado.
- **Rabbit holes**: transformar a auditoria em required check, em aprovador humano disfarçado ou em promoção de scripts SDD/advisories a gate; perseguir o estado ao vivo de Coolify/Forgejo sem autorização ou tentar reescrever o registro histórico para "corrigir" fotografias antigas; reabrir emendas pendentes e ações de deploy/rollback dentro do ciclo; copiar ou exibir credenciais; alargar o diff a `validate/`, `release/`, ADRs, constituição ou código.

### Option B — Confiar no registro existente e no workflow advisory (a menor coisa que poderia funcionar)
- **Sketch**: não criar auditoria nova: manter a ADR-0003 com a emenda, o gate autoritativo e o workflow diário de verificação de proteção de branch, interpretando seus sinais quando alguém olhar. Para o mantenedor, nada muda além da ausência de um relatório consolidado.
- **Appetite**: small — nenhum artefato novo, apenas uso do que já existe.
- **Trade-offs**: custo zero e nenhum risco de mutação de processo, mas não produz os cinco compromissos rastreáveis nem separa fatos de itens **[NV]**; a verificação continua circular (os registros do próprio ciclo como fonte) e dependente de alguém ler o sinal do job advisory. Não atende à métrica de auditoria viva e deixa divergências entre proteção real e documentação passarem despercebidas.
- **Rabbit holes**: tratar a ausência de divergência sinalizada como prova de coerência; usar o job diário como justificativa para adiar a reconsulta de estado ao vivo; transformar o silêncio do job em gate informal.

### Option C — Painel contínuo com acompanhamento dos itens diferidos
- **Sketch**: evoluir a auditoria pontual para um painel recorrente que acompanhe também os itens em aberto (Tauri/OTLP, leg RBAC, emendas pendentes, health de deploy), com responsáveis e prazos de reavaliação visíveis. O mantenedor veria não só o estado atual, mas o que ainda depende de decisão.
- **Appetite**: medium a large — envolve definir periodicidade, audiência, formato de publicação e manutenção do painel; sem evidência nos artefatos de que essa demanda tenha sido colocada.
- **Trade-offs**: reduz o risco de itens diferidos esquecidos e torna a auditoria realmente "viva" no tempo, mas adiciona processo e superfície de governança; a linha entre sinalizar e cobrar é fina, e o acompanhamento pode esbarrar na regra de não reintroduzir aprovador humano. Audiência e periodicidade são perguntas abertas, então o custo não pode ser estimado com segurança.
- **Rabbit holes**: criar SLA ou donos formais para itens diferidos; publicar o painel em canal externo; exigir verificação humana antes do merge; transformar o painel em fonte de verdades concorrentes com ADRs e STATE.

## Recommendation

Recomendo a **Opção A**. Ela entrega as cinco compromissos rastreáveis e a separação entre fatos verificados e itens **[NV]** exigidos pelo problema, sem criar required check, sem reintroduzir aprovador humano, sem promover scripts SDD e sem tocar em gate, ADRs, PRs ou produção. Preserva o gate autoritativo e a regra de CI green para merge autônomo, que são as métricas de sucesso do ciclo. A opção é documental por desenho; sua principal limitação — depender de registros locais e de estado ao vivo não observado — fica declarada como premissa a validar, não disfarçada de verde.

## Out of Scope (for the recommended option)

- Novos required checks, promoção de advisories ou dos 4 scripts SDD da factory a gate/evidência de verificação, e reintrodução de aprovador humano.
- Mudanças em gate, ADRs, constituição, código de aplicação, workflows existentes, PRs, merge, tags, deploy e produção.
- `validate/` e `release/` do próprio ciclo; reescrita do registro histórico ou reivindicação de verde não observado.
- Execução ou reexecução de deploy, rollback, promoção de emendas pendentes e reconsulta autorizada de estado ao vivo de Coolify/Forgejo.
- Exposição, cópia ou gravação de credenciais; PAT e secret apenas referidos por nome.
- Definição de audiência/periodicidade de um painel contínuo (questão em aberto, não assumida como decidida).

## Assumptions to Validate

- O estado ao vivo de required checks e da proteção de `dev`/`main` corresponde ao registro (4 checks com *strict* e `enforce_admins`, sem required reviews) — CI/proteção remotos **[NV]**.
- A emenda de 2026-11-20 à ADR-0003 é a regra vigente; demais emendas permanecem escaladas **[NV]** a maintainer, sem execução.
- Deploy de produção via ligação MDMV-Autonoma é uma decisão separada, com health e imagem de rollback preservados; estado ao vivo **[NV]** e registros posteriores (094/095) são fotografias datadas, não estado corrente.
- A audiência e a periodicidade do relatório de auditoria não foram quantificadas nos artefatos; generalização para múltiplos usuários não é suportada.
- A ausência de captura reproduzível da leitura `gh` nos artefatos enfraquece a independência da auditoria; separar registro de evidência ao vivo é necessário antes de promoção.
