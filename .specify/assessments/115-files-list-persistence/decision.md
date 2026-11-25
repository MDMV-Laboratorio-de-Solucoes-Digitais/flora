# Decision: Listagem persistida de arquivos por workspace

- **Slug**: 115-files-list-persistence
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | decision.md (anterior) — `problem.md` e `concept.md` ausentes no baseline

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `find_by_workspace_id` e `list_files` já existem, mas `flora-api/src/routes/files.rs:82-89` não expõe `GET /` e `/files` guarda arquivos só em `FileStore`, sumindo ao recarregar ou trocar de workspace (intake.md:5-7; research.md:7-13). |
| Evidence strength | strong | Consulta SQL com filtro `organization_id`/`workspace_id`/`is_deleted=false` e `created_at DESC`, trait em `repository.rs:257-262`, ausência de `GET /` e piso atual `files: 6` (total `77`) verificados em research.md:7-14. |
| Value vs. inaction | strong | Maior lacuna de usabilidade do núcleo Seed (400/625): sem persistência, o fluxo `workspace → arquivos` exige reupload e a listagem some em cada recarga (decision.md anterior:9). |
| Feasibility / appetite | strong | Aproveita `FileService::list_files` e `PgFileRepository::find_by_workspace_id` já existentes, sem schema ou migration e sem required checks novos (decision.md anterior:9; research.md:7-10). |
| Strategic fit | strong | Fecha lacuna #1 do núcleo Seed com contrato DB-backed e lockstep de piso, preservando tenancy e o guarda do gate (define.md:7-9; decision.md anterior:13-16). |
| Risk posture | adequate | Novo endpoint público exige `ensure_member`/`ensure_workspace_in_org`, clamp de `limit` e ausência de `.unwrap()`/`.expect()`; staging com arquivos de múltiplos usuários e produção permanecem **[NV]** (research.md:18; define.md:13). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: a lacuna é determinística, a solução reaproveita código existente e o alinhamento com o núcleo Seed e a disciplina de lockstep do gate é explícito. Preservam-se o cálculo e os fatos anteriores — score **400/625** (`impacto=5 × confiança=5 × (6−esforço=2)=4 × (6−risco=2)=4`), maior lacuna de usabilidade do núcleo Seed, uso de `FileService::list_files` e `PgFileRepository::find_by_workspace_id` já existentes, sem alterar schema nem required checks (decision.md anterior:9) — e todas as restrições: Briefing de Compreensão obrigatório (`tenant`, `.specify/templates/briefing-compreensao.md`, ciclo 109); lockstep obrigatório de `FLOORS["files"] = 7` em `check_gate_integrity.py` e `.github/workflows/rust.yml` junto ao novo contrato DB; passo `11b taskstoissues` após `analyze` e antes de `implement`; sem `.unwrap()`/`.expect()` em produção; SemVer `MINOR` (`0.7.16 → 0.8.0`) no `release.md` pelo novo endpoint `GET /api/v1/files` (decision.md anterior:11-16).

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; as perguntas remanescentes são de validação operacional, não bloqueantes.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Expor `GET /api/v1/files?workspace_id=<uuid>[&limit=<n>]` e carregar a lista persistida em `/files` (mount, troca de workspace e atualização manual).
- **Chosen approach**: Reaproveitar `FileService::list_files`/`PgFileRepository::find_by_workspace_id`; proteger com `AuthExtractor` + `ensure_member` + `ensure_workspace_in_org`; contrato DB-backed em `flora-tests/src/contract/files.rs` com lockstep `files: 6 → 7` (total `78`); atualizar `FileStore` e `+page.svelte` com guarda de concorrência e estender `seed-flow.e2e.ts`.
- **In scope / out of scope**: Inclui Briefing de Compreensão, nota de governança, recontagens append-only, roteiro/evidência e release MINOR; exclui remoção na UI, paginação por cursor, schema/migrations, required checks e regras de deploy.
- **Success metrics**: listagem persistida após recarga e troca de workspace; 422 para `workspace_id` ausente e BOLA cross-org; exclusão de soft-deleted e ausência de `storage_path`; piso `files: 7`/total `78` em lockstep; E2E em Chromium e WCAG sem achados em `/files`.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: staging com arquivos pré-existentes de múltiplos usuários e deploy em produção permanecem **[NV]** — quando serão observados?]; [NEEDS CLARIFICATION: a ordem `created_at DESC` e a dedup do `FileStore` devem ser normalizadas junto ao novo contrato?].
