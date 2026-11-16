# ROADMAP — Flora

**Atualizado em:** 2026-11-16 (ciclos 011–028 mergeados; 029 em preparação)  
**Norte:** alternativa self-hosted viável ao Macro para PMEs em VPS barata (Rust + PostgreSQL, Svelte 5; Tauri futuro), com debloat e operação enxuta.  
**Fonte de verdade operacional:** código, `Cargo.toml`, `.github/workflows/`, `.specify/memory/constitution.md`, `specs/001-019`, `.specify/assessments/003-019`, `OBJETIVO.md`.  
Itens “verificados” exigem evidência executada; caso contrário marcar NV.

## 1) Visão e horizonte

- **Produto:** workspace colaborativo self-hostable (AGPL-3.0), núcleo Rust+Postgres, UI Svelte 5, evolução para Tauri.
- **Perfis de implantação:** **Seed** (VPS barata, mínimo viável) → **Sprout** → **Grove** → **Forest**.
- **Estratégia:** monólito modular, Postgres-first, componentes opcionais, fatias verticais verificáveis; evitar paridade/complexidade do Macro no curto prazo.

## 2) Estado consolidado (2026-11-15)

| Trilha | Estado | Evidência resumida |
|---|---|---|
| Governança/SpecKit | Em andamento (maduro) | Constituição v1.3.0; specs 001-009; assessments 003-009 (go) |
| Direção estratégica | Atualizada | Norte “alternativa ao Macro para PMEs em VPS barata”, MVP 80/20 e debloat documentados |
| Baseline determinística Rust (local) | Parcialmente verificada | `cargo fmt --check`, `cargo check -p flora-api -p flora-core -p flora-tests`, `test -p flora-core --lib` (8), `test -p flora-tests --lib` (56 pass, 5 ignored) ok |
| Select-org DB-backed (local/CI config) | Parcialmente verificado | 5 testes `#[ignore]` passaram localmente em Postgres 18; job `db-tests` adicionado em `.github/workflows/rust.yml` (execução remota PR: NV) |
| Smoke integrado dev (`select org → workspace → task`) | Evidenciado (PG18, ciclo 010) | rc=0 em stack PG18 isolada (alt-port por conflito externo em 5432; mesmo fluxo migrate/seed/bootstrap/test-flora.sh); compose/docs alinhados para `uuidv7` nativo |
| Frontend canônico (`apps/frontend`) | Scaffold + fatia mínima (checks verdes, 010) | Fluxo auth→org→workspace→task (009) + verificação determinística local `check/lint/test:unit(server)/build` ok (010); paridade/contratos ampliados NV |
| Núcleo de produto (auth/tenancy, chat, tarefas, arquivos, busca, notificações, realtime) | Parcial e heterogêneo | Rotas/serviços/modelos existem; P0s em segurança/tenancy consistente, compatibilidade dev, wiring e completude de fluxos |

## 3) Marcos concluídos / em andamento

