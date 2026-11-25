# Concept: Caminho dev documentado verificável

- **Slug**: 010-compose-uuidv7-smoke-frontend-verify
- **Created**: 2026-11-25
- **Recommended option**: Caminho documentado evidenciado com gates do frontend

## Options

### Option A — Alinhar compose/docs ao PostgreSQL 18 e ajustar os testes do frontend

- **Sketch**: O checkout limpo volta a subir pelo caminho documentado: a imagem do banco no compose e as instruções passam a refletir a versão que a migração inicial realmente exige, e a configuração dos testes do frontend passa a enxergar os testes que já existem. É a menor mudança que devolve coerência entre o que está escrito e o que roda.
- **Appetite**: small
- **Trade-offs**: Corrige a causa da falha em fresh install sem tocar em migrações já aplicadas, com risco baixo e reversível. Não evita por si só a restrição de porta do host nem produz evidência dos gates do frontend; ainda é preciso executar o smoke e os checks para transformar coerência em prova.
- **Rabbit holes**: Editar a migração de inicialização para acomodar Postgres 16 e quebrar checksum/histórico do `sqlx`; esquecer a nota de reset de volume PG16→18 e a nota de chave do Meilisearch; incluir testes de browser no projeto de testes do servidor.

### Option B — Caminho documentado evidenciado com gates do frontend

- **Sketch**: Além do alinhamento de compose e documentação, o fluxo `selecionar org → workspace → tarefa` é executado e registrado pelo caminho compatível, com desvio ambiental explícito quando a porta padrão está ocupada, e os checks determinísticos do frontend são executados e registrados para a fatia já entregue. Operador e desenvolvedor passam a ter um caminho que funciona e uma evidência que acompanha a documentação.
- **Appetite**: medium
- **Trade-offs**: Fecha a distância entre o smoke em stack alternativa do ciclo 009 e o caminho oficial, e dá sinal determinístico ao slice auth→org→workspace→task. Custo médio: exige stack isolada/portas equivalentes, `.env` e segredo JWT consistentes, toolchain disponível e registro honesto de NV. Appetite médio com incerteza: porta `5432` ocupada externamente e volumes PG16 existentes não são controláveis aqui.
- **Rabbit holes**: Usar banco não descartável na porta ocupada; prometer execução literal na porta padrão como se tivesse acontecido; perseguir OIDC/JWKS e2e real, paridade ampla de UI ou CI remoto além do PR; corrigir testes defasados sem limite de escopo.

### Option C — Polyfill/novo baseline para Postgres 16 (não fazer)

- **Sketch**: Manter o compose em Postgres 16 e introduzir função/extensão compatível ou redefinir o baseline de migrações para que o fresh install funcione nessa versão.
- **Appetite**: large
- **Trade-offs**: Evita atualizar a imagem, mas exige bootstrap pré-migração, cuidado com monotonicidade e disponibilidade em imagem, e risco de quebrar histórico `sqlx`; o CI `db-tests` já usa Postgres 18, então aumentaria a divergência.
- **Rabbit holes**: Alterar migração aplicada; implementar `uuidv7` caseiro; conviver com compose, docs e CI em versões diferentes.

## Recommendation

Option B. Os goals do `problem.md` pedem caracterizar a incompatibilidade entre a versão de Postgres do compose/documentação e o requisito `uuidv7()` da migração inicial, distinguir evidência do caminho documentado da stack alternativa já observada, reconhecer porta ocupada e `.env`/volumes ausentes como condições ambientais, e delimitar a lacuna dos checks determinísticos do frontend. B atende isso preservando o limite registrado: não editar migrações `sqlx` aplicadas. Os fatos da opção são mantidos — compose para `postgres:18-alpine`, docs com requisito PG18/reset de volume PG16→18/conflito de portas e nota de chave do Meilisearch, bootstrap de token coerente com o segredo do `.env.example`, projeto `server` do Vitest cobrindo `tests/**` com exclusão de `tests/e2e/**`, e gates `check`, `lint`, `test:unit -- --run --project server` e `build`. Onde o ambiente bloqueia, registra-se NV.

## Out of Scope (for the recommended option)

- Editar migração `sqlx` já aplicada ou implementar polyfill/extensão `uuidv7` para PG16 no runtime.
- OIDC/JWKS e2e real, session authority no extrator, paridade/UX ampla de frontend e CI remoto além do PR.
- Afirmar smoke na porta padrão, compatibilidade com volumes PG16 ou gates de frontend executados com sucesso sem evidência.
- Mudanças de produto fora do caminho dev/smoke e da verificação da fatia auth→org→workspace→task.

## Assumptions to Validate

- `uuidv7()` é nativo em PostgreSQL 18 e ausente em 16; a migração inicial falha em fresh install na imagem antiga, enquanto o CI `db-tests` já usa Postgres 18.
- A imagem `postgres:18-alpine` está disponível localmente; a porta `5432` está ocupada por processo externo, enquanto `6379`/`7700`/`3000` estão livres e não há containers/volumes da Flora.
- Volumes PG16 existentes exigem reset consciente em ambiente descartável, e a nota de chave do Meilisearch no bootstrap documentado precisa ser coerente.
- O projeto `server` do Vitest inclui `src/**/*.{test,spec}` mas pode não cobrir `tests/unit|integration/**`, e alguns testes podem estar defasados após o ciclo 009.
- A toolchain com `bun` está disponível para executar os quatro checks do frontend; a entrega depende de PR para `dev`, CI verde, merge e sync do mirror.
