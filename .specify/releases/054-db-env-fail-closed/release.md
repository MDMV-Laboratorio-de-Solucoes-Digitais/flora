# Release: 054 — DB env fail-closed

**Ciclo:** 054 · **Data:** 2026-11-18T00:32Z · **Baseline:** `github/dev@dd982cadc`
**Tipo:** CI gate; **nenhuma tag, release ou deploy executado.**

## Entregue

- `.github/workflows/rust.yml`: `run_leg` falha imediatamente se
  `FLORA_DATABASE__POSTGRES_URL` ou `DATABASE_URL` estiver ausente.
- `scripts/ci/check_gate_integrity.py`: exige as duas chaves de env no bloco
  `db-tests`; remoção futura deixa o guardião RED.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.8` (correção de confiabilidade de CI, sem API
externa; considera propostas 0.1.1–0.1.7 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — expansões `:?` em `run_leg` eliminam o passe vacuo por env ausente.
- `FR-002` — guardião detecta remoção das linhas de env.
- `FR-003` — 8 pisos, 4 required checks e nomes de jobs inalterados.
- `FR-004` — nenhum teste `#[ignore]` modificado.
- `SC-001`–`SC-004` — verificados (sem env FAIL, com DB 8 legs verdes, mutação
  RED, gates locais).
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato V1 veio do ranking.
  Relacionado ao bug `ci-test-step-runs-zero-tests` (mesma classe de gate vacuo).

## Evidência

`.specify/validations/054-db-env-fail-closed/validate.md` ·
`specs/054-db-env-fail-closed/converge.md` ·
`specs/054-db-env-fail-closed/analyze.md`.

## Rollback

`git revert` dos commits `794474c0f` (`fix(ci): falha legs DB quando env de Postgres ausente`) e `83cf8cc9c` (`fix(ci): guardiao exige env do job db-tests`) restaura o comportamento
anterior. Nenhum efeito externo; nenhuma tag/deploy envolvido.

## Não feito / NV

Falso positivo local (`cargo test --ignored` sem env ainda reporta 50 passes),
piso de auth 12→13, ADR-0003, C5/C6: fora de escopo. Resultado no runner e
CI/PR ao vivo: **[NV]**. Sem merge/auto-merge/force-push nesta fase.
