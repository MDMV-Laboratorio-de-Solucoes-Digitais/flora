# Bug Assessment: required checks do merge autônomo sem guarda de integridade

- **Slug**: gate-integrity-unguarded
- **Created**: 2026-11-16
- **Source**: `docs/adr/0003-autonomous-merge-under-deterministic-gates.md:28-29` (riscos residuais) + `docs/research/2026-11-16-038-gate-integrity-decision.md`
- **Verdict**: valid
- **Severity**: high

## Report

ADR-0003 removeu a aprovação humana do merge e a substituiu por quatro required checks. A própria ADR registra que (a) GitHub conta `skipped` como sucesso e (b) renomear um job órfão o required check, travando todos os PRs. Nenhum mecanismo no repositório detectava qualquer uma dessas condições, nem a remoção dos pisos de 43 contratos DB introduzidos no ciclo 037.

## Symptom

**Observed:** configuração de proteção e workflows podem divergir sem sinal; um job obrigatório pode ganhar `continue-on-error`/`if:` e ficar verde sem executar nada.

**Expected:** falha determinística e imediata no CI quando nome de check, vacuidade ou pisos de teste mudarem.

## Reproduction

1. `gh api repos/{owner}/{repo}/branches/dev/protection` → 4 contexts (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`).
2. Renomear `rust-build` em `rust.yml` → o check antigo nunca mais reporta; PRs ficam "Expected" indefinitely (documentado pelo GitHub).
3. Adicionar `continue-on-error: true` a um job obrigatório → check verde sem verificação.

## Suspected Code Paths

- `.github/workflows/rust.yml` — job `rust-build` (step `Test`, pisos `db-tests`) e `dependency-review` (`if:` legítimo, PR-only).
- `.github/workflows/ci.yml` — job `frontend-build`.
- Proteção de `dev`/`main` (GitHub admin API, fora do repo).

## Root Cause Hypothesis

**Confidence: high.** A correção do ciclo 037 cobriu a vacuidade *dentro* dos testes (0 testes casados), mas não a vacuidade *da configuração dos gates*. A proteção vive no GitHub, não no repo, e não há espelho versionado.

## Proposed Remediation

**Preferred:** script stdlib `scripts/ci/check_gate_integrity.py`, executado como step do job **obrigatório** `rust-build` (não cria novo check → sem deadlock). Verifica: nomes de job presentes, ausência de `continue-on-error`, `if:` apenas no caso documentado de `dependency-review`, ausência de `paths:` em workflows com checks obrigatórios, presença de `--workspace`, e os seis pisos. Modo opcional `--remote` compara as proteções ao vivo (não usado no runner).

**Alternatives:** required check novo (exige sequência add→land→add name→remove old→delete, maior risco de deadlock); ruleset do GitHub (fora do repo, não versionável);_action de terceiros (dependência nova, proibida sem gate).

**Files likely to change:** `.github/workflows/rust.yml`, `scripts/ci/check_gate_integrity.py`, docs de política (`OBJETIVO.md`, `ROADMAP.md`, `036-authorization-matrix.md`), `.specify/holdout/*`, `docs/research/2026-11-16-038-*.md`.

**Tests to add or update:** holdout da Dark Factory replicando o `run_leg` com filtro inexistente; esperado exit 1 e `passed=0 floor=12`.

## Risks & Considerations

- Usar job existente evita mudar nomes de check — mas se o job `rust-build` for renomeado no futuro, o próprio guard deixa de rodar. Documentado como risco residual.
- YAML é parseado por regex/indentação (stdlib), não por parser completo; suficiente para as invariantes checadas, mas não substitui validação de sintaxe (`yaml.safe_load` executado à parte).
- Deploy e release continuam fora do gate, por decisão explícita.

## Open Questions

- Promover `cargo-vet`/`e2e-browser` a required checks após janela de 7 dias/20 PRs com CODEOWNERS — ciclo futuro.
- SHA-pin das 16 actions mutáveis — ciclo futuro de manutenção.
