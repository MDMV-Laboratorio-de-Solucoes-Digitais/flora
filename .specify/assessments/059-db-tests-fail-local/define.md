# Define: 059 — DB tests fail locally

## Problema

Testes de contrato DB reportam `passed` sem executar asserção quando as
variáveis de conexão não estão definidas — falsa confiança em execução local.

## Requisitos funcionais

- **FR-001**: os 9 `db_state()` DEVEM retornar `Err` quando nenhuma das duas
  env estiver definida.
- **FR-002**: os 50 chamadores NÃO DEVEM precisar ser editados.
- **FR-003**: com DB real, os mesmos testes DEVEM continuar passando.
- **FR-004**: nenhum check, workflow, ADR ou gate DEVE mudar.

## Critérios de sucesso

- **SC-001**: sem env, pelo menos um leg --ignored termina com exit != 0.
- **SC-002**: `--list` continua descobrindo 50 testes.
- **SC-003**: com DB isolado real, 50 testes passam.
- **SC-004**: fmt/clippy/deny/build/test + frontend permanecem verdes.

## Não-objetivos

- Não remover os `else { return Ok(()) }` inalcançáveis (diferença cosmética).
- Não alterar CI, gate, proteção de branch, ADRs ou constituição.
