# Concept: Slice seguro de identidade e tenant

- **Slug**: 006-secure-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Eliminar a confiança em cabeçalhos crus

## Options

### Option A — Eliminar a confiança em cabeçalhos crus

- **Sketch**: Nas rotas críticas, a identidade e a organização de quem chama passam a vir de um contexto verificado no servidor, nunca de valores que o cliente pode inventar. Operações sobre workspaces, canais, mensagens, tarefas, arquivos, busca e notificações só acontecem quando a associação da pessoa à organização e ao recurso foi checada; credenciais inválidas são recusadas explicitamente.
- **Appetite**: small
- **Trade-offs**: Menor coisa que poderia funcionar e ataca o risco P0 diretamente, com evidência determinística local disponível. Não torna o fluxo pós-login utilizável sozinho: a sessão ainda chega sem organização e o contrato com a interface continua desalinhado. Risco aceito: clientes antigos baseados em cabeçalhos quebram e precisam de migração compatível registrada.
- **Rabbit holes**: Refatorar todos os handlers de uma vez; eliminar clientes header-based sem registrar a quebra; confundir "extrator existe" com "contexto verificado em toda rota"; tentar resolver a seleção de organização neste mesmo recorte.

### Option B — Slice seguro com caminho núcleo verificável

- **Sketch**: Além da identidade verificada, o caminho feliz — entrar, cair em um workspace utilizável e completar uma tarefa ou mensagem — passa a ser percorrível de ponta a ponta com um contrato coerente entre as pontas, com evidência observável do fluxo núcleo.
- **Appetite**: medium
- **Trade-offs**: Fecha a distância entre servidor seguro e uso real, reduzindo retrabalho na integração. Custo maior: exige alinhar estado de login, contrato de org/workspace e a fatia escolhida (REST primeiro; mensagens acrescentam discovery e realtime), além de runtime integrado para provar o E2E — hoje NV. Appetite médio com incerteza.
- **Rabbit holes**: Tornar o alinhamento de contrato uma reescrita de frontend; priorizar canal/mensagem antes de haver discovery/WS estável; prometer E2E de UI sem ambiente integrado; ampliar RBAC fino e fanout completo.

### Option C — Reescrita ampla de autenticação e interface (não fazer)

- **Sketch**: Redesenhar toda a autenticação, o modelo de sessão e a interface numa única investida, buscando coerência total de uma vez.
- **Appetite**: large
- **Trade-offs**: Coerência eventual máxima, mas sem evidência parcial no meio do caminho e com risco alto de quebra de clientes e de drift aumentado; incompatível com o momento do Seed.
- **Rabbit holes**: Trocar o modelo de sessão antes de provar as rotas; migrar UI e WS em paralelo; perseguir paridade de providers externos.

## Recommendation

Option A. O goal central do `problem.md` é tratar a derivação de identidade/tenant a partir de dados controlados pelo cliente como risco de segurança e isolamento, e esclarecer o que significa "slice seguro" em comportamento observável — identidade validada, membership e tenancy do recurso checados. A opção entrega isso com os gates determinísticos já disponíveis e explicita o que permanece NV (E2E, integração runtime). A sessão sem organização e o contrato frontend↔backend são problemas vizinhos tratados nos ciclos seguintes, não pré-condição para eliminar o risco P0.

## Out of Scope (for the recommended option)

- Refatoração detalhada de handlers, assinatura de extratores ou desenho de endpoints.
- Seleção de organização, rotação de sessão/token e wiring completo de UI/WS.
- Afirmar E2E de UI ou integração com Postgres/Valkey/Meilisearch/OIDC sem evidência observada.
- Eliminação abrupta de clientes header-based sem registrar risco de quebra e migração compatível.

## Assumptions to Validate

- Os gates locais já executados (fmt/check e testes de biblioteca) continuam representando o estado atual do código.
- Existe estratégia de migração aceitável para `test-flora.sh` e para o frontend atual, seja fallback explícito em dev seja quebra imediata documentada.
- Uma fatia núcleo pode ser escolhida depois (`workspace → tarefa` ou `workspace → canal → mensagem`) sem comprometer a eliminação da confiança em headers.
- Há ambiente para validar o fluxo com runtime integrado; se não houver, o resultado segue NV.
