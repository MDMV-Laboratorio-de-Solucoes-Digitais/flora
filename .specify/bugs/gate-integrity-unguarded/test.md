# Bug Verification: required checks do merge autônomo sem guarda de integridade

- **Slug**: gate-integrity-unguarded
- **Tested**: 2026-11-16
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: partial

## Summary

O guard determinístico e o holdout foram executados localmente com sucesso. O veredito é `partial` até que o PR #254 rode na GitHub Actions: o holdout reproduz o shell do workflow, não o runner, e a comparação `--remote` das proteções é uma verificação local opcional.

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Guard de integridade | `python3 scripts/ci/check_gate_integrity.py` | pass | `gate-integrity: PASS`, 7 ok, exit 0. |
| YAML dos workflows | `python3 -c 'import yaml; ...'` | pass | `yaml-ok`. |
| Holdout (Dark Factory) | `python3 $DF/scripts/holdout/run_holdout.py ...` | pass | 1 cenário. |
| Checker independente | `python3 $DF/scripts/holdout/checker.py ...` | pass | `holdout-check: PASS scenarios=1`, exit 0. |
| Prova de falha do gate | `observed.json` | pass | `exit_code=1`; stdout `auth: passed=0 floor=12`, `0 passed ... 89 filtered out`. |
| Proteção ao vivo (modo opcional) | `python3 scripts/ci/check_gate_integrity.py --remote` | pass | Contextos de `dev`/`main` iguais aos 4 esperados (executado localmente). |
| CI remoto no PR #254 | `gh pr checks 254` | not-run | Preenchido após o merge. |
| Runner semantics / Forgejo | — | NV | Holdout não exercita GitHub Actions nem Forgejo. |

## Output Excerpts

```
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all six db contract floors intact (43 total)
holdout-run: 1 scenario(s) -> .specify/holdout/observed.json
holdout-check: PASS scenarios=1
```

## Residual Risks

- Se `rust-build` for renomeado, o guard deixa de rodar (o próprio nome é vigiado só pelo `--remote`, não no runner). Documentado.
- Parser stdlib por indentação não é parser YAML completo; sintaxe é verificada separadamente.
- 16 actions ainda com tags mutáveis; risco de supply chain permanece (fora deste apetite).

## Recommendation

Mesclar o PR #254 quando os quatro required checks estiverem verdes com o novo step presente no log de `rust-build`. Atualizar este `test.md` com a evidência remota após o merge.
