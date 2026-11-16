# OBJETIVO — Flora

**Data da consolidação:** 2026-11-15 (atualizado no ciclo 009)  
**Escopo:** repositório `flora` (workspace Rust modular monolith + `apps/frontend` Svelte 5).  
**Fontes:** inspeção do código/artefatos locais, `.specify/memory/constitution.md`, `specs/001-009`, `.specify/assessments/003-009`, verificações locais executadas em 2026-11-15, e pesquisa consolidada em `docs/research/2026-11-15-flora-seed-80-20-research.md` e `docs/research/2026-11-15-macro-alternative-80-20-research.md`.

## 1) Norte estratégico (o que a Flora quer ser)

Construir uma **alternativa viável, performática e self-hosted ao [Macro](https://github.com/macro-inc/macro)** para **pequenas e médias empresas (PMEs)**, operável em **VPS barata** (com limites realistas), como suíte colaborativa open-source (**AGPL-3.0**) em **Rust + PostgreSQL**, com UI em **Svelte 5** (e empacotamento **Tauri** para desktop/mobile em fases posteriores).

Princípios diretores:
- **Debloat:** remover/substituir dependências proprietárias e pesadas; evitar replicar a complexidade operacional do Macro “as-is”.
- **Postgres-first / componentes opcionais:** núcleo simples (monólito modular), adicionando Valkey/Meilisearch/RustFS/OIDC avançado etc. apenas por perfil/necessidade comprovada.
- **Fatias verticais verificáveis:** entregar fluxos núcleo utilizáveis com evidência antes de ampliar escopo.
- **VPS test:** viabilidade prática em host modesto (alvo recomendado: ~2 vCPU / 4 GB RAM; perfis menores com tuning/limites).

> O Macro (AGPL) é referência de produto/escopo (email, chat, docs, tarefas, calls, agents/IA, CRM, databases, calendar, storage, PRs), mas sua implementação é ampla e pesada para VPS barata; a Flora busca o **subconjunto 80/20 de maior valor para PMEs** com operação enxuta.

## 2) MVP 80/20 (Seed “Macro-like” enxuto)

### P0 — Núcleo a tornar utilizável e seguro
- **Identidade/tenancy:** auth e contexto (usuário/org/workspace) consistentes, sessões/JWT confiáveis, RBAC básico e isolamento multi-tenant.
- **Comunicação:** workspaces/canais (e DMs básicos quando viável), mensagens persistidas e realtime mínimo confiável.
- **Execução:** tarefas leves (criar/listar/atualizar/status/atribuição básica) vinculadas ao contexto.
- **Arquivos básicos:** upload/download real, metadados/permissões mínimas, quotas coerentes, storage local por padrão.
- **Busca básica:** Postgres FTS integrada aos objetos núcleo; motores externos opcionais.
- **Notificações in-app mínimas:** eventos core gerando sinalização/consulta básica.
- **Operabilidade:** setup dev reprodutível (compose), migrações, health/readiness úteis, backup/restore documentado.

### P1 — Consolidação
- Notas/docs markdown leves (sem CRDT pesado), permissões/compartilhamento mais finos, import/export, digest/email opcional, observabilidade básica.

### P2 — Adiar (até Seed estável e demanda comprovada)
- Cliente de email/sync multi-conta completo, calls/gravação/transcrição, CRM e databases genéricas, canvas, agentes/IA com memória/rotinas/sandbox avançados, integrações profundas (calendar/PR/WhatsApp), paridade de UX/superfície do Macro.

## 3) Estado atual (consolidado em 2026-11-15)

### 3.1 Governança e artefatos
- Constituição v1.2.0; specs 001-009; assessments 003-009 (go nos ciclos executados). Bug flow SpecKit usado pontualmente. Documentação de direção em `OBJETIVO.md`, `ROADMAP.md` e `docs/research/`.

### 3.2 Implementação observada
- **Backend:** `flora-api` (Axum) como entrypoint real. `flora-core` concentra domínio/infra; crates de feature existem com implementação parcial/duplicação.
- **Capacidades presentes (heterogêneas):** health, auth/OIDC com PKCE e JWT/sessão, `POST /api/v1/auth/select-organization` (membership + rotação jti/sessão), rotas de org/workspace/canais/mensagens/tarefas/arquivos/busca/notificações, WebSocket com persistência/publicação parcial.
- **Frontend:** `apps/frontend` (Svelte 5) é o alvo canônico; `apps/web` legado/importado não-canônico. Ciclo 009 adiciona fatia mínima (callback `state`, select-org + rotação de token, guards, workspace→task via API).
- **Dev/infra:** `docker-compose.yml` (PostgreSQL 18/Valkey/Meilisearch, alinhado a `uuidv7()` nativo da migração inicial, ciclo 010), workflows CI Rust/frontend; CI Rust inclui job `db-tests` (Postgres) para testes DB-backed `select-organization` (ciclo 009).

### 3.3 Verificado localmente (evidência executada, 2026-11-15/16)
- Baseline Rust (recorrente): `cargo fmt --check`, `cargo check -p flora-api -p flora-core -p flora-tests`, `test -p flora-core --lib`, `test -p flora-tests --lib` verdes nos ciclos; DB-backed contracts via jobs/testes env-gated em Postgres 18 descartável quando aplicável.
- **Ciclo 010:** compose/docs reconciliados para PostgreSQL 18 (`uuidv7` nativo, sem editar migração aplicada); smoke integrado `docs/dev-smoke.md` **rc=0** em stack PG18 isolada (alt-port por conflito externo em 5432); checks frontend determinísticos ok. **PR #213 mergeado.**
- **Ciclo 011 (PR #214, CI 6/6 verde, mergeado):** contrato `POST /tasks` (description/assignee_id), `ApiError` tipado (401 com `lockSession()`), `setOrganization` limpa flags, guard `/tasks`, testes/docs. `dev@598f08152`.
- **Ciclo 012 (PR #215, CI 6/6 verde incl. `db-tests`, mergeado):** autoridade de sessão DB-backed (`jti`) nos extractors HTTP, preservação de corpo 401 no cliente, 7 casos DB de sessão. `dev@8028aa28e`.
- **Ciclo 013 (PR #216, CI 6/6 verde incl. `contract::messaging::db_`, mergeado):** fatia REST `workspace→canal→mensagem` (listar/criar canais por workspace, post/list mensagens com `thread_id`, path tenancy C19), wiring frontend mínimo com DTOs/IDs reais, ajuste `ChannelType` sqlx para schema VARCHAR; 3 testes DB-gated messaging (PG18 alt-port). `dev@530d63fea`.

**NV/não exercitado:** smoke integrado em porta padrão (5432 ocupada externamente em ciclos recentes); autoridade de sessão DB no handshake `/ws` e alinhamento de protocolo/realtime fanout; `/messages/sync`; anexos/binários; DMs UX; wiring busca/notificações; OIDC/JWKS e browser e2e; perfil Seed/VPS com serviços opcionais realmente desacoplados/limitados.

### 3.4 Gaps prioritários (o que falta para o MVP P0)
- **Segurança/tenancy:** unificar autenticação/autorização/contexto de tenant nas rotas críticas (reduzir dependência de headers `X-User-ID`/`X-Organization-ID` não validados) e fechar lacunas OIDC/PKCE/JWKS conforme aplicável; session authority consistente no extrator.
- **Fluxos e2e núcleo:** consolidar caminho feliz verificável (workspace→canal→mensagem e/ou tarefa) com contratos estáveis frontend↔backend e realtime mínimo.
- **Compatibilidade dev/compose (parcialmente endereçado, ciclo 010):** compose/docs alinhados para PostgreSQL 18 (`uuidv7` nativo) e smoke evidenciado (rc=0) em caminho compatível; resta validar/operar em hosts sem conflito de porta padrão e orientar reset de volume PG16→18 quando aplicável.
- **Realtime:** fanout/assinatura efetiva para clientes WS (além de persistência/publicação pontual).
- **Arquivos:** suportar binário/upload/download real e alinhar quota/escopo.
- **Busca/notificações:** wiring de indexação e eventos→notificações para mutações core.
- **Opcionalidade/debloat e drift:** Postgres-first real, simplificar operação para Seed/VPS, reconciliar README/spec tasks e destino de `apps/web`.

## 4) Não-objetivos e guardrails

- Sem paridade com Macro e sem superfícies P2 como requisito do Seed.
- Sem dependências proprietárias obrigatórias; IA/integrações externas apenas opcionais/plugáveis.
- Sem stack pesada desnecessária para VPS barata; evitar microsserviços/infra complexa prematura.
- Sem reuso de código incompatível com AGPL e sem engenharia reversa não autorizada de componentes proprietários.
- Sem declarar “verde” sem evidência; sem deploy/produção, merges/releases, segredos ou serviços pagos sem autorização explícita.

## 5) Próximo 80/20 (ciclo 012 candidato)

Fechar a autoridade de sessão e elevar a confiança do fluxo núcleo sem ampliar produto:
1. **Session authority (P0):** `AuthExtractor`/contextos validam `jti` no PostgreSQL por request (sessão ativa, não expirada, `user_id`/`organization_id` coerentes), com testes DB-backed de revogação/expiração/mismatch e sem cache inicial.
2. **Contrato frontend 401:** preservar corpo/código estruturado também em respostas 401 (hoje o corpo é descartado antes do parse), mantendo `lockSession()`.
3. **Cobertura determinística:** ampliar job `db-tests` para os novos contratos de sessão e documentar execução local com Postgres 18 descartável.
4. **Debloat/Seed profile:** consolidar auditoria VPS (serviços opcionais, memória, backup) e roadmap de fatias seguintes (mensagens/realtime, arquivos, busca) conforme valor/risco.

Detalhes: `specs/011-frontend-api-contract-session-hardening/`, `.specify/assessments/011-frontend-api-contract-session-hardening/decision.md`, `specs/012-session-authority-revalidation/` (a criar), `ROADMAP.md`.
