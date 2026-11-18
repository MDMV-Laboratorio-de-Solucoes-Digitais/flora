# Release: 055 — Auth floor thirteen

**Ciclo:** 055 · **Data:** 2026-11-18T01:49Z · **Baseline:** `github/dev@d5d7ef5d`
**Tipo:** CI gate; **nenhuma tag, release ou deploy executado.**

## Entregue

- `.github/workflows/rust.yml:107`: piso do leg `auth` 12 → 13.
- `scripts/ci/check_gate_integrity.py`: `FLOORS["auth"]` 13 e texto "50 total".
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.9` (fortalecimento de CI, sem API externa;
considera propostas 0.1.1–0.1.8 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001`/`FR-002` — piso 13 nos dois locais, em lockstep verificado por mutação.
- `FR-003` — guardião imprime 50 total.
- `FR-004` — nenhum outro piso, check, teste ou proteção alterado.
- `SC-001`–`SC-004` — verificados (`--list`, guardião, DB real, diff).
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/055-auth-floor-thirteen/validate.md` ·
`specs/055-auth-floor-thirteen/converge.md` ·
`specs/055-auth-floor-thirteen/analyze.md`.

## Rollback

`git revert` dos commits `fix(ci): exige 13 testes no leg auth` e
`fix(ci): espelha piso 13 no guardiao e total 50` restaura o piso 12 e o texto
49 total. Nenhum efeito externo; nenhuma tag/deploy envolvido.

## Não feito / NV

ADR-0003 (texto 43), índice de research/PROGRESSO, piso fail-open local, N3,
C5/C6: fora de escopo. Resultado no runner e CI/PR ao vivo: **[NV]**.
Sem merge/auto-merge/force-push nesta fase.
