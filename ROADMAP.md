# ROADMAP — Flora

**Atualizado em:** 2026-11-17 (ciclos 011–046; 044 Seed closure + ADRs 0004–0006, 045 fix cross-tenant, 046 auth hardening). ADR-0003 rege push/PR/merge como decisão de máquina sob gates determinísticos.  
**Norte:** alternativa self-hosted viável ao Macro para PMEs em VPS barata (Rust + PostgreSQL, Svelte 5; Tauri futuro), com debloat e operação enxuta.  
**Fonte de verdade operacional:** código, `Cargo.toml`, `.github/workflows/`, `.specify/memory/constitution.md`, `specs/001-033`, `.specify/assessments/003-033`, `OBJETIVO.md`.  
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
- [x] Ciclo 029 (PR #238, CI verde, mergeado em `dev@b8972c0e`): clippy `--all-targets`, ADR-0002, drift 028 e smoke integrado rc=0 (JWT fresco, portas isoladas).
- [x] Ciclo 030 (PR #240, CI verde incl. e2e-browser, mergeado em `dev@ab3fc007`): 1 e2e smoke Chromium (`retries: 0`), minter feature-gated, drift documental corrigido.
- [x] Ciclo 031 (PR #242, CI verde, mergeado em `dev@aaa45536`): versão centralizada + release.yml tag-only/SBOM (sem tag), drift documental fechado.
- [x] Ciclo 032 (PR #244, CI verde, mergeado em `dev@cecaf686`): allowlist CORS backend + drift documental; ADR-0002 #2 segue NV.
- [x] Ciclo 033 (PR #246, CI verde, mergeado em `dev@f89678d5`): busca no header ligada ao backend verificado + drift documental; ADR-0002 #2 segue NV.
- [x] Ciclo 034 (PR #248, CI verde, mergeado em `dev@23ca350c`): ADR-0002 #2 atendido (guard WS + URLs runtime + prova T8); #1/#3/#4 seguem NV.
- [x] Ciclos 035–038 (PRs #241–#255, CI verde, mergeados em `dev@c177caa46`): ver evidência nos commits; ADR-0003 passou a reger merge como decisão de máquina sob 4 required checks.
- [x] Ciclo 039 (2026-11-17): restore drill executado (runbook 032) — B1–B9 PASS, 12/12 contagens iguais, `test-flora.sh` `rc=0`, RTO 97 s, caps 1 vCPU/1 GB enforcement confirmado. NV-1 do Seed backup fechado localmente. Sem código/deploy/tag. Ver `.specify/validations/039-restore-drill/` e `docs/research/2026-11-17-039-restore-drill-80-20.md`.
- [x] Ciclo 040 (2026-11-17): remoção de `apps/web` (5.844 arquivos/43 MB) + 2 e2e mortos; `KanbanBoard` real (props + PATCH status) com módulo puro testável; non-regression provado nos 4 gates frontend (30→36 testes). Ver `specs/040-seed-ui-debloat/` e `docs/research/2026-11-17-040-seed-ui-debloat.md`.
- [x] Ciclo 041 (2026-11-17): debloat Rust — remover 4 deps dead + 2 members órfãos (−34 pacotes/−369 linhas do Cargo.lock, zero adições); gates idênticos ao baseline. Ver `specs/041-rust-dead-crates/` e `docs/research/2026-11-17-041-rust-debloat.md`.
- [x] Ciclo 042 (2026-11-17): `GET /api/v1/roles` real (tenancy da sessão + gate `OrgRead|OrgAdmin` + DTO sem vazamento) com helper puro e 3 testes. Ver `specs/042-real-rbac-list/` e `docs/research/2026-11-17-042-real-rbac.md`.
- [x] Ciclo 043 (2026-11-17): manifesto `/llms.txt` sem drift de tenancy (org vem da sessão, não de header) + teste de ausência. Ver `docs/research/2026-11-17-043-manifest-tenancy-drift.md`.
- [x] Ciclo 044 (2026-11-17): Seed verification closure — ADRs 0004–0006 (roles/RLS/rollout, enforcement EVOLUTIVO), 5 contratos DB reais (workspace/tasks), 2 bugs corrigidos (Member perms, soft-delete). Ver `docs/research/2026-11-17-044-seed-verification-closure.md`.
- [x] Ciclo 045 (2026-11-17): bug cross-tenant em `POST /tasks` fechado (`workspace_id` do body validado contra a org da sessão); módulo `tenant.rs` compartilhado + contrato `db_task_cross_org_workspace_rejected`. Ver `docs/research/2026-11-17-045-cross-tenant-workspace-fix.md`.
- [x] Ciclo 046 (2026-11-17): auth hardening — handlers RBAC autenticados (R1) + logout real revogando só a sessão atual (C2); contrato DB de replay → 401. Ver `docs/research/2026-11-17-046-auth-hardening.md`.
- [ ] Ciclo 047 (candidato): R2 (revalidar membership em 7 handlers), R3 (`thread_id`), 404s frontend, converter stub `contract/rbac.rs`; promover contratos a legs `db-tests` e wrapper RLS exigem autorização SE. FASE 7 Documents segue adiado.
- [~] Frontend Svelte 5 canônico com lacunas de contrato/fluxo; `apps/web` **removido** no 040.

## 4) Backlog priorizado (80/20)

### P0 — Seed utilizável e seguro (concluído e verificado em 2026-11-16)
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
- Deploy/produção, publicação de tag/release, segredos ou serviços pagos sem autorização. Push/PR/merge são decisão de máquina sob gates determinísticos obrigatórios (ADR-0003) e não exigem autorização humana.

## Estado verificado — 2026-11-17T13:42Z (append-only)

- Âncora real `dev@04bf70bdd`; `check_gate_integrity.py` PASS. Contagens por `find`: specs=37 (até `042`), assessments=34, validations=33, releases=33, bugs=4, research=135.
- As seções anteriores registram ciclos até `046`, mas `specs/` para em `042`: ciclos `043-046` têm validação/release sem spec — dívida de fase, não ciclo em voo (árvore limpa, release `046` presente).
- `plan.md` ausente em `025-042`; nenhum `analyze.md`/`converge.md` no repo; `feature.json`/`STATE.md` ausentes. Não há spec ≥088, portanto a exigência de `analyze.md`+`converge.md` é N/A hoje.
- Decisão A (alvo): GitHub Actions + `gh` + `check_gate_integrity.py` permanecem autoritativos; nenhum artefato da factory `aa7600d` foi importado.
- **[NV]** CI/PR/branch protection ao vivo não verificados. Nenhum merge, auto-merge, tag ou deploy realizado nesta sessão.

## Estado verificado — 2026-11-17T14:34Z (append-only)

- Âncora `github/dev@4ae58ccd2`; `check_gate_integrity.py` PASS. Contagens por `find` nesta base: specs=38, assessments=35, validations=30, releases=30, bugs=4, research=132.
- Ciclo 048 corrige `AGENTS.md`: 5 membros + raiz (6 manifests `flora*`) e toolchain `1.98.1`; nenhuma regra, gate ou permissão alterada.
- Base `github/dev` não contém os ciclos 044–046 presentes no `dev` local; por isso `specs/` para em 042 e as contagens diferem da sessão anterior. Nenhum trabalho alheio foi reescrito.
- **[NV]** CI/branch protection/PRs ao vivo não verificados. Sem merge, auto-merge, tag, deploy ou force-push.

## Estado verificado — 2026-11-17T15:39Z (append-only)

- Âncora `github/dev@55363b3ca`; `check_gate_integrity.py` PASS; gates Rust/frontend todos exit 0.
- Ciclo 049 remove o transporte morto de telemetria no frontend (`logger.ts` local-only, contrato 002 atualizado, teste no projeto `server`): zero chamadas de rede, zero 404s.
- PR #263 MERGED e PR #262 DIRTY (conflito alheio, não tocado). Sem merge/auto-merge/tag/deploy/force-push nesta sessão.
- Próximos candidatos: C4 (3.75), C5 (2.08, decisão de dono), C6 (0.80, autorização SE). **[NV]** para estados remotos finais.

## Estado verificado — 2026-11-17T16:20Z (append-only)

- Âncora `github/dev@b3bfda505`; `check_gate_integrity.py` PASS; gates Rust/frontend exit 0.
- Ciclo 050 corrige C25: `classify_endpoint` reconhece os paths montados sob `/api/v1`, devolvendo os limites específicos de login/refresh/invite; 4 testes novos (Rust 115 passed).
- Base atual já contém os ciclos 047/048/049 mergeados; nenhum PR aberto no início deste ciclo.
- Próximos: N3 (4.0), C4 (3.75), C5 (2.08, decisão de dono), C6 (0.80, autorização SE). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-17T17:35Z (append-only)

- Âncora `github/dev@b3bfda505`; `check_gate_integrity.py` PASS; gates Rust/frontend exit 0.
- Ciclo 051 encerra C4 pela via mínima: remoção da UI de convite sem backend. Nenhum endpoint criado; nenhuma decisão de autorização tomada. Spec 002 marcada, não reescrita.
- PR #265 (ciclo 050) aberto e `CLEAN` nesta base; não tocado. Sem merge/auto-merge/tag/deploy nesta sessão.
- Próximos: tag `osv-scan@v2` (6.0, validade **[NV]**), C5 (2.08, decisão de dono), N3 (1.5), C6 (0.80, autorização SE).

## Estado verificado — 2026-11-17T23:44Z (append-only)

- Âncora `github/dev@01d822205`; `check_gate_integrity.py` PASS (4 required checks, pisos 43).
- Ciclo 052 fecha o item "tag `osv-scan@v2`" do 051: `osv-scan.yml:14` passa a `@v2.7.1`. Tag verificado em duas fontes primárias; job continua advisory (não required), sem ADR-0003 alterado.
- PR de `feature/052-osv-scan-ref-pin` → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: C25b (2.0), V1 (1.25), N3 (1.0), C5 (decisão de dono), C6 (autorização SE). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T00:10Z (append-only)

- Âncora `github/dev@22f16d6ac`; `check_gate_integrity.py` PASS (4 required checks).
- Ciclo 053 eleva a CI: cobertura de 44→50 contratos DB com dois legs novos no job existente (`tasks` 4, `workspace` 2). Guardião passa a 8 pisos / 49 total.
- Verificação local com DB real isolado confirmou 50 testes executados; PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: V1 (fail-open, 1.25), N3 (1.0), C5 (decisão de dono), C6 (autorização SE). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T01:13Z (append-only)

- Âncora `github/dev@dd982cadc`; `check_gate_integrity.py` PASS (4 required checks, 8 pisos/49).
- Ciclo 054 fecha V1: `run_leg` falha antes do cargo sem env de DB; guardião detecta remoção futura das linhas de env. CI verde agora implica testes reais.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: auth floor 12→13, N3 (1.0), C5 (decisão de dono), C6 (autorização SE). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T01:49Z (append-only)

- Âncora `github/dev@d5d7ef5d`; `check_gate_integrity.py` PASS (4 required checks, 8 pisos/50).
- Ciclo 055 remove a última folga de piso: auth 12→13 (13 testes descobertos e executados). Guardião valida em lockstep; mutação prova a rejeição.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: ADR drift 43→50 (5.0, governança), índice docs (3.33), N3 (1.0), C5/C6 (autorização). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T02:29Z (append-only)

- Âncora `github/dev@94fcf19ca`; `check_gate_integrity.py` PASS (4 checks, 8 pisos/50).
- Ciclo 056 fecha a dívida de índice: research (135 docs) passa a ter entrada verificada, e `PROGRESSO.md` registra estado datado. Correções pós-verificação aplicadas e confirmadas.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: ADR drift (5.0, escalado por aprovação), N3 (1.0), C5/C6 (autorização). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T02:48Z (append-only)

- Âncora `github/dev@97838cd9c`; `check_gate_integrity.py` PASS com total derivado de `FLOORS`.
- Ciclo 057 elimina a duplicação de verdade na saída do guardião; mutação com 9º piso prova a adaptação automática do texto.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: ADR drift (5.0, escalado), root `index.md` (2.5), N3 (1.0), C5/C6 (autorização). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T03:08Z (append-only)

- Âncora `github/dev@58921a337`; `check_gate_integrity.py` PASS (total derivado de FLOORS).
- Ciclo 058 cria `index.md` na raiz, fechando o último ponto de entrada ausente do Bloco 4. Índice sem duplicação de contagens; links 8/8 válidos.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: ADR drift (5.0, escalado), falso positivo local (1.875), N3 (1.0), C5/C6 (autorização). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T03:31Z (append-only)

- Âncora `github/dev@2730fd2dd`; `check_gate_integrity.py` PASS (8 pisos/50).
- Ciclo 059 elimina o "passed" vazio local: sem env os contratos DB falham; com DB real os 50 testes passam. CI permanece fail-closed (054) e inalterada.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- Próximos: ADR drift (5.0, escalado), N3 (1.0), C5/C6 (autorização). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T03:54Z (append-only)

- Âncora `github/dev@262ac042f`; `check_gate_integrity.py` PASS (8 pisos/50 derivados).
- Ciclo 060 atualiza o estado verificado de `PROGRESSO.md` com contagens atuais (specs 51/230, assessments 48/197, validations 46/47, releases 46/46), preservando o histórico.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- **Escalados [NV]:** ADR-0003/0006 (maintainer) e N3 (FK/SE). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T04:11Z (append-only)

- Âncora `github/dev@7ed232c46`; `check_gate_integrity.py` PASS (8 pisos/50 derivados).
- Ciclo 061 remove código morto alinhado ao Princípio I: circuit-breaker nunca compilado, config e erro órfãos. Zero mudança de comportamento.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- **Escalados [NV]:** ADR-0003/0006, N3, C5/C6. **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T04:31Z (append-only)

- Âncora `github/dev@104c581f3`; `check_gate_integrity.py` PASS (8 pisos/50 derivados).
- Ciclo 062 consolida a fila escalada em `STATE.md`, evitando reavaliação e execução sem autorização. OSV permanece sem runs até o próximo cron semanal.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- **Escalados [NV]:** ADR-0003/0006, N3, C5/C6. **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T04:50Z (append-only)

- Âncora `github/dev@e1ff815bf`; `check_gate_integrity.py` PASS (8 pisos/50 derivados).
- Ciclo 063 fecha o último [NV] executável: scan OSV rodou com sucesso no pin `@v2.7.1`; sinal advisory operacional.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- **Escalados [NV]:** ADR-0003/0006, N3, C5/C6. **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T11:29Z (append-only)

- Âncora `github/dev@448090327`; `check_gate_integrity.py` PASS (8 pisos/51).
- Ciclo 064 fecha N3 em código e schema: papéis passam a ser resolvidos dentro da org da sessão; banco rejeita ponteiro cross-org/órfão. Deploy em produção segue **[NV]** e depende de auditoria local lá.
- PR → `dev/` com CI observada ao vivo antes do merge (ADR-0003).
- **Autorizados e enfileirados:** ADR text, C5 `RoleWrite`, C6 canário (sem produção). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T13:09Z (append-only)

- Âncora `github/dev@aef13277a`; `check_gate_integrity.py` PASS (8 pisos/51).
- Ciclo 065 fecha a divergência documental entre ADRs e gate; nenhum código/gate alterado.
- **Próximos autorizados:** C5 `RoleWrite`, C6 desenho de canário (sem produção). **[NV]** para CI/PRs ao vivo.

## Estado verificado — 2026-11-18T14:50Z (append-only)

- Âncora `github/dev@8d0cb64b0`; `check_gate_integrity.py` PASS (8 pisos/51).
- Ciclo 066 encerra os handlers placeholder de RBAC: agora há autoridade explícita (`RoleWrite`) e imutabilidade de builtins.
- **Próximo autorizado:** C6 — desenho de canário RLS com rollback, sem habilitar em produção. **[NV]** CI/PRs ao vivo.

## Estado verificado — 2026-11-19T01:58Z (append-only)

- Âncora `dev@cbad031b5` (= `github/dev`); `check_gate_integrity.py` PASS (8 pisos / 51), `--remote` OK para `dev`/`main`.
- O intervalo verificado neste repositório chega ao ciclo 067; o cabeçalho histórico "ciclos 011–046" permanece como registro da data em que foi escrito.
- Ciclo 068: documentação de estado atualizada por `find`; sem mudança em roadmap estratégico, gates, required checks ou produção.
- **[NV]** CI/PRs futuros, Forgejo e produção; merge apenas após CI verde observada ao vivo (ADR-0003).

## Estado verificado — 2026-11-19T04:22:55Z (ciclo 072; append-only)

- Âncora de início `dev@cdf50eaff`; `check_gate_integrity.py` PASS com 4 required checks e 8 pisos / 51 às `04:14:31Z`.
- Intervalo verificado ampliado para `specs/001`–`specs/070` (61 diretórios); assessments 58, validations 57, releases 57, bugs 4, research 137 (sem `index.md`).
- Ciclos 069 e 070 estão fechados; o cabeçalho histórico “ciclos 011–046” e a nota 01:58Z permanecem como registro das respectivas datas.
- **Fora do roadmap:** leg RBAC no CI (aprov maintainer) e emenda constitucional (maintainer) continuam escaladas **[NV]**.

### Correção append-only — 2026-11-19T04:23:44Z (ciclo 072)

O intervalo 001–070 da nota `04:22:55Z` permanece; as contagens finais após
artefatos 072 são 62 specs / 59 assessments / 58 validations / 58 releases /
4 bugs / 137 research (138 com `index.md`). Nada acima foi reescrito.

## Estado verificado — 2026-11-20 (ciclo 085; append-only)

- Âncora `feature/085-local-first-seed@38184fa50`; `check_gate_integrity.py` PASS (4 required checks, 8 pisos / 51).
- Contagens atuais por `find`: `specs/` 75 diretórios, assessments 72, validations 70, releases 70, bugs 4, research 137 (138 com `index.md`). Cabeçalho histórico “ciclos 011–046” e notas 01:58Z / 04:22:55Z / 23:12Z permanecem intactos.
- Ciclo 085 concluído localmente: bootstrap one-time com E2E real `201` → `409` e `GET /api/v1/tasks` `200` com token; rota pública documentada em `specs/085-local-first-seed/contracts/bootstrap.md`. Sem mudança no roadmap estratégico.
- **Exceção FR-012 (registrada):** bootstrap sem verificação de e-mail, somente em instalação vazia; nota em `docs/governance/2026-11-20-085-fr-012-bootstrap-sem-verificacao-email.md`.
- **Fora do roadmap / [NV]:** leg RBAC no CI (aprov maintainer), emenda constitucional, CI futuro, Forgejo e produção; merge apenas após CI verde observada ao vivo.

### Revalidação determinística — 2026-11-20T09:34Z

- Gate PASS e PR #309 com checks verdes, sem conflitos; branch `feature/085-local-first-seed` é a única não mesclada em `dev`.
- Contagens atuais: specs 75, assessments 72, validations **71**, releases **71**, bugs 4; research 137/138.
- Mesmo com `required_pull_request_reviews=null` na proteção de `dev`, a regra explícita deste prompt exige aprovador distinto; ausência de reviews mantém o PR aberto, sem auto-merge/force-push.

## Estado verificado — 2026-11-22 (ciclo 114; append-only)

- Recontagem via `find` (`feature/114-wcag-ci-guard`): `specs/` 104 diretórios (556 arquivos, `001`–`114`), `.specify/assessments/` 102 (428), `.specify/validations/` 100 (106 pré-`evidence.md`), `.specify/releases/` 100 (100), `.specify/bugs/` 8 (26), `docs/adr/` 8, `docs/research/` 137 na raiz sem `index.md` (+11 em `mdmv-multiagent/` = 148).
- Ciclo 114 (`400/625`): blindagem WCAG 2.2 AA no `e2e-browser` + correções e auditoria em `/files` + `a11yAudit.ts` (issues `#373`..`#377`). Gate local `check_gate_integrity.py` PASS (10 legs / 77 pisos).
- Próximo candidato 80/20 mapeado (`300/625`): listagem `GET /api/v1/files?workspace_id=...` e persistência pós-reload em `/files`.
- **[NV]** CI remoto, ACR, modo escuro, staging real, Forgejo e produção; histórico preservado.

## Estado verificado — 2026-11-23 (ciclo 115; append-only)

- Recontagem via `find` (`feature/115-files-list-persistence`): `specs/` 105 diretórios (562 arquivos, `001`–`115`), `.specify/assessments/` 103 (433), `.specify/validations/` 101 (109 pré-`evidence.md`), `.specify/releases/` 101 (101), `.specify/bugs/` 8 (26), `docs/adr/` 8, `docs/research/` 137 na raiz sem `index.md` (+11 em `mdmv-multiagent/` = 148).
- Ciclo 115 (`400/625`): endpoint `GET /api/v1/files` escopado por organização/workspace, contrato DB (`files: 6 -> 7`, 78 total), persistência em `/files`, E2E `t['files.empty']` e Briefing de Compreensão `tenant` (`#379`..`#383`), SemVer `0.7.16 -> 0.8.0`.
- Próximo candidato 80/20 mapeado (`240/625`): lookup de identidade real de autores no chat (ciclo 116).
- **[NV]** CI remoto, ACR, modo escuro, staging real, Forgejo e produção; histórico preservado.

## Estado verificado — 2026-11-23 (ciclo 116; append-only)

- Recontagem via `find` (`feature/116-chat-author-identity`): `specs/` 106 diretórios (568 arquivos, `001`–`116`), `.specify/assessments/` 104 (438), `.specify/validations/` 102 (112 pré-`evidence.md`), `.specify/releases/` 102 (102), `.specify/bugs/` 8 (26), `docs/adr/` 8, `docs/research/` 137 na raiz sem `index.md` (+11 em `mdmv-multiagent/` = 148).
- Ciclo 116 (`240/625`): identidade real de autores no chat (`UserRepository::find_by_ids`, `resolve_sender_name`, `ChatLayout.svelte`, contrato `messaging: 4 -> 5`, 79 total, Briefing `tenant`, `#385`..`#389`), SemVer `0.8.0 -> 0.9.0`.
- Próximo candidato 80/20 mapeado (`240/625`): UI de notificações + ocultação/tratamento de cascas sem backend (`/dms` hardcoded e drawer de IA) ou tema escuro Tailwind v4 (ciclo 117).
- **[NV]** CI remoto, ACR, WS multi-usuário real em E2E, modo escuro, staging real, Forgejo e produção; histórico preservado.

## Estado verificado — 2026-11-23 (ciclo 117; append-only)

- Recontagem via `find` (`feature/117-notifications-ui-and-honest-navigation`): `specs/` 107 diretórios (573 arquivos, `001`–`117`), `.specify/assessments/` 105 (443), `.specify/validations/` 103 (116 com `evidence.md`), `.specify/releases/` 103 (103), `.specify/bugs/` 8 (26), `docs/adr/` 8, `docs/research/` 137 na raiz sem `index.md` (+11 em `mdmv-multiagent/` = 148).
- Ciclo 117 (`240/625`): `/api/v1/notifications` escopado por organização (`ensure_member`, `404` fail-closed cross-org, `target_url`, `POST /read-all`), contrato `notifications: 5 -> 6` (80 total), central `/notifications`, badge reativo na sidebar, remoção de atalhos placeholder (`/dms` e drawer de IA), `/dms` honesto, Passos 6 e 7 E2E com WCAG 2.2 AA, CI remoto GREEN (`45174768424`, `45174768337`), ACR `APROVADO` e Briefing `tenant` (`#391`..`#395`, PR `#396`), SemVer `0.9.0 -> 0.10.0`.
- Próximo candidato 80/20 mapeado (`240/625`): alinhamento consistente de tema escuro Tailwind v4 (`@custom-variant dark`, ciclo 118) e UI CRUD de RBAC / endpoint HTTP para `HitlReceiptPort`.
- **[NV]** Tema escuro em navegador real, staging real, Forgejo e produção; histórico preservado.

## Estado verificado — 2026-11-23 (ciclo 118; append-only)

- Recontagem via `find` (`feature/118-oidc-callback-mock-idp-exchange`): `specs/` 108 diretórios (578 arquivos, `001`–`118`), `.specify/assessments/` 106 (448), `.specify/validations/` 104 (119), `.specify/releases/` 104 (104), `.specify/bugs/` 8 (26), `docs/adr/` 8, `docs/research/` 137 na raiz sem `index.md` (+11 em `mdmv-multiagent/` = 148).
- Ciclo 117 follow-ups concluídos e mesclados em `github/dev@2ef8fd97e` (`PR #397` WCAG 2.5.3 badge + `PR #398` `.dark` Tailwind v4, persistência `flora.theme`, sidebar recolhida `#bits-s1`, `/files` reload e sub-fluxos granulares `/notifications` — `12/12` `WCAG 2.2 AA: sem achados`).
- Ciclo 118 (`2000/3125`, fecha `BF-01`): servidor IdP OIDC HTTP mock `MockOidcIdpServer` (`Ed25519`, Discovery, JWKS, PKCE `S256`) em `flora-tests/src/oidc_mock.rs`, troca extraída em `oidc.rs`, bloqueio `401` cross-subject e conta inativa em `auth.rs`, substituição dos stubs em `registration.rs`, `2` contratos DB (`auth: 23 -> 25`, `82` total) e Briefing `auth` (`#399`..`#402`), SemVer `0.10.0 -> 0.10.1`.
- Próximo candidato 80/20 mapeado: UI CRUD de RBAC/papéis (`BF-02`), endpoint HTTP para `HitlReceiptAdapter` (`BF-03`) ou seletor de tema em `/settings` (`BF-04`).
- **[NV]** CI remoto do PR 118, ACR independente, Zitadel externo real, staging real, Forgejo e produção; histórico preservado.


### Adendo Rodada 2 — 2026-11-23 (ciclo 118, append-only)

- Remediação ACR PR `#403` (`72547119`): `set_oidc_subject_if_absent` atômico (`flora-core`), `email_verified == Some(true)`, `oidc_http_client()` (`Policy::none()` + timeouts `5s`/`10s`), validação de `issuer` no Discovery + `validate_jwks_uri` estrita, sanitização de erros `Error::OidcProvider` e remoção de `base64` em `flora-tests/Cargo.toml` (`task-1323`: `flora-api` `50`, `contract::auth::db_` `25/25`, `check_gate_integrity.py` PASS `82` total). **[NV]**: ACR Rodada 2, CI remoto pós-remediação e Zitadel externo real.


## Estado verificado — 2026-11-24 (ciclo 119; append-only)

- Recontagem via `find` (`feature/119-rbac-roles-admin-ui` sobre `github/dev@42c38bf52`): `specs/` `109` diretórios (`583` arquivos, `001`–`119`), `.specify/assessments/` `107` (`453`), `.specify/validations/` `105` (`122`), `.specify/releases/` `105` (`105`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` na raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- Ciclo 119 (`1280/3125`, UI CRUD de papéis RBAC de `BF-02` fechada): `RoleItem`/`utils/roles.ts`/`api/roles.ts`/`RoleStore.svelte.ts` (`9/9` Vitest), `RoleManager.svelte` + `pt-BR.ts` em `/settings` (papéis nativos somente leitura e CRUD completo em personalizados com isolamento multi-contexto), ajuste WCAG em `layout.css` (`:root --primary`/`--ring: 152 76% 25%`, `5.85:1`), Passo 9 E2E em `.dark` e claro (`4 passed`, `14/14 WCAG 2.2 AA: sem achados`, capturas em Gist `6cef0d5c451c7050df6fa80aa2489468`) e Briefing `auth`/`rbac` (`#405`–`#408`), SemVer `0.10.1 -> 0.11.0`. PR `#404` já havia aterrissado em `github/dev@42c38bf52` com workflows manuais de deploy por ambiente (`ADR-0003`) e validadores `check_deploy_workflows.py`.
- Próximo candidato 80/20 mapeado: endpoint HTTP para `HitlReceiptAdapter` (`BF-03`), seletor de tema em `/settings` (`BF-04`) ou UI de atribuição de papéis a membros da organização (`member:*`, item remanescente de RBAC).
- **[NV]** CI remoto do PR 119, ACR independente, staging real, Forgejo e produção; histórico preservado.


## Estado verificado — 2026-11-24 (ciclo 120; append-only)

- Recontagem via `find` (`feature/120-hitl-receipt-http-endpoint` sobre `github/dev@e47983874`): `specs/` `110` diretórios (`588` arquivos, `001`–`120`), `.specify/assessments/` `108` (`458`), `.specify/validations/` `106` (`125`), `.specify/releases/` `106` (`106`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` na raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- Ciclo 120 (`2000/3125`, **`BF-03 -> fechado`**): `Error::ServiceUnavailable` → `503` sanitizado (`flora-core/src/error.rs`), rota RBAC `POST /api/v1/hitl/receipts` (`flora-api/src/routes/{hitl.rs,mod.rs}`), `3` contratos DB (`flora-tests/src/contract/hitl.rs`), trava lockstep `hitl: 3` (`82 -> 85`), Briefing `auth/rbac/hitl` (`#411`–`#413`) e SemVer MINOR `0.11.0 -> 0.12.0`.
- Próximo candidato 80/20 mapeado: seletor interativo de tema escuro/claro em `/settings` (`BF-04`) + contraste `:root --destructive` em tema claro, ou UI de atribuição/revogação de papéis a membros (`BF-02` remanescente).
- **[NV]** CI remoto do PR 120, ACR independente, staging real, Forgejo e produção; histórico preservado.
