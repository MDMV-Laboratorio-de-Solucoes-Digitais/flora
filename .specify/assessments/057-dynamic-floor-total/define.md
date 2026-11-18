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
