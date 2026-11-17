# Release: 041 — Rust dependency debloat

**Ciclo:** 041 · **Data:** 2026-11-17 · **Baseline:** `dev@ccd6d656` ·
**Tipo:** deleção pura de dependências, sem deploy/tag.

## Entregue

- 4 deps dead + 2 members órfãos removidos: 34 pacotes / 369 linhas do Cargo.lock
  (4898 → 4529), zero adições.
- 6 exemptions do cargo-vet removidas (dotenvy preservada).
- Gates idênticos ao baseline (fmt/clippy/deny/test/vet); non-regression provado.
- Decisão 80/20, assessment `go (scoped)`, spec/tasks, validação, research doc.

## Evidência

`.specify/validations/041-rust-dead-crates/validate.md` ·
`docs/research/2026-11-17-041-rust-debloat.md` ·
`/tmp/opencode/dark-factory-80-20/c041-04-author-blind-audit.md`.

## Não feito (autonomia preservada)

Nenhuma mudança de comportamento, migration, RbacService, RLS/role, ADR/constituição,
required checks, deploy/tag/release/segredos. RLS permanece `[NV]` — exige role
dedicada + policies + testes DB-gated, sob autorização do SE (c041-03).

## Follow-ups

1. RbacService real (GET /roles primeiro; revoke/assign exigem decisão de semântica).
2. Role least-privilege + policies RLS + FORCE — autorização do SE.
3. cargo-vet prune (aviso informativo, sem risco).
