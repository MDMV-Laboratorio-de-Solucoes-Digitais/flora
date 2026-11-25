# Problem Definition: Busca autoritativa ambígua e FTS incompleto

- **Slug**: 023-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | problem.md anterior

## Problem Statement

Após 022 (baseline `dev@023bd2467`, PR #225/#226), `GET /api/v1/search` prefere Meilisearch sempre que `search_service` está configurado, mas nenhum caminho de código escreve no índice: `index_task`, `index_file`, `index_message` e `delete_document` não têm call sites e `setup_index` nunca é invocado (`problem.md anterior:3`; `research.md:8`). Um Meilisearch configurado devolve, portanto, resultados vazios ou obsoletos enquanto aparenta saúde; a degradação do 022 cobre apenas erros de consulta, não respostas bem-sucedidas e vazias (`problem.md anterior:3`; `research.md:15`). Há ainda um defeito latente de filtro de tipo (plural no filtro contra singular nos documentos) que geraria zero resultados silenciosos caso o índice fosse populado (`research.md:8`).

Esse arranjo é um risco de correção e entra em conflito com o perfil Seed definido pela constituição — Flora + PostgreSQL, com Meilisearch no tier Grove (`intake.md:5`; `research.md:7`). A alternativa de manter o Meilisearch como backend confiável exigiria hooks de mutação, espera por tarefas assíncronas, rebuild e CI determinística, fora da proporção 80/20 para um componente fora do tier (`research.md:8-10`; `[NV]` sobre o comportamento documentado do SDK externo).

Em paralelo, as colunas geradas de FTS tokenizam nomes hifenizados de arquivos e tarefas como um único token sob a configuração `simple`, de modo que consultas por sub-token comuns em nomes de arquivo não encontram correspondência (`problem.md anterior:5`; `research.md:21`). Os padrões de configuração divergem (`enabled=true` no código contra `false` em `.env.example`) e a resposta da API não explicita qual backend produziu o resultado nem como ele degrada (`problem.md anterior:5`; `research.md:12`).

## Affected Users & Stakeholders

- **Users**: usuários que buscam tarefas e arquivos com nomes hifenizados; operadores que configuram `FLORA_SEARCH__*` e esperam comportamento coerente com o perfil Seed. [ASSUMPTION: perfis inferidos de `problem.md anterior:3-5` e `research.md:12`; não há relatos de usuários registrados.]
- **Stakeholders**: mantenedores da constituição e do contrato de busca, operadores de VPS de 2 vCPU/4 GB, equipe de CI e de documentação de configuração.

## Goals

- Eliminar a dependência da correção da busca em um índice que não é populado, de modo que o comportamento observado seja previsível tanto com quanto sem Meilisearch configurado (`research.md:8,15`).
- Recuperar a capacidade de encontrar nomes compostos por separadores comuns sem exigir serviços adicionais (`research.md:21`).
- Alinhar configuração e documentação ao perfil Seed e tornar explícito, na resposta da API, o backend que originou os resultados (`problem.md anterior:5`; `research.md:12`).

## Non-Goals

- Fora do problema: hooks de indexação no Meilisearch, propagação de exclusão lógica, wrapper de espera por tarefas, rebuild/backfill, swap blue-green, CI ancorada no Meilisearch e paridade entre engines (`research.md:22`; `decision.md:28`).
- Também ficam de fora: relevância via `pg_trgm`/i18n/CJK, ACL de busca, validação de carga/restauração em 2vCPU/4GB, adoção no frontend, novos serviços obrigatórios e remoção de crates (`decision.md:28-30`). A reativação futura do Meilisearch é uma decisão isolada, não parte deste enquadramento (`research.md:10`).

## Success Metrics

- A busca retorna resultados do backend transacional mesmo quando um `SearchService` está configurado, inclusive em fixture inalcançável (baseline: preferência por Meilisearch não escrito — `problem.md anterior:3`; `research.md:8`).
- Consultas por sub-token de nome hifenizado encontram o registro correspondente (baseline: token único sob `simple` — `problem.md anterior:5`).
- Configuração padrão e `.env.example` apresentam o mesmo valor para a capacidade de busca (baseline: `true` contra `false` — `research.md:12`).
- A resposta identifica a origem dos resultados e o contrato de degradação está documentado (baseline: campo ausente — `problem.md anterior:5`; `research.md:20`).

## Cost of Inaction

Um backend preferencial vazio pode fazer aplicações acreditarem que não há resultados quando há dados no Postgres; a falha é silenciosa e não é coberta pela degradação existente. Nomes de arquivos com separadores comuns permanecem difíceis de encontrar, e a divergência de configuração leva operadores a habilitar um componente que o próprio perfil Seed não suporta. Sem contrato explícito de capacidade, cada deploy interpreta a precedência de busca a seu modo, dificultando suporte e evolução.

## Open Questions

- [NEEDS CLARIFICATION: a adição de um campo de origem na resposta é compatível com consumidores externos não canônicos? `research.md:20` registra apenas que o frontend canônico não consome busca.]
- [NEEDS CLARIFICATION: qual nível de relevância para hifenização, CJK e `pg_trgm` é considerado suficiente? `research.md:21` trata o restante como NV, sem meta definida.]
- [NEEDS CLARIFICATION: qual dos valores de configuração (`true` no código ou `false` no `.env.example`) representa a intenção oficial? `research.md:12` aponta a divergência sem atribuir autoridade.]
- [NEEDS CLARIFICATION: a reativação do Meilisearch deve ter gatilho e critérios documentados já neste ciclo ou pode ser diferida integralmente? `research.md:10` registra apenas o diferimento.]
