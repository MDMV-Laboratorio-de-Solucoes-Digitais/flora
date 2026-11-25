# Concept: Próxima fatia vertical — encontrar e ser avisado sobre o trabalho

- **Slug**: 020-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Extensão Postgres-first de 019

## Options

### Option A — Ajustar só a limitação conhecida do contrato

- **Sketch**: Registrar que a retirada explícita de responsável não é representável no contrato atual e manter a busca como está, limitada a mensagens, sem prometer o que não existe.
- **Appetite**: small
- **Trade-offs**: Menor coisa que poderia funcionar para evitar mal-entendido sobre o contrato, com custo quase nulo. Não resolve o problema central: numa instância Seed sem Meilisearch, tarefas e arquivos continuam impossíveis de encontrar e quem recebe trabalho segue sem aviso dentro do app.
- **Rabbit holes**: Tratar a limitação de `PATCH` como se fosse uma correção de produto; documentar tanto o gap que ele passe a parecer aceito sem decisão; adiar indefinidamente a busca multi-entidade.

### Option B — Extensão Postgres-first de 019

- **Sketch**: Numa instância Seed sem serviço de busca dedicado, tarefas e arquivos passam a ser encontrados ao lado das mensagens, e quem recebe uma tarefa passa a ver um aviso no próprio app. O caminho continua funcionando no banco que já existe, com o mesmo contrato e o mesmo isolamento entre organizações, e a mudança é verificada em banco descartável.
- **Appetite**: medium
- **Trade-offs**: Fecha o ciclo "encontrar e agir sobre o trabalho" usando o que já existe, sem novo serviço obrigatório em VPS de 2 vCPU/4 GB e sem tocar no frontend. Custo médio: estende a busca e o hook de notificação, mantém a compatibilidade com o caminho do Meilisearch e exige contratos DB-gated determinísticos. Riscos conhecidos permanecem NV: relevância/tokenização/i18n com `'simple'`, ACL por canal/workspace, paridade de indexação com Meilisearch, dedup/idempotência/acoplamento transacional e a impossibilidade de retirar atribuição via `PATCH`.
- **Rabbit holes**: Prometer ordenação de relevância entre tipos sem critério definido; fabricar comportamento de "retirar atribuição"; transformar notificação best-effort em entrega transacional com outbox/workers/push; alterar a forma da API de notificações; perseguir benchmarks de carga ou e2e de browser.

### Option C — Paridade via Meilisearch e entrega push (não fazer)

- **Sketch**: Fazer tarefas e arquivos serem indexados no serviço dedicado e levar o aviso de atribuição para um canal de entrega em tempo real/push.
- **Appetite**: large
- **Trade-offs**: Maior fidelidade eventual, mas introduz serviço obrigatório, acoplamento e superfície de infra incompatíveis com o perfil Seed; resolve a paridade às custas da premissa de VPS barata.
- **Rabbit holes**: Mentions/completion, outbox e workers antes de fechar o básico; multi-nó; convergência de crates; UX de frontend dependente de backend ainda instável.

## Recommendation

Option B (S1). Os goals do `problem.md` pedem que tarefas e arquivos sejam encontrados no caminho que funciona sem serviços adicionais, preservando contrato, isolamento por `organization_id` e exclusão de registros excluídos logicamente; que uma transição efetiva de responsável para pessoa diferente do ator gere sinal in-app sem comprometer a mutação da tarefa; e que ambos tenham evidência determinística em PostgreSQL descartável. B é exatamente o slice escolhido na decisão 020. Os fatos convergidos são preservados: `search_tsv` gerado e armazenado com `'simple'` e GIN parcial para `tasks(title,description)` e `files(name,file_type)`, parsing seguro com `COALESCE(websearch_to_tsquery('simple',$q), plainto_tsquery('simple',$q))`, resultados tipados `message|task|file` com `LIMIT` global, caminho do Meilisearch inalterado, e notificação `Assignment` best-effort pós-persistência em mudança efetiva (`old != Some(new)` com `new != actor`), sem aviso para self-assign/no-op e sem simular unassign. O frontend canônico não integra busca nem notificações, então não há mudança de FE neste recorte.

## Out of Scope (for the recommended option)

- Relevância/tokenização/i18n/CJK, `pg_trgm`, ACL por canal/workspace na busca e paridade de indexação/consulta do Meilisearch para tarefas/arquivos.
- Retirada explícita de atribuição, dedup/idempotência/acoplamento transacional, outbox/workers/push/realtime e mudanças no formato da API de notificações.
- Benchmarks de 2 vCPU/4 GB, e2e de browser, interface frontend, remoção do Meilisearch, crates/infra multi-nó e novos serviços obrigatórios.

## Assumptions to Validate

- Não há telemetria ou pesquisa com usuários registrada que dimensione a prioridade de busca e atribuição em instâncias Seed; a demanda é inferida do roadmap e dos artefatos.
- O critério de ordenação relativa entre hits de mensagens, tarefas e arquivos no merge global ainda não foi definido.
- Colunas `organization_id` e `is_deleted` existem em `tasks` e `files`, e o filtro por organização é suficiente para isolamento na busca global; `files` tem `workspace_id` anulável legado.
- `NotificationType::Assignment`, schema, API e repositório já existem e as rotas de tarefa bypassam `TaskService`, permitindo o hook best-effort sem afetar a persistência.
- A limitação de `assignee_id.or(existing.assignee_id)` no `PATCH` permanece como gap de contrato, sem prioridade registrada para evoluí-la.
