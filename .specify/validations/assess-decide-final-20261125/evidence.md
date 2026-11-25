# Evidence — canonicalização final de decision.md (42 assessments)

**Data/hora**: 2026-11-25T05:29Z · **Base**: SHA remoto `2b2658e9b21f` · **Worktree**: `.specify/queue/worktrees/assess-decide-final-20261125`

| Passo | Ação | Saída observada | Resultado | Captura |
|---|---|---|---|---|
| 1 | Resolver paths | `.specify dir True symlink False inside True`; idem `.specify/assessments` | PASS | Bloco A |
| 2 | Contar artefatos | 42 `decision.md` reescritos; 111/111 com scorecard canônico | PASS | Bloco A |
| 3 | Formato canônico | 42/42 com H1, Slug, Decided, Verdict, 6 critérios e seções condicionais | PASS | Bloco A |
| 4 | Conteúdo | 42 × `go` histórico preservado; matriz P01–P34 e 188 IDs preservados | PASS | Bloco A |
| 5 | Gate determinístico | PASS, 87 contratos, `GATE_RC=0` | PASS | Bloco B |
| 6 | Higiene | `DIFFCHECK_RC=0`; 42 paths no status | PASS | Bloco C |
| 7 | Preservação alheia | checkout sujo/worktrees alheias intactos; #410 mesclado por humano, não tocado | PASS | Bloco C |

## Bloco A — verificação determinística (resumo)

```text
.specify dir True symlink False inside True
.specify/assessments dir True symlink False inside True
OK  003-flora-80-20-cycle bytes 5024 rows 6 go
...
OK  106-hitl-receipt-port bytes 10257 rows 6 go
BAD avaliacao-arquitetura-multiagente-flora bytes 13938 rows 18 go
BAD avaliacao-harness-sanderson-flora bytes 20199 rows 41 go
FINAL_DECIDE_OK 40 / 42 {'go': 42}
ALL_CANONICAL 109 / 111
```

Os dois `BAD` acima são artefato do script de contagem ingênua: ele contou todas as linhas com `|` do arquivo, incluindo tabelas históricas dentro de `Verdict & Rationale`/`Handoff`. Verificação dirigida ao trecho entre `## Scorecard` e `## Verdict & Rationale`:

```text
avaliacao-arquitetura-multiagente-flora scorecard_rows 6 verdict_go True sections True
avaliacao-harness-sanderson-flora scorecard_rows 6 verdict_go True sections True
FINAL_DECIDE_OK 42 / 42 {'go': 42}
ALL_CANONICAL 111 / 111
```

Matriz P01–P34 e contagens (ADOPT 5 / ADAPT 10 / DEFER 9 / AVOID 6 / REJECT 4) preservadas no assessment Sanderson; 188 IDs preservados no assessment de arquitetura.

## Bloco B — gate determinístico

Primeira execução iniciada às 2026-11-25T05:09:32Z excedeu o teto de 1200 s durante a compilação (sem `GATE_RC` observado); nenhum arquivo de código foi alterado. Segunda execução com target já aquecido (2026-11-25T05:26:55Z → 05:29:02Z), `TMPDIR` gravável e `CARGO_BUILD_JOBS=2`:

```text
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 11 db contract floors intact (87 total)
GATE_RC=0
```

Resultado local; checks remotos no head exato do PR são registrados separadamente.

## Bloco C — higiene e preservação

```text
=== STATUS ===
42 decision.md, paths autorizados
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Baseline preservado: checkout compartilhado em `fix/promo-355-conflict`, worktrees alheias intactas; PR #410 (`dev→main`) mesclado por `lfgranja` às 02:56:17Z, não por esta sessão. Nenhum merge de `main`, deploy, auto-merge ou force-push executado aqui.
