# Concept: Sessão só com identidade verificada

- **Slug**: 024-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Porta única de sessão verificada

## Options

### Option A — Porta única de sessão verificada
- **Sketch**: Toda sessão passa a nascer somente de um token cuja integridade e cujos claims obrigatórios foram conferidos: conteúdo alterado ou claim inválido não vira sessão. A identidade passa a ser o sujeito estável do token, e não o e-mail informado, e uma configuração incompleta falha rápido em vez de ser tratada como confiável. A verificação é demonstrável de forma determinística com dados locais e um endpoint simulado, sem provedor de identidade ao vivo.
- **Appetite**: medium
- **Trade-offs**: Fecha a única lacuna que pode admitir sessão não autorizada e transforma "Users & Roles (OIDC)" em afirmação com evidência executável, aproveitando a infraestrutura de sessão já existente. Em troca, exige decidir fontes de verdade para emissor, audiência e redirecionamentos, e é maior que a checagem mínima por incluir descoberta de chaves.
- **Rabbit holes**: transformar o corte em exercício de rotação de chaves e integração com provedor real; criar um subsistema de administração para a allowlist de redirecionamento; migrar o histórico de identidades por e-mail; mexer no frontend ou no trabalho paralelo em `/metrics`.

### Option B — Verificação mínima com chave configurada
- **Sketch**: A menor coisa que poderia funcionar: conferir assinatura e claims obrigatórios contra uma chave configurada explicitamente, sem descoberta automática. Para quem entra, o resultado é o mesmo — token inválido não entra —, mas o operador passa a trocar a chave manualmente quando o provedor muda.
- **Appetite**: small
- **Trade-offs**: Entrega a proteção central com menos superfície e evidência local fácil de reproduzir; sacrifica a adaptação automática a novas chaves e deixa a decisão de configuração nas costas do operador. Sozinha, não resolve a identidade inferida por e-mail nem a ausência de allowlist de redirecionamento.
- **Rabbit holes**: tratar a chave estática como solução definitiva em produção; acrescentar gestão de segredos e rotação "de graça"; usar a ressalva de validação TLS do perfil local como desculpa para pular `iss`/`aud`/`exp`/`nonce`.

### Option C — Terceirizar a identidade
- **Sketch**: Não fazer a verificação localmente: comprar/aderir a um provedor gerenciado que emita e valide a sessão, reduzindo a autenticação do produto a uma integração.
- **Appetite**: large (incerto: sem evidência de contrato, custo ou disponibilidade)
- **Trade-offs**: Remove a lacuna sem manter verificação criptográfica própria, mas introduz serviço obrigatório, custo e dependência de rede — em choque com a restrição de nenhum serviço novo e VPS barato — e não garante evidência determinística sem um provedor ao vivo.
- **Rabbit holes**: migração de contas e histórico de identidades, lock-in contratual e exigência de disponibilidade externa para provar o marco.

## Recommendation

Recomendo a **Option A**: ela ataca diretamente as métricas do problema — token manipulado ou com `iss`, `aud`, `exp` ou `nonce` inválidos não estabelece sessão; a identidade deriva de `sub`; existe evidência determinística com fixtures locais e endpoint mockado — e é a que converte o estado "PARTIAL" de "Users & Roles (OIDC)" em IMPLEMENTED com comportamento executável. A Option B é um bom degrau, mas deixa duas métricas sem fecho; a Option C quebra a restrição de nenhum serviço obrigatório novo.

## Out of Scope (for the recommended option)

- Aplicação da FASE 0 no CI e contabilização honesta de OTel.
- ADR de decisões do Seed e reconciliação ampla de documentação.
- Mudança de transporte de notificações, RustFS/S3 e reativação do Meilisearch.
- E2E de browser/Tauri, testes com provedor real e exercícios de rotação de JWKS.
- Novos serviços obrigatórios, alterações no frontend e interferência no trabalho paralelo em `/metrics`.

## Assumptions to Validate

- Qual emissor e audiência são canônicos quando o provedor real for configurado (o padrão local não define o valor de produção).
- Como tratar o histórico de identidades por e-mail ao migrar para `sub` (unicidade e backfill de `oidc_subject` são NV).
- Se a ressalva de validação TLS se aplica a algum perfil além do padrão; no perfil local ela não substitui assinatura nem claims.
- Quem administra a fonte de verdade da allowlist de redirecionamento.
- Que a infraestrutura de sessão existente (PKCE S256, store de uso único, sessões em banco, rotação de `jti`) permanece compatível.
