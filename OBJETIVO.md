# OBJETIVO — Flora

**Data da consolidação:** 2026-11-16 (atualizado no ciclo 033)  
**Escopo:** repositório `flora` (workspace Rust modular monolith + `apps/frontend` Svelte 5).  
**Fontes:** inspeção do código/artefatos locais, `.specify/memory/constitution.md`, `specs/001-033`, `.specify/assessments/003-033`, verificações locais executadas em 2026-11-15, e pesquisa consolidada em `docs/research/2026-11-15-flora-seed-80-20-research.md` e `docs/research/2026-11-15-macro-alternative-80-20-research.md`.

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

## 3) Estado atual (consolidado em 2026-11-16)

### 3.1 Governança e artefatos
- Constituição v1.3.0; specs 001-009; assessments 003-009 (go nos ciclos executados). Bug flow SpecKit usado pontualmente. Documentação de direção em `OBJETIVO.md`, `ROADMAP.md` e `docs/research/`.

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
- **Ciclo 014 (PR #217, CI 6/6 verde incl. `contract::websocket::db_`, mergeado):** autoridade de sessão DB-backed no handshake `/ws` (paridade HTTP via verificador compartilhado), alinhamento de path/transporte (`/api/v1/ws`, Bearer/`?token=`) e envelopes mínimos no cliente com hardening de ciclo de vida/reconnect; 8 testes DB-gated websocket + regressões auth/messaging. `dev@f59122aa8`.
- **Ciclo 015 (PR #218, CI 6/6 verde incl. `db_message_catch_up_after_cursor`, mergeado):** realtime mínimo/catch-up para mensagens persistidas (slice A) — cursor `after` determinístico (ASC, `(created_at,id)`), hub in-process + eventos canônicos pós-commit (REST/WS), assinaturas/fanout WS com isolamento org/canal e convergência idempotente no frontend. `dev@ec0be14e0`.
- **Ciclo 016 (PR #219, CI 6/6 verde, mergeado):** hardening Seed/VPS (slice C) — pool DB config-driven e init único, `search`/`messaging` opcionais com degradação/config-gating, readiness DB-aware e hints de capacidade, compose com profiles/limites/healthchecks (`core`/`realtime`/`search`), alinhamento `FLORA_*`/docs e notas mínimas de backup/restore. `dev@0989fe00d`.
- **Ciclo 017 (PR #220, CI verde, mergeado):** arquivos binários seguros (slice B) — migração aditiva `files.workspace_id`, upload multipart em streaming e download em streaming (storage local contido, temp+rename atômico, checksum SHA-256, limites/validação MIME básica), endpoints/contrato FE-compatível e authz/tenant checks no caminho feliz. `dev@12f39f626`.
- **Ciclo 018 (PR #221, CI verde, mergeado):** hardening do slice de arquivos + contratos DB-gated (slice A1) — validação `workspace_id ∈ org` em upload/metadata, endurecimento de multipart/erros, alinhamento FE, e testes DB-gated (PG descartável + storage temporário) para happy path, isolamento/authz, validações, deleção e legacy. `dev@3efeff127`.
- **Ciclo 019 (PR #222, CI verde, mergeado):** wiring essencial busca/notificações (slice S1) — migração aditiva `messages.search_tsv` + GIN e fallback Postgres FTS em `/api/v1/search` quando Meilisearch indisponível (contrato preservado, tenant-safe), emissão best-effort de notificação in-app `Reply` em respostas de thread (pai válido, mesma org/canal, não deletado, não self), e contratos DB-gated (PG descartável) para busca/notificações. `dev@c5e2f4e8a`.
- **Ciclo 020 (PR #223, CI verde, mergeado):** extensão PG FTS para tarefas/arquivos (slice S1) — migração aditiva `tasks/files.search_tsv` + GIN parcial, fallback PG multi-entidade `message|task|file` em `/api/v1/search` (contrato preservado, tenant-safe, Meilisearch inalterado), e notificação in-app `Assignment` best-effort em transição efetiva de assignee (create/update, sem self/no-op explícito), com contratos DB-gated (PG descartável). `dev@0dcde353a`.
- **Ciclo 021 (PR #224, CI verde, mergeado):** contrato tri-state de `assignee_id` em `PATCH /tasks/{id}` (omit/no change, `null`/unassign, value/set) com notificação `Assignment` apenas em transição efetiva para assignee não-ator (sem unassign/self/no-op) e dedup best-effort de `Assignment` não lida `(user_id,target_id)`; hardening/contratos DB-gated do fallback PG de busca (normalização de tipos/limites/invariantes) e decisão explícita de precedence/paridade Meilisearch (indexação task/file diferida, fallback PG autoritativo quando indisponível). `dev@b5be7d16f`.
- **Ciclo 022 (PR #225, CI verde, mergeado em `dev@a85e2020f`):** dedup idempotente de notificações não lidas no banco (índice parcial único `(user_id,event_type,target_id)` + `ON CONFLICT DO NOTHING`, create conflict-safe) eliminando janela TOCTOU; degradação fail-open de busca (erro de query Meilisearch → fallback PG FTS, mantendo precedence quando saudável) com contrato explícito de freshness/paridade Meilisearch adiado; expansão do CI `db-tests` para contratos DB-gated de `search/notifications/files` (PG18 descartável). Sem mudanças no frontend canônico.

- **Ciclo 023 (PR #228, CI verde, mergeado em `dev@52160a539`):** contrato de busca Postgres-autoritativa — `/api/v1/search` sempre responde via PG FTS com `source: "postgres"`, Meilisearch mantido dormente/não-autoritativo; migração aditiva normalizando separadores `-`/`.`/`_` em `tsvector` (nomes hifenizados passam a casar por sub-token); `SearchConfig` default alinhado ao perfil Seed; contratos DB-gated determinísticos (inclusive com `SearchService` configurado e inalcançável). Sem mudanças no frontend canônico.

- **Ciclo 024 (PR #230, CI verde, mergeado em `dev@bf1766cb`):** verificação real do ID token OIDC — descoberta/JWKS com seleção por `kid`, assinatura/`iss`/`aud`/`exp`/`sub`/`nonce`, rejeição de JWK simétrico, allowlist exata de `redirect_uri` e identidade por `sub`; 5 contratos unitários determinísticos. Fecha o gap decisivo de "Users & Roles (OIDC)" para Seed v0.1.

- **Ciclo 025 (PR #231, CI verde, mergeado em `dev@d09d39c1`):** ADR-0001 de reconciliação Seed (mecanismos autoritativos + gate de verificação), LICENSE AGPL completo e NOTICE, `deny.toml`, README mechanism-neutral; FASE 0 CI e OTel honesto ficam para 026.

**NV/não exercitado:** smoke integrado em porta padrão (5432 ocupada externamente em ciclos recentes); entrega multi-nó/Valkey/NOTIFY e revalidação pós-handshake WS; `/messages/sync`/replay completo além do catch-up 015; quota agregada concorrente/transacional por org/workspace, consistência avançada de purge/cleanup storage↔DB e cobertura DB-gated ampliada de bordas (multipart/quota/cleanup); DMs UX; extensão de busca FTS para tarefas/arquivos, relevância/i18n/CJK e ACL de canal em busca; eventos/notificações adicionais (mentions/assignment/completion), dedup/idempotência/tx-coupling e delivery realtime/push/outbox; quota agregada concorrente/purge↔storage cleanup e cobertura DB-gated ampliada de arquivos; lacunas realtime pós-handshake/multi-nó/replay ampliado; OIDC/JWKS e e2e multi-cliente (o smoke Chromium único já está verde); validação de carga/OOM e drills de restore em 2vCPU/4GB; enforcement de limites do compose por runtime (`docker stats`).

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

## 5) Próximo 80/20 (ciclo 034 candidato — fronteira de origem e produto)

- **Ciclo 033 (PR em preparação/CI):** busca no header canônico ligada ao endpoint verificado, com testes unitários no gate do CI e drift documental fechado.

- **Ciclo 032 (PR #244, CI verde, mergeado em `dev@cecaf686`):** allowlist CORS backend com testes determinísticos e drift documental fechado; ADR-0002 #2 permanece NV.

- **Ciclo 031 (PR #242, CI verde, mergeado em `dev@aaa45536`):** versão centralizada, `release.yml` tag-only com SBOM (sem tag/publicação) e drift documental fechado.

- **Ciclo 030 (PR #240, CI verde incl. e2e-browser, mergeado em `dev@ab3fc007`):** 1 e2e smoke Chromium com minter feature-gated (`retries: 0`), CI não-required e integridade documental corrigida.

- **Ciclo 029 (PR #238, CI verde, mergeado em `dev@b8972c0e`):** clippy estrito `--all-targets`, ADR-0002 (Tauri deferido), drift 028 corrigido e smoke integrado executado com JWT fresco em portas isoladas (**rc=0**).

- **Ciclo 028 (PR #236, CI verde, mergeado em `dev@4ecf1459`):** FASE 0 fechado com store cargo-vet versionado + exceção `multer`, gate informativo, dependency-review e sweep OSV semanal; smoke com JWT fresco; drift residual corrigido.

1. **P0 — Release SBOM:** workflow tag-only draft-first + CycloneDX (preparado em 031; tag/publicação exigem autorização explícita).
2. **P1 — CORS allowlist:** backend `AllowOrigin::list` (032); não reivindicar critério ADR-0002 #2 sem verificação non-browser.
3. **P1 — Tauri:** ADR-0002 já adotada (deferral); critérios de reentrada em `docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md`.
4. **P2 — Supply chain:** promover cargo-vet a gate bloqueante + CODEOWNERS após janela de observação; load/OOM/restore em 2vCPU/4GB. (Clippy `--all-targets` já é gate verde desde 029.)
5. **NVs:** cross-engine ranking/snippet parity, `pg_trgm`/i18n/CJK e ACL de busca; JWKS rotation/real-OP.

Detalhes: `docs/research/2026-11-16-028-*.md`, `docs/adr/0001-seed-v01-mechanism-substitutions.md`, `ROADMAP.md`.

- **Ciclo 027 (PR #234, CI verde, mergeado em `dev@044f40c6`):** retirada do adapter PGMQ/Valkey não conectado, `TraceLayer` sem custo e fechamento do drift ADR/README/planos/specs.

1. **P0 — cargo-vet como gate de nova dependência:** versionar `supply-chain/` (Hoje gitignored), adicionar 1 exceção `multer` + 3 imports, primeiro informativo e depois bloqueante; CODEOWNERS.
2. **P1 — Advisory/SBOM:** job agendado OSV/Dependabot (sinal advisory, não gate) e CycloneDX em release.
3. **P1 — E2E browser:** mint de JWT com `jti` seedado via bin feature-gated, Playwright Chromium smoke `workspace→canal→mensagem→tarefa`; Tauri explicitamente adiado.
4. **P2 — Observabilidade/validação:** decisão OTLP real, smoke integrado com evidência, load/OOM/restore em 2vCPU/4GB.
5. **NVs:** cross-engine ranking/snippet parity, `pg_trgm`/i18n/CJK e ACL de busca; JWKS rotation/real-OP.

Detalhes: `docs/research/2026-11-16-027-*.md`, `docs/adr/0001-seed-v01-mechanism-substitutions.md`, `ROADMAP.md`.

- **Ciclo 026 (PR #232, CI verde, mergeado em `dev@9a60766f`):** gate Seed executável em CI (`fmt`, `cargo clippy --lib --bins -D warnings`, `cargo deny check bans licenses sources`), `deny.toml` corrigido para `AGPL-3.0-or-later`, constituição v1.3.0 mechanism-neutral, drift de ROADMAP/AGENTS fechado, ciclo 025 finalizado e deps OTel fantasmas removidas.

1. **P0 — Transporte de notificações:** retirar/gatear scaffolding PGMQ/Valkey não conectado (`flora-notifications`, `pgmq_queue`, XADD) em PR behavior-preserving, com gatilhos de reativação.
2. **P1 — Supply chain:** tornar `supply-chain/` versionado e adicionar 3 imports ao cargo-vet como gate de nova dependência; job `advisories` separado (RustSec via rede) e SBOM na release.
3. **P1 — E2E browser:** JWT fresco via seed/bootstrap, slim Chromium smoke `workspace→canal→mensagem→tarefa`, Tauri explicitamente adiado.
4. **P2 — Observabilidade/verificação:** `TraceLayer` sem exporter, decisão OTLP real, smoke integrado com evidência, load/OOM/restore em 2vCPU/4GB.
5. **NVs:** cross-engine ranking/snippet parity, `pg_trgm`/i18n/CJK e ACL de busca; JWKS rotation/real-OP.

Detalhes: `docs/research/2026-11-16-026-*.md`, `docs/adr/0001-seed-v01-mechanism-substitutions.md`, `ROADMAP.md`.

1. **P0 — FASE 0 em CI:** gate determinístico `cargo deny check bans sources` (verificado localmente) no workflow; licenças/auditoria completa dependem de acesso ao índice crates.io na CI; decidir as 455 isenções do `cargo-vet`.
2. **P1 — OTel honesto:** remover/corrigir `opentelemetry-otlp 0.22` (versão inexistente), adotar `TraceLayer` sem custo e sem coletor pesado, documentar NV.
3. **P1 — Transporte de notificações:** desativar/gatear o scaffolding PGMQ/Valkey não conectado ou decidir adoção com gatilhos explícitos.
4. **P2 — Verificação integrada:** smoke Chromium já verde (#240); restam load/OOM/restore em 2vCPU/4GB e e2e multi-cliente.
5. **NVs remanescentes:** cross-engine ranking/snippet parity, `pg_trgm`/i18n/CJK e ACL de busca; JWKS rotation/real-OP.

Detalhes: `docs/adr/0001-seed-v01-mechanism-substitutions.md`, `deny.toml`, `docs/research/2026-11-16-seed-*.md`, `ROADMAP.md`.
