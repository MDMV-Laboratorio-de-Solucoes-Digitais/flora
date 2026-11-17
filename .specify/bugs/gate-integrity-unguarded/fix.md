# Bug Fix: required checks do merge autônomo sem guarda de integridade

- **Slug**: gate-integrity-unguarded
- **Fixed**: 2026-11-16
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Adicionado verificador determinístico da integridade dos gates, executado dentro de um job já obrigatório; evidência de holdout da Dark Factory; e reconciliação das docs de política com a ADR-0003.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `scripts/ci/check_gate_integrity.py` | novo | Stdlib-only. Checa 4 nomes de job, `continue-on-error`, `if:`, `paths:`, `--workspace` nos comandos de build/test e os 6 pisos (43). Modo `--remote` opcional via `gh api`. |
| `.github/workflows/rust.yml` | +1 step em `rust-build` | `python3 scripts/ci/check_gate_integrity.py` — nenhum novo nome de check, logo nenhuma mudança de proteção. |
| `.specify/holdout/flora-gate-determinism.jsonl` + `.expected.jsonl` | novos | Cenário: filtro `db_ZZZ_NOMATCH` com piso 12; esperado exit 1. |
| `.specify/holdout/observed.json` + `verdict.json` | novos | Evidência executada (`holdout-check: PASS`). |
| `docs/research/2026-11-16-038-gate-integrity-decision.md` | novo | Decisão, pesquisa multilíngue e fontes. |
| `OBJETIVO.md`, `ROADMAP.md`, `docs/research/2026-11-16-036-authorization-matrix.md` | editados | Removida a exigência obsoleta de autorização humana para merge; ADR-0003 é a norma vigente. Deploy/release continuam autorização-flagged. |

## Diff Highlights (optional)

```yaml
- name: Gate integrity (required checks / floors / no vacuity)
  run: python3 scripts/ci/check_gate_integrity.py
```

## Tests Added or Updated

- Holdout `zero-tests-match-fails` (Dark Factory `run_holdout.py` + `checker.py`).
- Nenhum teste Rust novo.

## Local Verification

| Comando | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | `PASS`, 7 verificações, exit 0 |
| `python3 -c 'import yaml; ...'` | `yaml-ok` |
| `run_holdout.py --scenario ... --target-repo $FL` | `holdout-run: 1 scenario(s)` |
| `checker.py --report ... --expected ...` | `holdout-check: PASS scenarios=1`, exit 0 |
| holdout observado | `exit_code=1`, `auth: passed=0 floor=12`, `0 passed ... 89 filtered out` |

## Deviations from Assessment

- Nenhuma. O guard roda em `rust-build` (previsto) e o holdout replica o shell do workflow (previsto).

## Follow-ups

- SHA-pin das 16 actions mutáveis (`@v4`, `@v6`, `@cargo-deny`, taiki-e por ferramenta); `osv-scan.yml:14` `@v2` aparenta tag inexistente — investigar antes de tocar.
- Reavaliar promoção de `cargo-vet` (455 exemptions, sem CODEOWNERS) e `e2e-browser` após 7 dias/20 PRs.
- Investigar role de conexão Postgres antes de mexer em RLS/`FORCE ROW LEVEL SECURITY`.
