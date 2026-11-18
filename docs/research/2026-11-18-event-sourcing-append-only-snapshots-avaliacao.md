# Avaliação: `append-only event sourcing + snapshots` por contexto de DB

**Data:** 2026-11-18 · **Tipo:** pesquisa com fontes primárias · **Estado:** informativo, sem mudança de código

> **Conclusão primeiro.** A adoção de event sourcing (ES) como **estratégia padrão para quase todo o DB do Flora é desaconselhada**: as próprias fontes oficiais (Microsoft, AWS) dizem que, para a maior parte dos sistemas e das partes de um sistema, CRUD tradicional é suficiente. A técnica é **possível** em quase todos os contextos, mas **desejável** em poucos. O caminho recomendado é **híbrido**: manter tabelas de estado como fonte da verdade, adicionar **outbox transacional + CDC** para eventos de integração, tornar **projeções re-reconstruíbleis** (FTS, listas, contadores) e usar **snapshots somente onde o replay for caro**. ES integral deve ser considerado no máximo para um subdomínio com necessidade real de auditoria/intenção/tempo (ex.: trilha de mensagens ou histórico de decisões de RBAC) — nunca como regime global.

---

## 1. Escopo, método e cobertura idiomática

- **Pergunta:** avaliar possibilidade, viabilidade, desejabilidade e interesse de adotar `append-only event sourcing + snapshots` em cada contexto do banco do Flora.
- **Método:** buscas ao vivo + leitura direta das páginas primárias; afirmações carregadas trianguladas entre pelo menos duas fontes independentes; citações com **título + URL + data**; itens não abridos ou sem data ficam marcados **[NV]**.
- **Idiomas alcançados:** **EN** (Fowler, Microsoft, AWS, Confluent/Kafka, Debezium, arXiv/ACM/IEEE, EventStoreDB/Kurrent, Axon), **PT-BR** (Microsoft pt-br, Elemar Júnior, Bankly, Nubank, Cleisson Barbosa), **PT-PT** (EXPANDES), **ES / ES-CL** (Microsoft es-es, SiempreListo, ITERIAM, Paradigma Digital, Rootstack, Lucio Durán, Nubank-es), **IT / FR / DE / JA** (traduções oficiais Microsoft do mesmo padrão). Cobertura real: 9 idiomas; **a maioria das fontes não-EN é tradução ou derivada das fontes EN**, portanto a triangulação independente é mais forte em EN do que em PT/ES.
- **Ferramentas de habilidade:** carregada a competência `research` do harness; `npx skills find "event sourcing research architecture"` executada (descoberta; sem instalação); `npx skills use sickn33/agentic-awesome-skills@event-sourcing-architect --agent opencode` → **não suportado** (o comando só gera prompt para `claude-code`, `codex`, `sarvam-code`); `--agent claude-code` gerou saída vazia. **Nada foi instalado.** Nenhuma credencial foi copiada.
- **Conteúdo web tratado como dado não-confiável:** instruções incorporadas em páginas foram ignoradas; nenhuma tentativa de injeção relevante encontrada.

---

## 2. O que a técnica é (definição operacional)

| Affirmation | Fonte (título, URL, data) |
|---|---|
| Capturar todas as mudanças de estado como sequência de eventos; permite reconstruir passado, replay e consulta temporal | **Event Sourcing** — Martin Fowler, https://martinfowler.com/eaaDev/EventSourcing.html, 2005-12-12 (o próprio autor marca o texto como *draft*) |
| Estado derivado por **reidratação** a partir do stream; eventos imutáveis; append com concorrência otimista | **Event Sourcing Pattern** — Microsoft Azure Architecture Center, https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing, ms.date 2026-03-27, atualizado 2026-09-25 |
| Repositório imutável, ordenado cronologicamente; estado atual/pontual por replay | **Event sourcing pattern** — AWS Prescriptive Guidance, https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/event-sourcing-pattern.html, acessado 2026-11-18 (sem data de publicação visível **[NV]**) |
| Em PT-BR: "repositório de eventos", "reidratação", "instantâneos"; mesmas ressalvas | **Padrão de Origem de Eventos** — Microsoft pt-br, https://learn.microsoft.com/pt-br/azure/architecture/patterns/event-sourcing, 2026-03-27 |
| Em ES: "almacén de solo anexión", "rehidratación", "instantáneas" | **Patrón de origen de eventos** — Microsoft es-es, https://learn.microsoft.com/es-es/azure/architecture/patterns/event-sourcing, 2026-03-27 |

