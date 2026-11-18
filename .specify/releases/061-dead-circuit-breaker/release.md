# Release: 061 — Dead circuit breaker

**Ciclo:** 061 · **Data:** 2026-11-18T04:11Z · **Baseline:** `github/dev@7ed232c46`
**Tipo:** debloat de código morto; **nenhuma tag, release ou deploy executado.**

## Entregue

- Exclusão de `flora-core/src/utils/circuit_breaker.rs` (68 linhas).
- Remoção de 2 campos/defaults em `config/messaging.rs` e da variante
  `Error::CircuitBreakerOpen` + braço de match em `error.rs`.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.15` (remoção de código morto, sem API
externa; considera propostas 0.1.1–0.1.14 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — placeholder nunca compilado removido.
- `FR-002` — campos de config órfãos removidos.
- `FR-003` — variante de erro órfã removida.
- `FR-004` — runtime/gate/ADRs inalterados.
- `SC-001`–`SC-004` — verificados (grep 0, gates, 3 arquivos, gate autoritativo).
- `T001`–`T004` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/061-dead-circuit-breaker/validate.md` ·
`specs/061-dead-circuit-breaker/converge.md` ·
`specs/061-dead-circuit-breaker/analyze.md`.

## Rollback

`git revert` dos commits `refactor(core): remove placeholder de circuit-breaker`,
`refactor(config): remove campos mortos de circuit breaker` e
`refactor(core): remove variante de erro orfa`. Nenhum efeito externo.

## Não feito / NV

ADR-0003/0006, N3, C5/C6: escalados. Estado remoto/CI: **[NV]**.
Sem merge/auto-merge/force-push nesta fase.
