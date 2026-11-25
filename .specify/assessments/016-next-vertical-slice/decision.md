# Decision: 016 — Next vertical slice (Seed/VPS operability hardening)

- **Slug**: 016-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | 015 decision | docs/research/2026-11-16-seed-vps-profile-audit.md | docs/research/2026-11-16-015-seed-vps-operability-quickwins.md | docs/research/2026-11-16-014-seed-vps-operability-international.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Runtime não é truthful/operável em VPS barato: Meilisearch hard-fail no boot apesar de `search.enabled` opcional, pool hardcoded 20 em `database.rs` ignorando config, pool/migrate duplicados em `main.rs`/`AppState`, messaging desalinhado, compose sem profiles/limits, `.env.example` com drift e backup não operacional (`intake.md:25-28`; decisão anterior:12). |
| Evidence strength | adequate | Auditorias de perfil Seed/VPS + consenso multilíngue (EN/pt-BR/es/de/fr/zh-CN/ja; ko/ru triangulados NV): core profile enxuto, deps opcionais/degradadas, limits/healthchecks, pooling conservador e `pg_dump`+restore drill (`research.md`; decisão anterior:14,38-43). |
| Value vs. inaction | strong | Cumpre diretriz de debloat/Postgres-first de `OBJETIVO.md`/`ROADMAP.md` e de-risca a implementação futura de B em hosts restritos; B tem superfície maior (multipart streaming, containment, checksum, quotas, authz/download) (`intake.md:14-17`; decisão anterior:12-13). |
| Feasibility / appetite | adequate | Mudanças cross-cutting porém localizadas em boot/config/compose/docs; risco de quebrar call sites/env e default de pool não calibrado, mitigado por compatibilidade aditiva e testes (`intake.md:29-36`; decisão anterior:27-31). |
| Strategic fit | strong | Meta explícita de self-hosted Macro alternative para SMB em 2 vCPU/4 GB, single-node, sem novos serviços obrigatórios, Postgres-first (`intake.md:8-17,30-34`; decisão anterior:7-8). |
| Risk posture | adequate | Guardrails não-goal de B, nenhum serviço obrigatório novo, defaults conservadores configuráveis e NVs para enforcement de `deploy.resources`, load/OOM e matrix CI de serviços opcionais (decisão anterior:27-43). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O scorecard sustenta a Opção C — Seed/VPS operability hardening (Postgres-first, Meilisearch/Valkey opcionais, 2 vCPU/4 GB) — sobre B (upload binário), conforme o decision space de `intake.md:12-17`. Base: `dev` pós-015/PR #218. Fatos preservados: `flora-api/src/main.rs` cria pool+migra e `AppState::new` cria outro; `flora-core/src/database.rs` hardcodeia `max_connections(20)`; `AppState` hard-faila init do Meilisearch apesar da intenção opcional; config de messaging exige `valkey_url` enquanto runtime tolera ausência; `docker-compose.yml` não tem profiles/limits e expõe os três serviços por padrão; `.env.example` mistura `FLORA_*__*` e vars OIDC sem prefixo; PG18 é mandatório (`uuidv7`) e backup/runbook não está operacionalizado. B foi adiado por superfície de segurança/contrato/schema/FE superior. Os NVs (runtime/load/OOM e restore drills não executados, enforcement de `deploy.resources` variável, acoplamento exaustivo search/messaging e matrix CI não provados, ko/ru triangulados) reforçam o recorte incremental, não o revertem.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; sintonia de defaults e enforcement de limits seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Runtime Seed não é operável nem truthful num VPS de 2 vCPU/4 GB: serviços "opcionais" são efetivamente obrigatórios no boot, pool/config duplicados e hardcoded, compose sem perfis/limites, env contract com drift e backup/restore não operacionalizado (`decisão anterior:7-12`; `intake.md:25-28`).
- **Chosen approach**: Opção C — endurecimento de operabilidade: (1) gating de serviços opcionais no boot/config com comportamento degraded explícito, (2) caminho único de pool/init com parâmetros vindos de config, (3) profiles `core`/`realtime`/`search` no compose com limits/healthchecks/binds locais/volumes persistentes e ajustes conservadores dos opcionais, (4) runbook mínimo de backup/restore Postgres + storage (decisão anterior:5-8,16-25).
- **In scope / out of scope**:
  - **In**: pool/init único e config-driven (timeouts inclusos); alinhamento search/messaging opcionalidade (config + boot + degraded handling mínimo); compose com profiles `core`/`realtime`/`search`, limits voltados a 4 GB, healthchecks, localhost binds e volumes; alinhamento canônico `FLORA_*__*` em `.env.example`/docs com notas truthful de runtime não suportado (ex.: SQLite); runbook mínimo Postgres + storage backup/restore e smoke/docs por profile; testes determinísticos de config/gating/pool e regressões direcionadas (decisão anterior:16-25). Detalhe file-by-file, verificação, riscos e debloat em `concept.md`.
  - **Out / NV**: slice B (upload/download binário, blobs, checksum/quotas/schema/FE), novos serviços obrigatórios, realtime multi-node/durável, features de indexação, redesign OIDC/SQLite, rollout S3, benchmarking de performance, deploy/merge/release. NV: load/OOM/restore drill em 2 vCPU/4 GB não executado; enforcement de `deploy.resources` varia por ambiente; acoplamento exaustivo search/messaging e matrix CI completa de serviços opcionais não provados; ko/ru por triangulação (decisão anterior:33-43).
- **Success metrics**: Boot com perfil `core` sobe sem Meilisearch/Valkey e degrada funções dependentes de forma explícita; pool/timeouts vêm de config num único caminho de init; compose com profiles e limites documentados; runbook de `pg_dump`+restore executável; testes de config/gating/pool verdes (`decisão anterior:16-25`; `intake.md:25-28`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quais defaults conservadores de pool/timeouts adotar sem benchmark no VPS alvo?]; [NEEDS CLARIFICATION: como garantir degraded handling sem quebrar call sites de search/messaging e manter compatibilidade aditiva de env?]; [NEEDS CLARIFICATION: o runtime aplica `deploy.resources` como esperado no ambiente de deploy?]. NV: matrix CI de serviços opcionais e restore drill (`decisão anterior:38-43`).