**Distinção crítica (fonte oficial):** *"Don't confuse an event store with an eventstream message broker. Message brokers such as Apache Kafka typically lack per-entity stream queries and optimistic concurrency… they aren't a substitute for an event store"* — Microsoft, op. cit. Kafka é camada de distribuição; não é event store.

---

## 3. Evidência sobre snapshots, replay e custos

| Achado | Fonte (título, URL, data) |
|---|---|
| Snapshots são **otimização, não substituição** do stream; reconstruíveis; equilíbrio entre custo de armazenamento e tempo de reidratação | Microsoft, op. cit., 2026-03/09 |
| "We recommend that you take regular snapshots"; frequência ligada ao **RPO**; replay de muitos eventos é demorado | AWS, op. cit., 2026-11-18 |
| Snapshot padrão = 1 por agregado, sobrescrito; snapshots incompatíveis são ignorados com fallback para replay completo | **Snapshots** — Axon Framework Reference 5.4, https://docs.axoniq.io/axon-framework-reference/5.4/snapshotting/, acessado 2026-11-18 |
| Estratégia híbrida (intervalo fixo + gatilhos adaptativos) reduziu recuperação para 40 s em carga *burst* vs. 95 s (fixo) e 65 s (adaptativo); protótipo Java/Spring Boot + PostgreSQL + Kafka, 10k–1M eventos, 10 execuções, IC 95% — **journal regional de baixo tier; números ilustrativos, não benchmark independente** | **Optimizing Event Replay with Hybrid Snapshotting in Event-Sourced Systems** — Shyngys, Serek, Kuanyshbay, https://journal2.uad.ac.id/index.php/biste/article/view/14214, 2026-05-08 |
| Logs de eventos não limitados são impraticáveis; *pruning* tem trade-offs | **Log Pruning in Distributed Event-sourced Systems** — Erb et al., DEBS 2018, DOI 10.1145/3210284.3219767 |
| Snapshots distributed-consistentes são não-triviais por causalidade entre logs | **Consistent Retrospective Snapshots in Distributed Event-sourced Systems** — Erb et al., NetSys 2017, DOI 10.1109/NetSys.2017.7903947 |
| WAL do SQLite é append-only com checkpoint; pode crescer sem limite sob leitores contínuos | **Write-Ahead Logging** — SQLite, https://www.sqlite.org/wal.html, atualizado 2026-08-25 |
| `wal_level=logical` aumenta volume de WAL; slots inativos impedem limpeza e podem levar a parada por wraparound | **WAL settings / Logical Decoding** — PostgreSQL 18, https://www.postgresql.org/docs/current/logicaldecoding-explanation.html, 2026-08-13 |

**Caso real (alerta):** **"A digital pine forest: the evergreen cache"** — Gustavo Bicalho, Nubank, https://building.nu.com/the-evergreen-cache/, "11 Feb, 20" (ano exibido de forma ambígua **[NV]**; versões PT-BR/ES no mesmo site). O saldo event-sourced (depósitos + retiradas em Datomic) passou a exigir **instâncias de 100 GB** com OOM e pausas de GC; a solução não foi abandonar eventos, mas criar um **cache imutável chaveado por `event-counter` por conta** (read-through via DynamoDB): RAM por instância caiu de **90 GB para 55 GB**, latência também caiu, consistência preservada. Lição: ES é viável, mas a **reconstrução repetida é o custo real**, e um "snapshot/cache com chave de versão" é a mitigateção prática.

---

## 4. Riscos estruturais (evidência)

