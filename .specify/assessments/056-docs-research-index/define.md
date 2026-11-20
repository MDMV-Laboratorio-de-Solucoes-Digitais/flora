# Define: 056 — Docs research index

## Problema

O corpus de 135 documentos de research não tem índice, e o repositório não tem
um `PROGRESSO.md` com estado verificado — a recuperação depende de `find`/`grep`
ad hoc e de notas dispersas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md`.

## Requisitos funcionais

- **FR-001**: `docs/research/index.md` DEVE listar os 135 arquivos presentes,
  agrupados por data, com link relativo e título extraído do cabeçalho.
- **FR-002**: o índice DEVE ser gerado deterministicamente dos arquivos atuais
  (sem entrada manual) e declarar a contagem verificada.
- **FR-003**: `docs/PROGRESSO.md` DEVE registrar nota datada de estado verificado
  com âncora, contagens por `find`, gates, dívidas e **[NV]**.
- **FR-004**: nenhum código, workflow, ADR ou constituição DEVE mudar.

## Critérios de sucesso

- **SC-001**: número de entradas do índice = `find docs/research -type f | wc -l` = 135.
- **SC-002**: todas as entradas são links relativos que resolvem para arquivos existentes.
- **SC-003**: verificação independente (subagent) confirma cobertura integral.
- **SC-004**: gates determinísticos do alvo permanecem verdes.

## Não-objetivos

- Não editar ADR-0003 (aprovação de maintainer pendente).
- Não criar `index.md` na raiz (fora do escopo de `docs/`).
- Não mudar código, testes, CI, C5/C6 ou deploy.
