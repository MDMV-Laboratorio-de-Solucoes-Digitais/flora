# Decision: Remover stub morto AuthService

- **Slug**: 071-dead-auth-service
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `flora-core/src/services/auth_service.rs` tem 5 métodos que retornam `ServiceUnavailable` e `is_available() == false`, sem uso em handlers (`intake.md:7`, `research.md:7-8`). |
| Evidence strength | strong | 25 ocorrências em 11 arquivos mapeadas e `grep -rn 'auth_service' flora-api/src/routes/` → 0; auth real em `routes/auth.rs` + extractor de sessão (`intake.md:12-19`, `research.md:13`). |
| Value vs. inaction | adequate | Remove conceito enganoso e reduz carga de leitura (Princípio I / debloat), sem ganho funcional direto (`define.md:5`). |
| Feasibility / appetite | strong | Remoção mecânica verificável por compilação e `grep` zero ocorrências (`shape.md:5-12`, `define.md:17`). |
| Strategic fit | strong | Alinhada ao ciclo 061 (circuit-breaker morto) e ao Princípio I (debloat) registrados no intake (`intake.md:7`). |
| Risk posture | strong | Zero mudança em handlers/OIDC/migrations/gates/pisos; `Error::ServiceUnavailable`, `AuthProvider` e `UserInfo` preservados (`research.md:14-16`, `shape.md:10`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown`, não há contradição nos artefatos e a remoção é verificável por compilação. A variante `ServiceUnavailable` continua em uso em `oidc_transactions.rs:85,119` e a trait `AuthProvider`/`UserInfo` é ponto de extensão documentado — ambas devem permanecer (`research.md:14-16`, `decision anterior:12`). CI futura, Forgejo, produção e aprovações seguem `[NV]`.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: um serviço que sempre falha ocupa campo público em `AppState`, é construído em 7 sites de teste e sugere capacidade de autenticação inexistente.
- **Chosen approach**: remoção mecânica completa das referências ao stub nos 11 arquivos declarados, preservando `Error::ServiceUnavailable` e a trait `AuthProvider`/`UserInfo` (`shape.md:5-12`).
- **In scope / out of scope**: dentro = 11 arquivos Rust, artefatos do ciclo e `STATE.md`; fora = handlers de auth, fluxo OIDC, migrations, workflows, gates, pisos, ADRs, constituição e remoção de `AuthProvider`/`UserInfo`/`ServiceUnavailable` (`intake.md:26`, `define.md:21-23`).
- **Success metrics**: SC-001 `grep -rn 'AuthService\|auth_service' --include='*.rs'` → 0 ocorrências; SC-002 gates do alvo verdes (autoritativo local/`--remote`, fmt/clippy/deny/build/test, frontend) sem piso/job alterado; SC-003 diff restrito aos 11 arquivos + artefatos, zero mudança de comportamento (`define.md:15-19`).
- **Carried-forward open questions**: nenhuma bloqueante; CI futura/Forgejo/produção/aprovações `[NV]`.
