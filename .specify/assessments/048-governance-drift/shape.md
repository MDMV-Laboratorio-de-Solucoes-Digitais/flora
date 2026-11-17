# Shape: 048 — Governance drift

## Forma escolhida

Editar duas linhas de `AGENTS.md` (inventário de crates e versão da toolchain) e
acrescentar notas datadas de estado verificado em `OBJETIVO.md`/`ROADMAP.md`, sem
apagar o histórico existente.

## Alternativas descartadas

- Reintroduzir os crates ausentes: mudaria arquitetura e dependências sem evidência.
- Alterar `Cargo.toml` para corresponder ao texto: inverteria a autoridade (código é a fonte).
- Reescrever `AGENTS.md` inteiro: risco de perder regras úteis; diff pequeno é revisável.

## Risco e reversão

Risco operacional baixo (nenhum código executável). Risco de governança: a edição
deve ser revisada deliberadamente porque `AGENTS.md` orienta agentes. Reversão:
`git revert` dos commits do ciclo.

## Verificação

`grep` de ausência + `cargo metadata` + gates nativos + `check_gate_integrity.py`.