| Risco | Fonte |
|---|---|
| Eventual consistency; não se pode "ler própria escrita" na projeção | Microsoft, op. cit.; **Distributed Data for Microservices — Event Sourcing vs. CDC** — Eric Murphy (Red Hat), https://debezium.io/blog/2020/02/10/event-sourcing-vs-cdc, 2020-02-10 |
| *Dual writes* entre journal e projeções sem transação conjunta | Murphy, 2020-02-10 |
| Versionamento: tolerant deserialization, event versioning, **upcasting**, e in-place migration como último recurso (quebra imutabilidade) | Microsoft, 2026-03/09; Axon 5.4 event versioning, acessado 2026-11-18 |
| Idempotência obrigatória (entrega *at least once*); rastrear último sequence number | Microsoft, 2026-03/09; PostgreSQL logical decoding, 2026-08-13; Kafka design, https://kafka.apache.org/44/design/design (acessado 2026-11-18) |
| LGPD/GDPR: direito ao apagamento conflicts com append-only; mitigação = PII fora do event store ou **crypto-shredding** | Microsoft, 2026-03/09; **GDPR Art. 17**, EUR-Lex, https://eur-lex.europa.eu/eli/reg/2016/679/art_17/oj/eng, 2016-04-27 (consolidado 2016-05-04); Kurrent scavenging, https://docs.kurrent.io/server/v26.0/operations/scavenge (acessado 2026-11-18) |
| Compactação Kafka **descarta histórico**: com retenção limitada o log não reconstrói estado; tombstones com `delete.retention.ms` default 1 dia podem ser perdidos por consumidores lentos | **Kafka Log Compaction** — Confluent, https://docs.confluent.io/kafka/design/log_compaction.html, atualizado 2026-11-06; kafka.apache.org topic configs |
| Projeções exigem uma por padrão de consulta; equipes sem DDD geram "monólito distribuído"; caso de gestão de usuários com 5 eventos foi "arquiteturalmente interessante e praticamente terrível" versus uma tabela `user_audit_log` | **Event Sourcing + CQRS: Diseño de Event Store, Proyecciones, Snapshots y Cuándo NO Usarlo** — Lucio Durán, https://lucioduran.com/es/blog/event-sourcing-cqrs-production-lessons, 2026-02-22 (atualizado 2026-11-10; notas editoriais retiraram números não sustentados) |
| "100% accurate audit log" falha quando eventos são reescritos/upcast; slice de 1 ano em produção mostrou custos de scaffolding e adesão filosófica da equipe | **Don't Let the Internet Dupe You, Event Sourcing is Hard** — Chris Kiehl, https://chriskiehl.com/article/event-sourcing-is-hard, 2019-02-03 (via archive) |

**Quando NÃO usar (fonte oficial, literal):** CRUD simples sem auditoria/replay; protótipos/MVPs; necessidade de consistência forte e tempo real; dados estáticos de referência; equipes sem experiência em event-driven — Microsoft, op. cit. A recomendação explícita: *"Event sourcing doesn't have to be an all-or-nothing decision… Use traditional CRUD for parts when the complexity isn't justified, such as user profile management or application configuration."*

---

## 5. Alternativa híbrida com evidência primária

- **Outbox transacional + CDC:** estado permanece nas tabelas; evento de domínio é inserido na tabela `outbox` **na mesma transação**; Debezium publica via CDC. Fonte da verdade continua o DB; consistência forte dentro do serviço e *read-your-writes*. **Outbox Event Router** — Debezium, https://debezium.io/documentation/reference/stable/transformations/outbox-event-router.html (acessado 2026-11-18); **Reliable Microservices Data Exchange With the Outbox Pattern** — Gunnar Morling, https://debezium.io/blog/2019/02/19/reliable-microservices-data-exchange-with-the-outbox-pattern, 2019-02-19.
- Murphy (2020-02-10) conclui explicitamente: *"Debezium, Quarkus, and the Outbox … is generally a more practical solution for your average developer team as compared to Event Sourcing solutions"*; prós incluem "source of truth stays within the application database tables and transaction log", "can read and query your own writes".
- **Temporal/bitemporal** é alternativa parcial: histórico de *estados*, não de deltas/intenção; bom para auditoria "como estava a tabela em T", não para replay de decisão de negócio. **Are Temporal Tables an alternative to Event Sourcing?** — Oskar Dudycz, https://event-driven.io/en/temporal_tables_and_event_sourcing, 2022-04-20; **Bitemporal History** — Fowler, https://martinfowler.com/articles/bitemporal-history.html, 2021-04-07.
- **CQRS não exige ES:** projeções podem ser alimentadas por outbox; Murphy, 2020-02-10.

---

## 6. Avaliação por contexto do DB do Flora

**Critérios (escalas 1–5, denominador comum = esforço × risco operacional):** **P** = possibilidade técnica no stack atual (Postgres 18 + sqlx + Axum); **V** = viabilidade (custo de operação vs. benefício); **D** = desejabilidade (alinhamento com requisitos reais do Flora: tenancy, auditoria, replay, VPS barato); **I** = interesse evidenciado em fontes para domínio análogo. Scores são **ordenação**, não veredito de prontidão.

