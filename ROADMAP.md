# ROADMAP — Flora

**Atualizado em:** 2026-11-15 (ciclo 009)  
**Norte:** alternativa self-hosted viável ao Macro para PMEs em VPS barata (Rust + PostgreSQL, Svelte 5; Tauri futuro), com debloat e operação enxuta.  
**Fonte de verdade operacional:** código, `Cargo.toml`, `.github/workflows/`, `.specify/memory/constitution.md`, `specs/001-009`, `.specify/assessments/003-009`, `OBJETIVO.md`.  
Itens “verificados” exigem evidência executada; caso contrário marcar NV.

## 1) Visão e horizonte

- **Produto:** workspace colaborativo self-hostable (AGPL-3.0), núcleo Rust+Postgres, UI Svelte 5, evolução para Tauri.
- **Perfis de implantação:** **Seed** (VPS barata, mínimo viável) → **Sprout** → **Grove** → **Forest**.
- **Estratégia:** monólito modular, Postgres-first, componentes opcionais, fatias verticais verificáveis; evitar paridade/complexidade do Macro no curto prazo.

## 2) Estado consolidado (2026-11-15)

| Trilha | Estado | Evidência resumida |
|---|---|---|
| Governança/SpecKit | Em andamento (maduro) | Constituição v1.2.0; specs 001-009; assessments 003-009 (go) |
| Direção estratégica | Atualizada | Norte “alternativa ao Macro para PMEs em VPS barata”, MVP 80/20 e debloat documentados |
| Baseline determinística Rust (local) | Parcialmente verificada | `cargo fmt --check`, `cargo check -p flora-api -p flora-core -p flora-tests`, `test -p flora-core --lib` (8), `test -p flora-tests --lib` (56 pass, 5 ignored) ok |
| Select-org DB-backed (local/CI config) | Parcialmente verificado | 5 testes `#[ignore]` passaram localmente em Postgres 18; job `db-tests` adicionado em `.github/workflows/rust.yml` (execução remota PR: NV) |
| Smoke integrado dev (`select org → workspace → task`) | Evidenciado (alternativo) / NV compose-documentado | rc=0 em stack efêmera alt-port (pg18:15432, valkey:16379, meili:17700); bloqueio compose por 5432 ocupado e migração `uuidv7()` incompatível com pg16 do compose |
| Frontend canônico (`apps/frontend`) | Scaffold + fatia mínima | Callback `state`, select-org/token rotation, guard org, workspace→task via API (ciclo 009); verificação determinística completa NV |
| Núcleo de produto (auth/tenancy, chat, tarefas, arquivos, busca, notificações, realtime) | Parcial e heterogêneo | Rotas/serviços/modelos existem; P0s em segurança/tenancy consistente, compatibilidade dev, wiring e completude de fluxos |

## 3) Marcos concluídos / em andamento

- [x/~] Workspace Rust e crates de domínio/serviço estabelecidos; `flora-api` como entrypoint real.
- [x] Ciclos 003-008: PKCE/baseline, compose dev, hardening JWT/tenant, `select-organization` + rotação jti, testes contrato CI-safe e bootstrap de smoke dev (PRs #204–#211, CI verde, mergeados em `dev`).
- [~] Ciclo 009 (este incremento): testes DB-backed env-gated/`#[ignore]` para `select-organization` (sucesso/rotação, não-membro, sessão ausente/inativa/mismatch), job CI `db-tests` (Postgres), smoke integrado executado com evidência (rc=0, alt-port/pg18) e fatia mínima frontend (state/callback, select-org/token rotation, workspace→task). **Pendente:** PR/CI remoto/merge/sync; NV compose pg16/`uuidv7` e verificação frontend completa.
- [~] Frontend Svelte 5 canônico com lacunas de contrato/fluxo; `apps/web` não-canônico ainda presente.

## 4) Backlog priorizado (80/20)

### P0 — Seed utilizável e seguro (próximo foco, ciclo 010 candidato)
1. **Compatibilidade dev/compose e smoke documentado:** reconciliar `docker-compose.yml`/migração para `uuidv7` no Postgres alvo e evidenciar `docs/dev-smoke.md` via caminho documentado.
2. **CI/evidência remota:** confirmar job `db-tests` verde no PR/`dev`; ampliar cobertura DB de contratos críticos conforme risco.
3. **Auth/contexto/tenancy consistente:** session authority no extrator/fluxos, reduzir confiança em headers crus, alinhar contratos frontend↔backend no caminho feliz.
4. **Fatia vertical e2e mínima:** `workspace → canal → mensagem` e/ou tarefa com persistência, leitura e realtime mínimo verificáveis em dev.
5. **Arquivos básicos reais e wiring essencial:** upload/download binário, quotas coerentes, indexação de busca e eventos→notificações para mutações core.

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
