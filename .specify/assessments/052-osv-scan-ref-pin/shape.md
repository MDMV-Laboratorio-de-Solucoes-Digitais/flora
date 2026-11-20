# Shape: 052 — OSV scan ref pin

## Forma escolhida

Substituir, em `.github/workflows/osv-scan.yml:14`, `@v2` por `@v2.7.1`.
Um único arquivo de código, uma única linha; nenhuma reestruturação do job.

## Alternativas descartadas

- Pin por SHA: contraria a convenção documentada pelo upstream e cria dívida de
  atualização manual sem benefício para um job advisory.
- Pin `v2.6.0`: existe, mas a doc oficial recomenda `v2.7.1`.
- Promover a required check: mudaria ADR-0003 e o conjunto de 4 checks — decisão
  de governança do alvo, fora deste ciclo.
- Adicionar `continue-on-error`: o job já é não-obrigatório; seria ruído.

## Risco e reversão

Risco baixo: tag verificado em duas fontes independentes; inputs conferidos no
workflow do tag. Reversão: `git revert` do commit de implementação restaura `@v2`.
Nenhum efeito externo, nenhuma credencial envolvida.

## Verificação

`grep` do ref, `check_gate_integrity.py`, gates Rust/frontend e inspeção do diff.
