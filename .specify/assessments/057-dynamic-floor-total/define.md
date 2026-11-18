# Define: 057 — Dynamic floor total

## Problema

O guardião imprime um número de pisos e um total escritos à mão, enquanto a
mesma informação já existe em `FLOORS`. Toda alteração de piso exige duas
edições; esquecer uma produz saída deterministicamente incorreta.

## Requisitos funcionais

- **FR-001**: a mensagem de sucesso DEVE derivar `len(FLOORS)` e
  `sum(FLOORS.values())`.
- **FR-002**: a regex de validação e as mensagens de falha NÃO DEVEM mudar.
- **FR-003**: a saída atual DEVE permanecer `"all eight db contract floors intact (50 total)"`.
- **FR-004**: nenhum required check, workflow, ADR ou teste DEVE mudar.

## Critérios de sucesso

- **SC-001**: guardião PASS com a mesma saída de antes.
- **SC-002**: mutação em cópia `/tmp` (adicionar 9º piso) → mensagem passa a
  dizer "nine ... (54 total)" sem editar a mensagem.
- **SC-003**: gates locais permanecem verdes.
- **SC-004**: diff de código = 1 linha em 1 arquivo.

## Não-objetivos

- Não editar ADR-0003/0006 (escalados), constituição, testes ou CI.
- Não alterar valores de `FLOORS` nem a regex.

## Correção pós-implementação (2026-11-18T02:47Z)

A mensagem passa a usar o numeral `8` (derivado de `len(FLOORS)`) em vez da
palavra `eight`. FR-003 passa a ler: saída atual `all 8 db contract floors
intact (50 total)` — conteúdo informacional idêntico ao texto anterior.
