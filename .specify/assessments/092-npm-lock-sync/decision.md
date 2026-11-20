# Decision: Sincronização do lockfile npm (ciclo 092)

- **Slug**: 092-npm-lock-sync
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/092-npm-lock-sync` @ `bf00caf5c`
- **Âncora**: `.specify/feature.json` → `specs/092-npm-lock-sync`
- **Artefatos revisados**: spec | plan | tasks

## Contexto / Problema

`apps/frontend/bun.lock` é o lockfile **canónico para CI**. Em contraste,
`apps/frontend/package-lock.json` estava **congelado**: não continha
`shadcn-svelte`, embora `apps/frontend/package.json` já declarasse a dependência
(`1.8.1`) e o `bun.lock` já a registrasse. O lockfile npm acumulava ainda
**25 alertas do Dependabot**. Há, portanto, drift entre o lockfile npm e a
fonte de verdade, com falsas expectativas para quem consome o `package-lock.json`.

O risco a evitar é "corrigir" mudando de gerenciador na CI, alterando o
manifesto, executando scripts de dependências ou alargando o diff a source,
workflows e aos artefatos de governança (`validate/`, `release/`,
`.specify/memory/STATE.md`).

## Decisão

Registrar o ciclo 092 como uma **regeneração determinística e restrita ao
lockfile npm**, com cinco compromissos rastreáveis:

1. **Conformidade (FR-001)**: `apps/frontend/package-lock.json` DEVE refletir
   `package.json` e o canónico `bun.lock`, incluindo `shadcn-svelte`.
2. **Comando contratual (FR-002)**: executar, em `apps/frontend`,
   `npm install --package-lock-only --ignore-scripts`. Nenhum outro mecanismo
   de regeneração é autorizado neste ciclo.
3. **Fontes intactas (FR-003)**: NÃO modificar `package.json` nem `bun.lock`.
4. **Escopo (FR-004)**: NÃO executar lifecycle scripts NEM alterar source, CI,
   `validate/`, `release/` ou `STATE`; única alteração de código é o lockfile npm.
5. **Evidência (FR-005)**: sincronização idempotente, com registro em decisão,
   spec, plano e tarefas.

## Evidência de entrada

- Manifesto declara `"shadcn-svelte": "1.8.1"`; `bun.lock` contém a entrada.
- Antes da regeneração, `package-lock.json` não continha a dependência.
- Inventário de entrada: 25 alertas do Dependabot no lockfile npm congelado.
- Diff resultante contra `bf00caf5c`: **+116 linhas em
  `apps/frontend/package-lock.json`**, com a entrada raiz
  `"shadcn-svelte": "1.8.1"` e suas transitivas; nenhum outro ficheiro de código
  no diff.

## Racional

**go.** O problema é medido, a correção é um único comando já contratado e o
resultado é reversível por restauro do ficheiro via controlo de versões. A CI
continua ancorada em `bun.lock`; o valor está em eliminar a divergência do
lockfile npm e reduzir ruído de alertas, sem mutar processo ou infraestrutura.

## [NV]

- **Contagem viva de alertas após a triagem**: o inventário registrado é o de
  entrada (25); o painel do Dependabot não foi relido neste ciclo.
- **CI do PR/merge/produção**: nenhum PR, merge ou deploy realizado/verificado;
  execução ao vivo do GitHub Actions permanece **[NV]**.

## Fora de âmbito

Alterações ao manifesto, ao `bun.lock`, ao pipeline de CI, a `validate`,
`release` ou `STATE`, rotação de credenciais e remediação individual dos
25 alertas do Dependabot.

## Handoff

`/speckit.specify` — especificar a sincronização como requisitos verificáveis
(FR-001..FR-005 / SC-001..SC-003) e tarefas T001..T006.
