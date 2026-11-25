# Problem Definition: Idempotência de notificações e resiliência da busca

- **Slug**: 022-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | problem.md anterior

## Problem Statement

Após 021 (baseline `dev@b5be7d16f`, PR #224), a deduplicação de notificações ainda depende de uma consulta best-effort antes da inserção, chaveada por `(user_id, event_type, target_id)` e limitada a linhas não-lidas (`research.md:6`). Não há unicidade imposta pelo banco, de modo que transições efetivas concorrentes ou retries podem produzir notificações não-lidas duplicadas; os caminhos de criação de notificações, incluindo respostas, compartilham o mesmo comportamento não-idempotente (`problem.md anterior:3`).

A busca segue ambígua em cenários de falha e desatualização. O Meilisearch é preferido quando configurado, mas os hooks de indexação de mutação não estão conectados e o adaptador não aguarda tarefas assíncronas, permitindo que frescor e paridade divirjam (`research.md:8`). Falhas de consulta não são degradadas explicitamente para a base de correção em PostgreSQL FTS (`problem.md anterior:5`). Além disso, nem todos os contratos DB-gated de busca, notificações e arquivos são executados na CI, reduzindo a proteção contra regressões em fluxos centrais do Seed (`problem.md anterior:5`; `research.md:9`).

Como limite já estabelecido, a retirada de atribuição não emite `Assignment` e não há sinal de producto exigindo mudança disso neste ciclo (`research.md:7`).

## Affected Users & Stakeholders

- **Users**: destinatários de notificações não-lidas que podem ver o mesmo sinal repetido; usuários de busca em instâncias com Meilisearch configurado; usuários de fluxos centrais do Seed afetados por regressões não detectadas. [ASSUMPTION: perfis inferidos de `research.md:6-9`; sem entrevistas registradas.]
- **Stakeholders**: mantenedores da API de notificações e de busca, operadores de instâncias Seed/VPS, equipe de CI e contratos DB-gated.

## Goals

- Fazer com que a supressão de duplicatas não-lidas seja determinística e resistente a concorrência e retries, sem depender apenas da ordem de operações na aplicação (`research.md:6`).
- Tornar explícito o comportamento da busca quando o backend preferencial falha, preservando a base transacional e tenant-safe como referência de correção (`research.md:8`).
- Ampliar a execução automatizada dos contratos DB-gated existentes para busca, notificações e arquivos, de modo que regressões nos fluxos centrais sejam detectadas na CI (`research.md:9`).

## Non-Goals

- Fora do enquadramento: hooks de indexação no Meilisearch, propagação de exclusão lógica, espera por tarefas assíncronas, ferramentas de rebuild/backfill, CI ancorada no Meilisearch e equivalência de ranking/snippets entre engines (`decision.md:39`).
- Também não compõem o problema: outbox e acoplamento transacional, política de coalescência/estado de leitura, evento específico para unassign, entrega em tempo real/push/menções, exposição de `target_url`, relevância via `pg_trgm`/i18n/CJK, ACL por canal/workspace, benchmarks de 2vCPU/4GB, frontend e novos serviços obrigatórios (`decision.md:40-42`). A supressão de duplicatas deve preservar notificações legítimas e distintas (`decision.md:34`).

## Success Metrics

- Inserções concorrentes ou repetidas do mesmo `(user_id, event_type, target_id)` não-lido não produzem mais de uma notificação ativa (baseline: guarda best-effort sem garantia no banco — `research.md:6`; `problem.md anterior:3`).
- Uma falha de consulta no backend preferencial resulta em resposta de busca consistente com a base PostgreSQL, sem degradar tenant/soft-delete (baseline: degradação não explicitada — `problem.md anterior:5`).
- As suítes DB-gated de busca, notificações e arquivos executam na CI em PostgreSQL 18 descartável (baseline: apenas `auth/messaging/websocket` no job `db-tests` — `research.md:9`).
- Notificações legítimas de tipos ou alvos distintos continuam visíveis após a supressão de duplicatas (baseline: comportamento atual não formalizado — `decision.md:34`).

## Cost of Inaction

Notificações duplicadas reduzem a utilidade do sinal de atribuição e de resposta, especialmente sob retries. A dependência de um backend preferencial sem indexação confiável mantém a busca sujeita a resultados vazios ou desatualizados, enquanto a ausência de uma regra de degradação explícita dificulta diagnosticar incidentes. Sem as suítes DB-gated na CI, correções de idempotência e busca podem regredir silenciosamente em fluxos centrais do Seed.

## Open Questions

- [NEEDS CLARIFICATION: a supressão de duplicatas deve se aplicar a todos os tipos de notificação não-lida ou apenas a `Assignment`? `research.md:6` sugere a aplicação geral, sem decisão de produto registrada.]
- [NEEDS CLARIFICATION: após a leitura de uma notificação, uma nova notificação para o mesmo alvo deve ser permitida? `research.md:6` assume que sim; o comportamento desejado não foi confirmado.]
- [NEEDS CLARIFICATION: a degradação para PostgreSQL em falha de consulta deve ser silenciosa ao cliente ou explicitada na resposta? `research.md:8` descreve a direção, sem definir contrato de exposição.]
- [NEEDS CLARIFICATION: o frescor do Meilisearch quando ele responde com sucesso, mas desatualizado, deve ser tratado neste ciclo? `research.md:15` a classifica como NV.]
