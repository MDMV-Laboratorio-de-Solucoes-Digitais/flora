# Define: 063 — Validate OSV run

## Problema

O pin `@v2.7.1` do scan OSV foi validado apenas estaticamente; sem execução,
o sinal advisory de supply-chain não tem evidência de funcionamento.

## Requisitos funcionais

- **FR-001**: DEVE ser disparada uma execução via `workflow_dispatch`.
- **FR-002**: DEVE ser registrado o resultado determinístico (`id`, `status`,
  `conclusion`, timestamp) em `STATE.md`, com apêndice datado.
- **FR-003**: NENHUM código, ADR, gate ou config DEVE mudar.
- **FR-004**: falha na execução DEVE ser registrada sem maquiagem.

## Critérios de sucesso

- **SC-001**: run criado e observado até conclusão.
- **SC-002**: `conclusion` colado com timestamp.
- **SC-003**: verificador/gates locais verdes.
- **SC-004**: diff sem código.

## Não-objetivos

- Não alterar o workflow se a execução falhar (ciclo separado).
- Não disparar outros workflows nem tocar branches/PRs remotos.
