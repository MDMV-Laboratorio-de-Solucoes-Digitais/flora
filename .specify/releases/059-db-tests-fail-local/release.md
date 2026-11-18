# Release: 059 — DB tests fail locally

**Ciclo:** 059 · **Data:** 2026-11-18T03:31Z · **Baseline:** `github/dev@2730fd2dd`
**Tipo:** testes; **nenhuma tag, release ou deploy executado.**

## Entregue

- 9 helpers `db_state()` em 8 arquivos `flora-tests/src/contract/*.rs` agora
  falham com `anyhow::bail!` quando a env de Postgres está ausente.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.13` (correção de confiabilidade de testes,
sem API externa; considera propostas 0.1.1–0.1.12 ainda não aplicadas).
Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — 9 helpers retornam `Err` sem env.
- `FR-002` — 50 chamadores intactos.
- `FR-003` — com DB real, 50 testes passam.
- `FR-004` — CI/gate/ADRs intactos.
- `SC-001`–`SC-004` — verificados (sem env exit 101, `--list` 50, DB 50, gates).
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/059-db-tests-fail-local/validate.md` ·
`specs/059-db-tests-fail-local/converge.md` ·
`specs/059-db-tests-fail-local/analyze.md`.

## Rollback

`git revert` dos 16 commits `fix(tests): falha contrato DB <arquivo> sem env` e
`style(tests): formata bail dos contratos <arquivo>`. Nenhum efeito externo;
nenhuma tag/deploy.

## Não feito / NV

ADR-0003/0006, N3, C5/C6: fora de escopo ou escalados. Resultado no runner:
**[NV]** (CI observada ao vivo no Bloco 7). Sem merge/auto-merge/force-push.
