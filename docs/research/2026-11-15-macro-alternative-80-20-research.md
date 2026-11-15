# Research Consolidada: Flora como alternativa self-hosted ao Macro para PMEs em VPS barata (80/20)

**Data:** 2026-11-15  
**Escopo:** síntese de evidências do repositório Flora, análise pública inicial do Macro, e resultados de pesquisas em background (codebase map, análise Macro/debloat, padrões cheap-VPS/self-hosted).  
**Limites:** sem acesso a telemetria/uso real de clientes, sem benchmark de carga neste ambiente, e sem auditoria jurídica formal; recomendações de priorização são hipóteses informadas por evidência técnica/operacional.

## 1) Norte estratégico (pedido do ciclo)

Transformar a Flora em alternativa **viável, performática e self-hosted** ao [Macro](https://github.com/macro-inc/macro) para **PMEs**, operável em **VPS barata** (com limites realistas), via:
- **Debloat:** remover/substituir dependências proprietárias e pesadas do Macro original.
- **Reimplementação gradual:** Rust + PostgreSQL como núcleo.
- **UI:** reescrita em Svelte 5, empacotamento Tauri (desktop/mobile) em fases posteriores.

## 2) Macro: escopo, stack e implicações de self-host (evidência pública)

### Produto e licença
- **Escopo “all-in-one workspace”:** email multi-conta, chat/canais/DMs, docs colaborativos (markdown/CRDT), tarefas, canvas, agents/IA com memória/rotinas, calls (vídeo/gravação/transcrição), CRM, databases, calendar, file storage, PRs, @links bidirecionais e busca unificada (README oficial).
- **Licença:** AGPL-3.0 (texto completo no repo). Permite self-host e modificação sob copyleft de rede (§13 para versões modificadas acessíveis via rede). Reimplementação limpa evita obrigações derivadas de código copiado, mas requer cuidado com ativos/marcas e conformidade se houver reuso (orientação técnica, não jurídica).

### Arquitetura/complexidade observada
- Monorepo grande: workspace Rust com muitos crates/serviços e ecossistema TS/JS amplo (Bun, workspaces, Lexical/Loro/wasm/patches), apps/infra/nix/vendor/tooling.
- Stack local/prod (documentação/compose/análises): múltiplos serviços (auth, gateway/websocket, sync, storage, email, calendar, notifications, search, agent harness, workers etc.), **Postgres, Redis, OpenSearch, Kafka, FusionAuth, LocalStack (S3/SQS/DynamoDB)**, runtimes/workers (incl. Cloudflare Workers/miniflare para sync/edição IA), Nix/Just para dev, indícios de AWS/Pulumi em produção.
- **Custo operacional:** alto para VPS pequena (muitos processos/containers, JVM/cluster-like deps, build pesado, integrações externas). Self-host “as-is” em 1–2 vCPU/2–4 GB é impraticável sem cirurgia grande.

### Alvos de debloat/substituição (Macro → Flora)

| Área Macro | Por que é pesada/proprietária-cara | Direção Flora (cheap VPS) |
|---|---|---|
| OpenSearch | JVM/memória/ops | **Postgres FTS** (`tsvector`/`pg_trgm`) primeiro; Meilisearch opcional só se necessário |
| Kafka / event broker complexo | Overhead operacional e memória | **Outbox/jobs em Postgres** (`SKIP LOCKED`/pgmq) ou in-process; eventos simples no monólito |
| FusionAuth / IdP pesado | Serviço extra + config | Auth embutida/módulo simples; OIDC/SSO opcional depois |
| LocalStack/AWS-shaped deps | Paridade cloud desnecessária no Seed | **Filesystem local** por padrão; S3-compat opcional |
| Cloudflare Workers/Durable Objects (sync/IA) | Runtime/tooling split | Runtime unificado; WebSocket/SSE no app; CRDT pesado adiado |
| Microserviços numerosos + Nix-first | RAM/build/ops | **Monólito modular** (Cargo workspaces) + Docker/Compose simples |
| Email sync multi-conta completo | OAuth/sync/indexação/entregabilidade | **Adiar**; integrações mínimas opcionais |
| Calls/mídia/transcrição | CPU/banda/storage/TURN/SFU | **Adiar/drop no MVP** |
| Agents/rotinas/memória unificada/MCP profundo | Custos de modelo, sandbox, complexidade | **Adiar**; hooks BYO-LLM opcionais/feature flag |
| CRM/databases genéricas/canvas/PRs profundos | Superfície ampla de modelo/UI | **Adiar**; primitivas simples primeiro |

## 3) Padrões “cheap VPS” (comparáveis self-hosted)

- **Alvo realista:** **2 vCPU / 4 GB RAM / SSD** como perfil recomendado; 2 GB possível só para demos/uso mínimo com tuning; 1 GB inviável para suíte completa. (Ex.: Outline ~512MB–1GB+ com Postgres+Redis; Zulip 2GB mínimo mas 4GB/2CPU recomendado; Plane 2c/4GB; Rocket.Chat ainda mais pesado com Mongo/replicas.)
- **Arquitetura:** monólito modular + **Postgres como sistema de registro** + componentes opcionais; evitar sidecars always-on desnecessários (cada um consome fatia fixa de 1–4 GB).
- **Componentes 80/20:** storage local → S3 opcional; FTS no Postgres → motor externo opcional; jobs/realtime simples (in-process/Postgres, SSE/WS no app) → Redis/Valkey/filas dedicadas só sob necessidade medida; search/IA/integrações pesadas desativáveis.
- **SvelteKit + Tauri 2:** para clientes empacotados preferir build estático/`adapter-static` quando aplicável; Tauri 2 requer toolchains nativos por plataforma e cuidados de assinatura/distribuição. Estratégia 80/20: Tauri como **cliente fino** da API self-hosted remota, evitando embutir servidor+Postgres no app inicialmente.

## 4) Estado atual da Flora (síntese de evidência do repo)

### O que existe (parcial/heterogêneo)
- Workspace Rust: `flora-api` (Axum, config→pool→migrations→AppState→serve), `flora-core` (config/DB/migrations/modelos/repos/serviços/storage), crates finas (`flora-messaging`, `flora-search`, `flora-files`, `flora-notifications`, `flora-tasks`) e `flora-tests`; `docker-compose.yml` dev (Postgres/Valkey/Meilisearch).
- Capacidades: health, fluxo auth/OIDC com PKCE/sessão/JWT (com lacunas), rotas básicas de org/workspace/canais/mensagens/tarefas/arquivos (metadados)/busca/notificações, WebSocket com persistência/publicação parcial.
- Frontend: `apps/frontend` (Svelte 5, scaffold/UI parcial) e `apps/web` legado não-canônico.

### Gaps/risco P0–P1 para “Seed utilizável em VPS barata”
- **Auth/tenancy inconsistente:** vários handlers confiam em headers `X-User-ID`/`X-Organization-ID` em vez de contexto JWT validado (risco de spoofing se exposto).
- **Fluxos ponta a ponta incompletos:** arquivos apenas metadados (sem binário/upload/download real), realtime sem fanout efetivo para clientes, tarefas/listagem e criação com lacunas, notificações/search sem wiring completo de eventos/indexação.
- **Contrato frontend↔backend desalinhado** (rotas/protocolo/contexto).
- **Dependências/config:** serviços externos assumidos em pontos (Meilisearch/Valkey/OIDC) com opcionalidade nem sempre efetiva; smoke runtime com serviços NV neste ambiente.
- **Duplicação core↔crates** e hardening pendente (verificação JWKS completa, RLS/policies, CI quality gates adicionais).

## 5) MVP 80/20 recomendado (PME, Seed em VPS barata)

**Objetivo:** um workspace colaborativo mínimo, seguro e operável, não um “Macro completo”.

### P0 — Núcleo (MVP Seed)
1. **Identidade e tenancy seguros:** auth/sessão/JWT consistentes, RBAC básico, isolamento por org/workspace.
2. **Comunicação:** canais (e DMs básicos se viável) + mensagens persistidas + entrega realtime confiável mínima.
3. **Execução:** tarefas leves (status/assignee/básico) vinculáveis ao contexto.
4. **Arquivos básicos:** upload/download real, quotas/permissões mínimas, storage local por padrão.
5. **Busca básica:** Postgres FTS integrada aos objetos núcleo; motores externos opcionais.
6. **Notificações in-app mínimas:** eventos core → notificação/indicador.
7. **Operabilidade:** `docker compose`/setup dev-prod simples, migrações, health/readiness, backup/restore documentado, limites de recurso.

### P1 — Consolidação
- Docs/notas markdown leves (sem CRDT pesado), permissões/compartilhamento mais finos, import/export, digest/email opcional, observabilidade básica.

### P2 — Adiar (até Seed estável e demanda comprovada)
- Email cliente/sync completo, calls/transcrição, CRM/databases genéricas/canvas, agentes/memória/rotinas avançadas, integrações profundas (calendar/PR/WhatsApp), paridade de UX Macro.

## 6) Próximo ciclo 80/20 (recomendado, técnico/verificável)

**Fatia vertical “Seed utilizável”:** tornar confiável um fluxo mínimo ponta a ponta em ambiente dev reprodutível:
- corrigir **consistência auth/contexto** nas rotas críticas (JWT/sessão/org) e fechar lacunas PKCE prioritárias;
- garantir **workspace + canal + mensagem persistida e visível** (e/ou tarefa) com contrato frontend↔backend alinhado para o caminho feliz;
- manter dependências **opcionais por padrão** onde possível (Postgres-first) e registrar evidência de smoke (comandos, resultados, NV).

Critério de pronto do ciclo: passos reproduzíveis + evidência executada para o slice escolhido, sem expandir escopo para features P2.

## 7) Fontes (seleção)

**Primárias/oficiais (Macro):** https://github.com/macro-inc/macro, https://raw.githubusercontent.com/macro-inc/macro/main/README.md, https://raw.githubusercontent.com/macro-inc/macro/main/LICENSE.txt, https://raw.githubusercontent.com/macro-inc/macro/main/Cargo.toml, https://raw.githubusercontent.com/macro-inc/macro/main/package.json, https://docs.macro.com/llms.txt  
**Comparáveis/self-hosting (oficiais):** Zulip requirements https://zulip.readthedocs.io/en/latest/production/requirements.html ; Plane Docker self-hosting https://developers.plane.so/self-hosting/methods/docker-compose ; Outline requirements/docker https://docs.getoutline.com/s/hosting/doc/requirements-ULdYnwi4wG ; Twenty self-hosting setup https://twenty.com/developers/section/self-hosting/setup ; Rocket.Chat requirements https://docs.rocket.chat/docs/system-requirements ; Postgres FTS https://www.postgresql.org/docs/current/textsearch.html ; Meilisearch self-hosted https://www.meilisearch.com/docs/learn/self_hosted/getting_started_with_self_hosted_meilisearch ; SvelteKit adapter-static https://svelte.dev/docs/kit/adapter-static ; Tauri v2 prerequisites/distribute https://v2.tauri.app/start/prerequisites/ https://v2.tauri.app/distribute/  
**Internas (Flora):** `OBJETIVO.md`, `ROADMAP.md`, `.specify/memory/constitution.md`, `specs/001-004`, `.specify/assessments/003-005`, `AGENTS.md`, código em `flora-*` e `apps/frontend`.
