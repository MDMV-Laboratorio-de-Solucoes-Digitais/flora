# Implementation Plan: 063 — Validate OSV run

**Data:** 2026-11-18T04:47Z · **Baseline:** `github/dev@e1ff815bf`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | estado/gatilho do workflow | — |
| dispatch | `gh workflow run osv-scan.yml` | research |
| registro | apêndice em `STATE.md` | dispatch |
| verification | gates + verificador | registro |

Ordem: research → dispatch → registro → verification.

## Abordagem

1. Disparar: `gh workflow run osv-scan.yml --repo MDMV-Laboratorio-de-Solucoes-Digitais/flora`.
2. Consultar `gh run list --workflow osv-scan.yml --limit 1 --json databaseId,status,conclusion,createdAt,url` até conclusão.
3. Acrescentar apêndice datado em `STATE.md` com os valores e a interpretação.
4. Rodar gates locais e verificador.

## Riscos

- Execução pode falhar por razão externa — registrada sem maquiagem.
- Nenhum risco a código/gate/proteção.

## Verificação

`gh`, `git diff`, verificador, `check_gate_integrity.py`, suíte local.

## Fora do plano

Código, workflow, ADRs, constituição, C5/C6, deploy.
