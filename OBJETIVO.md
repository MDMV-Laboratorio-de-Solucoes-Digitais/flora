# OBJETIVO — Flora

**Data da consolidação:** 2026-11-15 (atualizado no ciclo 005)  
**Escopo:** repositório `flora` (workspace Rust modular monolith + `apps/frontend` Svelte 5).  
**Fontes:** inspeção do código/artefatos locais, `.specify/memory/constitution.md`, `specs/001-005`, `.specify/assessments/003-005`, verificações locais executadas em 2026-11-15, e pesquisa consolidada em `docs/research/2026-11-15-flora-seed-80-20-research.md` e `docs/research/2026-11-15-macro-alternative-80-20-research.md`.

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
- Constituição v1.2.0; specs 001-004 (e 005 neste ciclo); assessments 003-004 (go) e 005 (go, reframe estratégico). Bug flow SpecKit usado pontualmente para correção de CI (registro em `.specify/bugs/`).
- Documentação de direção consolidada em `OBJETIVO.md`, `ROADMAP.md` e pesquisas em `docs/research/`.

### 3.2 Implementação observada
- **Backend:** `flora-api` (Axum) como entrypoint real (config/pool/migrations/AppState/router). `flora-core` concentra domínio/infra (config, DB, modelos, repos, serviços, storage). Crates de feature (`flora-messaging/search/files/notifications/tasks`) existem com implementações parciais/duplicação em relação ao core.
- **Capacidades presentes (heterogêneas):** health, auth/OIDC com PKCE e emissão de JWT/sessão (lacunas), rotas de org/workspace/canais/mensagens/tarefas/arquivos (metadados)/busca/notificações, WebSocket com persistência/publicação parcial.
- **Frontend:** `apps/frontend` (Svelte 5, scaffold/UI parcial) é o alvo canônico; `apps/web` legado/importado permanece não-canônico.
- **Dev/infra:** `docker-compose.yml` para Postgres/Valkey/Meilisearch (ciclo 004); workflows CI Rust e frontend em `.github/workflows/`.

### 3.3 Verificado localmente (evidência executada, 2026-11-15)
- `cargo check -p flora-api -p flora-core` — **ok**.
- `cargo fmt --all -- --check` — **ok**.
- `cargo test --workspace --no-run` — **ok**.
- `cargo test -p flora-core --lib` — **8 passed, 0 failed**.
- `cargo test -p flora-tests --lib` — **54 passed, 0 failed**.
- `docker compose config -q` — **ok** (compose dev).

**NV/não exercitado neste ambiente:** runtime ponta a ponta com Postgres/Valkey/Meilisearch/OIDC, migrações em execução integrada, testes de integração/contrato com serviços, smoke de API com dependências ativas, e execução remota completa de CI/Forgejo Actions pós-correções além do registrado em artefatos.

### 3.4 Gaps prioritários (o que falta para o MVP P0)
- **Segurança/tenancy:** unificar autenticação/autorização/contexto de tenant nas rotas críticas (reduzir dependência de headers `X-User-ID`/`X-Organization-ID` não validados) e fechar lacunas OIDC/PKCE/JWKS conforme aplicável.
- **Fluxos e2e núcleo:** alinhar contrato frontend↔backend e viabilizar caminho feliz verificável (workspace→canal→mensagem e/ou tarefa).
- **Realtime:** fanout/assinatura efetiva para clientes WS (além de persistência/publicação pontual).
- **Arquivos:** suportar binário/upload/download real e alinhar quota/escopo.
- **Busca/notificações:** wiring de indexação e eventos→notificações para mutações core.
- **Opcionalidade/debloat:** tornar componentes não essenciais realmente opcionais (Postgres-first) e simplificar operação para perfil Seed/VPS barata.
- **Drift/higiene:** reconciliar README/spec tasks com estado real; tratar `apps/web` não-canônico.

## 4) Não-objetivos e guardrails

- Sem paridade com Macro e sem superfícies P2 como requisito do Seed.
- Sem dependências proprietárias obrigatórias; IA/integrações externas apenas opcionais/plugáveis.
- Sem stack pesada desnecessária para VPS barata; evitar microsserviços/infra complexa prematura.
- Sem reuso de código incompatível com AGPL e sem engenharia reversa não autorizada de componentes proprietários.
- Sem declarar “verde” sem evidência; sem deploy/produção, merges/releases, segredos ou serviços pagos sem autorização explícita.

## 5) Próximo 80/20 (ciclo 006 recomendado)

Entregar uma **fatia vertical mínima segura e verificável** do MVP Seed:
1. Consistência de auth/contexto/tenant nas rotas núcleo.
2. Fluxo ponta a ponta reproduzível em dev: **workspace → canal → mensagem** (ou tarefa) com persistência e leitura/entrega mínima comprovadas.
3. Contrato frontend↔backend alinhado para o caminho feliz (ou smoke API documentado) e evidência registrada (comandos/resultados; NV quando bloqueado por ambiente).

Detalhes em `specs/005-macro-alternative-mvp/{spec,plan,tasks}.md` e `ROADMAP.md`.
