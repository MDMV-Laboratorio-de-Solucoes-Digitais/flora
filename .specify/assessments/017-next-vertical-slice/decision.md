# Decision: 017 — Next vertical slice (binary file upload/download seguro)

- **Slug**: 017-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | 016 decision | docs/research/2026-11-16-016-binary-file-upload-hardening.md | docs/research/2026-11-16-017-secure-binary-upload-download-vps.md | docs/research/2026-11-16-017-macro-files-mvp-parity.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `flora-api/src/routes/files.rs` é metadata-only (sem multipart/download) e storage não está wired: frontend espera `/upload` e `/download/{id}` com DTO diferente, `files` sem `workspace_id` e lacunas de repo/listing — bloqueia P0 de arquivos reais (`problem.md:3-11`; decisão anterior:40). |
| Evidence strength | adequate | Consolidação de research 016/017 + codebase audit (rotas metadata-only, trait/impl baseada em buffer e não wired, quota/checksum/streaming/containment não implementados, `flora-files` duplicado não wired) e plano de testes unit/route/contract + DB-gated PG18 (`decisão anterior:15,38-40`; `intake.md:7-11`). |
| Value vs. inaction | strong | 016 adiou B explicitamente após C; entrega troca de arquivos real para SMB sem complexidade Macro-coupled (auto-import/CRDT/canvas/cloud) (`intake.md:5-14`; decisão anterior:12-14). |
| Feasibility / appetite | adequate | Local disk + Postgres metadata alinham VPS barato/Postgres-first, mas exigem streaming, containment, validação, checksum e alinhamento FE com superfície de segurança moderada (`intake.md:13-21`; decisão anterior:26-30). |
| Strategic fit | strong | `OBJETIVO.md`/`ROADMAP.md` P0: upload/download real, metadados/permissões mínimas, quotas coerentes e storage local por padrão; baseline `dev@d6cac4d74` (016-C merged, PR #219, CI green) (`intake.md:5,9`; decisão anterior:12). |
| Risk posture | adequate | Mitigações explícitas para traversal/spoofing/authz bypass (containment, server-generated keys, allowlist/sniffing, tenant checks) e para OOM em 4 GB (streaming, byte caps, sem buffering); NVs de load/OOM/restore e e2e (`decisão anterior:26-35`; `intake.md:19-21`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O scorecard confirma a Opção B — upload/download binário seguro (local disk, Postgres metadata, Seed/VPS 2 vCPU/4 GB) — sobre alternativas de realtime/search/notifications e auth/OIDC/RBAC mais amplas (`intake.md:13-16`). Baseline: `dev@d6cac4d74` (016-C merged via PR #219, CI green). Fatos preservados: a decisão 016 adiou B após entregar C; `flora-api/src/routes/files.rs` só faz JSON metadata; `FileService` cria linha no DB com validações básicas, sem persistência de bytes nem checksum real e com quota mais próxima de cap por request do que de contabilidade agregada (a confirmar minimamente); `flora-core/src/storage/local.rs`, `traits/storage_provider.rs` e `flora-files` existem mas não estão wired; FE espera multipart/upload/download com DTO divergente. O recorte evita S3/presigned/resumable, indexação, previews/transcoding e storage multi-node; os NVs (load/OOM/restore em 2 vCPU/4 GB, AV scanning, browser e2e) e o risco de superfície de segurança são mitigados pelas mitigações listadas, não invertendo a escolha.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; quotas agregadas, DTO do FE e schema de storage seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Flora registra metadados de arquivo mas não implementa um caminho real e seguro de bytes: sem multipart streaming, sem download/stream, storage unwired e contrato do FE divergente (`problem.md:3-11`).
- **Chosen approach**: Opção B — caminho mínimo de bytes: upload multipart streaming + download streaming com authz/workspace, paths de storage controlados pelo servidor e contidos na raiz canônica (temp + finalize atômico, cleanup em falha), validação/limits conservadores (sanitização de nome, MIME allowlist/sniffing best-effort, SHA-256 e tamanho real persistidos), coerência mínima de quota/metadados e alinhamento FE/API para happy path; sem S3/presigned/resumable/previews/multi-node (decisão anterior:5-8,17-24).
- **In scope / out of scope**:
  - **In**: endpoints `upload`/`download` com streaming e authz/tenant isolation; wiring do storage local com root containment, temp+atomic finalize e limpeza em falha; validação de tamanho/nome/MIME/checksum e tamanho real; quota/metadados coerentes com config/schema existente (mudanças aditivas apenas se necessárias); alinhamento mínimo dos componentes de arquivo em `apps/frontend`; testes unit/route/contract determinísticos e DB-gated em PG18 descartável quando aplicável; CI verde (decisão anterior:17-24).
  - **Out / NV**: Macro document/collab parity, novos serviços obrigatórios, rewires amplos fora do slice de arquivos; NV: load/OOM/restore drills em 2 vCPU/4 GB, AV scanning, uploads resumáveis/chunked/CDN-style, S3/presigned, indexação de conteúdo, previews/transcoding, storage multi-node, matrix completa de browser e2e; non-goals adicionais: OIDC/JWKS real, deploy/merge/release (decisão anterior:31-35; `intake.md:19-21`).
- **Success metrics**: Usuário autenticado e escoped a tenant/workspace consegue fazer upload e download de bytes reais com paths contidos, checksum/tamanho persistidos do conteúdo efetivo, quotas coerentes e contrato FE alinhado no happy path; testes determinísticos/DB-gated cobrem autorização e limites (`problem.md:18-19`; decisão anterior:17-24,40).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quota deve ser contabilidade agregada transacional ou cap por arquivo/config neste ciclo (concorrência NV)?]; [NEEDS CLARIFICATION: como alinhar o DTO divergente esperado pelo FE sem quebrar clients existentes?]; [NEEDS CLARIFICATION: adicionar `workspace_id` e repositório/listing ao modelo `files` de forma aditiva?]; [NEEDS CLARIFICATION: `flora-files` duplicado e não wired deve ser integrado ou removido?]. NV: 2vCPU/4GB load/OOM/restore, AV, e2e (`decisão anterior:32-35,40`).
