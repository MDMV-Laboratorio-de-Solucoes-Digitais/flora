# Decision: Gate DB fail-closed por variáveis de ambiente (054)

- **Slug**: 054-db-env-fail-closed
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md (equivalente local; problem.md/concept.md ausentes no baseline) | shape.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | 9 pontos `db_state()` devolvem `Ok(None)` e 50 testes retornam `Ok(())`: remover `rust.yml:65-66` deixaria os 8 legs verdes sem asserção (intake.md:9-15; research.md:1-5). |
| Evidence strength | strong | `grep` 9/50, leitura de `rust.yml:64-66,89-104`, ausência de `POSTGRES_URL`/`DATABASE_URL` no guardião e CI do PR #271 com 50 testes reais (intake.md:16; research.md:1-7). |
| Value vs. inaction | strong | Inação permite um gate de merge vácuo e invisível se as linhas de env forem removidas — os required checks ficariam verdes sem significado (define.md:Problema). |
| Feasibility / appetite | strong | Correção em 2 arquivos, sem editar os 50 testes: expansão fatal `${VAR:?…}` no `run_leg` e bloco condicional no guardião (research.md:Decisão; shape.md). |
| Strategic fit | strong | Fail-closed alinhado a ADR-0003 sem novo required check nem mudança de proteção; 8 pisos, 4 checks e nomes de jobs intactos (define.md:FR-003). |
| Risk posture | strong | `:?` é expansão POSIX/bash padrão com `set -euo pipefail` já ativo; mutação em cópia temporária; execução no runner e CI ao vivo **[NV]** (decision.md anterior:NV). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard justifica a decisão: o modo de falha é demonstrado por contagens determinísticas e a correção protege tanto a execução do step quanto a integridade do guardião, com diff mínimo. Mantém-se o escopo do veredito anterior — 2 arquivos existentes (`.github/workflows/rust.yml`, `scripts/ci/check_gate_integrity.py`), artefatos do ciclo e notas datadas append-only; nenhum required check, teste ou proteção de branch é alterado, e nenhum PR/worktree/stash alheio é tocado (decision.md anterior:Delimitadores). As URLs de Postgres efêmero do CI são citadas como contexto, sem credenciais. A execução do step no runner e o estado de CI ao vivo são **[NV]**, a observar no PR antes de qualquer merge (ADR-0003). Não havia `problem.md`/`concept.md` no baseline; `define.md` e `shape.md` servem como equivalente local.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; a checagem no runner é acompanhamento, não bloqueio.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: o job `db-tests` pode ficar verde sem executar asserção alguma se `FLORA_DATABASE__POSTGRES_URL` ou `DATABASE_URL` forem removidas, e o guardião não protege essas linhas.
- **Chosen approach**: duas linhas de expansão fatal `: "${FLORA_DATABASE__POSTGRES_URL:?…}"` e `: "${DATABASE_URL:?…}"` dentro de `run_leg` em `rust.yml`, mais um bloco condicional em `check_gate_integrity.py` que exige as duas chaves de env no job `db-tests` (research.md:Decisão; shape.md:Forma escolhida).
- **In scope / out of scope**: in — `.github/workflows/rust.yml`, `scripts/ci/check_gate_integrity.py`, artefatos do ciclo e notas datadas; out — 9 `db_state()`, 50 chamadores `#[ignore]`, nomes de required checks/jobs, 8 pisos, ADR-0003, constituição, proteção de branch, C5/C6, deploy e PRs/worktrees/stashes alheios.
- **Success metrics**: step sem env → exit != 0 antes do `cargo test`; com DB real → 8 legs verdes / 50 testes; guardião PASS e RED ao remover uma linha de env em cópia temporária; gates locais (fmt/clippy/deny/build/test + frontend) verdes; execução no runner **[NV]** antes do merge (ADR-0003).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: execução do step no runner e estado de CI ao vivo?] **[NV]**; falso positivo local (`Ok(None)`) permanece dívida documentada fora de escopo; nenhum novo required check é criado (4 contextos de proteção intactos).