| Contexto (tabela/fluxo) | P | V | D | I | Veredito |
|---|---:|---:|---:|---:|---|
| `users`, `organizations`, `workspaces` | 5 | 2 | 1 | 2 | **Não.** Baixa taxa de escrita; PII; GDPR; nenhum ganho de contention. CRUD + audit log se necessário. |
| `memberships`, `roles` (RBAC) | 5 | 3 | 2 | 2 | **Não para ES integral.** Auditoria de decisão de papel é útil, mas uma tabela `role_audit_log` resolve; ES exigiria versionar intenção de autoridade. Manter CRUD + FK composta (ciclo 064) + `RoleWrite` (ciclo 066). |
| `sessions` | 4 | 1 | 1 | 1 | **Não.** Estado efêmero, revogação por `jti`, retenção curta; ES só aumentaria superfície. |
| `channels`, `messages` (threads) | 5 | 4 | 3 | 4 | **Único candidato real a ES seletivo**, mas hoje sem demanda que justifique: mensagens são naturalmente append-only; edições/deletes precisariam de eventos compensatórios. Se adotar, aqui — com snapshots por thread e projeção para listagem. |
| `tasks` | 5 | 3 | 2 | 3 | **Não agora.** Máquina de estados curta; baixo volume; outbox para notificações já cobre o efeito colateral. |
| `files` (metadados + conteúdo local) | 4 | 2 | 1 | 2 | **Não.** Conteúdo é imutável por natureza; metadados são CRUD. Storage content-addressable seria outro projeto. |
| `notifications` | 5 | 4 | 3 | 3 | **Aderência conceitual alta** (log append-only + dedup), mas hoje já é tabela de estado com dedup DB. Melhor: torná-la **projeção** de eventos via outbox, não event store. |
| `search` (FTS do Postgres) | 5 | 5 | 5 | 4 | **Sim como projeção, não como ES.** FTS já é derivado e reconstruível — é exatamente o "read model" que o padrão recomenda. |
| `schema_migrations` | 5 | 5 | 5 | 3 | **Já é append-only versionado**; não mudar. |
| Telemetria/audit logs (`/metrics`, logs de request) | 5 | 4 | 3 | 4 | Append-only **log**, não event sourcing de domínio; retenção e privacidade primeiro. |
| Artefatos SDD / ledger de concorrência (fora do DB) | 5 | 5 | 5 | 3 | Já são append-only em arquivo; manter. |

### Síntese da matriz
- **Nenhum contexto atual justifica ES como estratégia padrão.**
- **Um contexto justifica consideração seletiva futura** (`messages`) e **um justifica projeção reconstruível explícita** (`search`).
- `users`/PII têm o **maior conflito legal** com append-only (GDPR Art. 17).

---

## 7. Recomendação arquitetural para o Flora

