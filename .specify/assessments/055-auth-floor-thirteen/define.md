# Define: 055 — Auth floor thirteen

## Problema

O gate de merge aceita o leg `auth` com 12 testes aprovados embora 13 sejam
descobertos — um teste pode desaparecer ou ser ignorado sem tornar a CI vermelha.

## Requisitos funcionais

- **FR-001**: `rust.yml` DEVE exigir piso 13 para `run_leg auth`.
- **FR-002**: `check_gate_integrity.py` DEVE exigir piso 13 em `FLOORS["auth"]`.
- **FR-003**: o texto de consolidação do guardião DEVE refletir 50 total.
- **FR-004**: nenhum outro piso, check, nome de job ou teste DEVE mudar.

## Critérios de sucesso

- **SC-001**: `--list` → 13 testes; guardião PASS com 13/13.
- **SC-002**: mutação (piso 13 sem 13 testes reais) → leg/auth falha.
- **SC-003**: `cargo test --workspace` e gates frontend permanecem verdes.
- **SC-004**: diff de código ≤ 3 linhas em 2 arquivos.

## Não-objetivos

- Não editar ADR-0003 (governança), constituição, outros pisos ou testes.
- Não alterar proteção de branch, C5/C6 ou deploy.
