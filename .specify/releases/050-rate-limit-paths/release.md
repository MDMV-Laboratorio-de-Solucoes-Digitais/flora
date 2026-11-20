# Release: 050 — Rate limit paths

**Ciclo:** 050 · **Data:** 2026-11-17T16:20Z · **Baseline:** `github/dev@b3bfda505`
**Tipo:** middleware Rust; **nenhuma tag, release ou deploy executado.**

## Entregue

- `RateLimitClass` + `classify_endpoint` em `flora-api/src/middleware/rate_limit.rs`.
- Middleware usando o classificador, com limites/janelas preservados.
- 4 testes unitários puros; notas datadas em `OBJETIVO.md`/`ROADMAP.md`.

## SemVer

**Proposta:** patch `0.1.0` → `0.1.4` (correção de comportamento interno, sem API
externa; considera propostas 0.1.1–0.1.3 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001`–`FR-004` — classificação de login/refresh/invite/general.
- `FR-005` — 4 testes unitários sem servidor HTTP.
- `T001`–`T006` — concluídas; evidência em `validate.md`.
- Bug: nenhum em aberto; C25 veio do ranking 80/20, registrado em `intake.md`.

## Evidência

`.specify/validations/050-rate-limit-paths/validate.md` ·
`specs/050-rate-limit-paths/converge.md` ·
`specs/050-rate-limit-paths/analyze.md`.

## Rollback

`git revert` do commit do ciclo restaura a cadeia `if/else` anterior e remove os
testes e notas. Nenhum efeito externo a desfazer.

## Não feito / NV

Rotas, limites, state distribuído, C4/C5/C6: fora de escopo. CI/PR/aprovações ao
vivo: **[NV]**. Sem merge/auto-merge/force-push.
