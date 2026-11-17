# Shape: 047 — Release manifest lockstep

## Forma escolhida

Trocar o literal `8` por `6` na asserção `jq -e`, mantendo a cláusula de lockstep de
versão, e acrescentar commentário curto ligando o número aos membros do workspace.
Nenhuma abstração nova: o número é derivado de `cargo metadata` + `Cargo.toml`,
não de uma lista escrita à mão em outro lugar.

## Alternativas descartadas

- Derivar a contagem dinamicamente dentro do YAML: mais código, mais superfície de
  falha, e nenhuma fonte autoritativa adicional além do próprio `Cargo.toml`.
- Restaurar `flora-tasks`/`flora-files`: reintroduziria crates removidos e mudaria
  a arquitetura — fora de escopo e sem evidência de necessidade.
- Remover a checagem de contagem: enfraqueceria a proteção contra versionamento parcial.

## Risco e reversão

Risco baixo: nenhuma superfície HTTP, nenhuma autoria, nenhuma dependência.
Reversão: `git revert` do commit único; nenhum estado externo é criado.

## Verificação

Vermelho registrado em `intake.md`; verde será registrado em `validate.md` com o
mesmo comando `jq -e`, mais `check_gate_integrity.py`.
