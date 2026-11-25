# Decision: 006 — Secure Vertical Slice (Auth/Tenancy + Core Flow)

- **Slug**: 006-secure-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | specs/006-secure-vertical-slice/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Confiança em `X-User-ID`/`X-Organization-ID` e ausência de membership consistente eram P0 registrado em `intake.md:17` e `research.md:14-16`. |
| Evidence strength | adequate | Auditorias de rotas/contrato e gates locais de 2026-11-15 (fmt/check, 8 + 54 testes) em `research.md:29-33` e `spec.md`; runtime integrado/CI remota NV. |
| Value vs. inaction | strong | Sem verificação de identidade/tenant, qualquer tenant pode agir como outro; a inação também aprofunda o desalinhamento frontend↔backend (`problem.md:41-43`). |
| Feasibility / appetite | adequate | Opção A (`small`) foi executada com extrator já existente; a quebra de clientes header-based exige migração compatível registrada (`concept.md:12-14`). |
| Strategic fit | strong | Base necessária ao Seed seguro e verificável em VPS barata, alvo do handoff de 005 (`intake.md:7,13`; decisão original). |
| Risk posture | adequate | Risco médio explicitamente aceito: clientes header-based quebram; fallback em dev não é default e E2E/WS/RBAC fino ficam fora (`research.md:27,34-38`). |

## Verdict & Rationale

Veredito histórico **`go (executed, partial slice)` preservado como `go`**: nada nos artefatos contradiz a decisão de 2026-11-15. O scorecard sustenta o `go` pela validade do P0 de segurança/tenancy e pelas evidências determinísticas locais; a execução parcial é justamente o recorte escolhido (Opção A), com token org-scoped, alinhamento de contrato/UI, fanout/RBAC e runtime integrado marcados como NV/deferidos — não como pré-condições do `go`.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as incógnitas abaixo são transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Identidade e tenant derivados de cabeçalhos controlados pelo cliente, somados ao JWT pós-callback sem organização, tornavam inutilizáveis as rotas tenant-scoped e o fluxo núcleo (`problem.md:9-11`).
- **Chosen approach**: Opção A — eliminar a confiança em cabeçalhos crus (`concept.md:9-14,30-32`), recomendada e executada; Opção B (fatia e2e) e Opção C (reescrita ampla) não foram escolhidas.
- **In scope / out of scope**:
  - **In (executado em 2026-11-15)**: `AuthExtractor::require_org()` e JWT Bearer validado nas rotas de `workspace`, `tasks`, `channels`/messages, `org`, `files`, `search`, `notifications` e WS (Bearer-only); membership e tenancy do recurso, incluindo `workspace_id ∈ org` na criação de canal e checks em update/delete/get onde aplicável (`research.md:29-33`; `spec.md:Delivered`).
  - **Out/deferred**: token/sessão escopado após seleção de org (callback ainda emite org `nil`); alinhamento de contrato/frontend e E2E UI (NV); fanout/realtime completo, RBAC fino além do slice e refactors amplos; migração compatível de clientes header-based (`test-flora.sh`, frontend atual), cuja quebra foi risco aceito e explícito.
- **Success metrics**: rotas críticas sem headers não verificados, membership/tenancy checados antes da operação, sessões pós-callback utilizáveis em rotas tenant-scoped e caminho E2E de UI verificável (baseline: 0/NV); gates locais verdes — `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core`, `cargo test -p flora-core --lib` (8 passed), `cargo test -p flora-tests --lib` (54 passed); runtime integrado (Postgres/Valkey/Meilisearch/OIDC) e CI remota **NV** (`problem.md:33-39`; decisão original:21-26).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: estratégia de migração aceitável para clientes header-based existentes — fallback explícito em dev ou quebra imediata?]; [NEEDS CLARIFICATION: qual fatia núcleo priorizar primeiro, `workspace → tarefa` ou `workspace → canal → mensagem`?]; [NEEDS CLARIFICATION: que ambiente/serviços validam o fluxo com runtime integrado, ou permanece NV?] (`problem.md:45-49`).
