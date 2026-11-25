# Decision: 018 — Next vertical slice (file hardening + contratos DB-gated)

- **Slug**: 018-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | 017 decision/artifacts | docs/research/2026-11-16-018-file-contracts-hardening-vps.md | readiness `ses_e1d8d03e6ffehEXmO6kEraL1aX` | focused audit `ses_e1d8b443cffe9g6ewGSBRnZNNn`

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | 017 entregou bytes mas deixou NVs de P0: workspace∈org e authz não profundamente validados nas rotas de arquivo, contratos DB-gated ausentes, quota agregada concorrente NV e sem evidência de load/OOM/proxy (`intake.md:5-9`; decisão anterior:12). |
| Evidence strength | strong | Matriz C1–C8/I1–I6 de `docs/research/2026-11-16-018-file-contracts-hardening-vps.md` + readiness audit (`ses_e1d8d03e6ffehEXmO6kEraL1aX`) e focused audit (`ses_e1d8b443cffe9g6ewGSBRnZNNn`) confirmam gaps concretos em `files.rs`, `file_service.rs`, repository/storage/config e testes placeholder (decisão anterior:13; `intake.md:10-12`). |
| Value vs. inaction | strong | Protege e verifica com evidência determinística um caminho P0 sensível à segurança (arquivos binários) sem novo serviço obrigatório; alternativa de wiring search/notifications é mais ampla (contrato/crate/tx/produto), diferida para 019 (`intake.md:14-17`; decisão anterior:14-15). |
| Feasibility / appetite | adequate | Escopo de hardening localizado + testes PG18 descartáveis com fixtures isoladas; risco de tightenings quebrarem fluxos implícitos e complexidade de quota agregada concorrente mitigados por política de erro explícita e semântica testada (`intake.md:19-22`; decisão anterior:25-29). |
| Strategic fit | strong | `OBJETIVO.md`/`ROADMAP.md` e validação 017 apontam contratos DB-gated/hardening do slice de arquivos como prioridade; Postgres-first, VPS 2 vCPU/4 GB, CI green (decisão anterior:7,12,14; `intake.md:5,9,19-22`). |
| Risk posture | adequate | Env-gated tests, fixtures determinísticos, compatibilidade-minded para erros/URLs e NV explícito de quota concorrente/load/browser e2e; non-goals de parity search/notifications e refactors amplos (decisão anterior:25-34). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O scorecard sustenta a Opção A1 — file slice hardening + contratos DB-gated (PG18 descartável, Seed/VPS) — sobre B (wiring search/notifications) e a combinação A1+B, conforme `intake.md:14-17`. Baseline: `dev@b6440c534` (017 slice B merged via PR #220, CI green). Fatos preservados: os NVs de 017 (sem contratos DB-gated de arquivos, quota agregada/concorrência NV, workspace membership/authz não validada a fundo nas rotas de arquivo, sem evidência de 2vCPU/4GB/proxy/load/restore); as auditorias `ses_e1d8d03e6ffehEXmO6kEraL1aX` (recomenda A1) e `ses_e1d8b443cffe9g6ewGSBRnZNNn` (gaps de hardening); a matriz C1–C8/I1–I6 e os testes `flora-tests` de arquivo ainda placeholder. O recorte hardening+evidência determinística vem antes do wiring de search/notifications (ideias 018-S1 em `2026-11-16-018-search-notifications-wiring-vps.md`, `2026-11-16-018-postgres-search-outbox-vps.md`, `2026-11-16-018-macro-search-notifications-parity.md`), diferido para 019 salvo se trivialmente isolado. Os NVs e riscos reforçam o foco, não invertem a decisão.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; quota agregada concorrente e reconciliação storage/DB seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: NVs de segurança/evidência do slice B de 017: workspace∈org e authz inconsistentes nas rotas de arquivo, multipart/validação sem determinismo, quota/cleanup implícitos e testes de contrato placeholder (`intake.md:5-9`; decisão anterior:7,12-13).
- **Chosen approach**: Opção A1 — endurecer invariantes de 017 e substituir testes placeholder por contratos/integração DB-gated determinísticos com PG18 descartável e storage temporário isolado; validação workspace∈org + authz consistente em upload/get/download/delete/metadata legado, multipart tolerante a ordem de campos e determinístico, quota/integridade/cleanup explícitos (decisão anterior:5-8,17-23).
- **In scope / out of scope**:
  - **In**: existência da workspace e pertencimento à org (e acesso consistente do chamador) em upload/metadata legado e checks tenant/workspace em read/download/delete; parsing multipart/validação endurecido (ordem de campos, single-file/required fields), respostas/URLs FE-compatíveis e comportamento metadata-only legado explícito/testado; semântica de quota/integridade/cleanup explícita (caps por arquivo + checksum/tamanho implementados; guard agregado mínimo se viável, senão codificar/testar o comportamento per-file/config e registrar NV; consistência storage/DB best-effort em falha/delete/purge); contratos DB-gated (PG18, migrations, fixtures, temp storage, helpers multipart) para happy paths, isolamento/authz, validação/limits e cleanup/delete/quota; artefatos SDD, evidência, PR/CI green, merge/sync e updates de planejamento (decisão anterior:17-23).
  - **Out / NV**: parity Macro search/notifications (018-S1, diferido a 019), refators amplos de crate/service, novos serviços obrigatórios, outbox/workers/push, redesign do frontend. NV: load/OOM/proxy-limit/restore em 2vCPU/4GB, browser e2e, prova concorrente de quota agregada (salvo implementada), AV/sniffing inferencial, resumable/S3/presigned, indexação/previews, storage multi-node (decisão anterior:31-34; `intake.md:19-22`).
- **Success metrics**: Contratos DB-gated determinísticos substituem os placeholders e cobrem happy path, isolamento/authz, validação/limits e cleanup/delete/quota conforme semântica implementada; workspace fora da org e chamador não autorizado são rejeitados em upload/get/download/delete com erro explícito e compatível (`decisão anterior:17-23`; `problem.md` via `intake.md:7-9`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: implementar guard de quota agregada concorrente ou codificar semântica per-file/config e manter NV?]; [NEEDS CLARIFICATION: qual política de erro explícita preserva fluxos implícitos e compatibilidade de URL/DTO do FE?]; [NEEDS CLARIFICATION: até onde levar a reconciliação storage/DB em falha/delete/purge sem ampliar o escopo?]; [NEEDS CLARIFICATION: quando retomar o wiring de search/notifications em 019?]. NV: load/OOM/proxy/restore, browser e2e, quota concorrente, AV/indexação/multi-node (`decisão anterior:31-34`).
