# Decision: 052 — OSV scan ref pin

**Verdict:** go (scoped)

## Decisão

Fixar o workflow reutilizável de OSV scan no tag `v2.7.1`, existente e
recomendado pela documentação oficial, preservando os demais parâmetros do job.
Nenhuma decisão de autorização, CI obrigatória ou deploy é tomada aqui.

## Delimitadores

- 1 arquivo de código existente (`.github/workflows/osv-scan.yml`), 1 linha.
- Artefatos do ciclo + notas datadas de estado (append-only).
- PR independente; nenhum PR/worktree/stash alheio tocado.

## NV

Execução real do job no GitHub Actions após o pin: **[NV]** — verificar ao vivo
no próximo agendamento semanal ou via `workflow_dispatch`.
