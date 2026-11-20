# Release: 093 — Upgrade de segurança SvelteKit 2.70.3 → 3.1.1

**Ciclo**: 093 · **Data**: 2026-11-20 · **Branch**: `feature/093-sveltekit-security-upgrade` @ `b426e4f81` · **Baseline**: `b426e4f81`
**Tipo**: upgrade de segurança de dependência no frontend — `@sveltejs/kit 3.1.1`, `@sveltejs/adapter-node 6.0.5`, `svelte 5.57.1` e locks sem `2.70.3`, liberando o `dependency-review` da PR #318 (`dev` → `main`). **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/package.json` — `@sveltejs/kit 3.1.1` (corrige `GHSA-3r8g` high + 3 moderate da versão `2.70.3`), `@sveltejs/adapter-node 6.0.5` (peer `@sveltejs/kit ^3`) e `svelte 5.57.1`; `vite ^8.0.16`, `typescript ^6.0.3` e `@sveltejs/vite-plugin-svelte ^7.1.2` permanecem nas faixas já compatíveis com os peers do kit corrigido (`^8.0.12`, `^6.0.0`, `^7.0.0`).
- `apps/frontend/bun.lock` (canónico da CI) e `apps/frontend/package-lock.json` — atualizados para o mesmo conjunto de versões, com **zero ocorrências de `2.70.3`**; nenhuma troca de gerenciador de lockfile.
- Verificação: `npm run check`, `npm run lint`, `npm run build` e `npm run test` em `apps/frontend` sem falhas (`exit=0`), gate autoritativo `python3 scripts/ci/check_gate_integrity.py` **PASS** (`exit=0`, 10 legs / 58 pisos intactos) e `dependency-review` da PR #318 (`dev` → `main`) **verde**. Detalhe literal em `.specify/validations/093-sveltekit-security-upgrade/validate.md` (R4 **APROVADO**), com proveniência de evidência esperada/consolidada do handoff; execução remota ao vivo `[NV]`.
- Inventário de entrada: `GHSA-3r8g` (high) + 3 moderate em `@sveltejs/kit 2.70.3`; resolvidos pela promoção para `3.1.1`, não por isenção, supressão ou reconfiguração de gate. Contagem viva pós-upgrade `[NV]`.

## SemVer

**Proposta:** patch `0.4.7` → `0.4.8`. Motivo: atualização de segurança de dependências com as mesmas superfícies compatíveis declaradas — nenhuma alteração de API de aplicação, comportamento de runtime, schema, migration, contrato de produção ou funcionalidade do produto; o `bun.lock` continua canónico da CI e nenhum required check é alterado. Mesmo critério dos precedentes imediatos 090 (`0.4.4 → 0.4.5`), 091 (`0.4.5 → 0.4.6`) e 092 (`0.4.6 → 0.4.7`) para bumps de dependência/governança sem quebra de contrato. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` `@sveltejs/kit` promovido de `2.70.3` para `3.1.1` no manifesto e nos dois locks, eliminando `GHSA-3r8g` (high) e os 3 moderate sem supressão de alertas.
- `FR-002` `@sveltejs/adapter-node` promovido de `5.5.7` para `6.0.5` (peer `@sveltejs/kit ^3`) e `svelte` para `5.57.1`; `vite ^8.0.16`, `typescript ^6.0.3` e `@sveltejs/vite-plugin-svelte ^7.1.2` já satisfazem os peers do kit `3.1.1`.
- `FR-003` `bun.lock` e `package-lock.json` sincronizados com o manifesto, sem `2.70.3`, `5.5.7` ou `5.56.10` residuais; `bun.lock` permanece canónico da CI.
- `FR-004` `npm run check`, `npm run lint`, `npm run build` e `npm run test` sem falhas; gate autoritativo PASS; `dependency-review` verde na PR #318 (`dev` → `main`), com resultado remoto ao vivo `[NV]`.
- `FR-005` diff de código restrito a `apps/frontend/package.json`, `apps/frontend/bun.lock` e `apps/frontend/package-lock.json`; workflow, required checks, runtime, schema, PR/merge/produção, `validate/` e `STATE.md` intocados.
- `T001` inventário de advisories e baseline `b426e4f81` fixados · `T002` versões corrigidas e peers validados (kit `3.1.1`, adapter-node `6.0.5`, svelte `5.57.1`) · `T003` manifesto e locks atualizados, sem `2.70.3` · `T004` `check`/`lint`/`build`/`test` e gate verificados (SC-002) · `T005` `dependency-review` verde na PR #318 e fronteira do diff verificadas (SC-003) · `T006` rastreabilidade fechada em decisão/spec/plano/tarefas/analyze/converge/R4, sem tocar CI ou produção (FR-005).

## Evidência

`.specify/validations/093-sveltekit-security-upgrade/validate.md` (R4 **APROVADO**, saídas literais esperadas/consolidadas do handoff) · `specs/093-sveltekit-security-upgrade/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=5/5`, `tasks=6/6`) · `specs/093-sveltekit-security-upgrade/converge.md` (**Converged**, sem novas tarefas) · baseline `b426e4f81` · diff do ciclo: manifesto do frontend + 2 locks, 3 arquivos de código, mais os artefatos documentais 093. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Restaurar as versões anteriores ao ciclo 093, sem force-push e sem reescrita de histórico:

1. em `apps/frontend/package.json`, voltar a `@sveltejs/kit: ^2.63.0` (resolvido `2.70.3`), `@sveltejs/adapter-node: ^5.5.4` (resolvido `5.5.7`) e `svelte: ^5.56.1` (resolvido `5.56.10`), mantendo `vite`, `typescript` e `@sveltejs/vite-plugin-svelte` inalterados — eles já atendiam aos peers antes do ciclo;
2. em `apps/frontend/bun.lock` e `apps/frontend/package-lock.json`, restaurar o estado de `b426e4f81` com as versões acima — se as alterações ainda estiverem não commitadas, `git checkout -- apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json`; se commitadas, novo commit de reversão ou `git revert` do commit do ciclo;
3. **não reverter** workflows, required checks, constituição, ADRs, `.specify/memory/STATE.md`, código-fonte, testes ou config de produção — nenhum deles foi alterado pelo ciclo; nenhum banco de produção é tocado, alterado ou removido;
4. após a reversão, repetir `npm run check && npm run lint && npm run build && npm run test` em `apps/frontend` e `python3 scripts/ci/check_gate_integrity.py`: o esperado é que as verificações permaneçam PASS, enquanto o `dependency-review` da PR #318 volta a acusar `GHSA-3r8g` (high) + 3 moderate — a reversão reapresenta as vulnerabilidades e só deve ser executada se o upgrade inteiro for abandonado;
5. **Nenhum arquivo novo de código**, migration, teste, config de produção ou artefato de runtime para remover — apenas os documentos do ciclo 093 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, o R4 e este release), que são registros e não afetam o sistema.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions — inclusive o `dependency-review` remoto da PR #318 —, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Gate local PASS não substitui a CI remota para promoção.
- **Contagem viva de advisories**: registrado o inventário de entrada (`GHSA-3r8g` high + 3 moderate em `@sveltejs/kit 2.70.3`); a contagem pós-upgrade no painel do GitHub não foi lida e permanece `[NV]`. Nenhum alerta suprimido ou promovido a required check.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **DBs de produção**: nenhum acesso, escrita ou remoção.
- **Escala**: mudança em 3 arquivos de código (manifesto + 2 locks); sem teste de carga, UI ou comportamento de aplicação a verificar além das verificações do frontend, do gate e do `dependency-review`.
