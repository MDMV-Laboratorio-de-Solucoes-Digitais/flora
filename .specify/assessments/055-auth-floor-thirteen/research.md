# Research: 055 — Auth floor thirteen

**Data:** 2026-11-18T01:34Z · **Evidência local determinística**; CI remota **[NV]**.

## Primária (executada)

1. `cargo test -q -p flora-tests 'contract::auth::db_' -- --list` → 13 entradas
   `: test` (01:31Z, worktree 055).
2. `.github/workflows/rust.yml:107` — `run_leg auth contract::auth::db_ 12`.
3. `scripts/ci/check_gate_integrity.py:43` — `"auth": 12`; `:112-115` — a
   regex casa `run_leg auth … 12` e falha se o número mudar sem o guardião.
4. `scripts/ci/check_gate_integrity.py:120` — `"all eight db contract floors
   intact (49 total)"`; com auth 13 a soma passa a 50.
5. Ciclo 053 (`validate.md`): 8 legs com Postgres 18.6 real → auth 13 passed.
   Nenhum dos 13 é `#[ignore = …]` condicional além do requisito de Postgres;
   todos usam o mesmo `db_state()` fail-closed por env (ciclo 054).

## Decisão

Elevar o piso para 13 em ambos os locais e atualizar o texto para 50 total.
Nenhuma outra linha de `rust.yml` ou do guardião muda. ADR-0003 mantém o texto
antigo (43) — dívida de governança registrada, não editada sem aprovação.

## Alternativas descartadas

- Piso 14: inventaria folga artificial; exige teste inexistente.
- Mudar filtro: já cobre os 13; não há ganho.
- Editar testes: fora de escopo e desnecessário.
- Atualizar ADR no mesmo PR: documento de governança, mudança exige aprovação
  de maintainer (Constituição §Governance) — separar.

## Cobertura idiomática

PT-BR (artefatos do alvo) + EN (código/workflow). Runner ao vivo: **[NV]**.
