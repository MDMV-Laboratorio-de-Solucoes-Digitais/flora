# Decision: hang do setup E2E do minter de sessão

- **Slug**: 076-mint-session-hang · **Decidido**: 2026-11-19T12:44Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(4×5)/(2×2)=5.00 — candidato 076 eleito; leg RBAC depende de maintainer.

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | Compilação dentro dos 180s (`seed.setup.ts:14,36-53`), mascarada no CI (`rust.yml:192-195`) |
| Evidência | strong | Tempos medidos (0.00s / 0.39s / 30.18s) e linhas coladas |
| Valor vs. inação | adequate | Inação deixa o pendura local sem sinal (`rust.yml:150`) |
| Viabilidade | strong | Opção A small; pré-build já é padrão no CI |
| Fit | adequate | Isolamento do setup E2E; sem toque em código/CI |
| Risco | strong | Causa atacada sem mudar o binário; risco de config mitigado por SC-003 |

## Handoff

Opção A (globalSetup com pré-build); dentro = config/setup E2E + artefatos 076; fora = código, workflows, ADRs, constituição, STATE, specs/076-*, leg RBAC, issues. FR-001..FR-005 e SC-001..SC-004 de `define.md`. Perguntas carry-forward: nenhuma. `[NV]` para tempo com target frio e CI/produção.
