# Define: 062 — Escalation register

## Problema

Itens bloqueados por governança são reavaliados sem registro canônico, e o
estado do scan OSV permanece **[NV]** sem evidência de por quê.

## Requisitos funcionais

- **FR-001**: `STATE.md` DEVE conter tabela append-only com os 4 itens
  escalados (evidência, bloqueio, ação, status).
- **FR-002**: DEVE registrar o estado OSV verificado ao vivo (active, 0 runs,
  cron, pin ainda não validado).
- **FR-003**: NENHUM item bloqueado DEVE ser executado.
- **FR-004**: nenhum código, ADR, migração ou gate DEVE mudar.

## Critérios de sucesso

- **SC-001**: tabela com 4 linhas e colunas completas.
- **SC-002**: números/estados do OSV conferem com `gh api`.
- **SC-003**: verificador isolado PASS.
- **SC-004**: gates do alvo verdes.

## Não-objetivos

- Não alterar ADRs, N3, C5/C6, constituição ou CI.
- Não disparar o workflow OSV.
