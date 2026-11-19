# Define: 069 — corrigir nota falsa de dívida no índice de research

## Problema

Uma nota no índice de research afirma que 22 documentos (047+) não foram indexados. Como o índice é a ferramenta de descoberta usada por agentes, a afirmação falsa faz o leitor desconfiar de cobertura inexistente e pode levar a retrabalho (backfill de algo já feito).

## Requisitos funcionais

- **FR-001**: anexar a `docs/research/index.md` nota datada que corrija a afirmação, com o resultado da verificação de conjunto (137/137, `missing=[]`) e o comando reprodutível.
- **FR-002**: manter todas as notas anteriores intactas (append-only) e continuar declarando com precisão a diferença entre "links presentes" e "atribuição de ciclo nos títulos".
- **FR-003**: registrar decisão, execução e fechamento em `STATE.md` e artefatos SDD do ciclo, com um arquivo por commit e paths explícitos.

## Critérios de sucesso

- **SC-001**: após a edição, a verificação de conjunto continua `missing=[]`, `extra=[]` e 137 links.
- **SC-002**: gate autoritativo local/`--remote` PASS e gates Rust/frontend do alvo permanecem verdes.
- **SC-003**: `git diff --name-only dev..HEAD` contém apenas os paths declarados; nenhuma mudança em código, workflow, gate, ADR ou constituição.

## Fora de escopo

Backfill de entradas (já completo), renomeação em massa de títulos do índice, emenda da constituição, indexação remota/Forgejo, e qualquer merge sem CI verde observada ao vivo.
