# Intake: 061 — Dead circuit breaker

**Data:** 2026-11-18T04:06Z · **Âncora:** `github/dev@7ed232c46` · branch `feature/061-dead-circuit-breaker`

## Sinal

`flora-core/src/utils/circuit_breaker.rs` (68 linhas) descreve um placeholder
que "forwards calls without observing failures" e **não é declarado** em
`utils/mod.rs` (`pub mod jwt; pub mod pkce;`). É código morto: nunca compilado,
nunca testado. Seus dois campos de config e a variante de erro também estão
órfãos.

## Evidência (verificada)

- `grep -rn circuit_breaker` → apenas o próprio arquivo, `config/messaging.rs:19,21,33,34`
  e `error.rs:181,365` (match arm).
- `cat flora-core/src/utils/mod.rs` → `pub mod jwt; pub mod pkce;` — sem circuit_breaker.
- `config/mod.rs:44-46` usa Figment com `Env::prefixed("FLORA_").split("__")`;
  sem `deny_unknown_fields`, logo remover campos de config não quebra env extras.
- Princípio I da constituição: debloat e leveza — remoção de código morto é
  manutenção alinhada à missão.

## 80/20

`(I×C)/(E×R)` = (2×5)/(3×2) = **1.67**, maior candidato autônomo. ADR-0003/0006,
N3 e C5/C6 escalados; dívidas de fase não podem ser "fechadas" forjando artefatos.

## Fronteiras

Escopo: 1 arquivo excluído e 2 arquivos editados em `flora-core/src/`, mais
artefatos do ciclo. Fora: ADRs, constituição, CI, migrações, C5/C6, deploy.
Factory fonte `0cb34c4d…`, nada transplantado.
