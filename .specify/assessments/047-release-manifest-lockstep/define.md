# Define: 047 — Release manifest lockstep

## Problema

O workflow de release falha deterministicamente antes de qualquer publicação porque
usa um número de manifests (`8`) que não corresponde ao workspace real (`6`).
O objetivo do passo — garantir que todos os crates estejam na mesma versão da tag —
fica inalcançável.

## Requisitos funcionais (rascunho)

- **FR-001**: O workflow DEVE aceitar exactamente os 6 manifests `flora*` presentes no checkout.
- **FR-002**: A condição de versão em lockstep DEVE permanecer intacta.
- **FR-003**: A asserção DEVE ser executável localmente com o mesmo comando do workflow.
- **FR-004**: Nenhum outro workflow, gate ou arquivo de governança DEVE mudar.

## Não-objetivos

- Não criar/remover crates, não alterar `Cargo.toml`.
- Não disparar tag, release, deploy, auto-merge ou push.
- Não tocar constituição, ADRs, `.github/workflows/*` além de `release.yml`.
- Não corrigir as dívidas de fase 043–046 (registradas, fora deste ciclo).

## Critérios de sucesso

- `jq -e ...` com o número corrigido retorna exit 0 localmente (verde) com versões iguais.
- `check_gate_integrity.py` continua PASS.
- Diff revisável: 1 linha de código + commentário explicativo.
