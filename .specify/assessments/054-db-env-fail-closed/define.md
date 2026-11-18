# Define: 054 — DB env fail-closed

## Problema

O job `db-tests` pode ficar verde sem executar asserção alguma se as variáveis
de conexão Postgres forem removidas: os 50 testes retornam `Ok(())` e os pisos
são satisfeitos por contadores vazios de significado.

## Requisitos funcionais

- **FR-001**: `run_leg` DEVE falhar imediatamente se
  `FLORA_DATABASE__POSTGRES_URL` ou `DATABASE_URL` estiver ausente.
- **FR-002**: `check_gate_integrity.py` DEVE exigir que o job `db-tests` contenha
  as duas atribuições de env.
- **FR-003**: os 8 pisos, os 4 required checks e os nomes de jobs NÃO DEVEM mudar.
- **FR-004**: nenhum teste `#[ignore]` DEVE ser editado neste ciclo.

## Critérios de sucesso

- **SC-001**: executar o script do step sem env → exit != 0 antes do `cargo test`.
- **SC-002**: executar com env apontando para DB real → 8 legs verdes (50 testes).
- **SC-003**: guardião PASS; remover simulatedamente uma linha de env em cópia
  temporária → guardião RED (prova da regressão).
- **SC-004**: gates locais (fmt/clippy/deny/build/test + frontend) permanecem verdes.

## Não-objetivos

- Não alterar 9 `db_state()` nem os 50 chamadores (falso positivo local permanece
  documentado como dívida, fora de escopo).
- Não alterar ADR-0003, constituição, proteção de branch, C5/C6 ou deploy.
