# Implementation Plan: 060 — Refresh verified counts

**Data:** 2026-11-18T03:50Z · **Baseline:** `github/dev@262ac042f`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | contagens por `find` | — |
| progresso | apêndice datado em `docs/PROGRESSO.md` | research |
| verification | diff, contagens, verificador, gates | progresso |

Ordem: research → progresso → verification.

## Abordagem

1. Executar `find` por diretório e registrar números.
2. Acrescentar ao final de `docs/PROGRESSO.md` seção datada (03:50Z) com
   âncora `262ac042f`, tabela de contagens e nota **[NV]**.
3. Validar: `git diff --numstat` (0 remoções), cada número confere com `find`,
   verificador isolado e gates.

## Riscos

- Nenhum risco funcional; risco de erro de transcrição — mitigado por
  verificação determinística e subagent.

## Verificação

`find`, `git diff`, verificador, `check_gate_integrity.py`, suíte local.

## Fora do plano

ADRs, N3, constituição, código, CI, deploy.
