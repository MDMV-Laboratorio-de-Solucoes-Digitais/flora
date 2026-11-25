# Decision: 059 — DB tests fail locally

- **Slug**: 059-db-tests-fail-local
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md (problem equivalente) | shape.md (concept equivalente) | clarify.md | plan.md | decision.md (anterior) | specs/059-db-tests-fail-local/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | 9 helpers `db_state()` devolvem `Ok(None)` sem env e 50 testes contam `passed` sem nenhuma asserção — falsa confiança local (intake.md:9-13). |
| Evidence strength | strong | Repro determinístico: 4 passed em 0,00s / exit 0 sem DB; `grep` de 9 ocorrências em 8 arquivos e 50 chamadores; verificador isolado 5/5 PASS (converge.md). |
| Value vs. inaction | strong | Sem a mudança, `cargo test --ignored` local continua reportando sucesso vazio; E=1,875 era o maior candidato autônomo (intake.md:25). |
| Feasibility / appetite | strong | 9 blocos `else` em 8 arquivos de `flora-tests/src/contract/`; `anyhow` já é dependência do crate (`Cargo.toml:31`); 50 chamadores intactos (shape.md). |
| Strategic fit | strong | Casa com o fail-closed já adotado no ciclo 054 sem alterar CI, gate, ADRs ou required checks (define.md:10). |
| Risk posture | strong | Reversível por `git revert` por arquivo, sem credenciais e sem mudança de runtime; único resíduo é CI/runner ao vivo **[NV]** (decision.md anterior). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado**: o scorecard é forte em validade, evidência e valor, e o escopo foi desenhado exatamente para não alterar comportamento de runtime nem infraestrutura. O único ponto em aberto — CI/runner ao vivo **[NV]** (ADR-0003) — é observacional, não bloqueia a decisão e deve ser conferido no PR antes do merge. Âncora original `github/dev@2730fd2dd`, branch `feature/059-db-tests-fail-local`; factory fonte `1083d321…`, nada transplantado. A correção pós-implementação (03:30Z) fixou o texto exato da mensagem sem o verbo "is" em 4 artefatos (clarify.md/plan.md).

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go`.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Testes de contrato DB reportam `passed` sem asserção quando faltam `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`.
- **Chosen approach**: Substituir nos 9 `else` dos helpers `db_state()` o `Ok(None)`/`eprintln!("skipping…")` por `anyhow::bail!("FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests")`, mantendo a assinatura `Result<Option<…>>` e os 50 chamadores (`shape.md`).
- **In scope / out of scope**: 8 arquivos em `flora-tests/src/contract/`, artefatos do ciclo e notas datadas; fora CI, gate, ADRs, constituição, C5/C6 e deploy.
- **Success metrics**: sem env → exit ≠ 0; `--list` descobre 50 testes; com DB isolado real os 50 passam; fmt/clippy/deny/build/test + frontend verdes (define.md SC-001–SC-004).
- **Carried-forward open questions**: CI/runner ao vivo **[NV]** — observar no PR antes do merge; [NEEDS CLARIFICATION: a redação final da mensagem de erro está congelada pela correção 03:30Z — usar o texto sem "is"].