- [x/~] Workspace Rust e crates de domínio/serviço estabelecidos; `flora-api` como entrypoint real.
- [x] Ciclos 003-008: PKCE/baseline, compose dev, hardening JWT/tenant, `select-organization` + rotação jti, testes contrato CI-safe e bootstrap de smoke dev (PRs #204–#211, CI verde, mergeados em `dev`).
- [x/~] Ciclo 009: testes DB-backed env-gated/`#[ignore]` para `select-organization`, job CI `db-tests` (Postgres), smoke integrado evidenciado (rc=0, alt-port/pg18) e fatia mínima frontend (PR #212, mergeado).
- [x] Ciclo 010: compose/docs reconciliados para PostgreSQL 18 (`uuidv7` nativo, sem editar migração aplicada), `docs/dev-smoke.md` corrigido (token bootstrap), smoke integrado rc=0 em stack PG18 equivalente (alt-port por conflito externo), checks determinísticos frontend ok (`check/lint/test:unit server/build`) e artefatos SDD/validação/release criados. **PR #213 mergeado, CI verde.**
- [x] Ciclo 011: contrato `POST /tasks` (description/assignee_id), `ApiError` tipado no frontend, `setOrganization` limpa flags, guard `/tasks`, testes determinísticos e override de portas documentado. **PR #214 mergeado, CI 6/6 verde; `dev@598f08152` sincronizado/espelhado.**
- [x] Ciclo 012: session authority DB-backed (`jti`) nos extractors HTTP, preservação de corpo 401 no cliente, cobertura DB + CI. **PR #215 mergeado, CI 6/6 verde.**
- [x] Ciclo 013: fatia REST `workspace→canal→mensagem` (listar/criar canais, post/list com `thread_id`, path tenancy), wiring frontend mínimo, testes DB-gated messaging + CI. **PR #216 mergeado, CI 6/6 verde.**
- [x] Ciclo 014: autoridade de sessão DB-backed no handshake `/ws` + alinhamento de path/transporte/protocolo mínimo no frontend, testes DB-gated websocket + CI. **PR #217 mergeado, CI 6/6 verde.**
- [x] Ciclo 015: realtime mínimo/catch-up para mensagens persistidas (slice A) — cursor `after` determinístico, hub/eventos canônicos in-process pós-commit, assinaturas/fanout WS e convergência frontend idempotente. **PR #218 mergeado, CI 6/6 verde.**
- [x] Ciclo 016: hardening Seed/VPS (slice C) — pool/init único, search/messaging opcionais com degradação, readiness DB-aware, compose profiles/limites/healthchecks, alinhamento `FLORA_*`/docs e notas mínimas de backup/restore. **PR #219 mergeado, CI 6/6 verde.**
- [x] Ciclo 017: upload/download binário seguro (slice B) — migração `files.workspace_id`, upload/download em streaming com storage local contido, checksum SHA-256, limites/validação básica, authz/tenant e alinhamento FE/API. **PR #220 mergeado, CI verde.**
- [x] Ciclo 018: hardening de arquivos + contratos DB-gated (slice A1) — validação `workspace_id ∈ org` em upload/metadata, endurecimento de multipart/erros, alinhamento FE e testes DB-gated (PG descartável + storage temporário) para happy path, isolamento/authz, validações, deleção/legacy. **PR #221 mergeado, CI verde.**
- [x] Ciclo 019: wiring essencial busca/notificações (slice S1) — migração `messages.search_tsv` + GIN, fallback PG FTS em `/api/v1/search` sem Meilisearch (contrato preservado, tenant-safe), notificação in-app `Reply` best-effort em respostas de thread e contratos DB-gated (PG descartável). **PR #222 mergeado, CI verde.**
- [x] Ciclo 020: extensão PG FTS para tarefas/arquivos e notificação de atribuição (slice S1) — migração `tasks/files.search_tsv` + GIN, fallback PG multi-entidade `message|task|file` (contrato preservado, tenant-safe, Meilisearch inalterado), `Assignment` best-effort em transição efetiva de assignee e contratos DB-gated (PG descartável). **PR #223 mergeado, CI verde.**
- [x] Ciclo 021: contrato tri-state de unassign/atribuição de tarefas + dedup de notificação `Assignment` e hardening/contratos DB-gated do fallback PG de busca, com decisão explícita de precedence/paridade Meilisearch (indexação task/file diferida). **PR #224, CI verde, mergeado** (`dev@b5be7d16f`).
- [x] Ciclo 022 (PR #225, CI verde, mergeado em `dev@a85e2020f`): dedup idempotente DB-enforced de notificações não lidas, degradação fail-open Meilisearch→PG FTS com contrato explícito, e expansão de CI `db-tests` (search/notifications/files). Freshness/paridade de indexação Meilisearch adiada (023).
- [x] Ciclo 023 (PR #228, CI verde, mergeado em `dev@52160a539`): contrato de busca Postgres-autoritativa (`source: "postgres"`, Meilisearch dormente), normalização de separadores em FTS e contratos DB-gated; prepara o Seed verificável.
- [x] Ciclo 024 (PR #230, CI verde, mergeado em `dev@bf1766cb`): verificação real do ID token OIDC (JWKS/`iss`/`aud`/`exp`/`nonce`, allowlist de redirect, identidade por `sub`) com contratos determinísticos.
- [x] Ciclo 025 (PR #231, CI verde, mergeado em `dev@d09d39c1`): ADR-0001 de reconciliação Seed, LICENSE/NOTICE completos, `deny.toml`, README mechanism-neutral.
- [x] Ciclo 026 (PR #232, CI verde, mergeado em `dev@9a60766f`): gate Seed executável em CI (fmt/clippy/cargo-deny), `deny.toml` com `AGPL-3.0-or-later`, constituição v1.3.0 mechanism-neutral, deps OTel fantasmas removidas.
- [x] Ciclo 027 (PR #234, CI verde, mergeado em `dev@044f40c6`): retirada do adapter PGMQ/Valkey não conectado, `TraceLayer` sem custo, drift documental fechado.
- [x] Ciclo 028 (PR #236, CI verde, mergeado em `dev@4ecf1459`): FASE 0 closure (cargo-vet informativo + dependency-review + OSV), smoke com JWT fresco, drift corrigido.
- [ ] Ciclo 029 (candidato): e2e browser, SBOM em release (depende de tag), ADR-0002 Tauri, cargo-vet bloqueante.
- [~] Frontend Svelte 5 canônico com lacunas de contrato/fluxo; `apps/web` não-canônico ainda presente.

## 4) Backlog priorizado (80/20)

### P0 — Seed utilizável e seguro (próximo foco, ciclo 022)
1. **Busca/notificações (consolidação):** paridade mínima `task/file` no modo Meilisearch via wiring de indexação/mutações ou contrato explícito de degradação/rebuild/reindex determinístico; endurecer idempotência/dedup e cobertura de concorrência/transições de atribuição com contratos DB-gated.
2. **Arquivos (residual):** quota agregada concorrente/transacional, consistência avançada de purge/cleanup storage↔DB e ampliação de cobertura DB-gated de bordas.
3. **Realtime/resiliência complementar:** lacunas remanescentes do slice 015 (revalidação pós-handshake, `/messages/sync`/replay ampliado, multi-cliente/e2e, semântica de deleção/catch-up).
4. **Operabilidade/validação Seed:** carga/OOM e restore em 2vCPU/4GB, enforcement de limites compose, matriz opcional de serviços em CI/dev.
5. **Fatia vertical e2e mínima:** consolidar `workspace → canal → mensagem`/tarefas/arquivos com evidência em dev/CI (incl. browser quando viável).

### P1 — Consolidação do Seed
6. **Postgres-first opcionalidade:** FTS/jobs/realtime simples; tornar Valkey/Meilisearch/OIDC realmente opcionais por perfil.
7. **Qualidade/integração:** estabilização frontend (lint/check/build/tests), testes integração/contrato com serviços e gates CI relevantes.
8. **Operabilidade e higiene:** health/readiness reais, limites, observabilidade mínima, backup/restore; reconciliar README/spec tasks e destino de `apps/web`.

### P2–P3 — Produto e expansão controlada
9. Docs/notas leves, permissões finas, import/export, digest/email opcional; ajustes UX PME.
10. Sprout/Grove/Forest e módulos pesados/opcionais apenas com necessidade/demanda comprovada.

## 5) Debloat: mapa keep/replace/defer (resumo)

- **Keep/build (core):** org/workspace/usuários, canais/mensagens, tarefas leves, arquivos básicos, busca simples, notificações in-app, auth/RBAC básico.
- **Replace/simplify:** OpenSearch→Postgres FTS (default), Kafka/filas complexas→Postgres/jobs simples, FusionAuth pesado→auth embutida/OIDC opcional, storage cloud-shaped→local default/S3 opcional, microsserviços→monólito modular.
- **Defer/drop (Seed):** email cliente completo, calls/transcrição, CRM/databases genéricas, canvas, agentes/memória/rotinas, integrações profundas, paridade Macro.

## 6) Critérios para avançar

- **Seed utilizável (MVP P0):** auth/tenancy consistente + fluxo núcleo e2e verificável (mensagem ou tarefa) em ambiente dev reprodutível e compatível com documentação, com evidência rastreada; perfil operável em VPS modesta.
- **Sprout/Grove/Forest:** após Seed estável e necessidade comprovada de componentes/performance/escala.

## 7) Fora de escopo até decisão explícita

- Paridade com Macro; superfícies/integrações P2–P3 como requisito do Seed.
- Dependências proprietárias obrigatórias ou stack pesada para VPS barata.
- Reuso de código incompatível com AGPL ou engenharia reversa não autorizada.
- Deploy/produção, merges/releases, segredos ou serviços pagos sem autorização.
