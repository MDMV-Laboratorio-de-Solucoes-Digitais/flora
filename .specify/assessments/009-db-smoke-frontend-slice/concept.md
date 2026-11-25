# Concept: Fluxo núcleo verificado de forma integrada

- **Slug**: 009-db-smoke-frontend-slice
- **Created**: 2026-11-25
- **Recommended option**: Cobertura DB e fatia mínima do frontend

## Options

### Option A — Registrar o smoke em stack equivalente

- **Sketch**: Usar a evidência já obtida em uma stack efêmera equivalente — com serviços em portas alternativas — como prova de que `selecionar org → workspace → tarefa` funciona, documentando o desvio em relação ao caminho oficial em vez de fingir que ele foi percorrido.
- **Appetite**: small
- **Trade-offs**: Menor coisa que poderia funcionar para comunicar o que foi realmente observado e evitar infundar confiança em evidência handcrafted. Não resolve a incompatibilidade do caminho documentado, protege mal a autoridade da sessão na CI e não permite que uma pessoa percorra o fluxo pela interface.
- **Rabbit holes**: Apresentar rc=0 em stack alternativa como se fosse freshly install; esconder a incompatibilidade de versão do banco; tratar desvio ambiental como permanente.

### Option B — Cobertura DB e fatia mínima do frontend

- **Sketch**: O fluxo núcleo passa a ter três frentes coerentes: os casos que dependem de banco são automatizados e executáveis em ambiente com Postgres, o smoke é provado com evidência registrada, e a interface acompanha o caminho feliz — entrar, escolher organização e criar/listar uma tarefa — sem exigir paridade ampla.
- **Appetite**: medium
- **Trade-offs**: Cobre a autoridade da sessão, reduz a distância entre "passou em stack artesanal" e "caminho reproduzível" e fecha o contrato mínimo entre as pontas. Custo médio-alto: exige job separado com banco, toolchain frontend disponível e decisões sobre compatibilidade da imagem do banco. Parte permanece NV neste ambiente — execução do job em CI remota, checks completos do frontend e compose documentado. Appetite médio com incerteza.
- **Rabbit holes**: Quebrar o pipeline atual ao injetar banco no job existente; transformar a fatia mínima em reescrita de UI; prometer compatibilidade de Postgres sem evidência no caminho documentado; incluir OIDC/JWKS completo, realtime avançado ou busca/files neste recorte.

### Option C — Reconstruir compose e buscar paridade total (não fazer)

- **Sketch**: Refazer o ambiente documentado e a migração para qualquer versão de banco, além de levar a interface à paridade com o backend de uma vez.
- **Appetite**: large
- **Trade-offs**: Resolve a compatibilidade na raiz e elimina divergências, mas toca migrations já aplicadas e extrapola o que pode ser verificado num ciclo 80/20; risco alto de quebra de histórico e de drift.
- **Rabbit holes**: Reescrever migração aplicada; perseguir UX/paridade ampla; equiparar "ambiente perfeito" a "fluxo verificado".

## Recommendation

Option B. Os goals do `problem.md` apontam para verificabilidade incompleta em condições próximas do uso real: cobrir casos DB de `select-organization`, distinguir smoke em stack alternativa de caminho documentado, registrar a incompatibilidade observada entre `postgres:16-alpine` e `uuidv7()` e tornar coerente o caminho feliz do frontend. B é o escopo entregue na decisão 009, com NV explícito onde o ambiente não permite evidência; separar o que foi observado (rc=0 em stack de portas alternativas) do que não foi (CI remota, toolchain frontend, compose documentado) é parte da recomendação.

## Out of Scope (for the recommended option)

- Versão final de Postgres, desenho do job de CI, estratégia de migração ou implementação detalhada das telas.
- Tratar compatibilidade `uuidv7` em Postgres 16/18 como resolvida sem evidência no caminho documentado.
- Afirmar que o job `db-tests` passou em CI remoto ou que check/lint/build/test do frontend foram integralmente executados sem registro.
- OIDC/JWKS e2e real, session authority no extrator, paridade frontend ampla e realtime/files/search avançados.

## Assumptions to Validate

- Existe ambiente descartável com Postgres suficiente para os casos DB; o job em runners remotos só pode ser confirmado após execução observável.
- A toolchain frontend (incluindo `bun`) está disponível para executar check/lint/build/test; se não estiver, a verificação determinística fica NV/parcial.
- O smoke pelo caminho documentado é bloqueado neste host por conflito de porta `5432`, tornando a stack de portas alternativas a evidência disponível.
- A migração inicial exige `uuidv7()`, nativo em Postgres 18 e ausente na imagem `postgres:16-alpine` do compose.
