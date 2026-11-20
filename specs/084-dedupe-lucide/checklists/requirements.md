# Requirements Quality Checklist — 084

**Feature**: [spec.md](../spec.md) · **Slug**: 084-dedupe-lucide
**Created**: 2026-11-19T23:55Z

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T23:55Z, contra `define.md`/`decision.md` (acessados 2026-11-19T23:55Z). A execução dos comandos determinísticos, a reexecução do gate, a regressão visual em browser, o grafo final do bundler, o churn real dos lockfiles, CI/PR e o upgrade futuro para `1.76.0` permanecem `[NV]`.

## Completude

- [x] FR-001 especifica trocar a origem dos 3 imports de `'lucide-svelte'` para `'@lucide/svelte'`, preservando os 14 símbolos e a sintaxe de named import: `apps/frontend/src/lib/components/chat/ChatLayout.svelte:10` (`Send`, `Sparkles`), `apps/frontend/src/lib/components/search/SearchBox.svelte:2` (`Search`) e `apps/frontend/src/routes/(app)/+layout.svelte:8-19` (11 símbolos, com `Sparkles` repetido). O root de `@lucide/svelte@1.33.0` exporta todos os nomes (`dist/icons/index.js`, `dist/aliases/aliases.js:751` para `CheckSquare`), logo nenhum rename é necessário.
- [x] FR-002 especifica remover `"lucide-svelte": "^1.0.1"` de `apps/frontend/package.json:53` (bloco `dependencies`) via `bun remove lucide-svelte`, com reflexo em `apps/frontend/bun.lock:10` e `:517`. Nenhuma outra dependência de `dependencies` é tocada (`bits-ui:51`, `clsx:52`, `tailwind-merge:54` permanecem).
- [x] FR-003 especifica fixar a entrada de devDependencies `apps/frontend/package.json:22` como `"@lucide/svelte": "1.33.0"` (sem `^`) via `bun add -D -E @lucide/svelte@1.33.0`, com reflexo em `apps/frontend/bun.lock:16` e `:95`. O pacote permanece em devDependencies e o `"dev": true` de `package-lock.json:345` é preservado.
- [x] FR-004 especifica sincronizar `apps/frontend/package-lock.json` via `npm install --package-lock-only --ignore-scripts`: remover a entrada `node_modules/lucide-svelte` (`:3772-3781`), manter `node_modules/@lucide/svelte` em `1.33.0` (`:341-348`) e preservar o restante do arquivo. Churn não relacionado é fora de escopo e deve ser descartado na revisão do diff.
- [x] FR-005 limita o diff rastreado aos 3 arquivos de import, `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json` e aos 5 artefatos `.specify/assessments/084-dedupe-lucide/`, acompanhados dos documentos do próprio ciclo em `specs/084-dedupe-lucide/`. Exige formatação local (tabs, aspas simples em Svelte/JS, nenhuma trailing comma — `prettier.config.js:5-7`) e proíbe dependência nova, segredo, mutação de DB/rede, gate, CI, ADR, spec numerada ou PR. Base `9da1aaea6` preservada.

## Clareza e consistência

