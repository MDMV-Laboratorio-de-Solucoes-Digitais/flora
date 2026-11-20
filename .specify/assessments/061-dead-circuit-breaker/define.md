# Define: 061 — Dead circuit breaker

## Problema

Código morto (68 linhas) e configuração/variante de erro órfãs permanecem no
repositório, contrariando o princípio de debloat e aumentando a superfície
de leitura sem benefício.

## Requisitos funcionais

- **FR-001**: `utils/circuit_breaker.rs` DEVE ser removido.
- **FR-002**: os dois campos `circuit_breaker_*` e seus defaults DEVEM ser removidos de `config/messaging.rs`.
- **FR-003**: `Error::CircuitBreakerOpen` e seu braço de match DEVEM ser removidos de `error.rs`.
- **FR-004**: nenhum comportamento de runtime, CI, gate ou ADR DEVE mudar.

## Critérios de sucesso

- **SC-001**: `grep -rn circuit_breaker` → 0 ocorrências no código.
- **SC-002**: `cargo check/build/test` e clippy/deny/fmt verdes.
- **SC-003**: diff = 3 arquivos (1 exclusão, 2 edições) + artefatos.
- **SC-004**: gates do alvo permanecem verdes.

## Não-objetivos

- Não implementar circuit-breaker real.
- Não alterar Figment/Config, ADRs, workflows, proteção de branch.
