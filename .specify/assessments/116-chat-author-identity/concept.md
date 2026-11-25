# Concept: Identidade real de autores no chat

- **Slug**: 116-chat-author-identity
- **Created**: 2026-11-25
- **Recommended option**: Autoria real com campos aditivos e resolução no canal autorizado

## Options

### Option A — Autoria real com campos aditivos e resolução no canal autorizado
- **Sketch**: As mensagens dos canais passam a mostrar o nome e as iniciais de quem as escreveu, no histórico, no envio e em tempo real, em vez de um prefixo hexadecimal do identificador. A informação de autoria aparece apenas para usuários do próprio canal/organização e chega ao cliente de forma compatível com versões anteriores, que continuam funcionando. A experiência também melhora para quem usa tecnologia assistiva.
- **Appetite**: medium (semanas); incerteza moderada: a mudança atravessa API, eventos em tempo real e cliente, e o score registrado foi 240/625 com risco 2, o que pede coordenação entre camadas.
- **Trade-offs**: Elimina a lacuna de usabilidade sem criar serviço novo, sem migration e sem mudar required checks. O custo é a coordenação entre listagem, envio, edição, exclusão e eventos, mais o risco de vazar dados de usuário se a resolução de autoria sair do limite do canal autorizado; também há risco de corrida visual ao trocar rapidamente de canal.
- **Rabbit holes**: Criar endpoint público de diretório de usuários; implementar mensagens diretas reais; alterar schema ou migrations; expor e-mail, identificador de provedor externo ou campos internos; tornar obrigatórios campos hoje opcionais.

### Option B — Manter o identificador abreviado no cliente
- **Sketch**: Nada muda para quem usa o chat: a autoria continua aparecendo como "Usuário" seguida de poucos caracteres do identificador, tanto nas mensagens principais quanto nas respostas de thread.
- **Appetite**: small (dias).
- **Trade-offs**: Custo zero e nenhum risco de privacidade, mas mantém a lacuna de compreensão e o problema de acessibilidade apontados no núcleo Seed.
- **Rabbit holes**: Maquiar o identificador no cliente para parecer nome; tratar a ausência de reclamações como validação da experiência atual.

### Option C — Serviço de identidade compartilhado
- **Sketch**: Em vez de resolver autoria apenas dentro do chat, cria-se uma frente de identidade reutilizável com busca e perfis, capaz de atender outras áreas do produto no futuro.
- **Appetite**: large (meses); incerteza alta: escopo de privacidade, governança e contrato ainda não foi definido.
- **Trade-offs**: Evitaria duplicação futura e daria base a outras superfícies, mas amplia muito o problema original, introduz decisões de exposição de dados e exigiria autorização e desenho de produto que não foram concedidos neste ciclo.
- **Rabbit holes**: Abrir diretório público; incluir mensagens diretas; redesenhar modelo de usuário; tocar required checks e pisos do guarda; reabrir decisões de tenant.

## Recommendation

Recomendo a **Option A**. Ela resolve a lacuna verificada no núcleo Seed com o menor conjunto que entrega valor: nomes reais (`Alice Admin`, `Bob Builder`) visíveis no smoke do canal `# general`, sem N+1 na resolução de autoria, sem exposição de campos sensíveis e com compatibilidade retroativa nas respostas REST e nos eventos em tempo real. É condição deste conceito que a resolução permaneça restrita a mensagens de canais já autorizados — o Briefing de Compreensão de `tenant` deve comprovar que não há enumeração arbitrária de usuários — e que a atualização do piso de `messaging` seja feita em lockstep com o guarda e o fluxo de CI. SemVer MINOR, nenhuma migration e nenhum required check novo. Comportamento remoto permanece **[NV]**.

## Out of Scope (for the recommended option)

- Endpoint público de diretório de usuários e criação de mensagens diretas reais.
- Alteração de migrations, schema ou contrato existente que não seja aditivo e opcional.
- Exposição de senha, identificador externo, perfil ou e-mail quando houver nome de exibição.
- Required checks, pisos gerais do guarda além do lockstep acordado e deploy.
- Afirmar comportamento em produção/staging sem observação.

## Assumptions to Validate

- A verificação de pertencimento ao canal/organização continua sendo a autoridade antes de qualquer resolução de autoria.
- A resolução em lote não introduz consulta repetida por mensagem.
- O cliente tolera ausência do nome e cai de volta para a representação antiga sem quebrar mensagens legadas.
- A troca rápida de canal não mistura autoria de canais diferentes.
- Existem usuários de seed com nomes conhecidos e mensagens no canal usado pelo smoke.
- O Briefing de Compreensão de `tenant` comprova a impossibilidade de enumerar usuários fora dos canais autorizados.
- A atualização do piso de `messaging` em lockstep com o guarda e o CI é aceita como parte da mudança.