- [x] SC-001 é verificável: `grep -rn "lucide-svelte" apps/frontend/src apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json` retorna zero ocorrências do pacote legado; os 3 imports apontam para `'@lucide/svelte'` com os mesmos 14 símbolos; os 4 componentes UI com subpath `/icons/*` (`dialog-content.svelte`, `dropdown-menu-checkbox-item.svelte`, `dropdown-menu-radio-item.svelte`, `dropdown-menu-sub-trigger.svelte`) seguem byte-idênticos. `[NV]` até a edição.
- [x] SC-002 é verificável: `apps/frontend/package.json:22` é exatamente `"@lucide/svelte": "1.33.0"`; `bun pm ls` e `npm ls @lucide/svelte lucide-svelte` mostram apenas `@lucide/svelte@1.33.0` resolvido e nenhuma entrada `lucide-svelte`. `[NV]` até a edição.
- [x] SC-003 é verificável: em `apps/frontend`, `bun run check`, `bun run lint`, `bun run test:unit -- --run` e `bun run build` terminam sem regressão (`package.json:8,11,13,15-16`), e `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS. A regressão visual permanece `[NV]`: os 13 `iconNode`s comparados são idênticos, mas os gates não detectam aparência.
- [x] SC-004 é verificável: `git diff --name-only` lista somente os 3 imports + `package.json` + `bun.lock` + `package-lock.json` + os 5 artefatos 084 (e os documentos do próprio ciclo); zero mutação de gate/CI/scripts/workflows/ADRs/STATE/specs/DB/rede/segredos/`node_modules/`; zero PRs abertos. `[NV]` até a auditoria pós-edição.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004, FR-005 e SC-001, SC-002, SC-003, SC-004 — sem intervalos (R4/R7). A rastreabilidade por T001..T005 em `plan.md` e `tasks.md` usa cada ID nominalmente: T001→FR-001/SC-001; T002→FR-002/SC-001+SC-002; T003→FR-003/SC-002; T004→FR-004/SC-002; T005→FR-005/SC-001+SC-002+SC-003+SC-004.
- [x] Evidência determinística (linhas coladas em `package.json:22,53`, `bun.lock:10,16,95,517`, `package-lock.json:19,341-348,3772-3781`, imports em `ChatLayout.svelte:10`/`SearchBox.svelte:2`/`+layout.svelte:8-19`, 13/13 `iconNode`s idênticos, `deprecation field`, registry com URL/data, base `9da1aaea6`, E=2,50) distinguida de `[NV]` para os comandos pós-edição, regressão visual, grafo final do bundler, churn real dos lockfiles, CI/PR e upgrade futuro para `@lucide/svelte@1.76.0`.

## Edge cases

- [x] E se algum dos 14 símbolos não existir no root de `@lucide/svelte@1.33.0`? `dist/icons/index.js` e `dist/aliases/aliases.js:751` (`CheckSquare`) cobrem os nomes usados; nenhum rename é previsto. Falta de nome invalidaria SC-001 e exigiria reavaliação de escopo (fora desta fronteira).
- [x] E se `bun remove`/`bun add` ou `npm install --package-lock-only` gerarem churn não relacionado? FR-004/SC-004 exigem descartar churn na revisão do `git diff` de `package-lock.json` e `bun.lock`: ausência de `node_modules/lucide-svelte`, `@lucide/svelte` em `1.33.0`, zero mudanças alheias. Qualquer entrada extra invalida o ciclo.
- [x] E se uma atualização automática subir para `1.76.0`? O pin contratual `1.33.0` foi aprovado por Main neste ciclo; o comando `bun add -D -E @lucide/svelte@1.33.0` fixa a versão exata e SC-002 exige ausência de `^`. O latest `1.76.0` (registry 2026-11-19T23:47Z) fica registrado como `[NV]`, sem upgrade não avaliado.
- [x] E se os 4 componentes UI com subpath `/icons/*` forem tocados? FR-001/FR-005/SC-001 exigem byte-idênticos; qualquer diff nesses arquivos invalida a migração, pois eles já usam o pacote novo.
- [x] E se permanecer risco de dual package no bundle? Os 13 `iconNode`s dos símbolos únicos são idênticos entre as versões instaladas; o grafo final do bundler e a aparência em browser são `[NV]` — gates não detectam aparência e nenhuma mudança de runtime é introduzida.
- [x] E se o gate mudar de PASS na reexecução? Nenhum gate é executado ou alterado nesta avaliação; SC-003 mantém PASS conforme `decision.md`, e a reexecução pelo main é `[NV]` (handoff), não correção neste ciclo.
- [x] E se a base não for `9da1aaea6`? FR-005/SC-004 exigem base preservada; qualquer diff fora dos 3 imports + 3 manifestos/locks + artefatos 084 invalida o ciclo.

## Portarias

- [x] R1 — `go (scoped)` registrado em `decision.md` (2026-11-19T23:47Z) antes da escrita dos artefatos, com E=(1×5)/(2×1)=2,50 e opção A (3 trocas de string + 3 comandos determinísticos) eleita; nenhum critério `weak`/`unknown`.
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 083→084 documentada via `--dry-run`, sem bloqueio); os `[NV]` não são perguntas bloqueantes.
- [x] R4 e R7 — rastreabilidade individual respeitada: FR-001..FR-005 e SC-001..SC-004 em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; T001..T005 rastreáveis, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo — os 4 componentes UI já migrados, `specs/084-dedupe-lucide/spec.md` (template), a subida para `1.76.0`, `prettier.config.js`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/083-*`, DB/rede, segredos, `node_modules/`, `.tokens/`, fechamento de issues, leg RBAC, PRs e worktree espelho.
- [x] `[NV]` — execução dos comandos determinísticos, regressão visual em browser, grafo final do bundler, churn real dos lockfiles, CI/PR e upgrade futuro para `@lucide/svelte@1.76.0`, separados da evidência determinística das linhas coladas, dos 13 `iconNode`s idênticos, da deprecation field, do registry consultado e da base `9da1aaea6`.
