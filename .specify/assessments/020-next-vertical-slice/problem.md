# Problem Definition: Busca e atribuição incompletas no fallback PostgreSQL

- **Slug**: 020-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | problem.md anterior

## Problem Statement

Após 019 (baseline `dev@dde602b3b`, PR #222), o caminho de busca em PostgreSQL continua limitado a mensagens (`research.md:6`; `problem.md anterior:3`). Em uma instância Seed em VPS barata sem Meilisearch, itens de trabalho centrais — tarefas (`title`, `description`) e metadados de arquivos (`name`, `file_type`) — não podem ser descobertos via `/api/v1/search`, embora os dados já existam no banco e sejam filtráveis por organização (`research.md:7`). Ao mesmo tempo, a transição de responsável por uma tarefa não gera nenhum feedback in-app: o destinatário não é avisado quando trabalho lhe é atribuído, apesar de `NotificationType::Assignment`, do schema, da API e do repositório já existirem e de o padrão de notificação best-effort pós-persistência do 019 estar provado (`research.md:8`).

O problema é duplo e complementar: a capacidade de "encontrar trabalho" depende de um serviço opcional que o perfil Seed não garante, e a capacidade de "saber que o trabalho chegou" não fecha o loop com primitivos já disponíveis. Há ainda um limite de contrato que deve ser respeitado sem ser simulado: o `PATCH` de tarefas mescla `assignee_id.or(existing.assignee_id)`, logo a retirada explícita de atribuição não é representável (`research.md:8`; `decision.md:27`).

## Affected Users & Stakeholders

- **Users**: usuários de instâncias Seed em VPS sem Meilisearch que precisam localizar tarefas e arquivos pela busca; responsáveis por tarefas que hoje ficam sabendo da atribuição apenas por canais externos. [ASSUMPTION: perfis inferidos de `intake.md:3-5` e `research.md:6-8`; não há entrevistas ou telemetria registradas nos artefatos.]
- **Stakeholders**: manutenção do produto (`OBJETIVO.md`/`ROADMAP.md`, citados em `intake.md:8`), operadores self-hosted do perfil Seed e equipe responsável pelos contratos DB-gated e pela CI.

## Goals

- Permitir que tarefas e arquivos sejam encontrados no caminho de busca que funciona sem serviços adicionais, preservando o contrato atual, o isolamento por `organization_id` e a exclusão de registros excluídos logicamente (`research.md:6-7`).
- Fazer com que uma atribuição efetiva a usuário diferente do ator gere um sinal in-app usando modelos, repositórios e APIs existentes, sem que a falha da notificação comprometa a mutação da tarefa (`research.md:8`).
- Obter evidência determinística em PostgreSQL descartável de que ambos os comportamentos são verificáveis (`research.md:9`).

## Non-Goals

- Fora deste enquadramento: relevância/tokenização/i18n/CJK e `pg_trgm`, ACL por canal/workspace na busca, paridade de indexação e consulta do Meilisearch para tarefas/arquivos, retirada explícita de atribuição, dedup/idempotência/acoplamento transacional, outbox/workers/push/tempo real, benchmarks de 2vCPU/4GB e e2e de browser (`research.md:13-15`; `decision.md:32`).
- Também não compõem o problema: interface frontend, remoção do Meilisearch, alterações no formato da API de notificações, convergência de crates e infraestrutura multi-nó (`decision.md:33`). O frontend canônico não integra busca nem notificações (`research.md:10`).

## Success Metrics

- `/api/v1/search` no caminho de fallback retorna resultados tipados `message|task|file`, com filtros de tenant e soft-delete preservados (baseline: somente mensagens — `research.md:6`; `problem.md anterior:3`).
- Toda transição efetiva de responsável para usuário diferente do ator gera `Assignment` visível pela API de notificações; self-assign, no-op e ausência de transição não geram (baseline: nenhuma notificação de atribuição — `problem.md anterior:3`; `research.md:8`).
- Contratos DB-gated em PostgreSQL 18 descartável cobrem busca multi-entidade e notificação de atribuição com guardas determinísticas (baseline: contratos do 019 restritos a mensagens e resposta — `intake.md:9`; `research.md:9`).

## Cost of Inaction

Instalações Seed sem Meilisearch seguem incapazes de localizar itens de trabalho centrais, mantendo incompleto o ciclo "encontrar e agir sobre o trabalho". Responsáveis continuam sem feedback in-app e a coordenação permanece dependente de canais externos. Sem contratos DB-gated, regressões nesses fluxos não são detectadas cedo, e a pressão por paridade via Meilisearch tende a conflitar com a restrição registrada de nenhum serviço obrigatório novo em VPS de 2 vCPU/4 GB (`intake.md:19-20`).

## Open Questions

- [NEEDS CLARIFICATION: a impossibilidade de retirar a atribuição deve permanecer como limitação conhecida neste ciclo ou existe decisão de produto para evoluir o contrato? `research.md:8` a classifica como NV/contract gap, sem prioridade registrada.]
- [NEEDS CLARIFICATION: qual critério de ordenação entre hits de mensagens, tarefas e arquivos é esperado no merge global? `research.md:6` descreve o merge com `LIMIT` global, mas não define relevância relativa entre entidades.]
- [NEEDS CLARIFICATION: existe evidência de uso real de busca e atribuição em instâncias Seed para dimensionar a prioridade? Os artefatos não registram telemetria nem pesquisa com usuários.]