1. **Manter** o modelo relacional de estado (12 tabelas) como fonte da verdade.
2. **Adicionar**, quando houver efeito colateral entre agregados, uma tabela `outbox` com `id`, `aggregate_type`, `aggregate_id`, `type`, `payload` (schema do Debezium) escrita **na mesma transação** da mudança de estado; publicar via worker/CDC; consumidores idempotentes por `event_id`.
3. **Tratar FTS e contadores como projeções descartáveis**, com comando de rebuild — já alinhado com ADR-0001 (Postgres FTS autoritativa, Meilisearch dormente).
4. **Snapshots somente se** houver stream longo com replay mensurável: `messages` por thread, `notifications` por usuário. Começar com intervalo fixo (ex.: 100–500 eventos) e medir p95 de reidratação; evoluir para híbrido se o custo justificar (evidência Shyngys et al., 2026 — com ressalva de tier do journal).
5. **Não** usar Kafka como event store; usar no máximo como backbone de distribuição (Azure, op. cit.).
6. **Não** colocar PII em eventos: referência por ID + crypto-shredding se necessário (Microsoft 2026; Dudycz 2023-11-26, https://event-driven.io/en/gdpr_in_event_driven_architecture).
7. **Governança:** qualquer mudança nesse sentido exige ADR novo (o ADR-0003 atual rege merge, não storage) e decisão do dono/SE; este documento **não autoriza** implementação.

---

## 8. Condição de decisão (se o SE quiser propulsionar)

Gate: "existe necessidade de auditoria/intenção ou replay que uma tabela de histórico não resolve?" Se **não** → CRUD + outbox. Se **sim** → limitar a um subdomínio, medir volume de eventos e custo de replay, desenhar versionamento/upcasting antes do primeiro evento, e só então considerar ES com snapshots. Critério de parada: quando o tempo de reidratação p95 > SLO aceitável, adicionar snapshot — nunca o contrário.

---

## 9. Fontes citadas (consolidadas)

1. Fowler, **Event Sourcing**, https://martinfowler.com/eaaDev/EventSourcing.html, 2005-12-12.
2. Fowler, **Bitemporal History**, https://martinfowler.com/articles/bitemporal-history.html, 2021-04-07.
3. Microsoft, **Event Sourcing Pattern**, https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing, 2026-03-27 (upd. 2026-09-25); versões pt-br/es-es/it-it/fr-fr/de-de/ja-jp (mesma data).
4. AWS, **Event sourcing pattern**, https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/event-sourcing-pattern.html, acessado 2026-11-18.
5. Axon Framework 5.4, **Snapshots** / **Event Versioning**, https://docs.axoniq.io/axon-framework-reference/5.4/, acessado 2026-11-18.
6. Kurrent/EventStoreDB, **Projections** e **Scavenging**, https://docs.kurrent.io/server/v24.10/features/projections/ e /v26.0/operations/scavenge, acessado 2026-11-18.
7. Confluent, **Kafka Log Compaction**, https://docs.confluent.io/kafka/design/log_compaction.html, 2026-11-06; Apache Kafka topic configs, https://kafka.apache.org/43/generated/topic_config.html.
8. Debezium, **Outbox Event Router**, https://debezium.io/documentation/reference/stable/transformations/outbox-event-router.html; Morling, **Reliable Microservices Data Exchange…**, https://debezium.io/blog/2019/02/19/reliable-microservices-data-exchange-with-the-outbox-pattern, 2019-02-19.
9. Murphy, **Event Sourcing vs. CDC**, https://debezium.io/blog/2020/02/10/event-sourcing-vs-cdc, 2020-02-10.
10. EUR-Lex, **GDPR Art. 17**, https://eur-lex.europa.eu/eli/reg/2016/679/art_17/oj/eng, 2016-04-27.
11. Overeem et al., **An Empirical Characterization of Event Sourced Systems…**, https://arxiv.org/abs/2104.01146, 2021-04-02 (JSS 178, 2021).
12. Meißner et al., **Performance Engineering in Distributed Event-sourced Systems**, DEBS 2018, DOI 10.1145/3210284.3219770; **Log Pruning…**, DEBS 2018, DOI 10.1145/3210284.3219767; **Consistent Retrospective Snapshots…**, NetSys 2017, DOI 10.1109/NetSys.2017.7903947.
13. Shyngys et al., **Optimizing Event Replay with Hybrid Snapshotting…**, https://journal2.uad.ac.id/index.php/biste/article/view/14214, 2026-05-08.
14. Kiehl, **Event Sourcing is Hard**, https://chriskiehl.com/article/event-sourcing-is-hard, 2019-02-03.
15. Zimarev, **Overselling Event Sourcing**, https://alexey-zimarev.medium.com/overselling-event-sourcing-644a1c992e1f, 2020-11-26.
16. Dudycz, **Temporal Tables vs Event Sourcing**, https://event-driven.io/en/temporal_tables_and_event_sourcing, 2022-04-20; **GDPR in Event-Driven systems**, https://event-driven.io/en/gdpr_in_event_driven_architecture, 2023-11-26.
17. Bicalho/Nubank, **The Evergreen Cache**, https://building.nu.com/the-evergreen-cache/, "11 Feb, 20" (ano **[NV]**); versões PT-BR e ES.
18. Durán, **Event Sourcing + CQRS… Cuándo NO Usarlo**, https://lucioduran.com/es/blog/event-sourcing-cqrs-production-lessons, 2026-02-22 (upd. 2026-11-10).
19. SiempreListo (CL), **Introducción a Event Sourcing** / **Snapshots y Optimización**, https://www.siemprelisto.cl/tecnologias/event-sourcing/, 2026-01-25.
20. Bankly, **O que é Event Sourcing e Event Streaming?**, https://blog.bankly.com.br/event-sourcing-e-event-streaming, 2023-07-12.

**NV:** datas de publicação ausentes em AWS/algumas páginas PT/ES; ano exibido no artigo do Nubank; conteúdo de PDFs não abertos (Greg Young 2010, Snodgrass 1999); resultados de benchmark de journals regionais não reproduzidos de forma independente.
