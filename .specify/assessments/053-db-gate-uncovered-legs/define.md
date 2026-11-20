# Define: 053 — DB gate uncovered legs

## Problema

Seis contratos DB (tarefas e workspaces) nunca são executados: o guardião da CI
e o job `db-tests` cobrem apenas as outras áreas. A confiança do gate de merge é
menor do que a documentação sugere.

## Requisitos funcionais

- **FR-001**: o job `db-tests` DEVE executar `contract::tasks::db::db_` com piso 4.
- **FR-002**: o job `db-tests` DEVE executar `contract::workspace::db::db_` com piso 2.
- **FR-003**: `check_gate_integrity.py` DEVE conter os dois novos pisos e validar
  as oito linhas `run_leg`, sem remover ou reduzir nenhuma existente.
- **FR-004**: os 4 required checks e a proteção de branch NÃO DEVEM mudar.

## Critérios de sucesso

- **SC-001**: os dois novos legs aparecem em `rust.yml` com os filtros completos.
- **SC-002**: `cargo test --list` confirma 4 e 2 testes nos filtros.
- **SC-003**: `check_gate_integrity.py` → PASS, com mensagem de 8 pisos / 49 total.
- **SC-004**: gates locais (fmt/clippy/deny/build/test + frontend) permanecem verdes.

## Não-objetivos

- Não alterar nomes de jobs/checks, ADR-0003, constituição ou proteção de branch.
- Não corrigir o fail-open de `db_state()` quando a env absence (candidato V1).
- Não elevar o piso de auth de 12 para 13 neste ciclo.
- Não tocar bugs, branches, worktrees ou PRs de outras sessões.
