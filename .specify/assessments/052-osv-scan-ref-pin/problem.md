# Problem Definition: Scan OSV adivinhado em ref inexistente

- **Slug**: 052-osv-scan-ref-pin
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

`.github/workflows/osv-scan.yml:14` referencia o workflow reutilizável `google/osv-scanner-action/.github/workflows/osv-scanner-reusable.yml@v2`. `git ls-remote` ao vivo não encontrou `refs/heads/v2` nem `refs/tags/v2` — só `v2.2.3` … `v2.7.1`, com `v2.7.1` em `32d3cd482bdadfb38b186c0ad1b031902d55d932` — de modo que o `@v2` não resolve para nenhum ref e o scan semanal advisory não pode iniciar (intake.md:9-16; research.md:5-13). A documentação oficial recomenda `@v2.7.1` e o workflow nesse tag expõe exatamente os inputs passados (`scan-args`, `fail-on-vuln`). O job não está entre os 4 required checks (ADR-0003:23), por isso o gate autoritativo não acusa o defeito: o sinal de vulnerabilidades de dependências deixou de ser produzido silenciosamente (intake.md:12-13; research.md:16). Ranqueamento 80/20 = 15.0 (intake.md:25-29).

## Affected Users & Stakeholders

- **Users**: mantenedores que dependem do sinal advisory de vulnerabilidades nas dependências para decidir atualizações (intake.md:11-13).
- **Stakeholders**: responsáveis por supply-chain e pelo gate autoritativo, que precisam preservar os 4 required checks sem promover o job a obrigatório (`define.md:15-18`; `decision.md:11-13`).

## Goals

- Fazer `.github/workflows/osv-scan.yml` referenciar um tag existente e documentado: `v2.7.1`, verificado por `git ls-remote` e pela documentação oficial (`define.md:7-9`).
- Manter os inputs passados ao reusable workflow exatamente como estão (`define.md:10`; `research.md:11`).
- Limitar a mudança a uma linha em um arquivo e preservar `check_gate_integrity.py` PASS com os checks intactos (`define.md:19-24`; `decision.md:10`).
- Registrar o estado real da execução como **[NV]** até evidência de run (`decision.md:17`).

## Non-Goals

- Não promover o OSV scan a required check (`define.md:14`; `shape.md:12`).
- Não alterar `fail-on-vuln`, agendamento, permissões ou Dependabot (`define.md:15`).
- Não atualizar ADR-0003 nem a constituição (`define.md:16`; `decision.md:12`).
- Não usar pin por SHA, `v2.6.0`, remover o job ou adicionar `continue-on-error` (`research.md:19`; `shape.md:10-14`).
- Sem merge, deploy ou efeito externo neste ciclo (`decision.md:13-14`).

## Success Metrics

- `grep -n 'reusable.yml@' .github/workflows/osv-scan.yml` → `v2.7.1` (baseline: `@v2`, ref inexistente nos remotos — intake.md:9-11; define.md:20-21).
- `check_gate_integrity.py` → PASS, 4 required checks e pisos intactos (baseline: PASS; mudança fora do gate — define.md:22; research.md:16).
- Diff de código = 1 linha em 1 arquivo (baseline: nenhuma alteração de código — define.md:23; shape.md:5-7).
- Gates Rust e frontend verdes (baseline: verdes — define.md:24).
- Execução real do job no GitHub Actions após o pin: **[NV]** — verificar no próximo cron ou via `workflow_dispatch` (decision.md:17; research.md:22).

## Cost of Inaction

O scan advisory continua sem iniciar e nenhum required check sinaliza isso; vulnerabilidades de dependências podem passar despercebidas sem que o gate autoritativo mude de cor. A correção tem custo mínimo e é reversível por `git revert`; a execução real permanece **[NV]** até ser observada (research.md:16; shape.md:16-17).

## Open Questions

- [NEEDS CLARIFICATION: a validação do pin será por `workflow_dispatch` ou pelo próximo agendamento semanal?] (`decision.md:17`; intake.md:19)
- [NEEDS CLARIFICATION: o tag `v2.7.1` continua sendo a convenção documentada quando da próxima atualização?] (`research.md:8-10`)
- [NEEDS CLARIFICATION: em qual PR o resultado da execução remota será registrado?] (`decision.md:17`; **[NV]**)
