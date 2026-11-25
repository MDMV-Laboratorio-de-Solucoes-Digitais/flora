# Concept: Evidência determinística para seleção de org

- **Slug**: 008-select-org-tests-and-dev-smoke
- **Created**: 2026-11-25
- **Recommended option**: Testes CI-safe e smoke dev documentado

## Options

### Option A — Testes CI-safe e smoke dev documentado

- **Sketch**: O ponto mais sensível da autenticação passa a ter testes determinísticos que rodam onde a CI já chega, e o caminho de verificação em dev deixa de depender de um provedor externo: com o banco semeado e um bootstrap local, qualquer pessoa consegue repetir `selecionar org → workspace → tarefa` e saber exatamente o que foi exercitado.
- **Appetite**: small
- **Trade-offs**: Menor coisa que poderia funcionar e ataca a falta de evidência sem mudar comportamento de runtime nem exigir infra nova. Casos que dependem de banco continuam sem automação e o smoke integrado permanece NV até execução observada; a nuance da listagem por responsável precisa ser documentada para não gerar leitura errônea.
- **Rabbit holes**: Transformar "teste verde" em prova de caminho exercitado; usar segredos reais no bootstrap; confundir task criada sem responsável com falha de criação; tentar alinhar frontend neste mesmo recorte.

### Option B — Cobertura DB-backed na CI

- **Sketch**: Os casos de sucesso com rotação, membership negada e sessão ausente/inativa passam a rodar automaticamente em um ambiente que fornece banco real, fechando a lacuna entre "a autoridade da sessão existe" e "regressões são detectadas".
- **Appetite**: medium
- **Trade-offs**: Cobre o ponto de autoridade com sinal contínuo, mas exige serviço de banco na CI ou testes condicionados com job separado; risco de instabilidade e de alterar o pipeline atual. Appetite médio com incerteza: a configuração remota não é observável localmente.
- **Rabbit holes**: Adicionar Postgres ao job existente e quebrar o pipeline atual; depender de conexão preguiçosa que mascara falha de banco; deixar testes concorrentes disputarem fixtures; transformar a opção em projeto de infraestrutura.

### Option C — Alinhamento frontend amplo agora (adiado)

- **Sketch**: Ajustar de uma vez a persistência do estado de login, a chamada de seleção de org, a troca do token rotacionado e os tipos de workspace/tarefa na interface.
- **Appetite**: medium
- **Trade-offs**: Elimina a divergência de contrato, mas desloca o foco de verificabilidade para UI e depende de backend estável; foi explicitamente deferido na decisão do ciclo.
- **Rabbit holes**: Reescrever telas e DTOs sem contrato fechado; perseguir paridade de UI em vez do caminho núcleo.

## Recommendation

Option A. Os goals do `problem.md` pedem caracterizar a ausência de testes dedicados como risco de regressão em ponto de segurança e tenancy, explicitar a falta de bootstrap de sessão/token como causa da não reprodutibilidade, delimitar a limitação da CI sem Postgres e tornar visível a nuance da listagem por responsável. A opção A entrega sinal determinístico onde é possível e registra o resto como NV, exatamente como decidido no ciclo 008; B é o próximo passo natural, não pré-condição deste.

## Out of Scope (for the recommended option)

- Detalhamento de implementação de testes, fixtures, scripts de bootstrap ou estrutura de documentação do smoke.
- OIDC real/JWKS completo, alinhamento frontend amplo, deploy, serviços pagos ou segredos reais como pré-condição.
- Afirmar testes DB-backed ou execução integrada do smoke sem evidência observável.
- Alterar comportamento de runtime da API como objetivo.

## Assumptions to Validate

- O seed local contém organização, usuário, memberships, workspaces e tarefas, mas nenhuma sessão — é preciso bootstrapar sessão/JWT com o mesmo segredo local.
- Os casos que não tocam banco (validação antes de acesso, autenticação ausente, org nula) rodam na CI atual; os demais ficam NV até infra dedicada.
- A listagem de tarefas filtra pelo responsável do usuário autenticado, então uma tarefa criada sem responsável pode não aparecer depois — isso é comportamento esperado, não falha.
- Existe toolchain/ambiente para executar o smoke documentado; se não houver, o resultado é registrado como NV.
