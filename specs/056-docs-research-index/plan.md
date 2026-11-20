# Implementation Plan: 056 — Docs research index

**Data:** 2026-11-18T02:14Z · **Baseline:** `github/dev@94fcf19ca`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | contagens e convenção de nomes | — |
| índice | gerar `docs/research/index.md` | research |
| progresso | escrever `docs/PROGRESSO.md` | research |
| verification | contagem/links/subagent/gates | índice, progresso |

Ordem: research → índice → progresso → verification. Sem código de produção.

## Abordagem

1. Script Python efêmero (fora do repo) lê `docs/research/*.md`, extrai o
   primeiro `# ` e agrupa por prefixo de data; escreve o índice com cabeçalho
   contendo metodologia, contagem e comando de regeneração.
2. Escrever `docs/PROGRESSO.md` com nota datada: âncora, contagens `find`,
   gates, dívidas e **[NV]**, no padrão append-only dos docs existentes.
3. Validar: entradas = 135, links existem, subagent verificador isolado e
   gates determinísticos do alvo.

## Riscos

- Drift futuro do índice: mitigado com metodologia e comando de regeneração
  no cabeçalho.
- Títulos duplicados: aceitáveis; o link é o identificador.
- Nenhum risco a CI: nenhum arquivo de código/workflow é tocado.

## Verificação

`find | wc -l`, validação de links por Python, subagent e `check_gate_integrity.py`.

## Fora do plano

ADR-0003, constituição, código, workflows, C5/C6, deploy.
