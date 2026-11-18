# Implementation Plan: 058 — Root doc index

**Data:** 2026-11-18T03:04Z · **Baseline:** `github/dev@58921a337`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | pontos de entrada existentes | — |
| índice | escrever `index.md` | research |
| verification | links, linhas, verificador, gates | índice |

Ordem: research → índice → verification. Sem código de produção.

## Abordagem

1. Escrever `index.md` com cabeçalho datado (03:04Z), âncora
   `github/dev@58921a337`, três grupos de links relativos e nota **[NV]**.
2. Validar links por Python (`pathlib.exists()`), contar linhas (≤ 40) e
   submeter ao verificador isolado.
3. Rodar gates determinísticos do alvo.

## Riscos

- Links quebrados: mitigados por validação determinística.
- Drift de descrições: mitigado por frases curtas, sem contagens duplicadas.

## Verificação

Python de links, `wc -l`, verificador, `check_gate_integrity.py` e suíte local.

## Fora do plano

README, ADRs, constituição, código, workflows, C5/C6, deploy.
