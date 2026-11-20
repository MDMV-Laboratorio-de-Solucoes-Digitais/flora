# Release: 071 — remover stub morto AuthService

**Ciclo:** 071 · **Data:** 2026-11-19T04:05Z · **Baseline:** `github/dev@cdf50eaff`
**Tipo:** remoção de código morto. **Nenhuma tag, release, deploy ou produção.**

## Entregue

- Exclusão de `flora-core/src/services/auth_service.rs` e limpeza de exports, `AppState` e 7 construtores de teste — 10 arquivos Rust no diff.
- `Error::ServiceUnavailable`, `AuthProvider` e `UserInfo` preservados como pontos de extensão/uso legítimo.
- Artefatos completos: assess (5), spec (8), validate (R4), release (R7), `STATE.md`; correções append-only de contagem e de formatação.

## SemVer

**Proposta:** patch `0.2.3` → `0.2.4` (remoção de código morto, sem mudança de API externa usada; `auth_service` era campo público, mas sem consumidores). Propostas anteriores permanecem não aplicadas. **Nenhuma tag criada.**

## Changelog (IDs)

- `FR-001` arquivo/exports removidos; `FR-002` `AppState` limpo; `FR-003` 7 construtores limpos; `FR-004` trait/variante preservados; `FR-005` rastreabilidade.
- `SC-001` grep zero; `SC-002` gates verdes, pisos intactos; `SC-003` diff restrito aos 10 arquivos + artefatos.
- `T001`–`T007` executados; `T007` (PR/merge/higiene) conduzido no BLOCO 7.
- Bug: nenhum em `.specify/bugs/`; origem é decisão autônoma 80/20 em `STATE.md`.

## Evidência

`.specify/validations/071-dead-auth-service/validate.md` ·
`specs/071-dead-auth-service/converge.md` ·
`specs/071-dead-auth-service/analyze.md` ·
verificador isolado `PASS-WITH-NITS` (nit corrigido).

## Rollback

Reverter os commits do PR em `dev/` com um novo commit de reversão (sem force-push e sem reescrita de histórico). Nenhum dado, schema, required check, deploy ou estado de produção a restaurar.
