# Research: 053 — DB gate uncovered legs

**Data:** 2026-11-18T00:03Z · **Evidência local determinística**; CI remota **[NV]**.

## Primária (executada)

1. `cargo test -q -p flora-tests "<filtro>" -- --list` na worktree:
   - `contract::tasks::db_` → 0 testes;
   - `contract::tasks::db::db_` → 4 testes;
   - `contract::workspace::db_` → 0 testes;
   - `contract::workspace::db::db_` → 2 testes.
   Portanto um `run_leg` com o filtro curto cairia no problema do bug
   `ci-test-step-runs-zero-tests` (libtest retorna 0, mas o piso falha rápido).
2. `flora-tests/src/contract/tasks.rs:134-135,153-154,195-196,224-225` — quatro
   `#[ignore = "requires Postgres…"]` dentro de `mod db` (`:5`).
   `workspace.rs:132-133,156-157` — dois, dentro de `mod db` (`:8`).
3. `.github/workflows/rust.yml:89-103` — `run_leg` soma `N passed` e exige piso;
   `:105-110` — seis legs; `:65-66` — env do Postgres 18 no job.
4. `scripts/ci/check_gate_integrity.py:42-49` — `FLOORS` com seis entradas;
   `:110-114` — regex `run_leg\s+{label}\s+contract::\w+::db_\s+{floor}\b` e
   mensagem "all six db contract floors intact (43 total)".
5. `docs/adr/0003:19` e bug docs citam 43 — ficarão desatualizados (fora de escopo).

## Decisão

Adicionar dois legs usando os filtros completos (`db::db_`) com pisos 4 e 2,
e espelhar no `FLOORS` do guardião. A regex deve aceitar `db::db_` sem afrouxar
as seis entradas existentes. Nenhum nome de check muda → proteção de branch
permanece válida; o `--remote` do guardião já confirma os 4 contextos.

## Alternativas descartadas

- Filtro curto: casa zero → furaria o piso e tornaria a CI vermelha.
- Novo job/check: exigiria reconfigurar required checks (risco de deadlock).
- Elevar piso de auth 12→13: reforço válido, mas independente; fora deste escopo.
- Corrigir fail-open do env: candidato V1, esforço/risco maiores; fora daqui.

## Cobertura idiomática

PT-BR (artefatos do alvo) + EN (código, workflow, nomes de checks). **[NV]** para
execução real dos dois novos legs no runner do GitHub.
