# Problem Definition: Semântica de atribuição e precedência de busca inconclusivas

- **Slug**: 021-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | problem.md anterior

## Problem Statement

Após 020 (baseline `dev@0dcde353a`, PR #223), a atribuição de tarefas permanece apenas parcialmente correta. O contrato do `PATCH` não consegue expressar retirada de atribuição: `assignee_id` ausente e `null` colapsam na mesclagem atual, de modo que transições `Some -> None` são impossíveis e as semânticas de notificação para unassign não podem sequer ser representadas (`research.md:6`; `problem.md anterior:3`). As notificações de atribuição também não dispõem de uma guarda simples contra sinais não-lidos duplicados óbvios em transições ou retries repetidos (`research.md:7`; `problem.md anterior:3`).

Na busca, o fallback PostgreSQL passou a cobrir tarefas e arquivos, mas o comportamento de borda do contrato — normalização de tokens de tipo, limites e expectativas determinísticas — não está totalmente fixado por testes (`research.md:8`; `problem.md anterior:5`). Em paralelo, o modo Meilisearch não possui indexação de mutação conectada para tarefas, arquivos ou mensagens, o que cria ambiguidade sobre qual backend é precedente e sobre a correção multi-entidade em deployments opcionais (`research.md:8`).

O problema central é a falta de uma semântica explícita e verificável para "não alterar", "retirar" e "definir" responsável, combinada com uma expectativa de confiabilidade de busca que hoje depende de qual backend está configurado.

## Affected Users & Stakeholders

- **Users**: responsáveis que precisam atribuir, reatribuir ou retirar atribuição de uma tarefa; destinatários de notificações não-lidas; usuários de busca em instâncias com e sem Meilisearch. [ASSUMPTION: perfis inferidos de `intake.md:8` e `research.md:6-8`; nenhuma entrevista registrada.]
- **Stakeholders**: mantenedores do contrato da API de tarefas, operadores de deployments Seed e opcionais com Meilisearch, equipe de contratos DB-gated/CI.

## Goals

- Tornar a intenção de atribuição inequívoca no contrato, distinguindo ausência de alteração, retirada e definição sem depender de comportamento implícito de mesclagem (`research.md:6`).
- Garantir que o sinal de atribuição represente apenas transições efetivas e não produza ruído duplicado óbvio em retries (`research.md:7`).
- Fixar com evidência determinística qual comportamento de busca é considerado a base de correção e tornar explícita a relação entre os dois backends (`research.md:8-10`).

## Non-Goals

- Não fazem parte do problema: paridade de indexação de mutação do Meilisearch para tarefas/arquivos/mensagens, testes determinísticos ancorados no Meilisearch, equivalência de ranking e snippets entre engines, `pg_trgm`/i18n/CJK, ACL por canal/workspace (`research.md:15`; `decision.md:32`).
- Também ficam fora: constraint de unicidade, outbox transacional, idempotência forte, tipo de evento específico para unassign, mudanças na política de associação do responsável, entrega em tempo real/push/menções, interface frontend, novos serviços obrigatórios e infra multi-nó (`decision.md:33-35`). A mudança de significado de `null` é uma evolução de contrato e deve ser tratada como tal, não como detalhe interno (`research.md:13`).

## Success Metrics

- O contrato de atualização de tarefa diferencia "omitido", `null` e valor, permitindo representar `Some -> None` (baseline: ausente e `null` colapsam; unassign impossível — `research.md:6`; `problem.md anterior:3`).
- Transições de atribuição, reatribuição, retirada, self-assign e no-op têm comportamento de notificação observável e distinto, sem duplicata não-lida óbvia (baseline: guarda de dedup ausente — `research.md:7`; `problem.md anterior:3`).
- Contratos DB-gated em PostgreSQL 18 descartável cobrem transições de atribuição e casos de borda da busca, incluindo normalização, limites, tenant e soft-delete (baseline: cobertura parcial herdada do 020 — `research.md:10`; `problem.md anterior:5`).
- A precedência entre backends de busca está registrada de forma inequívoca nos artefatos (baseline: ambiguidade em deployments opcionais — `research.md:8`).

## Cost of Inaction

Sem uma semântica explícita, clientes não conseguem expressar a intenção de retirar um responsável e podem interpretar `null` de maneiras divergentes; notificações duplicadas geram ruído e erodem a confiança no sinal de atribuição. A ambiguidade de precedência entre Meilisearch e PostgreSQL mantém a correção multi-entidade dependente da configuração da instância, contrariando a direção Postgres-first registrada em `intake.md:8`. A ausência de contratos determinísticos deixa essas falhas sem regressão automatizada.

## Open Questions

- [NEEDS CLARIFICATION: a reinterpretação de `assignee_id: null` de "ignorado" para "retirar" é aceitável para clientes existentes? `research.md:13` aponta a mudança de comportamento, sem registro de consulta a consumidores.]
- [NEEDS CLARIFICATION: retirada de atribuição deve gerar algum evento próprio ou permanecer sem notificação? `research.md:7` adota a segunda opção como MVP; não há decisão de produto independente registrada.]
- [NEEDS CLARIFICATION: a janela de corrida residual da guarda de duplicatas é aceitável neste ciclo? `research.md:14` a classifica como NV, sem critério de aceitação definido.]
- [NEEDS CLARIFICATION: paridade de tarefas/arquivos no Meilisearch deve ser diferida ou tratada como requisito do contrato de busca? `research.md:8` registra o diferimento, mas não há fechamento de produto.]
