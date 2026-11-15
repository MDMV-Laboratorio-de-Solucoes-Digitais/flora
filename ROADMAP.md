# ROADMAP — Flora

**Atualizado em:** 2026-11-15 (ciclo 005)  
**Norte:** alternativa self-hosted viável ao Macro para PMEs em VPS barata (Rust + PostgreSQL, Svelte 5; Tauri futuro), com debloat e operação enxuta.  
**Fonte de verdade operacional:** código, `Cargo.toml`, `.github/workflows/`, `.specify/memory/constitution.md`, `specs/001-005`, `.specify/assessments/003-005`, `OBJETIVO.md`.  
Itens “verificados” exigem evidência executada; caso contrário marcar NV.

## 1) Visão e horizonte

- **Produto:** workspace colaborativo self-hostable (AGPL-3.0), núcleo Rust+Postgres, UI Svelte 5, evolução para Tauri.
- **Perfis de implantação:** **Seed** (VPS barata, mínimo viável) → **Sprout** → **Grove** → **Forest**.
- **Estratégia:** monólito modular, Postgres-first, componentes opcionais, fatias verticais verificáveis; evitar paridade/complexidade do Macro no curto prazo.

## 2) Estado consolidado (2026-11-15)

| Trilha | Estado | Evidência resumida |
|---|---|---|
| Governança/SpecKit | Em andamento (maduro) | Constituição v1.2.0; specs 001-005; assessments 003-005 (go); bug flow pontual registrado |
| Direção estratégica | Atualizada (ciclo 005) | Norte “alternativa ao Macro para PMEs em VPS barata”, MVP 80/20 e debloat documentados |
| Baseline determinística Rust (local) | Parcialmente verificada | `cargo check -p flora-api -p flora-core`, `fmt --check`, `test --workspace --no-run`, `test -p flora-core --lib` (8), `test -p flora-tests --lib` (54) ok; compose config ok |
| Runtime e2e (Postgres/Valkey/Meilisearch/OIDC) | NV/incompleto | Requer execução integrada com serviços; smoke ponta a ponta não evidenciado neste ambiente |
| Frontend canônico (`apps/frontend`) | Scaffold parcial | Svelte 5 presente; contrato com backend e fluxos núcleo desalinhados/incompletos |
| Núcleo de produto (auth/tenancy, chat, tarefas, arquivos, busca, notificações, realtime) | Parcial e heterogêneo | Rotas/serviços/modelos existem; P0s em segurança de contexto, wiring e completude de fluxos |

## 3) Marcos concluídos / em andamento

- [x/~] Workspace Rust e crates de domínio/serviço estabelecidos; `flora-api` como entrypoint real.
- [x/~] Specs 001-003 com implementação parcial e drift reconhecido; ciclos 003-004 (PKCE/baseline, compose dev + consolidação docs) concluídos.
- [x] Ciclo 005: reframe estratégico (Macro-alternative MVP para PMEs/VPS barata), assessment/spec/plan/tasks e pesquisa consolidada.
- [x] Ciclos 006-007: hardening JWT/tenant em rotas núcleo, `POST /api/v1/auth/select-organization` (membership + rotação de sessão/jti) e `test-flora.sh` Bearer/JWT (PRs #204-#210, CI verde, mergeados em `dev`).
- [~] Ciclo 008 (este incremento): testes de contrato CI-safe para `select-organization` e caminho de smoke dev reproduzível sem OIDC (`scripts/dev-smoke-session.sql`, `docs/dev-smoke.md`); casos DB-backed em CI e execução integrada real **NV**.
- [~] Frontend Svelte 5 canônico iniciado/desalinhado com contratos auth/tenant atuais; `apps/web` não-canônico ainda presente.

## 4) Backlog priorizado (80/20)

### P0 — Seed utilizável e seguro (próximo foco, ciclo 006 candidato)
1. **Auth/contexto/tenancy consistente:** validar JWT/sessão/org nas rotas críticas; reduzir confiança em headers crus; fechar lacunas PKCE/OIDC prioritárias com testes/evidência.
2. **Fatia vertical e2e mínima:** `workspace → canal → mensagem` (e/ou tarefa) com persistência, leitura e realtime mínimo verificáveis em dev.
3. **Contrato frontend↔backend (caminho feliz):** alinhar endpoints/protocolo/contexto para o fluxo núcleo ou prover smoke API reproduzível.
4. **Arquivos básicos reais:** upload/download binário e contrato/quotas coerentes para o escopo Seed.
5. **Wiring essencial:** indexação de busca e eventos→notificações para mutações core do fluxo escolhido.

### P1 — Consolidação do Seed
6. **Postgres-first opcionalidade:** FTS/jobs/realtime simples onde viável; tornar Valkey/Meilisearch/OIDC realmente opcionais por perfil.
7. **Qualidade/integração:** testes de integração/contrato com serviços, estabilização frontend (lint/tests) e gates CI relevantes.
8. **Operabilidade:** health/readiness reais, limites básicos, observabilidade mínima, backup/restore documentado/testável.
9. **Higiene:** reconciliar README/spec tasks com realidade; definir destino de `apps/web`.

### P2 — Produto (pós-Seed estável)
10. Docs/notas markdown leves, permissões/compartilhamento mais finos, import/export, digest/email opcional.
11. Ajustes de UX Svelte 5 para produtividade PME (sem replicar complexidade Macro).

### P3 — Expansão controlada (demanda comprovada)
12. Sprout/Grove/Forest: PGMQ/RustFS/Meilisearch/Valkey/OIDC avançado, réplicas, workers, HA/observabilidade — somente com necessidade e medição.
13. Integrações/funcionalidades pesadas (email sync completo, calls, CRM/databases, agents avançados, Tauri) apenas como módulos opcionais e após núcleo confiável.

## 5) Debloat: mapa keep/replace/defer (resumo)

- **Keep/build (core):** org/workspace/usuários, canais/mensagens, tarefas leves, arquivos básicos, busca simples, notificações in-app, auth/RBAC básico.
- **Replace/simplify:** OpenSearch→Postgres FTS (default), Kafka/filas complexas→Postgres/jobs simples, FusionAuth pesado→auth embutida/OIDC opcional, storage cloud-shaped→local default/S3 opcional, microsserviços→monólito modular.
- **Defer/drop (Seed):** email cliente completo, calls/transcrição, CRM/databases genéricas, canvas, agentes/memória/rotinas, integrações profundas, paridade Macro.

## 6) Critérios para avançar

- **Seed utilizável (MVP P0):** auth/tenancy consistente + fluxo núcleo e2e verificável (mensagem ou tarefa) em ambiente dev reprodutível, com evidência e verificações determinísticas rastreadas; perfil operável em VPS modesta.
- **Sprout/Grove:** após Seed estável e necessidade comprovada de componentes adicionais/performance.
- **Forest:** após requisitos claros de escala/resiliência e capacidade operacional.

## 7) Fora de escopo até decisão explícita

- Paridade com Macro; superfícies/integrações P2–P3 como requisito do Seed.
- Dependências proprietárias obrigatórias ou stack pesada para VPS barata.
- Reuso de código incompatível com AGPL ou engenharia reversa não autorizada.
- Deploy/produção, merges/releases, segredos ou serviços pagos sem autorização.
