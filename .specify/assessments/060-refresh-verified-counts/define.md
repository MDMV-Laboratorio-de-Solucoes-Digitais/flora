# Define: 060 — Refresh verified counts

## Problema

A nota de "estado verificado" em `docs/PROGRESSO.md` ficou desatualizada
pelos três ciclos seguintes; um leitor que toma a contagem como atual obtém
números incorretos.

## Requisitos funcionais

- **FR-001**: `PROGRESSO.md` DEVE conter nova seção datada com contagens
  atuais obtidas por `find`.
- **FR-002**: a seção anterior DEVE permanecer integralmente (append-only).
- **FR-003**: a nova seção DEVE declarar âncora e **[NV]** para estado remoto.
- **FR-004**: nenhum código, migration, ADR ou gate DEVE mudar.

## Critérios de sucesso

- **SC-001**: cada número da nova tabela confere com `find`.
- **SC-002**: `git diff` mostra apenas acréscimo ao final de `PROGRESSO.md`.
- **SC-003**: verificador isolado confirma as contagens e a preservação.
- **SC-004**: gates do alvo permanecem verdes.

## Não-objetivos

- Não corrigir ADRs (aprovação de maintainer pendente).
- Não executar N3 (migração com risco, escalada).
