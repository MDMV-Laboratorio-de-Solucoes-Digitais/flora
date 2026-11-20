# Release: 092 — npm lock sync

**Ciclo**: 092 · **Data**: 2026-11-20 · **Branch**: `feature/092-npm-lock-sync` @ `bf00caf5c` · **Baseline**: `bf00caf5c`
**Tipo**: sincronização de lockfile — `apps/frontend/package-lock.json` regenerado a partir do manifesto com `npm install --package-lock-only --ignore-scripts`, sem alterar `package.json` nem o lockfile canónico `bun.lock`. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/package-lock.json` — entrada `shadcn-svelte@1.8.1` e dependências transitivas adicionadas (diff de +116 linhas contra `bf00caf5c`); drift estrutural com o manifesto/`bun.lock` eliminado.
- `apps/frontend/package.json` (manifesto) e `apps/frontend/bun.lock` (canónico da CI) — **intactos**, fora do diff; nenhuma troca de gerenciador de lockfile.
- Comando contratual `npm install --package-lock-only --ignore-scripts` executado em `apps/frontend` com `exit=0`, sem lifecycle scripts, sem exposição de credenciais e sem alterações adicionais na reexecução (idempotência).
- Verificação: gate autoritativo `python3 scripts/ci/check_gate_integrity.py` **PASS** (`exit=0`, 10 legs / 58 pisos intactos) e `npm run check`, `npm run lint`, `npm run build` no frontend sem falhas. Detalhe literal em `.specify/validations/092-npm-lock-sync/validate.md` (R4 **APROVADO**), com proveniência de evidência esperada/consolidada do handoff.
- Inventário de entrada: 25 alertas do Dependabot no lockfile congelado; não promovidos a gate e não auditados individualmente. Contagem viva pós-triagem `[NV]`.

## SemVer

**Proposta:** patch `0.4.6` → `0.4.7`. Motivo: sincronização de arquivo de lock já declarado no manifesto e no lockfile canónico — nenhuma alteração de API, comportamento de runtime, schema, migration, contrato de produção ou superfície do produto; a CI continua usando `bun.lock`. Mesmo critério dos precedentes imediatos 089 (`0.4.3 → 0.4.4`), 090 (`0.4.4 → 0.4.5`) e 091 (`0.4.5 → 0.4.6`) para mudanças de dependências/governança sem quebra de contrato. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` `apps/frontend/package-lock.json` passa a refletir `package.json` e o canónico `bun.lock`, incluindo `shadcn-svelte@1.8.1` e suas entradas transitivas.
- `FR-002` sincronização executada em `apps/frontend` exclusivamente com `npm install --package-lock-only --ignore-scripts`, o comando contratado no `decision.md`.
- `FR-003` `apps/frontend/package.json` e `apps/frontend/bun.lock` não foram modificados; `bun.lock` permanece a fonte de verdade da CI.
- `FR-004` nenhum lifecycle script executado; código-fonte, configuração de CI, `validate/`, `release/` e `STATE.md` intocados — escopo exclusivo do lockfile npm.
- `FR-005` operação idempotente (reexecução gera zero diffs adicionais) com evidência registrada em `decision.md`, `spec.md`, `plan.md` e `tasks.md`.
- `T001` drift confirmado (lock congelado sem `shadcn-svelte` vs manifesto/`bun.lock`) · `T002` evidência registrada (bun.lock canónico, 25 alertas, comando contratual) · `T003` comando contratual executado em `apps/frontend` · `T004` sincronia e idempotência verificadas (SC-001) · `T005` escopo verificado: só `package-lock.json`, sem scripts e sem toque em manifesto/bun.lock/source/CI (SC-002, SC-003) · `T006` rastreabilidade fechada em decisão/spec/plano/tarefas, sem tocar `validate`/`release`/`STATE` (FR-005).

## Evidência

`.specify/validations/092-npm-lock-sync/validate.md` (R4 **APROVADO**, saídas literais esperadas/consolidadas do handoff) · `specs/092-npm-lock-sync/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=5/5`, `tasks=6/6`) · `specs/092-npm-lock-sync/converge.md` (**Converged**, sem novas tarefas) · baseline `bf00caf5c` · diff do ciclo: `apps/frontend/package-lock.json`, 1 arquivo de código, mais os artefatos documentais 092. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Restaurar o `apps/frontend/package-lock.json` anterior ao ciclo 092, sem force-push e sem reescrita de histórico:

1. em `apps/frontend/package-lock.json`, remover a entrada `shadcn-svelte@1.8.1` e as 116 linhas de dependências transitivas adicionadas, voltando ao estado de `bf00caf5c` — se as alterações ainda estiverem não commitadas, `git checkout -- apps/frontend/package-lock.json`; se commitadas, novo commit de reversão ou `git revert` do commit do ciclo;
2. **não reverter** `apps/frontend/package.json` nem `apps/frontend/bun.lock` — ambos estavam intactos antes, durante e depois do ciclo; revertê-los introduziria uma mudança que nunca existiu;
3. manter workflows, gate autoritativo, código-fonte, `.specify/memory/STATE.md`, ADRs e constituição intactos — nenhum deles foi alterado pelo ciclo;
4. após a reversão, repetir `npm install --package-lock-only --ignore-scripts`, `python3 scripts/ci/check_gate_integrity.py` e `npm run check && npm run lint && npm run build` em `apps/frontend`: o esperado é que o comando reapresente o drift original (lock sem `shadcn-svelte`) e que gate e verificações do frontend permaneçam PASS, já que a CI usa `bun.lock`.

Não há arquivos novos de código, migrations, testes, config de produção nem artefatos de runtime para remover — apenas os documentos do ciclo 092 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, o R4 e este release), que são registros e não afetam o sistema. **Nenhum banco de produção é tocado, alterado ou removido.**

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Gate local PASS não substitui a CI remota para promoção.
- **Contagem viva de alerts após a triagem**: registrado o inventário de entrada (25 alertas, 7 high, no `package-lock.json` congelado); a contagem pós-regeneração/pós-triagem no painel do Dependabot não foi lida e permanece `[NV]`. Nenhum alerta resolvido individualmente ou promovido a required check.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **DBs de produção**: nenhum acesso, escrita ou remoção.
- **Escala**: mudança em 1 arquivo de código (lockfile npm); sem teste de carga, UI ou comportamento de runtime a verificar além das verificações do frontend e do gate.
