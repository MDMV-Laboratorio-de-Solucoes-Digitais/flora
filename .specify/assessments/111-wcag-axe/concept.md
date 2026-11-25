# Concept: Varredura automatizada de acessibilidade

- **Slug**: 111-wcag-axe
- **Created**: 2026-11-25
- **Recommended option**: Varredura leve nos fluxos já cobertos

## Options

### Option A — Varredura leve nos fluxos já cobertos
- **Sketch**: Ao executar o fluxo autenticado de ponta a ponta que o projeto já usa, um relatório adicional aponta violações das regras WCAG 2.2 AA nas duas rotas representativas do dia a dia. Os achados aparecem em linguagem legível no log, em português, e alimentam correções posteriores. O job continua informativo: a varredura avisa, mas não decide o merge.
- **Appetite**: small (dias); incerteza: a cobertura das regras automatizadas é parcial, então o resultado não substitui a revisão humana prevista em FR-07.
- **Trade-offs**: Torna a verificação reproduzível e detecta regressões que a revisão manual não perceberia, reaproveitando sessão e rotas já existentes. Em troca, cobre apenas parte das regras e das páginas, e depende de um resultado determinístico; há risco de falso positivo de contraste em tema escuro e de divergência entre.lockfiles de desenvolvimento.
- **Rabbit holes**: Estender a varredura a todas as rotas; promover o job a required check sem autorização; relaxar regras para zerar achados; perseguir falso positivo de contraste com mudanças visuais fora de escopo; resolver divergência de lockfiles sem critério de fonte de verdade.

### Option B — Manter apenas a revisão humana
- **Sketch**: Nenhuma verificação automatizada entra no fluxo. A conformidade continua dependendo da revisão manual das telas e do julgamento de quem a faz.
- **Appetite**: small (dias).
- **Trade-offs**: Não adiciona dependência de desenvolvimento nem custo de CI, mas o sinal permanece lento, não reproduzível e cego a regressões introduzidas depois da revisão.
- **Rabbit holes**: Transformar a revisão manual em gate; usar ausência de achados reportados como prova de conformidade.

### Option C — Conformidade AA como condição de merge
- **Sketch**: A verificação automatizada passa a ser obrigatória e a cobrir o frontend inteiro, com resultado bloqueando o merge sempre que houver violação.
- **Appetite**: large (meses); incerteza alta: exige autorização de governança e uma política para resultados incertos que a automação não consegue classificar.
- **Trade-offs**: Daria sinal forte e contínuo, mas muda required checks, contraria o enquadramento ADAPT e pode gerar falsa segurança: a automação não equivaleria a auditoria AA completa nem eliminaria a necessidade de revisão humana.
- **Rabbit holes**: Perseguir 100% de critérios AA automatizáveis; criar exceções e listas de permissão para driblar o bloqueio; reabrir required checks e pisos do guarda; auditar todas as rotas de uma vez.

## Recommendation

Recomendo a **Option A**. Ela materializa F9/FR-06/FR-08 sem ultrapassar a autorização concedida: duas rotas autenticadas já usadas pelo smoke, cinco tags WCAG, dependência de desenvolvimento sincronizada nos lockfiles e nenhum required check novo; `e2e-browser` permanece advisory e o guarda do gate segue autoritativo. Os três achados reais do primeiro run viraram bug registrado e foram tratados no ciclo seguinte, o que mostra que o sinal informativo foi acionável. Não se deve afirmar conformidade AA plena nem comportamento em staging/produção sem observação — ambos permanecem **[NV]**.

## Out of Scope (for the recommended option)

- Auditoria humana completa e correção de todos os achados neste ciclo.
- Required checks novos, alteração de gate, pisos ou migrations.
- Deploy, credenciais no repositório ou em logs e dados temporais no teste.
- Afirmar conformidade WCAG 2.2 AA plena ou comportamento remoto não observado.
- Cobertura de rotas além das duas representativas e promoção do job a bloqueante.

## Assumptions to Validate

- As rotas e a sessão já usadas pelo smoke são representativas e estáveis para a varredura.
- O resultado é determinístico e legível em pt-BR na mensagem de asserção.
- A dependência de desenvolvimento pode ser adicionada mantendo os lockfiles sincronizados.
- O job `e2e-browser` continua advisory e os quatro required checks permanecem inalterados.
- Falsos positivos de contraste serão registrados como dívida, sem relaxar regras.
- A revisão manual de critérios não automatizáveis continuará sendo registrada separadamente.
