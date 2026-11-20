# Quality Checklist: 079 — instalação pinada do CLI `shadcn-svelte`

**Purpose**: revisão de qualidade dos requisitos FR-001, FR-002, FR-003, FR-004, FR-005 e dos critérios SC-001, SC-002, SC-003, SC-004, além das portarias do ciclo 079, antes da implementação.
**Created**: 2026-11-19T20:58Z
**Feature**: [spec.md](spec.md) · [define.md](../../.specify/assessments/079-install-shadcn-svelte/define.md) · [decision.md](../../.specify/assessments/079-install-shadcn-svelte/decision.md)

> Itens gerados sem marcações; `[x]` registra revisão de qualidade de requisitos, **não** implementação.

**Review Ownership**: artefato de revisão de qualidade de requisitos, de responsabilidade do revisor. Marcar `[x]` somente quando o critério de qualidade estiver satisfeito.
**Marker Semantics**: `[x]` = critério revisado e satisfeito quanto à qualidade dos requisitos; não significa implementação concluída. A revisão de referência está em `checklists/requirements.md`.

## Requisitos

- [x] FR-001 — declaração exata de `"shadcn-svelte": "1.8.1"` (sem `^`/`~`) em `devDependencies` de `apps/frontend/package.json`, via `bun add -D -E shadcn-svelte@1.8.1` em `apps/frontend`, especificada?
- [x] FR-002 — registro da resolução em `apps/frontend/bun.lock` (entrada do pacote + 4 deps transitivas `cn`, `postcss`, `commander`, `node-fetch-native`), com lockfile rastreado e consistente, especificado?
- [x] FR-003 — preservação byte-idêntica de `$schema`, `style`, `tailwind`, `aliases` de `components.json:1-15`, de `src/routes/layout.css`, dos 51 `.svelte` de `src/lib/components/ui/` e de `src/lib/utils.ts`, com proibição de `init`/`add`/`apply`, especificada?
- [x] FR-004 — manutenção da formatação de `package.json` (tabs, sem trailing commas, aspas duplas em JSON — `prettier.config.js:5-8`) e ausência de segredos, `.tokens/`, gate/scripts/workflows e mutação de DB/rede fora do registry, especificada?
- [x] FR-005 — diff rastreado restrito a `apps/frontend/package.json` + `apps/frontend/bun.lock` (além dos 5 artefatos 079), com `node_modules/` ignorado e local, especificado?

## Critérios de sucesso

- [x] SC-001 — `bun pm ls shadcn-svelte` reporta `shadcn-svelte@1.8.1` em `apps/frontend` e `grep` em `package.json`/`bun.lock` confirma a versão exata sem caret (`[NV]` até a instalação), verificável?
- [x] SC-002 — `bun run shadcn-svelte --help` em `apps/frontend` lista `init`, `add`, `apply` e `registry build`, com bin local sem `bun x` (`[NV]` até a instalação), verificável?
- [x] SC-003 — `bun install --frozen-lockfile --dry-run` sem diff de lockfile, `bun run lint` (com `prettier --check .`) e `bun run check` sem regressão (`[NV]` até a instalação), verificável?
- [x] SC-004 — `git diff --stat` mostra só `apps/frontend/package.json` (1 linha adicionada) e `apps/frontend/bun.lock` + 5 artefatos 079, `python3 scripts/ci/check_gate_integrity.py` PASS, zero mutação de DB/segredos e `components.json`/51 componentes byte-idênticos ao base `ea5cd2069`, verificável?

## Portarias

- [x] R1 — `go (scoped)` em `decision.md` (2026-11-19T20:57Z) antes de `specify`, com E=(3×5)/(2×1)=7,50 e opção A?
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 078→079 via `--dry-run`, sem bloqueio)?
- [x] R4 e R7 — rastreabilidade individual (FR-001..FR-005 e SC-001..SC-004 em `spec.md`, `plan.md` e `tasks.md`, sem intervalos) registrada, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — `components.json`, `layout.css`, 51 componentes, `utils.ts`, `prettier.config.js`, `vite.config.ts`, scripts/CI/gate, ADRs, constituição, STATE, `specs/079-*` (exceto artefatos), DB/rede fora do registry, segredos, `.tokens/`, issues, leg RBAC, tag `next` e `init`/`add`/`apply` fora de escopo?
- [x] `[NV]` — execução da instalação e dos comandos de verificação (`bun pm ls shadcn-svelte`, `bun run shadcn-svelte --help`, `bun install --frozen-lockfile --dry-run`, gates e CI) separada da evidência determinística de registry/docs/estado local?

## Notes

- Marcar `[x]` somente após revisão confirmar o critério de qualidade de requisitos; itens em aberto permanecem com `[ ]`.
- Este `checklist.md` é gerenciado por `/speckit-checklist`; `checklists/requirements.md` tem ciclo de vida próprio, mantido por `/speckit-specify` e `/speckit-clarify`.
- Registrar achados inline; vincular à evidência em `.specify/assessments/079-install-shadcn-svelte/`.
