# Bug Verification: required checks do merge autônomo sem guarda de integridade

- **Slug**: gate-integrity-unguarded
- **Tested**: 2026-11-16
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

O guard determinístico e o holdout foram executados localmente com sucesso e o guard **rodou dentro do required check `rust-build` no PR #254**, com as sete verificações passando. Permanece NV apenas a semântica do runner Forgejo e a reprodução em GitHub Actions de um cenário de filtro sem correspondência (o holdout replaya o shell do workflow, não o Actions).

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Guard de integridade | `python3 scripts/ci/check_gate_integrity.py` | pass | `gate-integrity: PASS`, 7 ok, exit 0. |
| YAML dos workflows | `python3 -c 'import yaml; ...'` | pass | `yaml-ok`. |
| Holdout (Dark Factory) | `python3 $DF/scripts/holdout/run_holdout.py ...` | pass | 1 cenário. |
| Checker independente | `python3 $DF/scripts/holdout/checker.py ...` | pass | `holdout-check: PASS scenarios=1`, exit 0. |
| Prova de falha do gate | `observed.json` | pass | `exit_code=1`; stdout `auth: passed=0 floor=12`, `0 passed ... 89 filtered out`. |
| Proteção ao vivo (modo opcional) | `python3 scripts/ci/check_gate_integrity.py --remote` | pass | Contextos de `dev`/`main` iguais aos 4 esperados (executado localmente). |
| CI remoto no PR #254 | `gh pr checks 254` | pass | Todos os checks terminal: `frontend-build`, `rust-build`, `db-tests`, `dependency-review`, `supply-chain-vet`, `e2e-browser` pass; merge em `91ff5eadf0d09a8694e6ccebd7e98618d1a6c715`. |
| Guard no required job (remoto) | `gh run view 44000561792 --log` | pass | Step `Gate integrity (required checks / floors / no vacuity)` executou `python3 scripts/ci/check_gate_integrity.py` → `gate-integrity: PASS` com as 7 verificações. |
| Pisos DB no remoto | mesmo log, job `db-tests` | pass | `auth: passed=12 floor=12` confirmado (demais pisos no mesmo step). |
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

Bug **fechado**: PR #254 mesclado por decisão de máquina sob os quatro required checks, com o guard presente no log de `rust-build` (ADR-0003). Próximos candidatos: SHA-pin das 16 actions mutáveis (`osv-scan.yml:14` `@v2` aparenta tag inválida) e, após janela de 7 dias/20 PRs com CODEOWNERS, reavaliar promoção de `cargo-vet`/`e2e-browser`. RLS e RBAC seguem como ciclos separados.
