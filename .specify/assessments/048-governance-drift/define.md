# Define: 048 — Governance drift

## Problema

`AGENTS.md` descreve um workspace que não existe neste checkout (7 crates, toolchain
1.98.0).Como é a instrução governante lida por agentes antes de qualquer tarefa, o erro
propaga-se para backlog, verificação e decisões de governança.

## Requisitos funcionais

- **FR-001**: `AGENTS.md` DEVE listar os 5 membros reais e o pacote raiz `flora` (6 manifests).
- **FR-002**: `AGENTS.md` DEVE declarar toolchain `1.98.1`, conforme `Cargo.toml` e workflows.
- **FR-003**: A correção DEVE ser factual: nenhuma regra, permissão ou gate pode ser enfraquecido.
- **FR-004**: `OBJETIVO.md`/`ROADMAP.md` DEVEM receber nota datada de estado verificado (append-only).

## Não-objetivos

- Não alterar constituição, ADRs, workflows, gates, código Rust ou frontend.
- Não reintroduzir `flora-tasks`/`flora-files` como crates.
- Não tocar PR #262, branches alheias ou trabalho em voo da factory.

## Critérios de sucesso

- `grep` por `flora-tasks|flora-files|1.98.0` em `AGENTS.md` → 0 ocorrências.
- Inventário por `cargo metadata` confere com a lista escrita.
- Gates do alvo permanecem verdes.
