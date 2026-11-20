# Quality Checklist: 084 — deduplicação de dependências lucide (`lucide-svelte` → `@lucide/svelte`)

**Purpose**: revisão de qualidade dos requisitos FR-001, FR-002, FR-003, FR-004, FR-005 e dos critérios SC-001, SC-002, SC-003, SC-004, além das portarias do ciclo 084, antes da implementação.
**Created**: 2026-11-19T23:55Z
**Feature**: [spec.md](spec.md) · [define.md](../../.specify/assessments/084-dedupe-lucide/define.md) · [decision.md](../../.specify/assessments/084-dedupe-lucide/decision.md)

> Itens gerados sem marcações; `[ ]` = aguardando revisão de qualidade de requisitos, **não** implementação.

**Review Ownership**: artefato de revisão de qualidade de requisitos, de responsabilidade do revisor. Marcar `[x]` somente quando o critério de qualidade estiver satisfeito.
**Marker Semantics**: `[ ]` = critério gerado e pendente de revisão; `[x]` = revisado e satisfeito quanto à qualidade dos requisitos. Nenhum marcador indica implementação concluída. A revisão de referência está em `checklists/requirements.md`.

## Requisitos

- [x] FR-001 — troca da origem dos 3 imports de `'lucide-svelte'` para `'@lucide/svelte'`, preservando os 14 símbolos e a sintaxe de named import: `ChatLayout.svelte:10` (`Send`, `Sparkles`), `SearchBox.svelte:2` (`Search`) e `routes/(app)/+layout.svelte:8-19` (11 símbolos, `Sparkles` repetido); root de `@lucide/svelte@1.33.0` exporta todos os nomes (`dist/icons/index.js`, `dist/aliases/aliases.js:751`), sem rename, especificada?
- [x] FR-002 — remoção de `"lucide-svelte": "^1.0.1"` de `apps/frontend/package.json:53` (bloco `dependencies`) via `bun remove lucide-svelte`, com reflexo em `bun.lock:10` e `:517`, preservando `bits-ui:51`, `clsx:52` e `tailwind-merge:54`, especificada?
- [x] FR-003 — pin exato `"@lucide/svelte": "1.33.0"` (sem `^`) em `apps/frontend/package.json:22` (devDependencies) via `bun add -D -E @lucide/svelte@1.33.0`, com reflexo em `bun.lock:16` e `:95`, mantendo `"dev": true` de `package-lock.json:345`, especificada?
- [x] FR-004 — sincronização de `apps/frontend/package-lock.json` via `npm install --package-lock-only --ignore-scripts`: remover `node_modules/lucide-svelte` (`:3772-3781`), manter `node_modules/@lucide/svelte` em `1.33.0` (`:341-348`) e descartar churn não relacionado na revisão do diff, especificada?
- [x] FR-005 — diff rastreado restrito aos 3 imports + `apps/frontend/package.json` + `bun.lock` + `package-lock.json` + 5 artefatos `.specify/assessments/084-dedupe-lucide/` (e documentos do próprio ciclo), com formatação local (tabs, aspas simples em Svelte/JS, sem trailing comma — `prettier.config.js:5-7`), base `9da1aaea6` preservada e nenhuma dependência nova, segredo, DB/rede, gate, CI, ADR, spec numerada ou PR, especificada?

## Critérios de sucesso

- [x] SC-001 — `grep -rn "lucide-svelte" apps/frontend/src apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json` retorna zero ocorrências do pacote legado; os 3 imports apontam para `'@lucide/svelte'` com os mesmos 14 símbolos; os 4 componentes UI com subpath `/icons/*` seguem byte-idênticos (`[NV]` até a edição), verificável?
- [x] SC-002 — `apps/frontend/package.json:22` é exatamente `"@lucide/svelte": "1.33.0"`; `bun pm ls` / `npm ls @lucide/svelte lucide-svelte` mostram apenas `@lucide/svelte@1.33.0` resolvido e nenhuma entrada `lucide-svelte` (`[NV]` até a edição), verificável?
- [x] SC-003 — em `apps/frontend`, `bun run check`, `bun run lint`, `bun run test:unit -- --run` e `bun run build` sem regressão; `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS; regressão visual permanece `[NV]` (gates não detectam aparência), verificável?
- [x] SC-004 — `git diff --name-only` lista somente os 3 imports + `package.json` + `bun.lock` + `package-lock.json` + 5 artefatos 084 (e documentos do ciclo); zero mutação de gate/CI/scripts/workflows/ADRs/STATE/specs/DB/rede/segredos/`node_modules/`; zero PRs abertos (`[NV]` até a auditoria), verificável?

## Portarias

- [x] R1 — `go (scoped)` em `decision.md` (2026-11-19T23:47Z) antes de `specify`, com E=(1×5)/(2×1)=2,50 e opção A (3 trocas de string + 3 comandos determinísticos) eleita, sem critério `weak`/`unknown`?
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 083→084 via `--dry-run`, sem bloqueio)?
- [x] R4 e R7 — rastreabilidade individual (FR-001..FR-005 e SC-001..SC-004 em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; T001→FR-001/SC-001, T002→FR-002/SC-001+SC-002, T003→FR-003/SC-002, T004→FR-004/SC-002, T005→FR-005/SC-001+SC-002+SC-003+SC-004) registrada, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — os 4 componentes UI já migrados, `specs/084-dedupe-lucide/spec.md` (template), a subida para `1.76.0`, `prettier.config.js`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/083-*`, DB/rede, segredos, `node_modules/`, `.tokens/`, fechamento de issues, leg RBAC, PRs e worktree espelho fora de escopo?
- [x] `[NV]` — execução dos comandos determinísticos, regressão visual em browser, grafo final do bundler, churn real dos lockfiles, CI/PR e upgrade futuro para `@lucide/svelte@1.76.0` separados da evidência determinística das linhas coladas, dos 13 `iconNode`s idênticos, da deprecation field, do registry consultado e da base `9da1aaea6`?

## Notes

- Marcar `[x]` somente após revisão confirmar o critério de qualidade de requisitos; itens em aberto permanecem com `[ ]`.
- Este `checklist.md` é gerenciado por `/speckit-checklist`; `checklists/requirements.md` tem ciclo de vida próprio, mantido por `/speckit-specify` e `/speckit-clarify`.
- Registrar achados inline; vincular à evidência em `.specify/assessments/084-dedupe-lucide/`.
