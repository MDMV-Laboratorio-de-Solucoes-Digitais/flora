# Implementation Plan: 047 — Release manifest lockstep

**Data:** 2026-11-17T13:45Z · **Baseline:** `feature/047-release-manifest-lockstep@04bf70bdd`

## Componentes e dependências

| Componente | Papel | Depende de |
|---|---|---|
| `research` | evidência local do defect | — |
| `workflow` | asserção de contagem/versão em `release.yml` | research |
| `verification` | comando `jq -e` + gate de integridade | workflow |

Ordem: research → workflow → verification. Não há paralelismo útil (um único arquivo de código).

## Abordagem

1. Editar apenas `.github/workflows/release.yml`: `length == 8` → `length == 6`,
   com commentário curto explicando que o número acompanha os manifests do workspace.
2. Manter integralmente a cláusula de versão em lockstep e o restante do step.
3. Reexecutar o teste determinístico (vermelho → verde) e o gate autoritativo do alvo.

## Riscos e mitigação

- **Risco**: reintroduzir drift futureiro. Mitigação: commentário no local da asserção.
- **Risco**: alterar required checks. Mitigação: `check_gate_integrity.py` como gate de fechamento.

## Pontos de verificação

- Antes: `RED_TEST_EXIT=1` (registrado).
- Depois: exit 0 do mesmo comando + `gate-integrity: PASS`.
- Nenhuma execução de tag/release/deploy.

## Fora do plano

Constituição, ADRs, outros workflows, crates, dívidas de fase 043–046, remotos **[NV]**.
